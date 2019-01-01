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
    const WHITE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

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
