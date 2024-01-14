use std::{path::PathBuf, fs};

use libccanvas::{client::{Client, ClientConfig}, bindings::{Discriminator, Colour, EventVariant}};
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
    
    // path of currently previewing path
    let mut selected: Option<PathBuf> = serde_json::from_value(client.get("selected".to_string(), Discriminator::master()).await.unwrap()).unwrap();
    // place to render to
    let mut screen: Rect = serde_json::from_value(client.get("file_screen".to_string(), Discriminator::master()).await.unwrap()).unwrap();

    client.watch("selected".to_string(), Discriminator::master()).await;
    client.watch("file_screen".to_string(), Discriminator::master()).await;
    render(&mut client, &selected, screen).await;

    // no explainations needed
    loop {
        let event = client.recv().await;
        match event.get() {
            EventVariant::ValueUpdated { label, new, .. } if label == "selected" => {
                selected = serde_json::from_value(new.clone()).unwrap();
                render(&mut client, &selected, screen).await;
            }
            EventVariant::ValueUpdated { label, new, .. } if label == "file_screen" => {
                screen = serde_json::from_value(new.clone()).unwrap();
                render(&mut client, &selected, screen).await;
            }
            _ => {}
        }
    }
}

// clear the entire rect, obviously
fn clear(client: &mut Client, screen: Rect) {
    for y in screen.y..screen.y + screen.height {
        for x in screen.x..screen.x + screen.width {
            client.setcharcoloured(x, y, ' ', Colour::Reset, Colour::Reset)
        }
    }
}

// render stuff
async fn render(client: &mut Client, selected: &Option<PathBuf>, screen: Rect) {
    clear(client, screen);

    // if nothing is selected, e.g. in an empty directory, this happens
    let selected = if let Some(path) = selected {
        path
    } else {
        for (x, c) in (screen.x..screen.x + screen.width).zip("Nothing to show".chars()) {
            client.setchar(x, screen.y, c);
        }
        client.renderall().await;
        return;
    };

    // if selected is a dir, show number of items
    if selected.is_dir() {
        let entries = match fs::read_dir(selected) {
            Ok(entries) => entries.into_iter().count(),
            Err(_) => 0
        };

        for (x, c) in (screen.x..screen.x + screen.width).zip(format!("Directory, {entries} items").chars()) {
            client.setchar(x, screen.y, c);
        }
        client.renderall().await;
        return;
    }

    // try to read the file
    let bytes = if let Ok(bytes) = fs::read(selected) {
        bytes
    } else {
        for (x, c) in (screen.x..screen.x + screen.width).zip("Could not read file".chars()) {
            client.setchar(x, screen.y, c);
        }
        client.renderall().await;
        return;
    };

    // try to decode the file
    let text = if let Ok(text) = String::from_utf8(bytes) {
        text
    } else {
        for (x, c) in (screen.x..screen.x + screen.width).zip("Binary content".chars()) {
            client.setchar(x, screen.y, c);
        }
        client.renderall().await;
        return;
    };

    // render the file line by line
    // discard all overflows
    for (y, line) in (screen.y..screen.y + screen.height).zip(text.lines()) {
        for (x, c) in (screen.x..screen.x + screen.width).zip(line.chars()) {
            client.setchar(x, y, c);
        }
    }

    client.renderall().await;
}
