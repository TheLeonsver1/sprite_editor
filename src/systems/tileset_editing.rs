use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_common::input::{
    data_components::CameraZoomLimit, marker_components::MainCamera, resources::MouseWorldPosition,
};

use crate::data::{
    assets::Pattern,
    resources::SelectedTool,
    shared_components::CurrentlySelected,
    tile_entity::{CornerContained, TileData, TileRect, TileSettings},
    tileset_entity::{NewlySelected, TileSetView},
};

///This function updates the [Texture](Texture) of [ColorMaterial](ColorMaterial)s used by [TileBundle](TileBundle)s when their [TileData](TileData) is changed
pub fn update_textures_for_changed_tile_data(
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (&TileSettings, &TileData, &Handle<ColorMaterial>),
        (
            With<CurrentlySelected>, //This should be redundant since Changed<TileData> should be only for currently selected tiles but whatever, i don't think this hurts
            Changed<TileData>,
        ),
    >,
) {
    for (tile_settings, tile_data, material_handle) in query.iter() {
        //This shouldn't fail really, i shouldn't delete any of them anywhere
        let material = materials.get_mut(material_handle).unwrap();
        //Add this texture to the resource and get the handle back
        let texture_handle = textures.add(Texture::new(
            Extent3d::new(tile_settings.tile_width, tile_settings.tile_height, 1),
            TextureDimension::D2,
            tile_data.data.clone(),
            TextureFormat::Rgba8UnormSrgb,
        ));
        //Set the material's texture handle
        material.texture = Some(texture_handle);
    }
}

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

                for (entity, tile_rect, tile_settings, global_transform, mut tile_data) in
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
///Changes the view from one tileset to another
pub fn update_selected_tileset(
    mut commands: Commands,
    mut camera_transform_query: Query<(&mut Transform, &mut CameraZoomLimit), With<MainCamera>>,
    newly_selected_query: Query<
        (Entity, &TileSetView),
        (With<NewlySelected>, Without<CurrentlySelected>),
    >,
    mut currently_unselected_children_query: Query<
        (Entity, &Parent, &mut Visible),
        (With<TileSettings>, Without<CurrentlySelected>),
    >,
    mut currently_selected_query: Query<(Entity, &mut TileSetView), With<CurrentlySelected>>,
    mut currently_selected_children_query: Query<
        (Entity, &mut Visible),
        (With<TileSettings>, With<CurrentlySelected>),
    >,
) {
    //If there is some newly selected tileset
    if let Ok((newly_selected_tileset_ent, newly_selected_tileset_view)) =
        newly_selected_query.single()
    {
        //If there was some tileset selected before
        if let Ok((currently_selected_tileset_ent, mut currently_selected_tileset_view)) =
            currently_selected_query.single_mut()
        {
            //For each child of this parent
            for (entity, mut visible) in currently_selected_children_query.iter_mut() {
                //Make these children transparent
                visible.is_visible = false;
                //Remove their currently selected marker so our queries will run on less Tiles
                commands.entity(entity).remove::<CurrentlySelected>();
            }
            //Remove the currently selected flag from the tileset entity because it's no longer selected
            commands
                .entity(currently_selected_tileset_ent)
                .remove::<CurrentlySelected>();
            //Update the no longer selected tileset's tileset_view component
            let (camera_transform, _) = camera_transform_query.single_mut().unwrap();
            currently_selected_tileset_view.camera_transform = camera_transform.to_owned();
        }
        //Change the camera's position to the newly selected tileset_view's camera_transform
        let (mut camera_transform, mut camera_zoom_limit) =
            camera_transform_query.single_mut().unwrap();
        *camera_transform = newly_selected_tileset_view.camera_transform;
        *camera_zoom_limit = newly_selected_tileset_view.camera_zoom_limits;
        //Remove the newly selected marker and insert currently selected
        commands
            .entity(newly_selected_tileset_ent)
            //First remove it's newly selected flag
            .remove::<NewlySelected>()
            //Then add it a currently selected flag
            .insert(CurrentlySelected);
        //Filter the query on children that their parent is the one newly selected
        let filtered = currently_unselected_children_query
            .iter_mut()
            .filter(|(_entity, parent, _visible)| parent.0 == newly_selected_tileset_ent);
        for (entity, _parent, mut visible) in filtered.into_iter() {
            //Mark the child selected
            commands.entity(entity).insert(CurrentlySelected);
            //Make the child visible
            visible.is_visible = true;
        }
    }
}
