use std::{
    collections::HashMap,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    sync::Arc,
};

use nu_json::Value;
use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot, Mutex, OnceCell,
    },
    task::JoinHandle,
};

use crate::bindings::{
    Colour, CursorStyle, Discriminator, Event, RenderRequest, Request, RequestContent, Response,
    ResponseContent, ResponseSuccess, Subscription, StateValue,
};

use super::ClientConfig;

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
    render_requests: Vec<RenderRequest>,
    /// confirmation handles for requests
    req_confirms: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseContent>>>>,
}

static mut REQID: OnceCell<u32> = OnceCell::const_new_with(0);

impl Client {
    pub async fn new(config: ClientConfig) -> Self {
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
        req_confirms.lock().await.insert(set_socket.id(), set_socket_sender);
        UnixStream::connect(&config.request_socket)
            .unwrap()
            .write_all(nu_json::to_vec(&set_socket).unwrap().as_slice())
            .unwrap();

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

                    let res: Response = match nu_json::from_str(&msg) {
                        Ok(res) => res,
                        Err(_) => continue,
                    };

                    match res.content {
                        // events have to be confirmed
                        ResponseContent::Event { content } => {
                            inbound_send
                                .send(Event::new(content, outbound_send.clone(), res.id))
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
                            .write_all(nu_json::to_vec(&req).unwrap().as_slice())
                            .unwrap();
                    });
                }
            })
        };

        let discrim = if let ResponseContent::Success { content: ResponseSuccess::ListenerSet { discrim } } = set_socket_res.await.unwrap() {
            discrim
        } else {
            panic!("no")
        };

        Self {
            listener_handle,
            request_handle,
            inbound_recv: Arc::new(Mutex::new(inbound_recv)),
            outbound_send,
            request_socket: config.request_socket,
            render_requests: Vec::new(),
            req_confirms,
            discrim
        }
    }

    /// get a unique request id
    pub fn reqid() -> u32 {
        let id = unsafe { REQID.get_mut() }.unwrap();
        *id += 1;
        *id
    }

    /// there should only be one recv() per program
    /// more than one recv() at a time results in almost randomised behaviour
    pub async fn recv(&self) -> Option<Event> {
        self.inbound_recv.lock().await.recv().await
    }

    /// send a request
    /// private method as the convenience functions should be used instead
    async fn send(&self, req: Request) -> ResponseContent {
        let (tx, rx) = oneshot::channel();
        self.req_confirms.lock().await.insert(req.id(), tx);
        self.outbound_send.send(req).unwrap();
        rx.await.unwrap()
    }
}

/// convenience functions
impl Client {
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

    pub async fn exit(&self) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Drop {
                discrim: Some(Discriminator::new(vec![1])),
            },
        );
        self.send(req).await
    }

    pub fn setchar(&mut self, x: u32, y: u32, c: char) {
        self.render_requests.push(RenderRequest::setchar(x, y, c))
    }

    pub fn setcharcoloured(&mut self, x: u32, y: u32, c: char, fg: Colour, bg: Colour) {
        self.render_requests
            .push(RenderRequest::setchar_coloured(x, y, c, fg, bg))
    }

    pub fn setcursorstyle(&mut self, style: CursorStyle) {
        self.render_requests.push(RenderRequest::setcursor(style))
    }

    pub fn showcursor(&mut self) {
        self.render_requests.push(RenderRequest::ShowCursor)
    }

    pub fn hidecursor(&mut self) {
        self.render_requests.push(RenderRequest::HideCursor)
    }

    pub fn clear_all(&mut self) {
        self.render_requests.push(RenderRequest::ClearAll)
    }

    pub async fn renderall(&mut self) -> ResponseContent {
        if self.render_requests.is_empty() {
            return ResponseContent::Success {
                content: ResponseSuccess::Rendered,
            };
        }

        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                flush: true,
                content: RenderRequest::RenderMultiple {
                    tasks: std::mem::take(&mut self.render_requests),
                },
            },
        );
        self.send(req).await
    }

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
            },
        );
        self.send(req).await
    }

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
            },
        );
        self.send(req).await
    }

    pub async fn focus_at(&self, discrim: Discriminator) -> ResponseContent {
        let req = Request::new(discrim, RequestContent::FocusAt);
        self.send(req).await
    }

    pub async fn new_space(&self, parent: Discriminator, label: String) -> ResponseContent {
        let req = Request::new(parent, RequestContent::NewSpace { label });
        self.send(req).await
    }

    pub async fn message(&self, target: Discriminator, content: String) -> ResponseContent {
        let req = Request::new(
            target.clone(),
            RequestContent::Message {
                content,
                sender: Discriminator::default(),
                target,
            },
        );
        self.send(req).await
    }

    pub async fn broadcast(&self, content: String) -> ResponseContent {
        let req = Request::new(
            Discriminator::master(),
            RequestContent::Message {
                content,
                sender: Discriminator::default(),
                target: Discriminator::master(),
            },
        );
        self.send(req).await
    }

    pub fn discrim(&self) -> &Discriminator {
        &self.discrim
    }

    pub async fn is_focused(&self) -> bool {
        let req = Request::new(self.discrim.clone(), RequestContent::GetState { label: StateValue::IsFocused });
        if let ResponseContent::Success { content: ResponseSuccess::Value { value: Value::Bool(b) } } = self.send(req).await {
            b
        } else {
            // this isnt supposed to happen
            false
        }
    }

    pub async fn focused(&self) -> Discriminator {
        let req = Request::new(self.discrim.clone(), RequestContent::GetState { label: StateValue::IsFocused });
        if let ResponseContent::Success { content: ResponseSuccess::Value { value } } = self.send(req).await {
            nu_json::from_value(value).unwrap()
        } else {
            // this isnt supposed to happen
            Discriminator::master()
        }
    }

    pub async fn term_size(&self) -> (u32, u32) {
        let req = Request::new(self.discrim.clone(), RequestContent::GetState { label: StateValue::TermSize });
        let res = self.send(req).await;
        if let ResponseContent::Success { content: ResponseSuccess::Value { value } } = res {
            #[derive(serde::Deserialize)]
            struct TermSize {
                x: u32,
                y: u32
            }

            let termsize: TermSize = nu_json::from_value(value).unwrap();
            (termsize.x, termsize.y)
        } else {
            // this isnt supposed to happen
            (0, 0)
        }
    }

    pub async fn current_directory(&self) -> PathBuf {
        let req = Request::new(self.discrim.clone(), RequestContent::GetState { label: StateValue::WorkingDir });
        let res = self.send(req).await;
        if let ResponseContent::Success { content: ResponseSuccess::Value { value } } = res {
            nu_json::from_value(value).unwrap()
        } else {
            PathBuf::new()
        }
    }

    pub async fn watch(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(target, RequestContent::Watch { label });
        self.send(req).await
    }

    pub async fn unwatch(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(target, RequestContent::Unwatch { label, watcher: self.discrim.clone() });
        self.send(req).await
    }

    pub async fn watch_self(&self, label: String) -> ResponseContent {
        self.watch(label, self.discrim.clone()).await
    }

    pub async fn unwatch_self(&self, label: String) -> ResponseContent {
        self.unwatch(label, self.discrim.clone()).await
    }

    pub async fn set(&self, label: String, target: Discriminator, value: Value) -> ResponseContent {
        let req = Request::new(target, RequestContent::SetEntry { label, value });
        self.send(req).await
    }

    pub async fn set_self(&self, label: String, value: Value) -> ResponseContent {
        self.set(label, self.discrim.clone(), value).await
    }

    pub async fn get_raw(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(target, RequestContent::GetEntry { label });
        self.send(req).await
    }

    pub async fn get(&self, label: String, target: Discriminator) -> Option<Value> {
        match self.get_raw(label, target).await {
            ResponseContent::Success { content: ResponseSuccess::Value { value } } => Some(value),
            _ => None
        }
    }

    pub async fn get_self(&self, label: String) -> Option<Value> {
        self.get(label, self.discrim.clone()).await
    }

    pub async fn remove(&self, label: String, target: Discriminator) -> ResponseContent {
        let req = Request::new(target, RequestContent::RemoveEntry { label });
        self.send(req).await
    }

    pub async fn remove_self(&self, label: String) -> ResponseContent {
        self.remove(label, self.discrim.clone()).await
    }
}

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
            .write_all(nu_json::to_vec(&req).unwrap().as_slice())
            .unwrap();
    }
}
