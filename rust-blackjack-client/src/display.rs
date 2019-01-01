use piston_window::{
    image,
    Context,
    G2dTexture,
    G2d,
    Transformed,
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
