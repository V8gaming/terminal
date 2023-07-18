use bevy::{prelude::*, window::WindowResized};
use crate::update::{History, TerminalInput, TerminalOutput};

#[derive(Default, Resource)]
pub struct FontResource {
    pub fira_sans: Handle<Font>,
}

#[derive(Default, Resource)]
pub struct NodeResource {
    pub bundle: NodeBundle
}

pub fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("Fira_Sans/FiraSans-Bold.ttf");
    commands.insert_resource(FontResource { fira_sans: font_handle });
}

pub fn setup(mut commands: Commands, 
    mut history: ResMut<History>, 
    mut resize_events: EventReader<WindowResized>,
    font_resource: Res<FontResource>,
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
        
            let output_node =
            commands.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(Val::Percent(100.0).try_sub_with_size(Val::Px(20.0), window_height).unwrap()),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    overflow: Overflow::clip(),
                    ..Default::default()
                },
                 
                ..Default::default()

            }).id();
            //println!("{}", output_node.index());
            commands.entity(output_node).with_children(|parent| {
                // output
                parent
                .spawn(TextBundle::from_sections([TextSection::from_style(
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )]))
                .insert(TerminalOutput);
            });
            commands.entity(main_node).add_child(output_node);
            // input
            commands.entity(main_node).with_children(|parent| 
                {
                parent.spawn(TextBundle::from_sections([
                    TextSection::from_style(TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::GREEN,
                    }),
                    TextSection::from_style(TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::BLUE,
                    }),
                    TextSection::from_style(TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    }),
                ]))
                .insert(TerminalInput {});
            });
}