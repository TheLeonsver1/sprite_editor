use super::shared_components::Uninitiated;
use bevy::{prelude::*, utils::HashMap};
#[derive(Debug, Default)]
pub struct OpenDocumentsMap {
    pub document_map: HashMap<usize, TileSetSettings>,
}

///This bundle represents an uninitiated tileset
#[derive(Debug, Bundle, Default)]
pub struct TileSetBundle {
    tileset_settings: TileSetSettings,
    uninitiated: Uninitiated,
    transform: Transform,
    global_transform: GlobalTransform,
}
impl TileSetBundle {
    pub fn new(tileset_settings: TileSetSettings) -> Self {
        Self {
            tileset_settings,
            ..Default::default()
        }
    }
}
///The basic info of the tileset, a collection of unique tiles
#[derive(Debug, Clone, Copy)]
pub struct TileSetSettings {
    pub tile_width: u32,
    pub tile_height: u32,
    pub tileset_height: u8,
    pub tileset_width: u8,
}
//The default would be a single sprite
impl Default for TileSetSettings {
    fn default() -> Self {
        Self::single_tile()
    }
}
impl TileSetSettings {
    ///Creates a tileset for a single sprite
    fn single_tile() -> Self {
        Self {
            tile_width: 256,
            tile_height: 256,
            tileset_height: 10,
            tileset_width: 10,
        }
    }
    #[allow(dead_code)]
    ///Creates a tileset made out of multiple sprites
    fn multiple_tiles() -> Self {
        Self {
            tile_width: 32,
            tile_height: 32,
            tileset_height: 10,
            tileset_width: 10,
        }
    }
}
