use bevy::prelude::*;
use bevy_common::input::resources::MouseWorldPosition;
pub mod brush;
use crate::{
    data::{
        resources::MousePixelPosition, shared_components::CurrentlySelected,
        tileset_entity::TileSetSettings,
    },
    systems::initializing::get_total_tileset_size_pixels,
};
pub use brush::*;

///This sets a resource that holds the mouse's pixel position for this frame, if it's not on a tileset, it's set to None
pub fn get_mouse_pixel_tileset_pos(
    mouse_world_position: Res<MouseWorldPosition>,
    mut mouse_pixel_pos: ResMut<MousePixelPosition>,
    query: Query<(&TileSetSettings, &GlobalTransform), With<CurrentlySelected>>,
) {
    //let world_position = Vec2::new(f32::floor(world_position.x), f32::floor(world_position.y));
    if let Ok((tileset_settings, global_transform)) = query.single() {
        let tileset_size = get_total_tileset_size_pixels(&tileset_settings);
        let tileset_size_ivec = tileset_size.as_i32();
        let world_position_reverse_scaled_to_pixels = Vec2::new(
            mouse_world_position.position.x / global_transform.scale.x,
            mouse_world_position.position.y / global_transform.scale.y,
        );
        let world_position_reverse_offset =
            world_position_reverse_scaled_to_pixels.as_i32() + tileset_size_ivec / 2;
        if world_position_reverse_offset >= IVec2::new(0, 0)
            && world_position_reverse_offset < tileset_size_ivec
        {
            mouse_pixel_pos.pixel_position = Some(world_position_reverse_offset.as_u32());
        } else {
            mouse_pixel_pos.pixel_position = None;
        }
    }
}
