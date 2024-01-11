use libccanvas::{client::*, bindings::{Subscription, KeyCode, EventVariant, Discriminator}};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let mut client = Client::new(ClientConfig::default()).await;

    let (mut width, mut height) = client.term_size().await;
    client.hidecursor();
    let mut screen_protect = render(&mut client, width, height).await;

    client.set("path".to_string(), Discriminator::master(), nu_json::to_value(&client.current_directory().await).unwrap()).await;

    // load in the two components
    // but not explorer-file, as that should be loaded by dir
    client.spawn("path".to_string(), "ccanvas-explorer-path".to_string(), Vec::new()).await;
    client.spawn("dir".to_string(), "ccanvas-explorer-dir".to_string(), Vec::new()).await;

    client.subscribe_multiple(vec![
        // these are the events it actually listens to
        Subscription::ScreenResize.into(),
        Subscription::specific_keycode(KeyCode::Char('q')).into(),
        // these events will be captured if screen size is not sufficient
        // hence their priority 0 (highest priority)
        Subscription::specific_keycode(KeyCode::Up).with_priority(0),
        Subscription::specific_keycode(KeyCode::Down).with_priority(0),
        Subscription::specific_keycode(KeyCode::Left).with_priority(0),
        Subscription::specific_keycode(KeyCode::Right).with_priority(0),
        Subscription::specific_keycode(KeyCode::Backspace).with_priority(0),
        Subscription::specific_keycode(KeyCode::Char('\n')).with_priority(0),
    ]).await;

    while let Some(mut event) = client.recv().await {
        match event.get() {
            EventVariant::Key(key) if key.code == KeyCode::Char('q') => {
                // exit if 'q' is pressed
                client.exit().await;
                break;
            }
            EventVariant::Resize { width: new_width, height: new_height } => {
                // rerender if resized
                width = *new_width;
                height = *new_height;
                screen_protect = render(&mut client, width, height).await;
            }
            _ => if screen_protect {
                // if screen protector is on, then capture the event
                // so other components wont get it
                // this prevents from triggering their rerendering
                event.done(false);
            }
        }
    }
}

#[derive(Serialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

// get dimensions of the main screen
// (offsetx, offsety, width, height, split)
fn get_dimensions(term_width: u32, term_height: u32) -> (u32, u32, u32, u32, u32) {
    (term_width / 4, term_height / 4 + 1, term_width / 2, term_height / 2, term_width / 4)
}

// this is only triggered when resize
async fn render(client: &mut Client, term_width: u32, term_height: u32) -> bool {
    // so we should clear all the junk on screen
    // which is now broken because the screen width may have changed
    // especially if characters in the same line becomes in multiple lines, making a mess
    // thats why we cannot just "unrender" the characters this component previously occupy
    client.clear_all();

    // screen protector!
    if term_width < 30 || term_height < 10 {
        "Require 30 x 10".chars().enumerate().for_each(|(x, c)| client.setchar(x as u32, 0, c));
        format!("Got {term_width} x {term_height}").chars().enumerate().for_each(|(x, c)| client.setchar(x as u32, 1, c));
        client.renderall().await;
        return true;
    }

    let (offsetx, offsety, width, height, split) = get_dimensions(term_width, term_height);

    // these are the dimensions of where the components should render in
    let dir_screen = Rect::new(offsetx + 1, offsety + 1, split - 1, height - 2);
    let file_screen = Rect::new(offsetx + split + 1, offsety + 1, width - split - 2, height - 2);
    let path_screen = Rect::new(offsetx + 1, offsety - 2, width - 2, 1);

    // draw the frames
    for x in offsetx + 1..offsetx + width - 1 {
        // path panel
        client.setchar(x, offsety - 1, '─');
        client.setchar(x, offsety - 3, '─');

        if split + offsetx == x {
            client.setchar(x, offsety, '┬');
            client.setchar(x, offsety + height - 1, '┴');
            continue;
        }

        client.setchar(x, offsety, '─');
        client.setchar(x, offsety + height - 1, '─');
    }

    for y in offsety + 1..offsety + height - 1 {
        client.setchar(offsetx, y, '│');
        client.setchar(split + offsetx, y, '│');
        client.setchar(offsetx + width - 1, y, '│');
    }

    // draw the corners
    client.setchar(offsetx, offsety, '╭');
    client.setchar(offsetx + width - 1, offsety, '╮');
    client.setchar(offsetx + width - 1, offsety + height - 1, '╯');
    client.setchar(offsetx, offsety + height - 1, '╰');

    // path bar
    client.setchar(offsetx, offsety - 3, '╭');
    client.setchar(offsetx + width - 1, offsety - 3, '╮');
    client.setchar(offsetx + width - 1, offsety - 1, '╯');
    client.setchar(offsetx, offsety - 2, '│');
    client.setchar(offsetx + width - 1, offsety - 2, '│');
    client.setchar(offsetx, offsety - 1, '╰');
    
    client.renderall().await;

    // set the screen sizes
    // other components watching for change will recieve an event when these values change
    // other components also have a Rect to deseriaise the Value
    client.set("dir_screen".to_string(), Discriminator::master(), nu_json::to_value(&dir_screen).unwrap()).await;
    client.set("file_screen".to_string(), Discriminator::master(), nu_json::to_value(&file_screen).unwrap()).await;
    client.set("path_screen".to_string(), Discriminator::master(), nu_json::to_value(&path_screen).unwrap()).await;

    false
}
