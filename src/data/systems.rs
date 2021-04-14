use crate::AppState;

use super::{
    shared_components::Uninitiated,
    tile_entity::{TileData, TileSettings},
    tileset_entity::TileSetSettings,
};
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
use bevy::{prelude::*, tasks::ComputeTaskPool};
pub fn init_tileset(
    mut commands: Commands,
    query: Query<(Entity, &TileSetSettings), With<Uninitiated>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    //println!("Called init_tileset, {:?}", query.iter().size_hint());
    for (tileset_entity, tileset_settings) in query.iter() {
        //This should probably always succede but
        let window = windows.get_primary().unwrap();
        let scale =
            get_scale_fit_tileset_to_screen(tileset_settings, window.width(), window.height());
        /*
        let translation:Vec3;
        if tileset_settings.tileset_height!=1 || tileset_settings.tileset_width!=1 {
            let (tileset_total_width, tileset_total_height) = get_total_tileset_size(tileset_settings);
            translation =/* (1.0/ scale) **/ Vec3::new(tileset_total_width * -1.0, tileset_total_height * -1.0, 0.0);
        }
        else{
            translation = Vec3::default();
        }
         */
        //TODO: Disable 1 pixel sprite creation
        let texture = Texture::default();
        commands
            .entity(tileset_entity)
            .insert(Transform {
                scale:Vec3::new(scale,scale,1.0),
                //translation,
                ..Default::default()
            })
            .with_children(|tileset_parent| {
                //println!("{:?}",tileset_settings);
                for y_tileset in 0..tileset_settings.tileset_height {
                    for x_tileset in 0..tileset_settings.tileset_width {
                        let texture_handle = textures.add(texture.clone());
                        let material_handle = materials.add(ColorMaterial::texture(texture_handle));
                        tileset_parent.spawn_bundle(super::tile_entity::TileBundle::new(
                            TileSettings{tile_width:tileset_settings.tile_width,tile_height:tileset_settings.tile_height},
                            material_handle,
                            Transform {
                                //scale,
                                translation: /*scale
                                    * */ Vec3::new(
                                        (x_tileset as f32 - tileset_settings.tileset_width as f32/2.0) * tileset_settings.tile_width as f32 + tileset_settings.tile_height as f32/2.0,
                                        (y_tileset as f32 - tileset_settings.tileset_height as f32/2.0) * tileset_settings.tile_height as f32 + tileset_settings.tile_height as f32/2.0,
                                        0.0,
                                    ),
                                ..Default::default()
                            },
                        ));
                        //Works: commands.spawn_bundle(TileBundle::new(material_handle));
                        //commands.spawn_bundle(SpriteBundle{material:material_handle,..Default::default()});
                    }
                }
            });
        commands.entity(tileset_entity).remove::<Uninitiated>();
    }
}

pub fn get_total_tileset_size(tileset_settings: &TileSetSettings) -> (f32, f32) {
    (
        tileset_settings.tileset_width as f32 * tileset_settings.tile_width as f32,
        tileset_settings.tileset_height as f32 * tileset_settings.tile_height as f32,
    )
}
pub fn get_scale_fit_tileset_to_screen(
    tileset_settings: &TileSetSettings,
    window_width: f32,
    window_height: f32,
) -> f32 {
    let (tileset_total_width, tileset_total_height) = get_total_tileset_size(tileset_settings);
    let percent;
    if tileset_total_height >= tileset_total_width {
        percent = tileset_total_height / (window_height - 100.0);
    } else {
        percent = tileset_total_width / (window_width - 100.0);
    }
    1.0 / percent
}
pub fn init_tile(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(Entity, &TileSettings, &mut TileData, &mut Visible), With<Uninitiated>>,
) {
    println!("Called init_tile");
    //Initiate a transparent tile in a parralel manner
    /*query.par_for_each_mut(
        &pool,
        1,
        |(entity, tile_settings, mut tile_data, mut visible)| {
            //Creating a transparent texture once
            //let mut sprite_data: Vec<u8> = Vec::with_capacity(
            //    (tile_settings.tile_width * tile_settings.tile_height * 4) as usize,
            //);
            //Going in reverse because in textures y is higher at the bottom
            /*let mut vec = Vec::<u8>::with_capacity(
                (tile_settings.tile_width * tile_settings.tile_height * 4) as usize,
            );
            for _y_tile in (0..tile_settings.tile_height).rev() {
                for _x_tile in 0..tile_settings.tile_width {
                    vec.push(255);
                    vec.push(255);
                    vec.push(255);
                    vec.push(0);
                }
            }
            visible.is_visible = true;
            tile_data.data = vec;
            */
            println!("{:?}", entity);
        },
    );
    */

    for (_, tile_settings, mut tile_data, mut visible) in query.iter_mut() {
        tile_data.data = Vec::<u8>::with_capacity(
            (tile_settings.tile_width * tile_settings.tile_height * 4) as usize,
        );
        for y_tile in (0..tile_settings.tile_height).rev() {
            for x_tile in 0..tile_settings.tile_width {
                tile_data.data.push(255);
                tile_data.data.push(255);
                tile_data.data.push(255);
                tile_data.data.push(0);
            }
        }
        visible.is_visible = true;
    }
    //Remove the Unitiated component from the entity
    for (entity, _, _, _) in query.iter_mut() {
        commands.entity(entity).remove::<Uninitiated>();
    }
}
pub fn update_textures_for_changed_tile_data(
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&TileSettings, &TileData, &Handle<ColorMaterial>), Changed<TileData>>,
    mut app_state: ResMut<State<AppState>>,
) {
    println!("Called update_textures_for_changed_tile_data:");
    let mut count: u32 = 0;
    for (tile_settings, tile_data, material_handle) in query.iter() {
        //This shouldn't fail really, i shouldn't delete any of them anywhere
        let material = materials.get_mut(material_handle).unwrap();
        let texture_handle = textures.add(Texture::new(
            Extent3d::new(tile_settings.tile_width, tile_settings.tile_height, 1),
            TextureDimension::D2,
            tile_data.data.clone(),
            TextureFormat::Rgba8UnormSrgb,
        ));
        material.texture = Some(texture_handle);
        count += 1;
    }
    if count != 0 {
        app_state.set(AppState::EditingTileSet).unwrap();
    }
}
