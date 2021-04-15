use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};

use crate::data::{
    shared_components::Uninitiated,
    tile_entity::{TileData, TileSettings},
};

///This function updates the [Texture](Texture) of [ColorMaterial](ColorMaterial)s used by [TileBundle](TileBundle)s when their [TileData](TileData) is changed
pub fn update_textures_for_changed_tile_data(
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (&TileSettings, &TileData, &Handle<ColorMaterial>),
        (Changed<TileData>, Without<Uninitiated>),
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
