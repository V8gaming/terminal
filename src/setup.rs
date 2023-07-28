use std::collections::HashMap;

use bevy::{window::WindowResized, prelude::{
    Resource, Handle, Font, ResMut,
    NodeBundle, Commands, AssetServer, Res,
    TextStyle, TextSection, Style, Val,
    FlexDirection, JustifyContent, AlignItems, Overflow,
    EventReader, TextBundle, Color, Camera2dBundle, BuildChildren
}};
use crate::{update::{History, TerminalInput, TerminalOutput, ImageIdenifier}, terminal::Terminal};

#[derive(Default, Resource)]
pub struct FontResource {
    pub fira_sans: Handle<Font>,
}

#[derive(Default, Resource)]
pub struct EntityDatabase {
    pub hashmap: HashMap<String, u32>
}

#[derive(Default, Resource)]
pub struct NodeResource {
    pub bundle: NodeBundle
}

pub fn load_font(mut commands: Commands, asset_server: Res<AssetServer>, terminal: Res<Terminal>) {
    let font_handle = asset_server.load(terminal.font_path());
    commands.insert_resource(FontResource { fira_sans: font_handle });
}

pub fn setup(mut commands: Commands, 
    mut history: ResMut<History>, 
    mut resize_events: EventReader<WindowResized>,
    font_resource: Res<FontResource>,
    mut image_idenifier: ResMut<ImageIdenifier>,
    terminal: Res<Terminal>,
    mut database: ResMut<EntityDatabase>,
) {
    let font = &font_resource.fira_sans;
    history.vec.push("".to_string());
    commands.spawn(Camera2dBundle::default());
    //root node
    let mut window_height: f32 = 100.0;
    for e in resize_events.iter() { 
        window_height = e.height
    }
    let main_node = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }).id();
        database.hashmap.insert("main".to_string(), main_node.index());
            let output_node =
            commands.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(Val::Percent(100.0).try_sub_with_size(Val::Px(terminal.font_size()), window_height).unwrap()),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    overflow: Overflow::clip(),
                    ..Default::default()
                },
                 
                ..Default::default()

            }).id();
            database.hashmap.insert("output".to_string(), output_node.index());
            image_idenifier.output_id = Some(output_node.index());
            //println!("{}", output_node.index());
            commands.entity(output_node).with_children(|parent| {
                // output
                parent
                .spawn(TextBundle::from_sections([TextSection::from_style(
                    TextStyle {
                        font: font.clone(),
                        font_size: terminal.font_size(),
                        color: Color::WHITE,
                    },
                )]))
                .insert(TerminalOutput);
            });
            commands.entity(main_node).add_child(output_node);
            // input
            let input_node = commands.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(terminal.font_size()),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                ..Default::default()
            }).id();
            database.hashmap.insert("input".to_string(), input_node.index());
            commands.entity(main_node).add_child(input_node);
            commands.entity(input_node).with_children(|parent| 
                {
                parent.spawn(TextBundle::from_sections([
                    TextSection::from_style(TextStyle {
                        font: font.clone(),
                        font_size: terminal.font_size(),
                        color: Color::GREEN,
                    }),
                    TextSection::from_style(TextStyle {
                        font: font.clone(),
                        font_size: terminal.font_size(),
                        color: Color::BLUE,
                    }),
                    TextSection::from_style(TextStyle {
                        font: font.clone(),
                        font_size: terminal.font_size(),
                        color: Color::WHITE,
                    }),
                ]))
                .insert(TerminalInput {});
            });
}