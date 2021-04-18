use bevy::prelude::*;
use bevy_common::input::resources::MouseWorldPosition;

use crate::data::{
    assets::Pattern,
    resources::SelectedTool,
    shared_components::CurrentlySelected,
    tile_entity::{CornerContained, TileData, TileRect, TileSettings},
};

pub fn use_pencil_tool_seq(
    mouse_world_position: Res<MouseWorldPosition>,
    mouse_input: Res<Input<MouseButton>>,
    tool: Res<SelectedTool>,
    patterns: Res<Assets<Pattern>>,
    mut query: Query<
        (
            Entity,
            &TileRect,
            &TileSettings,
            &GlobalTransform,
            &mut TileData,
        ),
        With<CurrentlySelected>,
    >,
) {
    //If the user clicked the left mouse button
    if mouse_input.just_pressed(MouseButton::Left) {
        match &*tool {
            SelectedTool::Pencil { pattern_handle } => {
                let pattern = patterns.get(pattern_handle).unwrap();
                let mut count = 0;

                for (_entity, tile_rect, tile_settings, global_transform, mut tile_data) in
                    query.iter_mut()
                {
                    //This here is a calculation of the rect of the pattern, based on the mouse position
                    let pattern_rect = TileRect {
                        left: mouse_world_position.position.x
                            - pattern.size[0] as f32 / 2.0 * global_transform.scale.x,
                        right: mouse_world_position.position.x
                            + pattern.size[0] as f32 / 2.0 * global_transform.scale.x,
                        top: mouse_world_position.position.y
                            + pattern.size[1] as f32 / 2.0 * global_transform.scale.y,
                        bottom: mouse_world_position.position.y
                            - pattern.size[1] as f32 / 2.0 * global_transform.scale.y,
                    };
                    /*
                    let scaled_corner_contained =
                        tile_rect.is_other_inside_scaled(global_transform.scale, &pattern_rect);
                    if scaled_corner_contained != CornerContained::NotContained {
                        println!("{:?}", scaled_corner_contained);
                        count += 1;
                    }
                    */
                    //TODO: DONT FORGET Y IS DOWN IN TEXTURES
                    let pixels_x: usize;
                    let pixels_y: usize;
                    let is_contained_scaled =
                        tile_rect.is_other_inside_scaled(global_transform.scale, &pattern_rect);
                    match is_contained_scaled {
                        CornerContained::NotContained => {}
                        CornerContained::BottomLeft {
                            units_right,
                            units_up,
                        } => {
                            pixels_x = tile_settings.tile_width as usize - units_right as usize;
                            pixels_y = units_up as usize;
                            count += 1;
                            draw_pixels(pattern, tile_settings, &mut tile_data, pixels_x, pixels_y);
                        }
                        CornerContained::BottomRight {
                            units_left,
                            units_up,
                        } => {
                            pixels_x = units_left as usize;
                            pixels_y = units_up as usize;
                            count += 1;
                            draw_pixels(pattern, tile_settings, &mut tile_data, pixels_x, pixels_y);
                        }
                        CornerContained::TopLeft {
                            units_right,
                            units_down,
                        } => {
                            pixels_x = tile_settings.tile_width as usize - units_right as usize;
                            pixels_y = tile_settings.tile_height as usize - units_down as usize;
                            count += 1;
                            draw_pixels(pattern, tile_settings, &mut tile_data, pixels_x, pixels_y);
                        }
                        CornerContained::TopRight {
                            units_left,
                            units_down,
                        } => {
                            pixels_x = units_left as usize;
                            pixels_y = tile_settings.tile_height as usize - units_down as usize;
                            count += 1;
                            draw_pixels(pattern, tile_settings, &mut tile_data, pixels_x, pixels_y);
                        }
                    }
                }
                println!("amount of tiles changed: {:?}\n", count);
            }
            _ => {}
        }
    }
}
//FIXME: This is only drawing one pixel of the pattern, debug why
pub fn draw_pixels(
    pattern: &Pattern,
    tile_settings: &TileSettings,
    tile_data: &mut TileData,
    pixels_x: usize,
    pixels_y: usize,
) {
    let y_max = usize::min(
        pattern.size[1],
        tile_settings.tile_height as usize - pixels_y,
    );
    let x_max = usize::min(
        pattern.size[0],
        tile_settings.tile_width as usize - pixels_x,
    );
    for y in (0..y_max).rev() {
        for x in 0..x_max {
            for p in 0..4 {
                tile_data.data[((pixels_y + y) * tile_settings.tile_width as usize) * 4
                    + (pixels_x + x) * 4
                    + p] = pattern.pattern_pixels[y * x][p];
            }
        }
    }
    //TODO: this could be disable later
    println!("x_max, y_max is on ={:?},{:?} ", x_max, y_max);
    println!("corner is on x,y ={:?},{:?} ", pixels_x, pixels_y);
}
