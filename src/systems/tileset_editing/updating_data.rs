use crate::data::{
    shared_components::CurrentlySelected,
    tile_entity::{TileData, TileRect, TileSettings},
    tileset_entity::{NewlySelected, TileSetView},
};
use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_common::input::{data_components::CameraZoomLimit, marker_components::MainCamera};
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
///This function recalculates a tiles rect in world coordinates
pub fn recalculate_tile_rect(
    mut query: Query<(&GlobalTransform, &TileSettings, &mut TileRect), Changed<GlobalTransform>>,
) {
    for (global_transform, tile_settings, mut tile_rect) in query.iter_mut() {
        let half_tile_width = tile_settings.tile_width as f32 / 2.0;
        let half_tile_height = tile_settings.tile_height as f32 / 2.0;
        //Tile
        tile_rect.left =
            global_transform.translation.x - half_tile_width * global_transform.scale.x;
        tile_rect.right =
            global_transform.translation.x + half_tile_width * global_transform.scale.x;
        tile_rect.top =
            global_transform.translation.y + half_tile_height * global_transform.scale.y;
        tile_rect.bottom =
            global_transform.translation.y - half_tile_height * global_transform.scale.y;
    }
}
