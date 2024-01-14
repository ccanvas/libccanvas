use libccanvas::{
    bindings::{EventVariant, KeyCode, Subscription},
    client::{Client, ClientConfig},
};

#[tokio::main]
async fn main() {
    let client = Client::new(ClientConfig::default()).await;
    // spawns in the 2 processes
    client
        .spawn(
            "snake-main".to_string(),
            "ccanvas-snake-main".to_string(),
            Vec::new(),
        )
        .await;
    client
        .spawn(
            "snake-scoreboard".to_string(),
            "ccanvas-snake-scoreboard".to_string(),
            Vec::new(),
        )
        .await;

    client
        .subscribe(Subscription::specific_keycode(KeyCode::Char('q')))
        .await;

    loop {
        let event = client.recv().await;
    // and listens for 'q' to exit the canvas
        if let EventVariant::Key(key) = event.get() {
            if key.code == KeyCode::Char('q') {
                client.exit().await;
                break;
            }
        }
    }
}
