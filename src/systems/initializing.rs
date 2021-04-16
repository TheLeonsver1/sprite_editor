use crate::data::{
    shared_components::Uninitiated,
    tile_entity::{TileBundle, TileData, TileSettings},
    tileset_entity::TileSetSettings,
};
use bevy::utils::HashMap;
use bevy::{prelude::*, tasks::ComputeTaskPool};
///Initiates a newly Created [TileSetBundle](TileSetBundle) entity and it's [TileBundle](TileBundle) children
///
///TODO: Consider moving the calling of this fn into on_enter() and on_resume() and spawn the TileSetBundle entity in here to disentangle more gui and data(this would maybe be a bit annoying and would require a resource)
///TODO: Instead of states, mark the newest edited tileset here
pub fn init_tileset(
    mut commands: Commands,
    query: Query<(Entity, &TileSetSettings), With<Uninitiated>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    for (tileset_entity, tileset_settings) in query.iter() {
        //This should probably always succede but
        let window = windows.get_primary().unwrap();
        let scale =
            get_scale_fit_tileset_to_screen(tileset_settings, window.width(), window.height());
        //TODO: Disable 1 pixel sprite creation
        let texture = Texture::default();
        commands
            .entity(tileset_entity)
            .insert(Transform {
                scale: Vec3::new(scale, scale, 1.0),
                //translation,
                ..Default::default()
            })
            .with_children(|tileset_parent| {
                for y_tileset in 0..tileset_settings.tileset_height {
                    for x_tileset in 0..tileset_settings.tileset_width {
                        let texture_handle = textures.add(texture.clone());
                        let material_handle = materials.add(ColorMaterial::texture(texture_handle));
                        tileset_parent.spawn_bundle(TileBundle::new(
                            TileSettings {
                                tile_width: tileset_settings.tile_width,
                                tile_height: tileset_settings.tile_height,
                            },
                            material_handle,
                            Transform {
                                //scale,
                                translation: Vec3::new(
                                    (x_tileset as f32
                                        - tileset_settings.tileset_width as f32 / 2.0)
                                        * tileset_settings.tile_width as f32
                                        + tileset_settings.tile_height as f32 / 2.0,
                                    (y_tileset as f32
                                        - tileset_settings.tileset_height as f32 / 2.0)
                                        * tileset_settings.tile_height as f32
                                        + tileset_settings.tile_height as f32 / 2.0,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                        ));
                    }
                }
            });
        //Don't forget to remove the marker component so this function won't run for it again if another tileset is created
        commands.entity(tileset_entity).remove::<Uninitiated>();
    }
    //TODO: Make all other tilesets invisible and set their view based on current camera position
}
///Calculates the total size of the [TileSetBundle](TileSetBundle)
pub fn get_total_tileset_size(tileset_settings: &TileSetSettings) -> (f32, f32) {
    (
        tileset_settings.tileset_width as f32 * tileset_settings.tile_width as f32,
        tileset_settings.tileset_height as f32 * tileset_settings.tile_height as f32,
    )
}
///Returns the scale(on one axis) that is required to fit the [TileSetBundle](TileSetBundle) in the screen
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
///
///This initiates a newly created [TileBundle](TileBundle) in a parralel manner
#[allow(dead_code)]
pub fn init_tile_par(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(Entity, &TileSettings, &mut TileData), With<Uninitiated>>,
) {
    query.par_for_each_mut(&pool, 100, |(entity, tile_settings, mut tile_data)| {
        //Creating a transparent texture once
        let mut sprite_data: Vec<u8> =
            Vec::with_capacity((tile_settings.tile_width * tile_settings.tile_height * 4) as usize);
        //Going in reverse because in textures y is higher at the bottom
        for _y_tile in (0..tile_settings.tile_height).rev() {
            for _x_tile in 0..tile_settings.tile_width {
                sprite_data.push(255);
                sprite_data.push(255);
                sprite_data.push(255);
                sprite_data.push(0);
            }
        }
        tile_data.data = sprite_data;
    });

    //Remove the Unitiated component from the entity
    for (entity, _, _) in query.iter_mut() {
        commands.entity(entity).remove::<Uninitiated>();
    }
}
///This initiates a newly created [TileBundle](TileBundle) in a sequential manner
pub fn init_tile_seq(
    mut commands: Commands,
    mut query: Query<(Entity, &TileSettings, &mut TileData), With<Uninitiated>>,
) {
    //println!("update");
    //Honestly, there shouldn't be two sets of new tiles in the same frame but to be safe
    //Also, this could belong in a Local, but i don't even know if it'll be a perf improvement
    let mut hm: HashMap<(u32, u32), Vec<u8>> = HashMap::default();

    for (entity, tile_settings, mut tile_data/*, mut visible*/) in query.iter_mut() {
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
        //Remove the marker so the data won't be deleted if we decide to create a new Tileset later
        commands.entity(entity).remove::<Uninitiated>();
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::SystemLabels;
    use bevy::prelude::*;

    #[test]
    #[should_panic]
    fn did_init_tileset() {
        let mut world = World::default();
        world.insert_resource(Assets::<Texture>::);
        let mut update_stage = SystemStage::parallel();
        //Create the default tileset settings
        let tileset_settings = TileSetSettings::default();
        let entity = world
            .spawn()
            .insert_bundle(TileSetBundle::new(tileset_settings))
            .id();
        //Checks whether the TileSetBundle has the marker componet by default
        assert!(world.get::<Uninitiated>(entity).is_some());
        update_stage.add_system(init_tileset.system().label(SystemLabels::InitTileset));
        update_stage.run(&mut world);
        assert!(world.get::<Uninitiated>(entity).is_none());
        update_stage.add_system(
            init_tile_seq
                .system()
                .label(SystemLabels::InitTile)
                .after(SystemLabels::InitTile),
        );
        //world.
        //let mut state
        //let mut
    }
    fn try_app_buidler() {
        let app = AppBuilder::default().add_plugins(DefaultPlugins).run();
    }
}
*/
