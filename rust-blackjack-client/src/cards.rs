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
pub fn load_all_cards_textures(window: &mut PistonWindow) -> Vec<G2dTexture> {

    vec![
        load_one_card_texture(window, "2_of_clubs"),
        load_one_card_texture(window, "3_of_clubs"),
    ]
}

