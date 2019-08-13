#![deny(warnings)]

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
mod display;

use piston_window::{
    clear,
    PistonWindow,
    WindowSettings,
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
use display::{
    display_cards,
    display_remaining_cards_amount,
    display_player_name,
    display_information,
    display_bank_points,
    display_hand_points,
    display_title,
};

fn main() {

    let player_cards_mutex_arc = Arc::new(Mutex::new(vec![]));
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
    let player_cards_mutex_arc_clone = player_cards_mutex_arc.clone();
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
                player_cards_mutex_arc: player_cards_mutex_arc_clone.clone(),
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

        let pressed_key = event.press_args();

        if let Some(Button::Keyboard(Key::Escape)) = pressed_key {
            break;
        }

        let mut player_cards = player_cards_mutex_arc.lock().unwrap();
        let mut bank_cards = bank_cards_mutex_arc.lock().unwrap();

        if let Some(Button::Keyboard(Key::Return)) = pressed_key {

            let hand_points = hand_points_arc.lock().unwrap();

            const MAX_HAND_POINTS: u8 = 21;
            if *hand_points > MAX_HAND_POINTS {
                player_cards.clear();
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

        else if let Some(Button::Keyboard(Key::Space)) = pressed_key {

            player_cards.clear();
            bank_cards.clear();

            let stand_message = SocketMessage {
                action: MessageAction::Stand,
                card_index: 0,
                cards_amount: 0,
                text: "".to_string(),
                player_handpoints: 0,
            };
            let message = serde_json::to_string(&stand_message).unwrap();
            sender.send(message).unwrap();
        }

        window.draw_2d(
            &event,
            |context, mut window| {

                const GREEN_COLOR: [f32; 4] = [0.2, 0.5, 0.3, 1.0];
                clear(
                    GREEN_COLOR,
                    window,
                );

                display_title(
                    window,
                    &context,
                    &mut glyphs,
                );

                display_hand_points(
                    window,
                    &context,
                    &mut glyphs,
                    &hand_points_arc,
                );

                display_bank_points(
                    window,
                    &context,
                    &mut glyphs,
                    &bank_points_arc,
                );

                display_information(
                    window,
                    &context,
                    &mut glyphs,
                    &hand_points_arc,
                );

                display_player_name(
                    window,
                    &context,
                    &mut glyphs,
                    &player_name,
                );

                display_remaining_cards_amount(
                    window,
                    &context,
                    &mut glyphs,
                    &remaining_cards_amount_arc,
                );

                if player_cards.is_empty() {
                    return;
                }

                const PLAYER_CARD_HORIZONTAL_POSITION: f64 = 300.0;
                const PLAYER_CARD_VERTICAL_POSITION: f64 = 400.0;

                display_cards(
                    &mut window,
                    &context,
                    &cards_images,
                    &player_cards,
                    PLAYER_CARD_HORIZONTAL_POSITION,
                    PLAYER_CARD_VERTICAL_POSITION,
                );

                const BANK_CARD_HORIZONTAL_POSITION: f64 = 300.0;
                const BANK_CARD_VERTICAL_POSITION: f64 = 100.0;

                display_cards(
                    &mut window,
                    &context,
                    &cards_images,
                    &bank_cards,
                    BANK_CARD_HORIZONTAL_POSITION,
                    BANK_CARD_VERTICAL_POSITION,
                );
            }
        );
    }

    /* the socket thread is terminated here as well,
       after the window has been closed by the user,
       we voluntarily dont wait for it to be terminated (no join) */
}
