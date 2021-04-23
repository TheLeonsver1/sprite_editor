use bevy::{
    math::UVec2,
    prelude::{Assets, FromWorld, Handle},
};

use super::assets::Pattern;
pub enum SelectedTool {
    Pan,
    Pencil { pattern_handle: Handle<Pattern> },
}
pub struct MousePixelPosition {
    pub pixel_position: Option<UVec2>,
}
impl Default for MousePixelPosition {
    fn default() -> Self {
        Self {
            pixel_position: None,
        }
    }
}
