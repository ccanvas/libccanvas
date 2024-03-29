use libccanvas::{
    bindings::{Colour, EventVariant, Subscription},
    client::{Client, ClientConfig},
};

#[tokio::main]
async fn main() {
    let client = Client::new(ClientConfig::default()).await;
    // listen to all messages - including broadcasts from snake-main
    client.subscribe(Subscription::specific_message_tag("score".to_string())).await;

    // draws "Score: 0" in canvas
    client.setchar(0, 0, 'S');
    client.setchar(1, 0, 'c');
    client.setchar(2, 0, 'o');
    client.setchar(3, 0, 'r');
    client.setchar(4, 0, 'e');
    client.setchar(5, 0, ':');
    client.setcharcoloured(7, 0, '0', Colour::Red, Colour::Reset);
    client.renderall().await;

    loop {
        let event = client.recv().await;
        if let EventVariant::Message { content, .. } = event.get() {
            let content: u32 = serde_json::from_value(content.clone()).unwrap();

            for (x, c) in content.to_string().chars().enumerate() {
                // draws the scroe in canvas
                client.setcharcoloured(7 + x as u32, 0, c, Colour::LightRed, Colour::Reset);
                client.renderall().await;
            }
        }
    }
}
