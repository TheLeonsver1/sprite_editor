use bevy::prelude::*;
use bevy_common::input::resources::MouseWorldPosition;

use crate::{
    data::{
        assets::Pattern,
        resources::{MousePixelPosition, SelectedTool},
        shared_components::CurrentlySelected,
        tile_entity::{TileData, TilePosition, TileRect, TileSettings},
        tileset_entity::TileSetSettings,
    },
    systems::initializing::get_total_tileset_size_pixels,
};

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

pub fn use_pencil_tool_seq(
    mouse_pixel_position: Res<MousePixelPosition>,
    mouse_input: Res<Input<MouseButton>>,
    tool: Res<SelectedTool>,
    patterns: Res<Assets<Pattern>>,
    mut query: Query<
        (Entity, &TileSettings, &TilePosition, &mut TileData),
        With<CurrentlySelected>,
    >,
) {
    //If the user is pressing the left mouse button
    if mouse_input.pressed(MouseButton::Left) {
        //And his current tool is the Brush
        match &*tool {
            SelectedTool::Pencil { pattern_handle } => {
                //If the user is hovering on the tileset and
                if let Some(mouse_pixel) = mouse_pixel_position.pixel_position {
                    let pattern = patterns.get(pattern_handle).unwrap();
                    //Get the pattern's corner pixels' positions
                    let (top_left_pixel, top_right_pixel, bottom_left_pixel, bottom_right_pixel) = (
                        (mouse_pixel.as_i32()
                            + IVec2::new(-(pattern.size.x as i32 / 2), pattern.size.y as i32 / 2))
                        .as_u32(),
                        (mouse_pixel.as_i32()
                            + IVec2::new(pattern.size.x as i32 / 2, pattern.size.y as i32 / 2))
                        .as_u32(),
                        (mouse_pixel.as_i32()
                            + IVec2::new(
                                -(pattern.size.x as i32 / 2),
                                -(pattern.size.y as i32 / 2),
                            ))
                        .as_u32(),
                        (mouse_pixel.as_i32()
                            + IVec2::new(pattern.size.x as i32 / 2, -(pattern.size.y as i32 / 2)))
                        .as_u32(),
                    );

                    println!(
                        "{:?}------{:?}\n---{:?}---\n{:?}------{:?}\n",
                        top_left_pixel,
                        top_right_pixel,
                        mouse_pixel,
                        bottom_left_pixel,
                        bottom_right_pixel
                    );
                    for (_entity, tile_settings, tile_position, mut tile_data) in query.iter_mut() {
                        let tile_min_pixel = UVec2::new(
                            tile_position.position.x * tile_settings.tile_width as u32,
                            tile_position.position.y * tile_settings.tile_height as u32,
                        );
                        let tile_max_pixel = UVec2::new(
                            (tile_position.position.x + 1) * tile_settings.tile_width as u32 - 1,
                            (tile_position.position.y + 1) * tile_settings.tile_height as u32 - 1,
                        );

                        //Collision Check
                        if (top_left_pixel.x >= tile_min_pixel.x
                            && top_left_pixel.y >= tile_min_pixel.y)
                            && (top_left_pixel.x <= tile_max_pixel.x
                                && top_left_pixel.y <= tile_max_pixel.y)
                        {
                            let pixel_in_tile_coords = top_left_pixel - tile_min_pixel;

                            draw_pixels_in_tile(
                                pixel_in_tile_coords,
                                Horizontal::Left,
                                Vertical::Top,
                                &mut tile_data,
                                tile_settings,
                                pattern,
                            );
                        } else if (top_right_pixel.x >= tile_min_pixel.x
                            && top_right_pixel.y >= tile_min_pixel.y)
                            && (top_right_pixel.x <= tile_max_pixel.x
                                && top_right_pixel.y <= tile_max_pixel.y)
                        {
                            let pixel_in_tile_coords = top_right_pixel - tile_min_pixel;

                            draw_pixels_in_tile(
                                pixel_in_tile_coords,
                                Horizontal::Right,
                                Vertical::Top,
                                &mut tile_data,
                                tile_settings,
                                pattern,
                            );
                        } else if (bottom_left_pixel.x >= tile_min_pixel.x
                            && bottom_left_pixel.y >= tile_min_pixel.y)
                            && (bottom_left_pixel.x <= tile_max_pixel.x
                                && bottom_left_pixel.y <= tile_max_pixel.y)
                        {
                            let pixel_in_tile_coords = bottom_left_pixel - tile_min_pixel;

                            draw_pixels_in_tile(
                                pixel_in_tile_coords,
                                Horizontal::Left,
                                Vertical::Bottom,
                                &mut tile_data,
                                tile_settings,
                                pattern,
                            );
                        } else if (bottom_right_pixel.x >= tile_min_pixel.x
                            && bottom_right_pixel.y >= tile_min_pixel.y)
                            && (bottom_right_pixel.x <= tile_max_pixel.x
                                && bottom_right_pixel.y <= tile_max_pixel.y)
                        {
                            let pixel_in_tile_coords = bottom_right_pixel - tile_min_pixel;
                            draw_pixels_in_tile(
                                pixel_in_tile_coords,
                                Horizontal::Right,
                                Vertical::Bottom,
                                &mut tile_data,
                                tile_settings,
                                pattern,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
enum Horizontal {
    Left,
    Right,
}
#[derive(Debug)]
enum Vertical {
    Top,
    Bottom,
}
#[derive(Debug)]
struct XData {
    tile_x_min: u32,
    tile_x_max: u32,
    pattern_start_x: u32,
    pattern_end_x: u32,
}
#[derive(Debug)]
struct YData {
    tile_y_min: u32,
    tile_y_max: u32,
    pattern_start_y: u32,
    pattern_end_y: u32,
}
fn get_x_for_drawing_loop(
    side: &Horizontal,
    pattern: &Pattern,
    pixel_in_tile_coords: UVec2,
    tile_settings: &TileSettings,
) -> XData {
    //println!("get_x::pixel_in_tile_coords:{:?}", pixel_in_tile_coords);
    let tile_min_pixel = UVec2::ZERO;
    match side {
        Horizontal::Left => XData {
            //we are the left corner, from left to right, we start at our pos
            tile_x_min: pixel_in_tile_coords.x,
            //we need to end either after the pattern ends, or at the end of the tile
            tile_x_max: u32::min(
                pixel_in_tile_coords.x + pattern.size.x,
                tile_settings.tile_width as u32,
            ),
            //we are the left corner, our pattern's start is on zero
            pattern_start_x: 0,
            //we either end when our pattern ends, or at the tile's end
            pattern_end_x: u32::min(
                pattern.size.x,
                tile_settings.tile_width as u32 - pixel_in_tile_coords.x,
            ),
        },
        Horizontal::Right => XData {
            //we are the right corner, from left to right, we either start at where our pattern can fit, or at the start of the tile
            tile_x_min: i32::max(
                tile_min_pixel.x as i32,
                pixel_in_tile_coords.x as i32 - pattern.size.x as i32,
            ) as u32,
            //we are the right corner, the pattern ends on us
            tile_x_max: pixel_in_tile_coords.x + 1,
            //we are the right corner, our pattern's start is dependant if the pattern can fit, else, what's left
            pattern_start_x: i32::max(0, pattern.size.x as i32 - 1 - pixel_in_tile_coords.x as i32)
                as u32,
            //we either end when our pattern ends, or at the tile's end
            pattern_end_x: pattern.size.x,
        },
    }
}
///FIXME:idk how and why this works, this will probably be technical debt later
fn get_y_for_drawing_loop(
    side: &Vertical,
    pattern: &Pattern,
    pixel_in_tile_coords: UVec2,
    tile_settings: &TileSettings,
) -> YData {
    let tile_min_pixel = UVec2::ZERO;
    match side {
        Vertical::Top => YData {
            tile_y_min: i32::max(
                tile_min_pixel.y as i32,
                pixel_in_tile_coords.y as i32 - (pattern.size.y as i32 - 1),
            ) as u32,
            tile_y_max: u32::min(pixel_in_tile_coords.y + 1, tile_settings.tile_height as u32),
            pattern_start_y: i32::max(0, pattern.size.y as i32 - 1 - pixel_in_tile_coords.y as i32)
                as u32,
            pattern_end_y: pattern.size.y,
        },

        Vertical::Bottom => YData {
            //FIXME: THERE'S A BUG HERE
            tile_y_min: pixel_in_tile_coords.y,
            tile_y_max: u32::min(
                pixel_in_tile_coords.y + pattern.size.y,
                tile_settings.tile_height as u32,
            ),
            pattern_start_y: 0,
            pattern_end_y: u32::min(
                pattern.size.y,
                tile_settings.tile_height as u32 - pixel_in_tile_coords.y,
            ),
        },
    }
}
fn draw_pixels_in_tile(
    pixel_in_tile_coords: UVec2,
    horizontal: Horizontal,
    vertical: Vertical,
    tile_data: &mut TileData,
    tile_settings: &TileSettings,
    pattern: &Pattern,
) {
    let x_data = get_x_for_drawing_loop(&horizontal, pattern, pixel_in_tile_coords, tile_settings);
    let y_data = get_y_for_drawing_loop(&vertical, pattern, pixel_in_tile_coords, tile_settings);
    println!(
        "Vertical:{:?},Horizontal:{:?}\npixel_in_tile_coords:{:?}\nx_data:{:?}\ny_data:{:?}",
        vertical, horizontal, pixel_in_tile_coords, x_data, y_data
    );
    for (i_y, y_tile) in (y_data.tile_y_min..y_data.tile_y_max).enumerate() {
        for (i_x, x_tile) in (x_data.tile_x_min..x_data.tile_x_max).enumerate() {
            for p in 0..4 {
                tile_data.data[(tile_settings.tile_height - 1 - y_tile as usize)
                    * tile_settings.tile_width
                    * 4
                    + (x_tile as usize) * 4
                    + p] = pattern.pattern_pixels[(y_data.pattern_start_y as usize + i_y)
                    * pattern.size.x as usize
                    + (x_data.pattern_start_x as usize + i_x)][p];
            }
        }
    }
    println!("");
}
