`#[cfg(feature = "saver")]`

# ccanvas saver

Bindings to [`ccanvas-saver`](https://github.com/ccanvas/ccanvas-saver), display an overlay when the terminal dimensions are too small.

## Quickstart

```rs
let client = Client::new(ClientConfig::default()).await;

// load in and enable saver to limit terminal dimensions to be above 40 x 20
tokio::join!(
    client.enable_saver(40, 20),
    client.spawn("saver".to_string(), "ccanvas-saver".to_string(), Vec::new()),
);
```

## Behaviour

When screen is smaller than the specified dimension, ccanvas-saver will:
- **Display an overlay** to the screen.
- **Suppress all events** to other components.

All subscriptions with a lower priority than 10 will not receive any input events. However, non-input events such as messages, value updates will still pass through.

The following line returns if the screen saver is on.

```rs
let is_on = client.saver_ison().await;
```

Or watch changes of the value instead.

```rs
client.watch(SAVER_ISON_LABEL.to_string(), Discriminator::master()).await;
```
