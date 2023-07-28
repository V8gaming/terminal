use std::env;

use bevy::prelude::{
    Resource, Handle, Font, ResMut,
    Image, Commands, Component, Res,
    TextStyle, TextSection, ReceivedCharacter, KeyCode,
    Input, Query, Text, With, Without,
    EventReader, Entity, Color, Assets,
    DespawnRecursiveExt
};

use bevy::window::Window;

use crate::basic::run_commands;
use crate::image::render_image;
use crate::setup::{FontResource, EntityDatabase};

#[derive(Resource, Default)]
pub struct MyMaterialResource {
    pub material: Handle<Image>,
}

/// Input Compenent Idenifier
#[derive(Component)]
pub struct TerminalInput;

/// Image Idenifier
#[derive(Resource, Default)]
pub struct ImageIdenifier{
    pub id: Vec<Option<u32>>,
    pub output_id: Option<u32>,
}

/// Output Compenent Idenifier
#[derive(Component)]
pub struct TerminalOutput;

#[derive(Resource, Default)]
pub struct History {
    pub vec: Vec<String>,
    pub index: usize,
}

#[derive(Resource, Default)]
pub struct TextInputResource {
    pub value: String,
}

pub fn text_input(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut text_input: ResMut<TextInputResource>,
    mut history: ResMut<History>,
) {
    if kbd.just_pressed(KeyCode::Up) && !history.vec.is_empty() {
        handle_up_key(&mut text_input, &mut history);
    }

    if kbd.just_pressed(KeyCode::Down) && !history.vec.is_empty() {
        handle_down_key(&mut text_input, &mut history);
    }

    if kbd.just_pressed(KeyCode::Tab) {
        let user = env::var("USER").unwrap();
        let path = strip_path(env::current_dir().unwrap().to_string_lossy().to_string(), &user);
        let absolute_path = format!("/home/{}/{}/", user,path);
        let files = std::fs::read_dir(absolute_path);
        if files.is_err() {
            return;
        }
        let file_name = files.unwrap().flatten().next().unwrap().file_name().into_string().unwrap_or("file name read failed".to_string());
        text_input.value.push_str(&file_name);

    }

    if kbd.just_pressed(KeyCode::Back) {
        text_input.value.pop();
    }

    for ev in evr_char.iter() {
        if !ev.char.is_control() {
            text_input.value.push(ev.char);
        }
    }
}

pub fn handle_return_key(
    mut text_input: ResMut<TextInputResource>, 
    mut history: ResMut<History>, 
    mut output: Query<&mut Text, (With<TerminalOutput>, Without<TerminalInput>)>, 
    kbd: Res<Input<KeyCode>>,
    mut commands: Commands,
    font_resource: Res<FontResource>,
    mut query: Query<Entity>,
    materials: ResMut<Assets<Image>>,
    mut image_idenifier: ResMut<ImageIdenifier>,
    window: Query<&mut Window>,
    database: ResMut<EntityDatabase>,
) {
    if kbd.just_pressed(KeyCode::Return) {
        let font = &font_resource.fira_sans;
        let user = env::var("USER").unwrap();
        let path = strip_path(env::current_dir().unwrap().to_string_lossy().to_string(), &user);
        let hostname = hostname::get().unwrap().to_string_lossy().to_string();
        let command_return = run_commands(text_input.value.clone());
        let mut parts = text_input.value.split_whitespace();
        let command = parts.next().unwrap();
        match command {
            "clear" => {
                for mut text in output.iter_mut() {
                    text.sections.clear()
                }
                let id = image_idenifier.id.clone();
                for i in id.iter().rev() {
                    if i.is_some() {
                        let image = query.iter_mut().collect::<Vec<Entity>>()[i.unwrap() as usize];
                        //println!("{}",image.index());
                        commands.entity(image).despawn_recursive();
                        image_idenifier.id.remove(image.index() as usize);
                    }
                }
            
            },
            "exit" => {
                std::process::exit(0);
            },
            _ => {
                for mut text in output.iter_mut() {
                    prepend_text_sections(&mut text.sections, font, &user, &hostname, &path, &text_input.value, &command_return);
                }
                history.vec.insert(0, text_input.value.clone());
            }
        }
        render_image(command, user.clone(), path.clone(), window, materials, 
            image_idenifier, commands, query, database);


    
        text_input.value.clear();
    }

}

fn prepend_text_sections(
    sections: &mut Vec<TextSection>,
    font: &Handle<Font>,
    user: &str, 
    hostname: &str, 
    path: &str, 
    text_input_value: &str, 
    command_return: &str
) {
    //println!("{path}");
    sections.insert(0, create_text_section(format!("{}@{}",user, hostname), font, 20.0, Color::GREEN));
    sections.insert(1, create_text_section(path.to_string(), font, 20.0, Color::BLUE));
    sections.insert(2, create_text_section(format!("$ {}", text_input_value), font, 20.0, Color::WHITE));
    sections.insert(3, create_text_section(format!("\n{}", command_return), font, 20.0, Color::WHITE));
}

fn create_text_section(value: String, font: &Handle<Font>, font_size: f32, color: Color) -> TextSection {
    TextSection { 
        value, 
        style: TextStyle {
            font: font.clone(),
            font_size,
            color,
        }
    }
}

fn handle_up_key(text_input: &mut ResMut<TextInputResource>, history: &mut ResMut<History>) {
    let index = history.index;
    text_input.value = history.vec[index].clone();
    if index < history.vec.len() - 1 {
        history.index += 1;
    }
}

fn handle_down_key(text_input: &mut ResMut<TextInputResource>, history: &mut ResMut<History>) {
    let index = history.index;
    text_input.value = history.vec[index].clone();
    if index > 0 {
        history.index -= 1;
    }
}

pub fn update_window_and_input_system(
    mut windows: Query<&mut Window>,
    mut input: Query<&mut Text, (With<TerminalInput>, Without<TerminalOutput>)>,
    text_input: Res<TextInputResource>,
) {
    let user = env::var("USER").unwrap();
    let path = strip_path(env::current_dir().unwrap().to_string_lossy().to_string(), &user);
    let hostname = hostname::get().unwrap().to_string_lossy().to_string();

    for mut window in windows.iter_mut() {
        window.title = format!("{}@{}:~{}", user, hostname, path);
    }
    for mut text in input.iter_mut() {
        text.sections[0].value = format!("{}@{}:", user, hostname);
        text.sections[1].value = format!("~{}", path);
        text.sections[2].value = format!("$ {}", text_input.value);
    }
}

fn strip_path(path: String, user: &str) -> String {
    path.strip_prefix("/home/").unwrap().to_string().strip_prefix(user).unwrap().to_string()
}
