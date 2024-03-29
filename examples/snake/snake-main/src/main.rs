use std::time::Duration;

use ccanvas_snake_main::{Direction, Snake};
use libccanvas::{
    bindings::{EventVariant, KeyCode, Subscription},
    client::{Client, ClientConfig},
};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let mut client = Client::new(ClientConfig::default()).await;
    client
        .subscribe_multiple(vec![
            Subscription::specific_keycode(KeyCode::Up),
            Subscription::specific_keycode(KeyCode::Down),
            Subscription::specific_keycode(KeyCode::Left),
            Subscription::specific_keycode(KeyCode::Right),
        ])
        .await;
    client.hidecursor();

    // draw the frame
    for x in 1..43 {
        client.setchar(x, 1, '─');
        client.setchar(x, 23, '─');
    }

    for y in 2..24 {
        client.setchar(0, y, '│');
        client.setchar(43, y, '│');
    }

    client.setchar(43, 1, '╮');
    client.setchar(0, 1, '╭');
    client.setchar(43, 23, '╯');
    client.setchar(0, 23, '╰');

    let mut snake = Snake::new(&mut client).await;

    client.renderall().await;

    // give it a bit of suspense
    let mut next_tick = Instant::now() + Duration::from_millis(500);

    let mut score = 0;

    loop {
        tokio::select! {
            // if there are no inputs until the next tick, then great
            _ = tokio::time::sleep_until(next_tick) => {},
            event = client.recv() => {
                // otherwise, do something with the event
                if let EventVariant::Key(key) = event.get() {
                    match key.code {
                        KeyCode::Up if snake.previous_heading != Direction::Down => snake.heading = Direction::Up,
                        KeyCode::Down if snake.previous_heading != Direction::Up => snake.heading = Direction::Down,
                        KeyCode::Left if snake.previous_heading != Direction::Right => snake.heading = Direction::Left,
                        KeyCode::Right if snake.previous_heading != Direction::Left => snake.heading = Direction::Right,
                        _ => {}
                    }

                    // and continue waiting until the next game tick
                    continue;
                }
            }
        }

        // render and forward the snake by 1 pixel
        snake.render_forward(&mut client, &mut score).await;

        // if game is over, then just exit the game script
        // this will freeze the screen and not exit yet
        if snake.game_over(&mut client).await {
            // flush all screen updates
            client.renderall().await;
            break;
        }

        client.renderall().await;

        // do some maths to make the snake go faster and faster
        // plot the graph in desmos to see how it looks like
        next_tick = Instant::now()
            + Duration::from_millis(60 - (((score as f32 + 2_f32).log2() * 8_f32) as u64));
    }
}
