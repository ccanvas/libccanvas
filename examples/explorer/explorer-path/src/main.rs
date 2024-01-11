use std::path::{Path, PathBuf};

use libccanvas::{
    bindings::{Discriminator, EventVariant},
    client::{Client, ClientConfig},
};
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[tokio::main]
async fn main() {
    let mut client = Client::new(ClientConfig::default()).await;

    // do all four tasks at the same time for maximum efficiency!
    let (path, screen, _, _) = tokio::join!(
        client.get("path".to_string(), Discriminator::master()),
        client.get("path_screen".to_string(), Discriminator::master()),
        client.watch("path_screen".to_string(), Discriminator::master()),
        client.watch("path".to_string(), Discriminator::master())
    );

    // currently exploring directory path
    let mut path: PathBuf = nu_json::from_value(path.unwrap()).unwrap();
    // the dimensions this component should render in
    let mut screen: Rect = nu_json::from_value(screen.unwrap()).unwrap();

    render(&mut client, screen, &path).await;

    while let Some(event) = client.recv().await {
        match event.get() {
            // update the values - path and screen if an update is detected
            // and rerender
            EventVariant::ValueUpdated { label, new, .. } if label == "path" => {
                path = nu_json::from_value(new.clone()).unwrap();
                render(&mut client, screen, &path).await
            }
            EventVariant::ValueUpdated { label, new, .. } if label == "path_screen" => {
                screen = nu_json::from_value(new.clone()).unwrap();
                render(&mut client, screen, &path).await
            }
            _ => {}
        }
    }
}

async fn render(client: &mut Client, screen: Rect, path: &Path) {
    // simply render the path.to_str() on canvas
    // and add a `/` at the end for aesthetics
    for (x, c) in (screen.x..screen.x + screen.width).zip(format!("{}{}", path.to_str().unwrap(), if path.to_str().unwrap().ends_with('/') { "" } else { "/" }).chars().chain(std::iter::repeat(' '))) {
        client.setchar(x, screen.y, c)
    }

    client.renderall().await;
}
