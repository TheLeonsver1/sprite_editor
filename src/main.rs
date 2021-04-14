use bevy::{
    ecs::component::ComponentDescriptor,
    render::{
        pipeline::{
            BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite, CompareFunction,
            CullMode, DepthBiasState, DepthStencilState, FrontFace, PipelineDescriptor,
            PolygonMode, PrimitiveState, PrimitiveTopology, StencilFaceState, StencilState,
        },
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
};
use bevy::{
    ecs::component::{Component, StorageType},
    prelude::*,
    reflect::TypeUuid,
    render::camera::RenderLayers,
};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_tilemap::prelude::*;

use enum_index::EnumIndex;
use enum_index_derive::EnumIndex;
mod data;
use data::{shared_components::Uninitiated, systems::*};
mod ui;

///The default font for the app, everything should use this
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
#[derive(Debug, EnumIndex, Clone)]
///The different cameras that view the world and their corresponding render layers(their index in the enum is their layer)
enum Windows {
    ///The Main Editor Panel, here you edit the tileset
    TileSetEditor,
    ///The Minimized View of the tileSetView
    ColorPicker,
    ///The Panel where you can test your tilesets, you can edit your tiles here too
    TilePlayground,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabels {
    DrawGui,
    InitTileset,
    InitTile,
    ChangeTileData,
    UpdateTexturesForVisual,
    ChangeStateToEditing,
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
        .add_startup_system(spawn_cameras_system.system())
        .add_startup_system(setup_tile_pipeline.system())
        //We always need our gui to be drawn
        .add_system(
            ui::bevy_egui::draw_gui
                .system()
                .label(SystemLabels::DrawGui),
        )
        //Start the application logic that is dependent on state
        .add_state(AppState::Empty)
        .add_system_set(
            SystemSet::on_update(AppState::CreateNewTileSet)
                .with_system(init_tileset.system().label(SystemLabels::InitTileset))
                .with_system(
                    init_tile
                        .system()
                        .label(SystemLabels::InitTile)
                        .after(SystemLabels::InitTileset),
                )
                .with_system(
                    update_textures_for_changed_tile_data
                        .system()
                        .label(SystemLabels::UpdateTexturesForVisual)
                        .after(SystemLabels::InitTile),
                ),
        )
        //.add_system_set(add_init_systems_to_system_set(SystemSet::on_resume(AppState::CreateNewTileSet)))
        //.add_system_set(SystemSet::on_update(AppState::EditingTileSet).with_system(update_textures_for_changed_tile_data.system().label(SystemLabels::UpdateTexturesForVisual)))
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
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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
