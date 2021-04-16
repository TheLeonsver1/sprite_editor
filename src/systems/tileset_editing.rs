use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_common::input::marker_components::MainCamera;

use crate::data::{
    shared_components::{CurrentlySelected, Uninitiated},
    tile_entity::{TileData, TileSettings},
    tileset_entity::{NewlySelected, TileSetView},
};

///This function updates the [Texture](Texture) of [ColorMaterial](ColorMaterial)s used by [TileBundle](TileBundle)s when their [TileData](TileData) is changed
pub fn update_textures_for_changed_tile_data(
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (&TileSettings, &TileData, &Handle<ColorMaterial>),
        (
            Without<Uninitiated>,
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

pub fn use_brush() {}

///Changes the view from one tileset to another
pub fn update_selected_tileset(
    mut commands: Commands,
    mut camera_transform_query: Query<&mut Transform, With<MainCamera>>,
    newly_selected_query: Query<
        (Entity, &TileSetView),
        (With<NewlySelected>, Without<CurrentlySelected>),
    >,
    mut currently_selected_query: Query<(Entity, &mut TileSetView), With<CurrentlySelected>>,
    mut currently_selected_children_query: Query<
        (Entity, &Parent, &mut Visible),
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
            //Filter the query on children that their parent is the one selected before
            let filtered = currently_selected_children_query
                .iter_mut()
                .filter(|(_entity, parent, _visible)| parent.0 == currently_selected_tileset_ent);
            //For each child of this parent
            for (entity, _parent, mut visible) in filtered.into_iter() {
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
            let camera_transform = camera_transform_query.single_mut().unwrap();
            currently_selected_tileset_view.camera_transform = camera_transform.to_owned();
        }
        //Change the camera's position to the newly selected tileset_view's camera_transform
        let mut camera_transform = camera_transform_query.single_mut().unwrap();
        *camera_transform = newly_selected_tileset_view.camera_transform;
        //Remove the newly selected marker and insert currently selected
        commands
            .entity(newly_selected_tileset_ent)
            //First remove it's newly selected flag
            .remove::<NewlySelected>()
            //Then add it a currently selected flag
            .insert(CurrentlySelected);
    }
}
