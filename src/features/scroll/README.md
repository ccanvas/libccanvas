`#[cfg(feature = "scroll")]`

# ccanvas scroll

Bindings to [ccanvas-scroll](https://github.com/ccanvas/ccanvas-scroll) - scrolling text display for ccanvas.

## Quickstart

```rs
let client = Client::new(ClientConfig::default()).await;
client.subscribe(Subscription::specific_message_tag(SCROLL_READY.to_string())).await;

// get the discrim of ccanvas-scroll
let scroll_discrim = client.spawn("scroll".to_string(), "ccanvas-scroll".to_string(), Vec::new()).await.into_spawned().unwrap();

// wait for the ready message
loop {
    let event = client.recv().await;

    if let EventVariant::Message { sender, tag, .. } = event.get() {
        if tag == SCROLL_READY && sender == &scroll_discrim {
            break;
        }
    }
}

// get the handle to ccanvas-scroll
let scroll = client.scroll(scroll_discrim);

// add the lines to be displayed
scroll.push("Line 1".into()).await;
scroll.push("Line 2".into()).await;
scroll.push("Line 3".into()).await;
scroll.push("Line 4".into()).await;
```
