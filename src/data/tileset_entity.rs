use super::shared_components::Uninitiated;
use bevy::{prelude::*, utils::HashMap};
use bevy_common::input::data_components::CameraZoomLimit;
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
    pub fn new(tileset_settings: TileSetSettings, index: u32) -> Self {
        let mut name = "Tileset ".to_string();
        name.push_str(&u32::to_string(&index));
        Self {
            tileset_settings,
            tileset_name: TileSetName { name },
            ..Default::default()
        }
    }
}
///The basic info of the [TileSetBundle](TileSetBundle)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSetSettings {
    pub tile_width: usize,
    pub tile_height: usize,
    pub tileset_height: usize,
    pub tileset_width: usize,
}
//The default would be a single sprite
impl Default for TileSetSettings {
    fn default() -> Self {
        Self::multiple_tiles()
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
#[derive(Debug)]
pub struct TileSetView {
    pub camera_transform: Transform,
    pub camera_zoom_limits: CameraZoomLimit,
}
impl Default for TileSetView {
    fn default() -> Self {
        let far = 1000.0;
        Self {
            camera_transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            camera_zoom_limits: CameraZoomLimit::default(),
        }
    }
}
#[derive(Debug, Default)]
pub struct TileSetName {
    pub name: String,
}

///This is a marker to help us know which [TileSetBundle](TileSetBundle) is currently viewed
#[derive(Debug, Default)]
pub struct NewlySelected;
