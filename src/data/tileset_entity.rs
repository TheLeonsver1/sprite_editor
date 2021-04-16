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
    tileset_view: TileSetView,
    tileset_name: TileSetName,
    newly_selected: NewlySelected,
}
impl TileSetBundle {
    pub fn new(tileset_settings: TileSetSettings) -> Self {
        Self {
            tileset_settings,
            ..Default::default()
        }
    }
}
///The basic info of the [TileSetBundle](TileSetBundle)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn single_tile() -> Self {
        Self {
            tile_width: 32,
            tile_height: 32,
            tileset_height: 1,
            tileset_width: 1,
        }
    }
    ///Creates a tileset made out of multiple sprites
    pub fn multiple_tiles() -> Self {
        Self {
            tile_width: 32,
            tile_height: 32,
            tileset_height: 10,
            tileset_width: 10,
        }
    }
}
///The view of this tileset's camera, it's "tab" information basically, it's the last information before we switched to edit something else
#[derive(Debug, Default)]
pub struct TileSetView {
    pub camera_transform: Transform,
}
#[derive(Debug, Default)]
pub struct TileSetName {
    pub name: String,
}

///This is a marker to help us know which [TileSetBundle](TileSetBundle) is currently viewed
#[derive(Debug, Default)]
pub struct NewlySelected;
