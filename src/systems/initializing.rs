use crate::data::{
    shared_components::Uninitiated,
    tile_entity::{TileBundle, TileData, TilePosition, TileSettings},
    tileset_entity::{TileSetSettings, TileSetView},
};
use bevy::utils::HashMap;
use bevy::{prelude::*, tasks::ComputeTaskPool};
use bevy_common::input::data_components::CameraZoomLimit;
///Initiates a newly Created [TileSetBundle](TileSetBundle) entity and it's [TileBundle](TileBundle) children
pub fn init_tileset(
    mut commands: Commands,
    mut query: Query<(Entity, &TileSetSettings, &mut TileSetView), With<Uninitiated>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    for (tileset_entity, tileset_settings, mut tileset_view) in query.iter_mut() {
        //This should probably always succede but
        let window = windows.get_primary().unwrap();
        //Get The scale to fit the tileset to screen
        let scale =
            get_scale_fit_tileset_to_screen(tileset_settings, window.width(), window.height());
        //Each tileset stores it's camera's settings
        //We want the tileset's camera to zoom to certain limits, since each tileset has a different size that means
        //We Currently only set this up at the tileset's creation
        //TODO: If the Edit context menu will allow to change tileset size, this ^ would need to be updated
        tileset_view.camera_zoom_limits = CameraZoomLimit {
            max_zoom: Vec3::new(scale / 10.0, scale / 10.0, 1.0),
            min_zoom: Vec3::new(1.0, 1.0, 1.0),
        };
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
                            TilePosition {
                                position: UVec2::new(x_tileset as u32, y_tileset as u32),
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
}
///Calculates the total size of the [TileSetBundle](TileSetBundle)
pub fn get_total_tileset_size_pixels(tileset_settings: &TileSetSettings) -> Vec2 {
    Vec2::new(
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
    let tileset_total_size = get_total_tileset_size_pixels(tileset_settings);
    let percent;
    if tileset_total_size.y >= tileset_total_size.x {
        percent = tileset_total_size.y / (window_height - 100.0);
    } else {
        percent = tileset_total_size.x / (window_width - 100.0);
    }
    1.0 / percent
}
///FIXME: When this commit: https://github.com/bevyengine/bevy/pull/1945 is merged, change to this instead of init_tile_seq?
///
///This initiates a newly created [TileBundle](TileBundle) in a parralel manner
#[allow(dead_code)]
pub fn init_tile_par(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(Entity, &TileSettings, &mut TileData), With<Uninitiated>>,
) {
    query.par_for_each_mut(&pool, 100, |(_entity, tile_settings, mut tile_data)| {
        //Creating a transparent texture once
        let mut sprite_data: Vec<u8> =
            Vec::with_capacity((tile_settings.tile_width * tile_settings.tile_height * 4) as usize);
        //Going in reverse because in textures y is higher at the bottom
        for _y_tile in (0..tile_settings.tile_height).rev() {
            for _x_tile in 0..tile_settings.tile_width {
                sprite_data.push(0);
                sprite_data.push(0);
                sprite_data.push(0);
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
    //Honestly, there shouldn't be two sets of new tiles in the same frame but to be safe
    //Also, this could belong in a Local, but i don't even know if it'll be a perf improvement
    let mut hm: HashMap<(usize, usize), Vec<u8>> = HashMap::default();

    for (entity, tile_settings, mut tile_data/*, mut visible*/) in query.iter_mut() {
        if let Some(texture_data) = hm.get(&(tile_settings.tile_width, tile_settings.tile_height)) {
            //Cloning existing texture data for tile_settings we already encountered
            tile_data.data = texture_data.clone();
        } else {
            //Creating a transparent texture for these newly encountered settings
            let mut texture_data = Vec::<u8>::with_capacity(
                (tile_settings.tile_width * tile_settings.tile_height * 4) as usize,
            );
            //TODO: This could be a one dimensional loop, maybe replace it after you copied into the pencil mechanism
            for _y_tile in (0..tile_settings.tile_height).rev() {
                for _x_tile in 0..tile_settings.tile_width {
                    texture_data.push(0);
                    texture_data.push(0);
                    texture_data.push(0);
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
