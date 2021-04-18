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
///A [TileBundle](TileBundle)'s rect, so it won't be recalculated every frame
#[derive(Debug, Default, Clone)]
pub struct TileRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
#[derive(Debug, PartialEq)]
pub enum CornerContained {
    ///The other rect's bottom left is inside, towards the right there are _ units
    NotContained,
    BottomLeft {
        units_right: f32,
        units_up: f32,
    },
    BottomRight {
        units_left: f32,
        units_up: f32,
    },
    TopLeft {
        units_right: f32,
        units_down: f32,
    },
    TopRight {
        units_left: f32,
        units_down: f32,
    },
}
impl TileRect {
    ///This function tells us if another rect is inside our rect
    pub fn is_other_inside(&self, other: &TileRect) -> bool {
        if ((other.left > self.left && other.left < self.right)
            || (other.right < self.right && other.right > self.left))
            && ((other.top < self.top && other.top > self.bottom)
                || (other.bottom > self.bottom && other.bottom < self.top))
        {
            //println!("other's left is bigger than my left and smaller than my right");
            return true;
        }
        false
    }
    ///This function gives us the corner of the other rect that is contained within us in world units
    pub fn is_other_inside_world_units(&self, other: &TileRect) -> CornerContained {
        //If the other's top is contained
        if other.top < self.top && other.top > self.bottom {
            //If the other's left is contained
            if other.left > self.left && other.left < self.right {
                return CornerContained::TopLeft {
                    units_down: other.top - self.bottom,
                    units_right: self.right - other.left,
                };
            }
            //If the other's right is contained
            if other.right < self.right && other.right > self.left {
                return CornerContained::TopRight {
                    units_down: other.top - self.bottom,
                    units_left: other.right - self.left,
                };
            }
        }
        //If the other's bottom is contained
        if other.bottom > self.bottom && other.bottom < self.top {
            //If the other's left is contained
            if other.left > self.left && other.left < self.right {
                return CornerContained::BottomLeft {
                    units_up: self.top - other.bottom,
                    units_right: self.right - other.left,
                };
            }
            //If the other's right is contained
            if other.right < self.right && other.right > self.left {
                return CornerContained::BottomRight {
                    units_up: self.top - other.bottom,
                    units_left: other.right - self.left,
                };
            }
        }
        return CornerContained::NotContained;
    }
    ///This function gives us the corner of the other rect that is contained within us scaled with our scale
    pub fn is_other_inside_scaled(&self, scale: Vec3, other: &TileRect) -> CornerContained {
        match self.is_other_inside_world_units(other) {
            CornerContained::NotContained => return CornerContained::NotContained,
            CornerContained::BottomLeft {
                units_right,
                units_up,
            } => {
                return CornerContained::BottomLeft {
                    units_right: units_right / scale.x,
                    units_up: units_up / scale.y,
                };
            }
            CornerContained::BottomRight {
                units_left,
                units_up,
            } => {
                return CornerContained::BottomRight {
                    units_left: units_left / scale.x,
                    units_up: units_up / scale.y,
                };
            }
            CornerContained::TopLeft {
                units_right,
                units_down,
            } => {
                return CornerContained::TopLeft {
                    units_right: units_right / scale.x,
                    units_down: units_down / scale.y,
                };
            }
            CornerContained::TopRight {
                units_left,
                units_down,
            } => {
                return CornerContained::TopRight {
                    units_left: units_left / scale.x,
                    units_down: units_down / scale.y,
                };
            }
        }
    }
}
