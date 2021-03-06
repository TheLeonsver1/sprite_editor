use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::data::{
    shared_components::CurrentlySelected,
    tileset_entity::{NewlySelected, TileSetBundle, TileSetName, TileSetSettings},
};
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
}
///Drawing the egui app ui
pub fn draw_gui(
    mut commands: Commands,
    mut ctx_menu_state: Local<ContextMenuState>,
    mut new_tileset_window_data: Local<TileSetSettings>,
    mut tileset_entities: Local<Vec<Entity>>,
    mut tileset_entity_names_query: Query<&mut TileSetName>,
    selected_tileset_entity_query: Query<&CurrentlySelected>,
    mut added_tilesets: Local<u32>,
    egui_context: ResMut<EguiContext>,
    input: Res<Input<KeyCode>>,
) {
    //Todo: implement clean/revert on escape
    if input.pressed(KeyCode::Escape) {
        *ctx_menu_state = ContextMenuState::None;
    }
    let ctx = egui_context.ctx();
    //The Menu
    egui::TopPanel::top("my_top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("File").clicked() {
                *ctx_menu_state = ContextMenuState::File(SelectedFileContextMenuItem::None);
            };
            if ui.button("Options").clicked() {
                *ctx_menu_state = ContextMenuState::Options(SelectedOptionsContextMenuItem::None);
            };
        });
    });
    //Side Panel for Context Menu
    if *ctx_menu_state != ContextMenuState::None {
        //Create a side menu
        egui::SidePanel::left("context_menu", 200.0).show(ctx, |ui|{
            //Make a vertical ui
            ui.vertical(|ui|{
                match &*ctx_menu_state{
                    //If we want to display the File ui, show appropriate ui
                    ContextMenuState::File(selected) => {
                        //If we pressed the new button now or earlier, show a window for that
                        if *selected == SelectedFileContextMenuItem::New ||  ui.button("New").clicked() {
                            //Make sure the window doesn't disappear on the next update
                            *ctx_menu_state = ContextMenuState::File(SelectedFileContextMenuItem::New);
                            //Showing the window itself
                            egui::Window::new("New Tileset").show(ctx, |ui|{
                                ui.vertical(|ui|{
                                    //Tile settings
                                    ui.label("Tile:");
                                    ui.horizontal(|ui|{
                                        ui.label("Width:").on_hover_text("A single tile's width");
                                        ui.add(egui::DragValue::new(&mut new_tileset_window_data.tile_width).clamp_range(1_f32..=u16::MAX as f32));
                                        ui.label("Height:").on_hover_text("A single tile's height");
                                        ui.add(egui::DragValue::new(&mut new_tileset_window_data.tile_height).clamp_range(1_f32..=u16::MAX as f32));
                                    });
                                    //Tileset settings
                                    ui.label("TileSet:");
                                    ui.horizontal(|ui|{
                                        ui.label("Width:").on_hover_text("How many tiles in the horizontal direction of your tileset?");
                                        ui.add(egui::DragValue::new(&mut new_tileset_window_data.tileset_width).clamp_range(1_f32..=64_f32));
                                        ui.label("Height:").on_hover_text("How many tiles in the vertical direction of your tileset?");
                                        ui.add(egui::DragValue::new(&mut new_tileset_window_data.tileset_height).clamp_range(1_f32..=64_f32));
                                    });
                                    //If we confirmed the creation of a new tileset
                                    if ui.button("Create new").clicked(){
                                        //Spawn a TileSet Entity
                                        let new_tileset_entity = commands.spawn_bundle(TileSetBundle::new(*new_tileset_window_data,*added_tilesets + 1)).id();
                                        tileset_entities.push(new_tileset_entity);
                                        *new_tileset_window_data = TileSetSettings::default();
                                        *ctx_menu_state = ContextMenuState::None;
                                        *added_tilesets += 1;
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

    //Tab View
    //if entities were created
    if tileset_entities.len() > 0 {
        //Show a tab view of them
        egui::TopPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for entity in tileset_entities.iter() {
                    if let Ok(mut tileset_name) = tileset_entity_names_query.get_mut(*entity) {
                        if ui
                            .selectable_label(
                                selected_tileset_entity_query.get(*entity).is_ok(),
                                tileset_name.name.clone(),
                            )
                            .clicked()
                            && selected_tileset_entity_query.get(*entity).is_err()
                        {
                            commands.entity(*entity).insert(NewlySelected);
                        }
                    }
                }
            });
        });
    }
}
