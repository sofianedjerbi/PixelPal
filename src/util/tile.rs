use crate::constants::tileset::*;

/// Generates a random tile ID based on the given level.
///
/// # Parameters
/// - `level`: The level at which the tile ID should be generated.
///
/// # Returns
/// A random tile ID corresponding to the specified level.
///
/// This function uses a random number generator to select a tile ID
/// from a predefined map (`TEXTURE_RELIEF_IDS_MAP`) based on the given level.
pub fn get_random_tile_id(level: u32) -> u32 {
    let tile_distribution = TEXTURE_RELIEF_IDS_MAP
        .get(&level)
        .expect("Unable to get tile probability map!");
    tile_distribution.get_random() + TEXTURE_ID_OFFSET_MAP[&level]
}

/// Converts a mask and a value to a specific tile ID.
///
/// # Parameters
/// - `mask`: The mask used to select the tile.
/// - `value`: An additional value influencing the selection.
///
/// # Returns
/// A tile ID based on the combination of the provided mask and value.
///
/// This function combines the mask and value using predefined maps
/// (`TEXTURE_CORNER_IDS_MAP` and `TEXTURE_ID_OFFSET_MAP`) to produce a specific tile ID.
pub fn mask_to_id(mask: u32, value: u32) -> u32 {
    TEXTURE_CORNER_IDS_MAP[&mask] + TEXTURE_ID_OFFSET_MAP[&value]
}
