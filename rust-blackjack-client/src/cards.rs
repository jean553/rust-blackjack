//! Load cards textures.

use piston_window::{
    PistonWindow,
    G2dTexture,
    Texture,
    TextureSettings,
    Flip,
};

/// Loads one card texture. Refactored as called multiple times.
///
/// # Args:
///
/// `window` - piston window reference for textures loading
/// `file_name` - name of the texture file to load (without prefix/suffix)
fn load_one_card_texture(
    window: &mut PistonWindow,
    file_name: &str
) -> G2dTexture {

    let mut file_path: String = "res/".to_string();
    file_path.push_str(file_name);
    file_path.push_str(".png");

    Texture::from_path(
        &mut window.factory,
        &file_path,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap()
}

/// Loads all cards textures into an array. Refactored for readability.
///
/// # Args:
///
/// `window` - piston window reference for textures loading
pub fn load_all_cards_textures(window: &mut PistonWindow) -> [G2dTexture; 52] {

    [
        load_one_card_texture(window, "2_of_clubs"),
        load_one_card_texture(window, "2_of_diamonds"),
        load_one_card_texture(window, "2_of_hearts"),
        load_one_card_texture(window, "2_of_spades"),

        load_one_card_texture(window, "3_of_clubs"),
        load_one_card_texture(window, "3_of_diamonds"),
        load_one_card_texture(window, "3_of_hearts"),
        load_one_card_texture(window, "3_of_spades"),

        load_one_card_texture(window, "4_of_clubs"),
        load_one_card_texture(window, "4_of_diamonds"),
        load_one_card_texture(window, "4_of_hearts"),
        load_one_card_texture(window, "4_of_spades"),

        load_one_card_texture(window, "5_of_clubs"),
        load_one_card_texture(window, "5_of_diamonds"),
        load_one_card_texture(window, "5_of_hearts"),
        load_one_card_texture(window, "5_of_spades"),

        load_one_card_texture(window, "6_of_clubs"),
        load_one_card_texture(window, "6_of_diamonds"),
        load_one_card_texture(window, "6_of_hearts"),
        load_one_card_texture(window, "6_of_spades"),

        load_one_card_texture(window, "7_of_clubs"),
        load_one_card_texture(window, "7_of_diamonds"),
        load_one_card_texture(window, "7_of_hearts"),
        load_one_card_texture(window, "7_of_spades"),

        load_one_card_texture(window, "8_of_clubs"),
        load_one_card_texture(window, "8_of_diamonds"),
        load_one_card_texture(window, "8_of_hearts"),
        load_one_card_texture(window, "8_of_spades"),

        load_one_card_texture(window, "9_of_clubs"),
        load_one_card_texture(window, "9_of_diamonds"),
        load_one_card_texture(window, "9_of_hearts"),
        load_one_card_texture(window, "9_of_spades"),

        load_one_card_texture(window, "10_of_clubs"),
        load_one_card_texture(window, "jack_of_clubs"),
        load_one_card_texture(window, "queen_of_clubs"),
        load_one_card_texture(window, "king_of_clubs"),
        load_one_card_texture(window, "10_of_diamonds"),
        load_one_card_texture(window, "jack_of_diamonds"),
        load_one_card_texture(window, "queen_of_diamonds"),
        load_one_card_texture(window, "king_of_diamonds"),
        load_one_card_texture(window, "10_of_hearts"),
        load_one_card_texture(window, "jack_of_hearts"),
        load_one_card_texture(window, "queen_of_hearts"),
        load_one_card_texture(window, "king_of_hearts"),
        load_one_card_texture(window, "10_of_spades"),
        load_one_card_texture(window, "jack_of_spades"),
        load_one_card_texture(window, "queen_of_spades"),
        load_one_card_texture(window, "king_of_spades"),

        load_one_card_texture(window, "ace_of_clubs"),
        load_one_card_texture(window, "ace_of_diamonds"),
        load_one_card_texture(window, "ace_of_hearts"),
        load_one_card_texture(window, "ace_of_spades"),
    ]
}

