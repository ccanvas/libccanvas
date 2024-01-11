use std::{
    fs,
    path::{Path, PathBuf}, ops::Add,
};

use libccanvas::{
    bindings::{Colour, Discriminator, EventVariant, Subscription, KeyCode},
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

    // subscribe to stuff
    // notice that priority is not specified
    // the events will pass through explorer-manager first, before reaching here
    // so they have the option to capture it if they wanted to
    client
        .subscribe_multiple(vec![
            Subscription::specific_keycode(KeyCode::Up),
            Subscription::specific_keycode(KeyCode::Down),
            Subscription::specific_keycode(KeyCode::Left),
            Subscription::specific_keycode(KeyCode::Right),
            Subscription::specific_keycode(KeyCode::Char('\n')),
            Subscription::specific_keycode(KeyCode::Backspace),
        ])
        .await;

    let (screen, path) = tokio::join!(
        client.get("dir_screen".to_string(), Discriminator::master()),
        client.get("path".to_string(), Discriminator::master()),
    );

    // where the component should render to
    let mut screen: Rect = nu_json::from_value(screen.unwrap()).unwrap();
    // currently exploring directory path
    let mut path: PathBuf = nu_json::from_value(path.unwrap()).unwrap();

    // items in the current directory
    let mut items = dir_items(&path).unwrap();
    // index of the currently selected item (from items)
    let mut selected = 0;
    // as the name suggests, how many rows did it scroll
    let mut scroll = 0;
    // path history, backspace to go back
    let mut history = Vec::new();
    // the current event is triggered by a backspace
    let mut is_backspace = false;

    // set currently selected file path so explorer-file would know
    client.set("selected".to_string(), Discriminator::master(), nu_json::to_value(&items.get(selected)).unwrap()).await;

    // NOW, finally we can spawn in the file preview component
    client.spawn("file".to_string(), "ccanvas-explorer-file".to_string(), Vec::new()).await;

    render(&mut client, &items, selected, scroll, screen).await;

    client
        .watch("dir_screen".to_string(), Discriminator::master())
        .await;
    client
        .watch("path".to_string(), Discriminator::master())
        .await;

    while let Some(event) = client.recv().await {
        match event.get() {
            // rerender, nothing special
            EventVariant::ValueUpdated { label, new, .. } if label == "dir_screen" => {
                screen = nu_json::from_value(new.clone()).unwrap();
                update_scroll(&mut scroll, selected, screen);
                render(&mut client, &items, selected, scroll, screen).await;
            }
            // rerender and push to history, update state
            EventVariant::ValueUpdated { label, new, .. } if label == "path" => {
                if !is_backspace {
                    history.push(path);
                } else {
                    is_backspace = false;
                }
                path = nu_json::from_value(new.clone()).unwrap();
                items = match dir_items(&path) {
                    Some(items) => items,
                    None => {
                        client.set("path".to_string(), Discriminator::master(), nu_json::to_value(&history.pop().unwrap()).unwrap()).await;
                        is_backspace = true;
                        continue;
                    }
                };
                selected = 0;
                scroll = 0;
                render(&mut client, &items, selected, scroll, screen).await;
                client.set("selected".to_string(), Discriminator::master(), nu_json::to_value(&items.get(selected)).unwrap()).await;
            }
            // just doing stuff with keys, nothing all that special really
            EventVariant::Key(key) => {
                match key.code {
                    KeyCode::Up => selected = selected.saturating_sub(1),
                    KeyCode::Down => selected = selected.add(1).min(items.len().saturating_sub(1)),
                    KeyCode::Right | KeyCode::Char('\n') => if let Some(child) = items.get(selected) {
                        if child.is_dir() && fs::read_dir(child).is_ok() {
                            client.set("path".to_string(), Discriminator::master(), nu_json::to_value(child).unwrap()).await;
                            client.set("selected".to_string(), Discriminator::master(), nu_json::to_value(&items.get(selected)).unwrap()).await;
                        }
                        continue;
                    } else {
                        continue;
                    }
                    KeyCode::Backspace if !history.is_empty() => {
                            client.set("path".to_string(), Discriminator::master(), nu_json::to_value(&history.pop().unwrap()).unwrap()).await;
                            client.set("selected".to_string(), Discriminator::master(), nu_json::to_value(&items.get(selected)).unwrap()).await;
                            is_backspace = true;
                            continue;
                    }
                    KeyCode::Left => if let Some(parent) = path.parent() {
                        if fs::read_dir(parent).is_ok() {
                            client.set("path".to_string(), Discriminator::master(), nu_json::to_value(parent).unwrap()).await;
                            client.set("selected".to_string(), Discriminator::master(), nu_json::to_value(&items.get(selected)).unwrap()).await;
                        }
                        continue;
                    } else {
                        continue;
                    }
                    _ => continue
                }

                // rerender!
                client.set("selected".to_string(), Discriminator::master(), nu_json::to_value(&items.get(selected)).unwrap()).await;
                update_scroll(&mut scroll, selected, screen);
                render(&mut client, &items, selected, scroll, screen).await;
            }
            _ => {}
        }
    }
}

// clear everything in a rect
fn clear(client: &mut Client, screen: Rect) {
    for y in screen.y..screen.y + screen.height {
        for x in screen.x..screen.x + screen.width {
            client.setcharcoloured(x, y, ' ', Colour::Reset, Colour::Reset)
        }
    }
}

// automatically adjust scroll position to a position where cursor is visible
fn update_scroll(scroll: &mut usize, selected: usize, screen: Rect) {
    if selected.saturating_sub(*scroll) >= screen.height as usize {
        *scroll = selected + 1 - screen.height as usize
    } else if selected < *scroll {
        *scroll = selected
    }
}

// list dir items, filter out ones with illegal characters in their path
// cuz its making the component crash, which is annoying
fn dir_items(path: &Path) -> Option<Vec<PathBuf>> {
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut out: Vec<PathBuf> = entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.file_name().is_some_and(|name| name.to_str().is_some()))
                .collect();
            out.sort();
            Some(out)
        }
        Err(_) => None
    }
}

async fn render(
    client: &mut Client,
    items: &[PathBuf],
    selected: usize,
    scroll: usize,
    screen: Rect,
) {

    clear(&mut client, screen);
    if items.is_empty() {
        client.renderall().await;
        return;
    }

    // just render every entry line by line, with cool symbol as well
    for (y, (i, path)) in (screen.y..screen.y + screen.height).zip(
        items
            .iter()
            .map(|path| format!("{}{}", if path.is_dir() { " " } else { " " }, path.file_name().unwrap().to_str().unwrap()))
            .enumerate()
            .skip(scroll),
    ) {
        if selected == i {
            // magenta for cursor :D
            for (x, c) in (screen.x..screen.x + screen.width).zip(path.chars().chain(std::iter::repeat(' '))) {
                client.setcharcoloured(x, y, c, Colour::Magenta, Colour::Reset);
            }
        } else {
            for (x, c) in (screen.x..screen.x + screen.width).zip(path.chars().chain(std::iter::repeat(' '))) {
                client.setchar(x, y, c);
            }
        }
    }

    client.renderall().await;
}
