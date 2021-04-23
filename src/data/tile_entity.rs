use bevy::prelude::*;
use bevy::render::pipeline::RenderPipeline;

use super::shared_components::Uninitiated;
//A visual representation of a single tile/sprite
#[derive(Bundle, Clone)]
pub struct TileBundle {
    #[bundle]
    pub sprite: SpriteBundle, //This is the visualizing part, not the data, there's a transaction that changes the texture of the handle in the sprite when the data changes
    pub tile_settings: TileSettings,
    pub data: TileData,
    pub uninitiated: Uninitiated,
    pub name: TileName,
    pub rect: TileRect,
    pub tile_position: TilePosition,
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
            rect: TileRect::default(),
            tile_position: TilePosition::default(),
        }
    }
}

impl TileBundle {
    pub fn new(
        tile_settings: TileSettings,
        tile_position: TilePosition,
        material_handle: Handle<ColorMaterial>,
        transform: Transform,
    ) -> Self {
        let mut default = Self::default();
        default.tile_settings = tile_settings;
        default.sprite.material = material_handle;
        default.sprite.transform = transform;
        default.tile_position = tile_position;
        default
    }
}
//TODO: Optimization: make data an option, initiate the tile when the pencil touches it and directly insert the changes on the first draw, before touch make the a single handle for all untouched tiles
#[derive(Debug, Default, Clone)]
pub struct TileData {
    pub data: Vec<u8>,
}
#[derive(Debug, Default, Clone)]
pub struct TilePosition {
    pub position: UVec2,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct TileSettings {
    pub tile_width: usize,
    pub tile_height: usize,
}

///A [TileBundle](TileBundle)'s name
#[derive(Debug, Default, Clone)]
pub struct TileName {
    pub name: String,
}
///A [TileBundle](TileBundle)'s rect, so it won't be recalculated every frame
#[derive(Debug, Default, Clone)]
pub struct TileRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
impl TileRect {
    ///This function tells us if another rect is inside our rect
    pub fn is_other_inside(&self, other: &TileRect) -> bool {
        if ((other.left >= self.left && other.left <= self.right)
            || (other.right <= self.right && other.right >= self.left))
            && ((other.top <= self.top && other.top >= self.bottom)
                || (other.bottom >= self.bottom && other.bottom <= self.top))
        {
            //println!("other's left is bigger than my left and smaller than my right");
            return true;
        }
        false
    }
}
