use bevy::prelude::*;
use bevy::render::pipeline::RenderPipeline;

use super::shared_components::{CurrentlySelected, Uninitiated};
//A visual representation of a single tile/sprite
#[derive(Bundle, Clone)]
pub struct TileBundle {
    #[bundle]
    pub sprite: SpriteBundle, //This is the visualizing part, not the data, there's a transaction that changes the texture of the handle in the sprite when the data changes
    pub tile_settings: TileSettings,
    pub data: TileData,
    pub uninitiated: Uninitiated,
    pub name: TileName,
    pub currently_selected: CurrentlySelected,
}
impl Default for TileBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    crate::CUSTOM_SPRITE_PIPELINE_HANDLE.typed(),
                )]),
                ..Default::default()
            },
            data: TileData::default(),
            tile_settings: TileSettings::default(),
            uninitiated: Uninitiated::default(),
            name: TileName::default(),
            currently_selected: CurrentlySelected::default(),
        }
    }
}

impl TileBundle {
    pub fn new(
        tile_settings: TileSettings,
        material_handle: Handle<ColorMaterial>,
        transform: Transform,
    ) -> Self {
        let mut default = Self::default();
        default.tile_settings = tile_settings;
        default.sprite.material = material_handle;
        default.sprite.transform = transform;
        default
    }
}
#[derive(Debug, Default, Clone)]
pub struct TileData {
    pub data: Vec<u8>,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct TileSettings {
    pub tile_width: u32,
    pub tile_height: u32,
}

///A [TileBundle](TileBundle)'s name
#[derive(Debug, Default, Clone)]
pub struct TileName {
    pub name: String,
}
