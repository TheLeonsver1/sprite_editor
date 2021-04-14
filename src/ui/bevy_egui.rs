use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::data::{
    resources::SelectedTileSetEntity,
    tileset_entity::{TileSetBundle, TileSetSettings},
};

#[derive(Default, Debug)]
pub struct UIState {
    pub context_menu: ContextMenuState,
    pub tileset_settings: TileSetSettings,
    pub amount_of_documents_opened: usize,
}
///The Current Selection of the Context Menu
#[derive(PartialEq, Eq, Debug)]
pub enum ContextMenuState {
    None,
    File(SelectedFileContextMenuItem),
    Options(SelectedOptionsContextMenuItem),
}
impl Default for ContextMenuState {
    fn default() -> Self {
        Self::None
    }
}
///The Current Selection of the File Context Menu
#[derive(PartialEq, Eq, Debug)]
pub enum SelectedFileContextMenuItem {
    None,
    New,
}
///The Current Selection of the Options Context Menu
#[derive(PartialEq, Eq, Debug)]
pub enum SelectedOptionsContextMenuItem {
    None,
    New,
}
///Drawing the egui app ui
pub fn draw_gui(
    mut commands: Commands,
    mut ctx_menu_state: Local<UIState>,
    mut egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<crate::AppState>>,
    input: Res<Input<KeyCode>>,
    selected_tileset: Option<ResMut<SelectedTileSetEntity>>,
) {
    //Todo: implement clean/revert on escape
    if input.pressed(KeyCode::Escape) {
        ctx_menu_state.context_menu = ContextMenuState::None;
    }
    let ctx = egui_context.ctx();
    //The header
    egui::TopPanel::top("my_top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("File").clicked() {
                ctx_menu_state.context_menu =
                    ContextMenuState::File(SelectedFileContextMenuItem::None);
            };
            if ui.button("Options").clicked() {
                ctx_menu_state.context_menu =
                    ContextMenuState::Options(SelectedOptionsContextMenuItem::None);
            };
        });
    });
    //If we pressed on to see some context menu
    if ctx_menu_state.context_menu != ContextMenuState::None {
        //Create a side menu
        egui::SidePanel::left("context_menu", 200.0).show(ctx, |ui|{
            //Make a vertical ui
            ui.vertical(|ui|{
                match &ctx_menu_state.context_menu{
                    //If we want to display the File ui, show appropriate ui
                    ContextMenuState::File(selected) => {
                        //If we pressed the new button now or earlier, show a window for that
                        if *selected == SelectedFileContextMenuItem::New ||  ui.button("New").clicked() {
                            //Make sure the window doesn't disappear on the next update
                            ctx_menu_state.context_menu = ContextMenuState::File(SelectedFileContextMenuItem::New);
                            //Showing the window itself
                            egui::Window::new("New Tileset").show(ctx, |ui|{
                                ui.vertical(|ui|{
                                    //Tile settings
                                    ui.label("Tile:");
                                    ui.horizontal(|ui|{
                                        ui.label("Width:").on_hover_text("A single tile's width");
                                        ui.add(egui::DragValue::new(&mut ctx_menu_state.tileset_settings.tile_width).clamp_range(1_f32..=u16::MAX as f32));
                                        ui.label("Height:").on_hover_text("A single tile's height");
                                        ui.add(egui::DragValue::new(&mut ctx_menu_state.tileset_settings.tile_height).clamp_range(1_f32..=u16::MAX as f32));
                                    });
                                    //Tileset settings
                                    ui.label("TileSet:");
                                    ui.horizontal(|ui|{
                                        ui.label("Width:").on_hover_text("How many tiles in the horizontal direction of your tileset?");
                                        ui.add(egui::DragValue::new(&mut ctx_menu_state.tileset_settings.tileset_width).clamp_range(1_f32..=64_f32));
                                        ui.label("Height:").on_hover_text("How many tiles in the vertical direction of your tileset?");
                                        ui.add(egui::DragValue::new(&mut ctx_menu_state.tileset_settings.tileset_height).clamp_range(1_f32..=64_f32));
                                    });
                                    //If we confirmed the creation of a new tileset
                                    if ui.button("Create new").clicked(){
                                        println!("Clicked Create New");
                                        //Spawn a TileSet Entity
                                        //TODO: Spawn A tileset
                                        let new_tileset_entity = commands.spawn_bundle(TileSetBundle::new(ctx_menu_state.tileset_settings)).id();
                                        if let Some(mut selected_entity_res) = selected_tileset{
                                            selected_entity_res.selected_entity = new_tileset_entity;
                                        }
                                        else{
                                            commands.insert_resource(SelectedTileSetEntity{selected_entity:new_tileset_entity});
                                        }
                                        ctx_menu_state.tileset_settings = TileSetSettings::default();
                                        ctx_menu_state.context_menu = ContextMenuState::None;
                                        app_state.set(crate::AppState::CreateNewTileSet).unwrap();
                                        println!("Set AppState::CreateNewTileSet");
                                    }
                                });
                            });
                        }
                    }
                    //If we want to display the Options ui, show appropriate ui
                    ContextMenuState::Options(_selected) => {

                    }
                    //This can't happen since it's checked above but rust is weird
                    ContextMenuState::None => {}
                }
            });
        });
    }
}
