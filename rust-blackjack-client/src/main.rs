#![deny(warnings)]
#![feature(uniform_paths)]

extern crate piston_window;
extern crate ws;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod cards;
mod event;
mod client;
mod message_action;
mod socket_message;

use piston_window::{
    clear,
    image,
    text,
    PistonWindow,
    WindowSettings,
    Transformed,
    PressEvent,
    Button,
    Key,
    Glyphs,
    TextureSettings,
};

use ws::connect;

use std::io::stdin;
use std::thread;
use std::sync::{
    Mutex,
    Arc,
};
use std::sync::mpsc;

use client::Client;
use event::Event;
use message_action::MessageAction;
use socket_message::SocketMessage;

fn main() {

    let cards_mutex_arc = Arc::new(Mutex::new(vec![]));
    let bank_cards_mutex_arc = Arc::new(Mutex::new(vec![]));
    let hand_points_arc: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    let bank_points_arc: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    let remaining_cards_amount_arc: Arc<Mutex<u16>> = Arc::new(Mutex::new(0));

    println!("Player name: ");
    let mut player_name: String = String::new();
    stdin().read_line(&mut player_name).expect("Input error.");
    let player_name = player_name.trim().to_string();

    let (
        channel_sender,
        channel_receiver,
    ) = mpsc::channel::<Event>();

    /* the asynchronous reference counters on mutexes
       have to be copied once here as they are moved
       when passed to the client thread below and
       still used by the main thread */
    let cards_mutex_arc_clone = cards_mutex_arc.clone();
    let bank_cards_mutex_arc_clone = bank_cards_mutex_arc.clone();
    let hand_points_arc_clone = hand_points_arc.clone();
    let bank_points_arc_clone = bank_points_arc.clone();
    let remaining_cards_amount_arc_clone = remaining_cards_amount_arc.clone();

    /* the socket handling is performed into a dedicated thread,
     * otherwise the program would just block here waiting for messages */
    thread::spawn(move || {

        const SERVER_ADDRESS: &str = "ws://127.0.0.1:3000";
        let _ = connect(SERVER_ADDRESS, |sender| {
            Client {
                cards_mutex_arc: cards_mutex_arc_clone.clone(),
                bank_cards_mutex_arc: bank_cards_mutex_arc_clone.clone(),
                hand_points_arc: hand_points_arc_clone.clone(),
                bank_points_arc: bank_points_arc_clone.clone(),
                cards_amount_arc: remaining_cards_amount_arc_clone.clone(),
                socket_sender: sender,
                channel_sender: channel_sender.clone(),
            }
        });
    });

    /* the program halts here until a concrete
       connection attempt status is established */
    let channel_message = channel_receiver.recv().unwrap();

    let sender = match channel_message {
        Event::Connect(s) => s,
        _ => {
            panic!("Unexpected channel message.");
        }
    };

    let new_player_message = SocketMessage {
        action: MessageAction::NewPlayer,
        card_index: 0,
        cards_amount: 0,
        text: player_name.clone(),
        player_handpoints: 0,
    };

    let message = serde_json::to_string(&new_player_message).unwrap();
    sender.send(message).unwrap();

    const WINDOW_WIDTH: f64 = 800.0;
    const WINDOW_HEIGHT: f64 = 600.0;

    let mut window: PistonWindow = WindowSettings::new(
        "rust-blackjack",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ]
    )
    .fullscreen(false)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let cards_images = cards::load_all_cards_textures(&mut window);

    const TITLE_FONT_PATH: &str = "res/title_font.ttf";

    let mut glyphs = Glyphs::new(
        TITLE_FONT_PATH,
        window.factory.clone(),
        TextureSettings::new()
    ).unwrap();

    while let Some(event) = window.next() {

        if Ok(Event::Disconnect) == channel_receiver.try_recv() {
            break;
        }

        let hand_points = hand_points_arc.lock().unwrap();
        let bank_points = bank_points_arc.lock().unwrap();

        let mut displayed_cards = cards_mutex_arc.lock().unwrap();
        let mut bank_cards = bank_cards_mutex_arc.lock().unwrap();

        if let Some(Button::Keyboard(Key::Return)) = event.press_args() {

            const MAX_HAND_POINTS: u8 = 21;
            if *hand_points > MAX_HAND_POINTS {
                displayed_cards.clear();
                bank_cards.clear();
            }

            let hit_message = SocketMessage {
                action: MessageAction::Hit,
                card_index: 0,
                cards_amount: 0,
                text: "".to_string(),
                player_handpoints: 0,
            };

            let message = serde_json::to_string(&hit_message).unwrap();

            sender.send(message).unwrap();
        }

        window.draw_2d(
            &event,
            |context, window| {

                const GREEN_COLOR: [f32; 4] = [0.2, 0.5, 0.3, 1.0];

                clear(
                    GREEN_COLOR,
                    window,
                );

                const WHITE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
                const RED_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

                const TITLE_FONT_SIZE: u32 = 64;
                const TITLE_HORIZONTAL_POSITION: f64 = 275.0;
                const TITLE_VERTICAL_POSITION: f64 = 80.0;

                text::Text::new_color(
                    WHITE_COLOR,
                    TITLE_FONT_SIZE,
                ).draw(
                    "Blackjack",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        TITLE_HORIZONTAL_POSITION,
                        TITLE_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                const POINTS_FONT_SIZE: u32 = 32;
                const POINTS_HORIZONTAL_POSITION: f64 = 400.0;
                const POINTS_VERTICAL_POSITION: f64 = 400.0;

                text::Text::new_color(
                    WHITE_COLOR,
                    POINTS_FONT_SIZE,
                ).draw(
                    &*hand_points.to_string(),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        POINTS_HORIZONTAL_POSITION,
                        POINTS_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                const BANK_POINTS_HORIZONTAL_POSITION: f64 = 400.0;
                const BANK_POINTS_VERTICAL_POSITION: f64 = 250.0;

                text::Text::new_color(
                    WHITE_COLOR,
                    POINTS_FONT_SIZE,
                ).draw(
                    &*bank_points.to_string(),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        BANK_POINTS_HORIZONTAL_POSITION,
                        BANK_POINTS_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                const INFO_FONT_SIZE: u32 = 24;
                const INFO_HORIZONTAL_POSITION: f64 = 10.0;
                const INFO_VERTICAL_POSITION: f64 = 570.0;

                const MAX_HAND_POINTS: u8 = 21;
                if *hand_points > MAX_HAND_POINTS {

                    text::Text::new_color(
                        RED_COLOR,
                        INFO_FONT_SIZE,
                    ).draw(
                        "Burst! - Press Enter",
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(
                            INFO_HORIZONTAL_POSITION,
                            INFO_VERTICAL_POSITION,
                        ),
                        window,
                    ).unwrap();

                } else {

                    text::Text::new_color(
                        WHITE_COLOR,
                        INFO_FONT_SIZE,
                    ).draw(
                        "HIT - press Enter",
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(
                            INFO_HORIZONTAL_POSITION,
                            INFO_VERTICAL_POSITION,
                        ),
                        window,
                    ).unwrap();
                }

                const PLAYER_NAME_FONT_SIZE: u32 = 16;
                const PLAYER_NAME_HORIZONTAL_POSITION: f64 = 300.0;
                const PLAYER_NAME_VERTICAL_POSITION: f64 = 380.0;

                text::Text::new_color(
                    WHITE_COLOR,
                    PLAYER_NAME_FONT_SIZE,
                ).draw(
                    &player_name,
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        PLAYER_NAME_HORIZONTAL_POSITION,
                        PLAYER_NAME_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                const REMAINING_CARDS_AMOUNT_FONT_SIZE: u32 = 16;
                const REMAINING_CARDS_AMOUNT_HORIZONTAL_POSITION: f64 = 700.0;
                const REMAINING_CARDS_AMOUNT_VERTICAL_POSITION: f64 = 50.0;

                let remaining_cards_amount = remaining_cards_amount_arc.lock().unwrap();

                text::Text::new_color(
                    WHITE_COLOR,
                    REMAINING_CARDS_AMOUNT_FONT_SIZE,
                ).draw(
                    &*remaining_cards_amount.to_string(),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        REMAINING_CARDS_AMOUNT_HORIZONTAL_POSITION,
                        REMAINING_CARDS_AMOUNT_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                if displayed_cards.is_empty() {
                    return;
                }

                const CARD_HORIZONTAL_POSITION: f64 = 300.0;
                const CARD_VERTICAL_POSITION: f64 = 400.0;
                const CARD_DIMENSIONS_SCALE: f64 = 0.5;

                let mut card_horizontal_position: f64 = CARD_HORIZONTAL_POSITION;
                let mut card_vertical_position: f64 = CARD_VERTICAL_POSITION;

                const ONE_GAME_CARDS_AMOUNT: usize = 52;

                for card_index in 0..displayed_cards.len() {

                    image(
                        &cards_images[
                            *displayed_cards.get(card_index)
                                .unwrap() as usize % ONE_GAME_CARDS_AMOUNT
                        ],
                        context.transform.trans(
                            card_horizontal_position,
                            card_vertical_position,
                        ).scale(
                            CARD_DIMENSIONS_SCALE,
                            CARD_DIMENSIONS_SCALE
                        ),
                        window,
                    );

                    const CARDS_DISTANCE: f64 = 40.0;
                    card_horizontal_position += CARDS_DISTANCE;
                    card_vertical_position += CARDS_DISTANCE;
                }

                const BANK_CARD_HORIZONTAL_POSITION: f64 = 300.0;
                const BANK_CARD_VERTICAL_POSITION: f64 = 100.0;

                let mut card_horizontal_position: f64 = BANK_CARD_HORIZONTAL_POSITION;
                let mut card_vertical_position: f64 = BANK_CARD_VERTICAL_POSITION;

                for card_index in 0..bank_cards.len() {

                    image(
                        &cards_images[
                            *bank_cards.get(card_index)
                                .unwrap() as usize % ONE_GAME_CARDS_AMOUNT
                        ],
                        context.transform.trans(
                            card_horizontal_position,
                            card_vertical_position,
                        ).scale(
                            CARD_DIMENSIONS_SCALE,
                            CARD_DIMENSIONS_SCALE
                        ),
                        window,
                    );

                    const CARDS_DISTANCE: f64 = 40.0;
                    card_horizontal_position += CARDS_DISTANCE;
                    card_vertical_position += CARDS_DISTANCE;
                }
            }
        );
    }

    /* the socket thread is terminated here as well,
       after the window has been closed by the user,
       we voluntarily dont wait for it to be terminated (no join) */
}
