use bevy::prelude::{Assets, FromWorld, Handle};

use super::assets::Pattern;
pub enum SelectedTool {
    Pan,
    Pencil { pattern_handle: Handle<Pattern> },
}
