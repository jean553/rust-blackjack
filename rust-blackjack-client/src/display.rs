use piston_window::{
    image,
    Context,
    G2dTexture,
    G2d,
    Transformed,
    Glyphs,
    text,
};

use std::sync::{
    Mutex,
    Arc,
};

const WHITE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

/// Displays the given cards at the given position.
///
/// # Args:
///
/// `window` - the window where to draw
/// `context` - the rendering loop context
/// `all_cards_images` - the array of all the cards images
/// `cards` - the cards to display
/// `horizontal_position` - the origin horizontal position of the cards
/// `vertical_position` - the origin vertical position of the cards
pub fn display_cards(
    window: &mut G2d,
    context: &Context,
    all_cards_images: &[G2dTexture],
    cards: &Vec<u16>,
    horizontal_position: f64,
    vertical_position: f64,
) {

    for card_index in 0..cards.len() {

        const CARDS_DISTANCE: f64 = 40.0;
        const CARD_DIMENSIONS_SCALE: f64 = 0.5;
        const ONE_GAME_CARDS_AMOUNT: usize = 52;

        image(
            &all_cards_images[
                *cards.get(card_index)
                    .unwrap() as usize % ONE_GAME_CARDS_AMOUNT
            ],
            context.transform.trans(
                horizontal_position + card_index as f64 * CARDS_DISTANCE,
                vertical_position + card_index as f64 * CARDS_DISTANCE,
            ).scale(
                CARD_DIMENSIONS_SCALE,
                CARD_DIMENSIONS_SCALE
            ),
            window,
        );
    }
}

/// Displays the remaining cards amount according to the given mutex.
///
/// # Args:
///
/// `window` - the window where to draw
/// `context` - the rendering loop context
/// `glyphs` - the text rendering Piston glyph
/// `amount` - the amount to display
pub fn display_remaining_cards_amount(
    window: &mut G2d,
    context: &Context,
    glyphs: &mut Glyphs,
    amount: &Arc<Mutex<u16>>,
) {

    const REMAINING_CARDS_AMOUNT_FONT_SIZE: u32 = 16;
    const REMAINING_CARDS_AMOUNT_HORIZONTAL_POSITION: f64 = 700.0;
    const REMAINING_CARDS_AMOUNT_VERTICAL_POSITION: f64 = 50.0;

    let amount = amount.lock().unwrap();

    text::Text::new_color(
        WHITE_COLOR,
        REMAINING_CARDS_AMOUNT_FONT_SIZE,
    ).draw(
        &amount.to_string(),
        glyphs,
        &context.draw_state,
        context.transform.trans(
            REMAINING_CARDS_AMOUNT_HORIZONTAL_POSITION,
            REMAINING_CARDS_AMOUNT_VERTICAL_POSITION,
        ),
        window,
    ).unwrap();
}

/// Displays the current player name.
///
/// # Args:
///
/// `window` - the window where to draw
/// `context` - the rendering loop context
/// `glyphs` - the text rendering Piston glyph
/// `player_name` - the name of the player
pub fn display_player_name(
    window: &mut G2d,
    context: &Context,
    glyphs: &mut Glyphs,
    player_name: &str,
) {

    const PLAYER_NAME_FONT_SIZE: u32 = 16;
    const PLAYER_NAME_HORIZONTAL_POSITION: f64 = 300.0;
    const PLAYER_NAME_VERTICAL_POSITION: f64 = 380.0;

    text::Text::new_color(
        WHITE_COLOR,
        PLAYER_NAME_FONT_SIZE,
    ).draw(
        player_name,
        glyphs,
        &context.draw_state,
        context.transform.trans(
            PLAYER_NAME_HORIZONTAL_POSITION,
            PLAYER_NAME_VERTICAL_POSITION,
        ),
        window,
    ).unwrap();
}

/// Displays the current player information about his possible actions.
///
/// # Args:
///
/// `window` - the window where to draw
/// `context` - the rendering loop context
/// `glyphs` - the text rendering Piston glyph
/// `hand_points` - the amount of the player hand points
pub fn display_information(
    window: &mut G2d,
    context: &Context,
    glyphs: &mut Glyphs,
    hand_points: u8,
) {

    const RED_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    const INFO_FONT_SIZE: u32 = 24;
    const INFO_HORIZONTAL_POSITION: f64 = 10.0;
    const INFO_VERTICAL_POSITION: f64 = 570.0;

    const MAX_HAND_POINTS: u8 = 21;
    if hand_points > MAX_HAND_POINTS {

        text::Text::new_color(
            RED_COLOR,
            INFO_FONT_SIZE,
        ).draw(
            "Burst! - Press Enter",
            glyphs,
            &context.draw_state,
            context.transform.trans(
                INFO_HORIZONTAL_POSITION,
                INFO_VERTICAL_POSITION,
            ),
            window,
        ).unwrap();

        return;
    }

    text::Text::new_color(
        WHITE_COLOR,
        INFO_FONT_SIZE,
    ).draw(
        "HIT - press Enter",
        glyphs,
        &context.draw_state,
        context.transform.trans(
            INFO_HORIZONTAL_POSITION,
            INFO_VERTICAL_POSITION,
        ),
        window,
    ).unwrap();
}

/// TODO
pub fn display_bank_points(
    window: &mut G2d,
    context: &Context,
    glyphs: &mut Glyphs,
    bank_points: &Arc<Mutex<u8>>,
) {
    const BANK_POINTS_HORIZONTAL_POSITION: f64 = 400.0;
    const BANK_POINTS_VERTICAL_POSITION: f64 = 250.0;
    const POINTS_FONT_SIZE: u32 = 32;

    let bank_points = bank_points.lock().unwrap();

    text::Text::new_color(
        WHITE_COLOR,
        POINTS_FONT_SIZE,
    ).draw(
        &*bank_points.to_string(),
        glyphs,
        &context.draw_state,
        context.transform.trans(
            BANK_POINTS_HORIZONTAL_POSITION,
            BANK_POINTS_VERTICAL_POSITION,
        ),
        window,
    ).unwrap();
}
