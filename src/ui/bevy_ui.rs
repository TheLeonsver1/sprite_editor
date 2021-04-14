use bevy::prelude::*;
///Spawns The App's interface
fn spawn_interface(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load(crate::DEFAULT_FONT);
    let transparent_material = materials.add(ColorMaterial::color(Color::rgba(1.0, 1.0, 1.0, 0.0)));
    //Root
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: transparent_material,
            ..Default::default()
        })
        .with_children(|root| {
            //Banner
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(
                        Val::Percent(100.0),
                        Val::Percent(4.0), /*Val::Px(40.0)*/
                    ),
                    align_self: AlignSelf::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|banner| {
                spawn_banner_button(banner, "File", font.to_owned());
                spawn_banner_button(banner, "Options", font.to_owned());
                //spawn_banner_button(banner, "File",font);
            });
            //File Drop Down
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(4.0), Val::Percent(96.0)),
                    align_self: AlignSelf::FlexStart,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|banner| {
                spawn_banner_button(banner, "New", font.to_owned());
                spawn_banner_button(banner, "Load", font.to_owned());
                //spawn_banner_button(banner, "File",font);
            });
        });
}
///Utility function to spawn buttons to eliminate boilerplate
fn spawn_banner_button(banner: &mut ChildBuilder, name: &str, font: Handle<Font>) {
    let padding = Val::Px(4.0);
    banner
        .spawn_bundle(ButtonBundle {
            style: Style {
                padding: Rect {
                    bottom: padding,
                    top: padding,
                    left: padding,
                    right: padding,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|button| {
            button.spawn_bundle(TextBundle {
                text: Text::with_section(
                    name,
                    TextStyle {
                        color: Color::BLACK,
                        font_size: 20.0,
                        font,
                        ..Default::default()
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Top,
                    },
                ),
                ..Default::default()
            });
        });
}
