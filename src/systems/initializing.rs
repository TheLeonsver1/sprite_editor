use crate::{
    data::{
        shared_components::Uninitiated,
        tile_entity::{TileBundle, TileData, TileSettings},
        tileset_entity::TileSetSettings,
    },
    AppState,
};
use bevy::{prelude::*, tasks::ComputeTaskPool};
use bevy::{
    render::texture::{Extent3d, TextureDimension, TextureFormat},
    utils::HashMap,
};
///Initiates a new TileSet entity and it's tile children
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
                        tileset_parent.spawn_bundle(TileBundle::new(
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
///Calculates the total size of the tileset
pub fn get_total_tileset_size(tileset_settings: &TileSetSettings) -> (f32, f32) {
    (
        tileset_settings.tileset_width as f32 * tileset_settings.tile_width as f32,
        tileset_settings.tileset_height as f32 * tileset_settings.tile_height as f32,
    )
}
///Returns the scale(on one axis) that is required to fit the tileset in the screen
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
///FIXME:(maybe) This crashes for some reason
///This initiates a newly created tile in a parralel manner
#[allow(dead_code)]
pub fn init_tile_par(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(Entity, &TileSettings, &mut TileData, &mut Visible), With<Uninitiated>>,
) {
    query.par_for_each_mut(
        &pool,
        1,
        |(entity, tile_settings, mut tile_data, mut visible)| {
            //Creating a transparent texture once
            let mut sprite_data: Vec<u8> = Vec::with_capacity(
                (tile_settings.tile_width * tile_settings.tile_height * 4) as usize,
            );
            //Going in reverse because in textures y is higher at the bottom
            for _y_tile in (0..tile_settings.tile_height).rev() {
                for _x_tile in 0..tile_settings.tile_width {
                    sprite_data.push(255);
                    sprite_data.push(255);
                    sprite_data.push(255);
                    sprite_data.push(0);
                }
            }
            visible.is_visible = true;
            tile_data.data = sprite_data;
            println!("{:?}", entity);
        },
    );

    //Remove the Unitiated component from the entity
    for (entity, _, _, _) in query.iter_mut() {
        commands.entity(entity).remove::<Uninitiated>();
    }
}
///This initiates a newly created tile in a sequential manner
pub fn init_tile_seq(
    mut commands: Commands,
    mut query: Query<(Entity, &TileSettings, &mut TileData, &mut Visible), With<Uninitiated>>,
) {
    //Honestly, there shouldn't be two sets of new tiles in the same frame but to be safe
    //Also, this could belong in a Local, but i don't even know if it's a perf improvement
    let mut hm: HashMap<(u32, u32), Vec<u8>> = HashMap::default();

    for (entity, tile_settings, mut tile_data, mut visible) in query.iter_mut() {
        if let Some(texture_data) = hm.get(&(tile_settings.tile_width, tile_settings.tile_height)) {
            //Cloning existing texture data for tile_settings we already encountered
            tile_data.data = texture_data.clone();
        } else {
            //Creating a transparent texture for these newly encountered settings
            let mut texture_data = Vec::<u8>::with_capacity(
                (tile_settings.tile_width * tile_settings.tile_height * 4) as usize,
            );
            //TODO: This could be a one dimensional loop, replace it after you copied into the brush mechanism
            for _y_tile in (0..tile_settings.tile_height).rev() {
                for _x_tile in 0..tile_settings.tile_width {
                    texture_data.push(255);
                    texture_data.push(255);
                    texture_data.push(255);
                    texture_data.push(0);
                }
            }
            //Insert this data to the hash map so we could just clone it instead of creating a new vec
            hm.insert(
                (tile_settings.tile_width, tile_settings.tile_height),
                texture_data.clone(),
            );
            //Don't forget to set the tile's data
            tile_data.data = texture_data;
        }
        //Make the tile visible since it should be immediately rendered after this is done
        visible.is_visible = true;
        //Remove the marker so the data won't be deleted if we decide to create a new Tileset later
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
