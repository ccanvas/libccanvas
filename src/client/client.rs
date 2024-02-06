use std::{
    collections::{BTreeMap, HashMap},
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    process,
    sync::Arc,
};

use serde_json::Value;
use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot, Mutex, OnceCell,
    },
    task::JoinHandle,
};

use crate::bindings::{
    Colour, CursorStyle, Discriminator, Event, RenderRequest, Request, RequestContent, Response,
    ResponseContent, ResponseSuccess, StateValue, Subscription,
};

use super::{ClientConfig, LifetimeSuppressor};

/// Handles all interactions between the ccanvas server and your code.
pub struct Client {
    /// task handle to the listener loop
    listener_handle: JoinHandle<()>,
    /// task handle for sender loop
    request_handle: JoinHandle<()>,
    /// incoming events
    inbound_recv: Arc<Mutex<UnboundedReceiver<Event>>>,
    /// request to ccanvas
    outbound_send: UnboundedSender<Request>,

    /// self discrim
    discrim: Discriminator,

    /// path to request socket
    request_socket: PathBuf,
    /// unflushed render requests
    render_requests: std::sync::Mutex<Vec<RenderRequest>>,
    /// confirmation handles for requests
    req_confirms: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseContent>>>>,
}

static REQID: OnceCell<std::sync::Mutex<u32>> = OnceCell::const_new_with(std::sync::Mutex::new(0));

impl Client {
    /// Create a new instance of self, will panic if connection fails.
    pub async fn new(config: ClientConfig) -> Self {
        if !std::env::var("CCANVAS_COMPONENT").is_ok_and(|val| val.as_str() == "1") {
            println!("Compontent not ran from by ccanvas, please run with CCANVAS_COMPONENT=1 if it is what you want.");
            process::exit(-1);
        }

        if !config.request_socket.exists() {
            panic!("request socket not found, a component is not to be executed outside context of a canvas");
        }
        // creates the listener socket
        let listener = UnixListener::bind(&config.listener_socket).unwrap();

        let (inbound_send, inbound_recv) = mpsc::unbounded_channel();
        let (outbound_send, mut outbound_recv) = mpsc::unbounded_channel();
        let req_confirms: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseContent>>>> =
            Arc::new(Mutex::new(HashMap::default()));

        // connects and set the listener
        let set_socket = Request::new(
            Discriminator::default(),
            RequestContent::SetSocket {
                path: config.listener_socket,
            },
        );

        let (set_socket_sender, set_socket_res) = oneshot::channel();
        req_confirms
            .lock()
            .await
            .insert(set_socket.id(), set_socket_sender);
        UnixStream::connect(&config.request_socket)
            .unwrap()
            .write_all(serde_json::to_vec(&set_socket).unwrap().as_slice())
            .unwrap();

        #[cfg(feature = "layout")]
        static SELF_DISCRIM: OnceCell<Discriminator> = OnceCell::const_new();

        let listener_handle = {
            let outbound_send = outbound_send.clone();
            let req_confirms = req_confirms.clone();
            tokio::task::spawn_blocking(move || {
                let outbound_send = outbound_send.clone();
                for stream in listener.incoming() {
                    let mut stream = match stream {
                        Ok(stream) => stream,
                        Err(_) => continue,
                    };

                    let mut msg = String::new();
                    match stream.read_to_string(&mut msg) {
                        Ok(_) => {}
                        Err(_) => continue,
                    }

                    let res: Response = match serde_json::from_str(&msg) {
                        Ok(res) => res,
                        Err(_) => continue,
                    };

                    match res.content {
                        // events have to be confirmed
                        ResponseContent::Event { content } => {
                            #[cfg(not(feature = "layout"))]
                            inbound_send
                                .send(Event::new(content, outbound_send.clone(), res.id))
                                .unwrap();
                            #[cfg(feature = "layout")]
                            inbound_send
                                .send(Event::new(
                                    content,
                                    outbound_send.clone(),
                                    res.id,
                                    SELF_DISCRIM
                                        .get()
                                        .map(Discriminator::clone)
                                        .unwrap_or_else(Discriminator::master),
                                ))
                                .unwrap();
                        }
                        // these are responses from canvas
                        // and dont have to be confirmed
                        // but their wait locks have to be released
                        // so the callers can know the task is done
                        ResponseContent::Error { .. }
                        | ResponseContent::Success { .. }
                        | ResponseContent::Undelivered => {
                            if let Some(entry) = tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(req_confirms.lock())
                                .remove(&res.request.unwrap())
                            {
                                let _ = entry.send(res.content);
                            }
                        }
                    }
                }
            })
        };

        let request_handle = {
            let request_socket = config.request_socket.clone();
            // simply sends Request to canvas
            tokio::task::spawn(async move {
                while let Some(req) = outbound_recv.recv().await {
                    let request_socket = request_socket.clone();
                    tokio::task::spawn_blocking(move || {
                        UnixStream::connect(request_socket)
                            .unwrap()
                            .write_all(serde_json::to_vec(&req).unwrap().as_slice())
                            .unwrap();
                    });
                }
            })
        };

        let discrim = if let ResponseContent::Success {
            content: ResponseSuccess::ListenerSet { discrim },
        } = set_socket_res.await.unwrap()
        {
            discrim
        } else {
            panic!("no")
        };

        #[cfg(feature = "layout")]
        SELF_DISCRIM.set(discrim.clone()).unwrap();

        Self {
            listener_handle,
            request_handle,
            inbound_recv: Arc::new(Mutex::new(inbound_recv)),
            outbound_send,
            request_socket: config.request_socket,
            render_requests: std::sync::Mutex::new(Vec::new()),
            req_confirms,
            discrim,
        }
    }

    /// Generates a never-before-seen unique request ID.
    pub fn reqid() -> u32 {
        let mut id = REQID.get().unwrap().lock().unwrap();
        *id += 1;
        *id
    }

    /// Waits for an event from ccanvas.
    ///
    /// There should only be one active `recv()` for each Client,
    /// more than one `recv()` at a time leads to undetermined behaviour on who gets the event.
    pub async fn recv(&self) -> Event {
        self.inbound_recv.lock().await.recv().await.unwrap()
    }

    /// Send a request and waits for response
    /// This is a private method as the task specific functions should be used instead.
    pub async fn send(&self, req: Request) -> ResponseContent {
        let (tx, rx) = oneshot::channel();
        self.req_confirms.lock().await.insert(req.id(), tx);
        self.outbound_send.send(req).unwrap();
        rx.await.unwrap()
    }
}

/// Task specific functions.
impl Client {
    /// Subscribe to one channel.
    pub async fn subscribe<T: Into<(Subscription, Option<u32>)>>(
        &self,
        channel: T,
    ) -> ResponseContent {
        let (channel, priority) = channel.into();
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Subscribe {
                channel,
                priority,
                component: None,
            },
        );
        self.send(req).await
    }

    /// Subscribe to multiple channels at once.
    pub async fn subscribe_multiple<T: Into<(Subscription, Option<u32>)>>(
        &self,
        channels: Vec<T>,
    ) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Subscribe {
                channel: Subscription::Multiple {
                    subs: channels.into_iter().map(|item| item.into()).collect(),
                },
                priority: None,
                component: None,
            },
        );
        self.send(req).await
    }

    /// Unsubscribe from one channels.
    pub async fn unsubscribe(&self, channel: Subscription) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Unsubscribe {
                channel,
                component: None,
            },
        );
        self.send(req).await
    }

    /// Tell the ccanvas server to exit immetiately.
    ///
    /// Alias to `self.drop_component(Discriminator::master())`
    pub async fn exit(&self) -> ResponseContent {
        self.drop_component(Discriminator::master()).await
    }

    /// Drop a single component.
    pub async fn drop_component(&self, discrim: Discriminator) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Drop {
                discrim: Some(discrim),
            },
        );
        self.send(req).await
    }

    /// Add a set character task to the render queue.
    pub fn setchar(&self, x: u32, y: u32, c: char) {
        self.render_requests
            .lock()
            .unwrap()
            .push(RenderRequest::setchar(x, y, c))
    }

    /// Add a set character (coloured) task to the render queue.
    pub fn setcharcoloured(&self, x: u32, y: u32, c: char, fg: Colour, bg: Colour) {
        self.render_requests
            .lock()
            .unwrap()
            .push(RenderRequest::setchar_coloured(x, y, c, fg, bg))
    }

    /// Add a set cursor style task to render queue.
    pub fn setcursorstyle(&self, style: CursorStyle) {
        self.render_requests
            .lock()
            .unwrap()
            .push(RenderRequest::setcursor(style))
    }

    /// Add a show cursor task to render queue.
    pub fn showcursor(&self) {
        self.render_requests
            .lock()
            .unwrap()
            .push(RenderRequest::ShowCursor)
    }

    /// Add a hide cursor task to render queue.
    pub fn hidecursor(&self) {
        self.render_requests
            .lock()
            .unwrap()
            .push(RenderRequest::HideCursor)
    }

    /// Add a clear all task to render queue.
    pub fn clear_all(&self) {
        self.render_requests
            .lock()
            .unwrap()
            .push(RenderRequest::ClearAll)
    }

    /// Flush and complete all tasks in render queue.
    pub async fn renderall(&self) -> ResponseContent {
        let tasks = {
            let mut render_requests = self.render_requests.lock().unwrap();
            if render_requests.is_empty() {
                return ResponseContent::Success {
                    content: ResponseSuccess::Rendered,
                };
            }
            std::mem::take(&mut *render_requests)
        };

        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                flush: true,
                content: RenderRequest::RenderMultiple { tasks },
            },
        );
        self.send(req).await
    }

    /// Spawn a new process at a specific space.
    pub async fn spawn_at(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
        parent: Discriminator,
    ) -> ResponseContent {
        let req = Request::new(
            parent,
            RequestContent::Spawn {
                command,
                args,
                label,
                env: BTreeMap::new(),
            },
        );
        self.send(req).await
    }

    /// Spawn a new process at a specific space with environment vars
    pub async fn spawn_with_env_at(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
        parent: Discriminator,
        env: BTreeMap<String, String>,
    ) -> ResponseContent {
        let req = Request::new(
            parent,
            RequestContent::Spawn {
                command,
                args,
                label,
                env,
            },
        );
        self.send(req).await
    }

    /// Spawn a new process in the same parent space.
    pub async fn spawn(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
    ) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Spawn {
                command,
                args,
                label,
                env: BTreeMap::new(),
            },
        );
        self.send(req).await
    }

    /// Spawn a new process in the same parent space.
    pub async fn spawn_with_env(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
        env: BTreeMap<String, String>,
    ) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Spawn {
                command,
                args,
                label,
                env,
            },
        );
        self.send(req).await
    }

    /// Set focus to a space.
    pub async fn focus_at(&self, discrim: Discriminator) -> ResponseContent {
        let req = Request::new(discrim, RequestContent::FocusAt);
        self.send(req).await
    }

    /// Create a new space.
    pub async fn new_space(&self, parent: Discriminator, label: String) -> ResponseContent {
        let req = Request::new(parent, RequestContent::NewSpace { label });
        self.send(req).await
    }

    /// Send a message to a component.
    ///
    /// If the selected component is a space, then all its members (including subspaces) will also
    /// recieve the message. Including the sender copmonent.
    pub async fn message(
        &self,
        target: Discriminator,
        content: Value,
        tag: String,
    ) -> ResponseContent {
        let req = Request::new(
            target.clone(),
            RequestContent::Message {
                content,
                sender: Discriminator::default(),
                target,
                tag,
            },
        );
        self.send(req).await
    }

    /// Send a message to all components, including self.
    ///
    /// Alias to `client.message(Discriminator::master(), message)`
    pub async fn broadcast(&self, content: Value, tag: String) -> ResponseContent {
        let req = Request::new(
            Discriminator::master(),
            RequestContent::Message {
                content,
                sender: Discriminator::default(),
                target: Discriminator::master(),
                tag,
            },
        );
        self.send(req).await
    }

    /// Get the discriminator of the current (process) component.
    pub fn discrim(&self) -> &Discriminator {
        &self.discrim
    }

    /// Check if parent space is in focus.
    ///
    /// Returns true if it is the focused element, or child of the focused element.
    pub async fn is_focused(&self) -> bool {
        let req = Request::new(
            self.discrim.clone(),
            RequestContent::GetState {
                label: StateValue::IsFocused,
            },
        );
        if let ResponseContent::Success {
            content: ResponseSuccess::Value {
                value: Value::Bool(b),
            },
        } = self.send(req).await
        {
            b
        } else {
            // this isnt supposed to happen
            false
        }
    }

    /// Returns discriminator of the currently focused component.
    pub async fn focused(&self) -> Discriminator {
        let req = Request::new(
            self.discrim.clone(),
            RequestContent::GetState {
                label: StateValue::IsFocused,
            },
        );
        if let ResponseContent::Success {
            content: ResponseSuccess::Value { value },
        } = self.send(req).await
        {
            serde_json::from_value(value).unwrap()
        } else {
            // this isnt supposed to happen
            Discriminator::master()
        }
    }

    /// Returns terminal size (x, y)
    pub async fn term_size(&self) -> (u32, u32) {
        let req = Request::new(
            self.discrim.clone(),
            RequestContent::GetState {
                label: StateValue::TermSize,
            },
        );
        let res = self.send(req).await;
        if let ResponseContent::Success {
            content: ResponseSuccess::Value { value },
        } = res
        {
            #[derive(serde::Deserialize)]
            struct TermSize {
                x: u32,
                y: u32,
            }

            let termsize: TermSize = serde_json::from_value(value).unwrap();
            (termsize.x, termsize.y)
        } else {
            // this isnt supposed to happen
            (0, 0)
        }
    }

    /// Returns the full path to which the `ccanvas` command is ran.
    pub async fn current_directory(&self) -> PathBuf {
        let req = Request::new(
            self.discrim.clone(),
            RequestContent::GetState {
                label: StateValue::WorkingDir,
            },
        );
        let res = self.send(req).await;
        if let ResponseContent::Success {
            content: ResponseSuccess::Value { value },
        } = res
        {
            serde_json::from_value(value).unwrap()
        } else {
            PathBuf::new()
        }
    }

    /// Subscribe to changes of a variable.
    pub async fn watch(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(target, RequestContent::Watch { label });
        self.send(req).await
    }

    /// Remove subscription to changes of a variable.
    pub async fn unwatch(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(
            target,
            RequestContent::Unwatch {
                label,
                watcher: self.discrim.clone(),
            },
        );
        self.send(req).await
    }

    /// Subscribe to changes of a variable owned by the current (process) component.
    pub async fn watch_self(&self, label: String) -> ResponseContent {
        self.watch(label, self.discrim.clone()).await
    }

    /// Remove subscription to changes of a variable owned by the current (process) component.
    pub async fn unwatch_self(&self, label: String) -> ResponseContent {
        self.unwatch(label, self.discrim.clone()).await
    }

    /// Set a variable.
    pub async fn set(&self, label: String, target: Discriminator, value: Value) -> ResponseContent {
        let req = Request::new(target, RequestContent::SetEntry { label, value });
        self.send(req).await
    }

    /// Set a variable owned by the current (process) component.
    pub async fn set_self(&self, label: String, value: Value) -> ResponseContent {
        self.set(label, self.discrim.clone(), value).await
    }

    /// Get a variable with raw responses.
    pub async fn get_raw(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(target, RequestContent::GetEntry { label });
        self.send(req).await
    }

    /// Get a variable, returning an `Option<Value>`.
    pub async fn get(&self, label: String, target: Discriminator) -> Option<Value> {
        match self.get_raw(label, target).await {
            ResponseContent::Success {
                content: ResponseSuccess::Value { value },
            } => Some(value),
            _ => None,
        }
    }

    /// Get a variable owned by the current (process) component, returning an `Option<Value>`.
    pub async fn get_self(&self, label: String) -> Option<Value> {
        self.get(label, self.discrim.clone()).await
    }

    /// Remove a variable.
    pub async fn remove(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(target, RequestContent::RemoveEntry { label });
        self.send(req).await
    }

    /// Remove a variable owned by the current (process) component.
    pub async fn remove_self(&self, label: String) -> ResponseContent {
        self.remove(label, self.discrim.clone()).await
    }

    /// Suppress all subscriptions of that channel with a lower than specified priority
    pub async fn suppress_at(
        &self,
        channel: Subscription,
        priority: u32,
        target: Discriminator,
    ) -> ResponseContent {
        let req = Request::new(target, RequestContent::Suppress { channel, priority });
        self.send(req).await
    }

    /// Remove suppress for a single suppressor
    pub async fn unsuppress_at(
        &self,
        channel: Subscription,
        id: u32,
        target: Discriminator,
    ) -> ResponseContent {
        let req = Request::new(target, RequestContent::Unsuppress { channel, id });
        self.send(req).await
    }

    /// Suppress a channel within the lifetime of the returned LifetimeSuppressor
    pub async fn suppress(
        &self,
        channel: Subscription,
        priority: u32,
        target: Discriminator,
    ) -> Option<LifetimeSuppressor> {
        if let ResponseContent::Success {
            content: ResponseSuccess::Suppressed { id },
        } = self
            .suppress_at(channel.clone(), priority, target.clone())
            .await
        {
            Some(LifetimeSuppressor::new(
                id,
                channel,
                target,
                self.outbound_send.clone(),
            ))
        } else {
            None
        }
    }

    /// Remove suppress using a lifetime suppressor.
    pub async fn unsuppress(&self, suppressor: LifetimeSuppressor) -> bool {
        if let Some((id, channel, target, _)) = suppressor.deconstruct() {
            self.unsuppress_at(channel, id, target).await;
            true
        } else {
            false
        }
    }
}

/// Let the ccanvas server know when the client gets dropped.
impl Drop for Client {
    fn drop(&mut self) {
        self.listener_handle.abort();
        self.request_handle.abort();
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Drop { discrim: None },
        );
        UnixStream::connect(self.request_socket.clone())
            .unwrap()
            .write_all(serde_json::to_vec(&req).unwrap().as_slice())
            .unwrap();
    }
}
