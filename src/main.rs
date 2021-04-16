use bevy::{
    ecs::component::ComponentDescriptor,
    render::{
        pipeline::PipelineDescriptor,
        shader::{Shader, ShaderStage},
    },
};
use bevy::{ecs::component::StorageType, prelude::*, reflect::TypeUuid};
use bevy_common::input::{
    bundles::{CommonCameraBundle, MainCameraBundle},
    data_components::{CameraMoveSpeed, CameraZoomLimit},
    events::MouseDragEvent,
    resources::MouseWorldPosition,
    systems::*,
};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;

mod data;
mod systems;
mod ui;
use data::{
    shared_components::{CurrentlySelected, Uninitiated},
    tile_entity::TileBundle,
    tileset_entity::{NewlySelected, TileSetBundle},
};
use systems::{initializing::*, tileset_editing::*};

///TODO: The default font for the app, everything should use this
pub const DEFAULT_FONT: &str = "Roboto-Regular.ttf";
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
///The app's current state
pub enum AppState {
    ///There isn't any edited tileset atm
    Empty,
    ///We're creating a new tileset, we need to run systems to initialize it
    CreateNewTileSet,
    ///We're editing some tileset now, we need to run systems to handle user input and data updating
    EditingTileSet,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabels {
    DrawGui,
    GetMousePos,
    TrackMiddleMouseDragging,
    DrawSomething,
    InitTileset,
    InitTile,
    ChangeTileData,
    UpdateTexturesForVisual,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum StageLabels {
    ///Initialize the newly created [TileSetBundle](TileSetBundle)
    InitalizeTileSet,
    ///Updates the currently selected [TileSetBundle](TileSetBundle)
    UpdateView,
    ///Initialize the newly created [TileBundle](TileBundle)
    InitializeTiles,
    UpdateTiles,
}
fn main() {
    AppBuilder::default()
        //Turning on deps
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor { 0: Color::BLACK })
        //.add_plugins(TilemapDefaultPlugins)
        //Enabling Logging:
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        //Startup
        .register_component(ComponentDescriptor::new::<Uninitiated>(
            StorageType::SparseSet,
        ))
        .register_component(ComponentDescriptor::new::<NewlySelected>(
            StorageType::SparseSet,
        ))
        .register_component(ComponentDescriptor::new::<CurrentlySelected>(
            StorageType::SparseSet,
        ))
        .add_startup_system(spawn_cameras_system.system())
        .add_startup_system(setup_tile_pipeline.system())
        //We always need our gui to be drawn
        .add_system(
            ui::bevy_egui::draw_gui
                .system()
                .label(SystemLabels::DrawGui),
        )
        //Here we initiallize our newly created tileset
        .add_stage_after(
            CoreStage::Update,
            StageLabels::InitalizeTileSet,
            SystemStage::single_threaded().with_system(init_tileset.system()),
        )
        //Initialize the newly created tiles
        .add_stage_after(
            StageLabels::InitalizeTileSet,
            StageLabels::InitializeTiles,
            SystemStage::single_threaded().with_system(init_tile_par.system()),
        )
        //Here we set the currently selected view
        .add_stage_after(
            StageLabels::InitializeTiles,
            StageLabels::UpdateView,
            SystemStage::single_threaded().with_system(update_selected_tileset.system()),
        )
        //This is the stage where we can actually use the app
        //We need a mouse world position resource for this
        .insert_resource(MouseWorldPosition::default())
        .add_event::<MouseDragEvent>()
        .add_stage_after(
            StageLabels::UpdateView,
            StageLabels::UpdateTiles,
            SystemStage::parallel()
                .with_system(
                    get_mouse_world_position
                        .system()
                        .label(SystemLabels::GetMousePos),
                )
                .with_system(
                    track_middle_mouse_dragging
                        .system()
                        .label(SystemLabels::TrackMiddleMouseDragging)
                        .after(SystemLabels::GetMousePos),
                )
                .with_system(
                    move_camera_with_middle_mouse_drag
                        .system()
                        .after(SystemLabels::TrackMiddleMouseDragging),
                )
                .with_system(
                    use_brush
                        .system()
                        .label(SystemLabels::DrawSomething)
                        .after(SystemLabels::GetMousePos),
                )
                .with_system(
                    update_textures_for_changed_tile_data
                        .system()
                        .label(SystemLabels::UpdateTexturesForVisual)
                        .after(SystemLabels::DrawSomething),
                )
                .with_system(move_camera_with_wasd_scaled_by_zoom.system())
                .with_system(zoom_in_camera_with_mouse_scroll.system()),
        )
        .run();
}
pub const CUSTOM_SPRITE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 2785347850338765446);

fn setup_tile_pipeline(
    mut render_pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    //If the sprite's original render pipeline already exists
    if let Some(original_sprite_render_pipeline) =
        render_pipelines.get(bevy::sprite::SPRITE_PIPELINE_HANDLE)
    {
        //We can clone the pipeline so we won't have to copy the code for setting it up
        let mut pipeline_clone = original_sprite_render_pipeline.clone();
        //In this example, we only want to override the fragment shader and so:
        pipeline_clone.shader_stages.fragment = Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("sprite.frag"),
        )));
        //Adding our custom pipeline and making it untracked so it won't get removed automatically when no sprite uses it
        render_pipelines.set_untracked(CUSTOM_SPRITE_PIPELINE_HANDLE, pipeline_clone);
    }
}
///Spawns The Cameras Needed for the editor
fn spawn_cameras_system(mut commands: Commands) {
    //Spawning the camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(MainCameraBundle {
            common_camera: CommonCameraBundle {
                move_speed: CameraMoveSpeed { speed: 650.0 },
                zoom_limit: CameraZoomLimit {
                    max_zoom: Vec3::new(0.05, 0.05, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 20.0,
            ..Default::default()
        },
        ShapeColors::new(Color::BLACK),
        DrawMode::Fill(FillOptions::default()),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    ));
}
