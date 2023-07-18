use std::env;
use std::path::Path;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::ui::FocusPolicy;
use bevy::utils::HashMap;
use bevy::window::{Window, WindowResized};

use image::imageops;
use image::io::Reader as ImageReader;

use crate::basic::run_commands;
use crate::setup::FontResource;

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
    pub id: Option<u32>,
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
    mut materials: ResMut<Assets<Image>>,
    mut image_idenifier: ResMut<ImageIdenifier>,
    mut window: Query<&mut Window>,
) {
    if kbd.just_pressed(KeyCode::Return) {
        let font = &font_resource.fira_sans;
        let user = env::var("USER").unwrap();
        let path = strip_path(env::current_dir().unwrap().to_string_lossy().to_string(), &user);
        let hostname = hostname::get().unwrap().to_string_lossy().to_string();
        let command_return = run_commands(text_input.value.clone());
        let mut parts = text_input.value.split_whitespace();
        let command = parts.next().unwrap();
        let img_extensions: [&str; 5] = [".png", ".jpg", ".jpeg", ".gif", "webp"];
        if command.ends_with(".webp") {
            let absolute_path = format!("/home/{}/{}/{}", user,path, command);
            // Absolute path to your image
            let image_path = &absolute_path;
    
            // Use the image crate to open the image
            //println!("{image_path}");
            let img = ImageReader::open(Path::new(image_path))
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba8();
            
            // Create a new texture from the loaded image
            let mut image_size: (u32,u32) = (img.width(), img.height());
            let mut window_size: (u32,u32) = (100,100);
            for e in window.iter() { 
                window_size = (e.width() as u32, (e.height()) as u32)
            };
            if image_size.0 > window_size.0 || image_size.1 > window_size.1 {
                let width_ratio = image_size.0 as f32 / window_size.0 as f32;
                let height_ratio = image_size.1 as f32 / window_size.1 as f32;
                let max_ratio = width_ratio.max(height_ratio);
            
                image_size.0 = (image_size.0 as f32 / max_ratio) as u32;
                image_size.1 = (image_size.1 as f32 / max_ratio) as u32;
            }

            let new_image = imageops::resize(&img, image_size.0, image_size.1, imageops::FilterType::CatmullRom);
            let texture = Image::new_fill(
                Extent3d {
                    width: new_image.width(), 
                    height: new_image.height(),
                    depth_or_array_layers: 1
                },
                TextureDimension::D2,
                &new_image.into_raw(),
                TextureFormat::Rgba8UnormSrgb,
            );
    
            // Create a material from the texture
            let texture_handle = materials.add(texture);
            let material = texture_handle.clone();
            let entity = query.iter_mut().collect::<Vec<Entity>>()[2];// entity 2 is the output 
            let image_bundle = commands.spawn(ImageBundle {
                visibility: Visibility::Visible,
                focus_policy: FocusPolicy::Block,
                image: UiImage { 
                    texture: material.clone(), 
                    flip_x: false, 
                    flip_y: false },
                ..Default::default()
            }).id();
            //println!("{}",entities[2].index());
            //println!("{}",image_bundle.index());
            image_idenifier.id = Some(image_bundle.index());
            commands.entity(entity).add_child(image_bundle);
            

        } else {
            match command {
                "clear" => {
                    for mut text in output.iter_mut() {
                        text.sections.clear()
                    }
                    if image_idenifier.id.is_some() {
                        let image = query.iter_mut().collect::<Vec<Entity>>()[image_idenifier.id.unwrap() as usize];
                        //println!("{}",image.index());
                        commands.entity(image).despawn_recursive();
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
        }

    
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
