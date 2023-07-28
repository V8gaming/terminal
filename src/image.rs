use bevy::prelude::{Entity, Visibility, ImageBundle, Query, 
    ResMut, Assets, Image, Commands, BuildChildren};
use bevy::window::Window;
use image::imageops;
use image::io::Reader as ImageReader;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::ui::{FocusPolicy, UiImage};
use std::path::Path;

use crate::setup::EntityDatabase;
use crate::update::ImageIdenifier;

pub fn render_image(
    command: &str, user: String, path: String, 
    window: Query<&mut Window>,
    mut materials: ResMut<Assets<Image>>,
    mut image_idenifier: ResMut<ImageIdenifier>,
    mut commands: Commands,
    mut query: Query<Entity>,
    mut entity_database: ResMut<EntityDatabase>,
) {
    let img_extensions: [&str; 5] = [".png", ".jpg", ".jpeg", ".gif", "webp"];
    if img_extensions.iter().any(|&i| command.ends_with(i)) {
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
        let entity = query.iter_mut().collect::<Vec<Entity>>()[image_idenifier.output_id.unwrap()as usize];// entity 2 is the output 
        
        let image_bundle = commands.spawn(ImageBundle {
            visibility: Visibility::Visible,
            focus_policy: FocusPolicy::Block,
            image: UiImage { 
                texture: material.clone(), 
                flip_x: false, 
                flip_y: false },
            ..Default::default()
        }).id();
        entity_database.hashmap.insert(command.to_string(), image_bundle.index());
        //println!("{}",entities[2].index());
        //println!("{}",image_bundle.index());
        image_idenifier.id.push(Some(image_bundle.index()));
        commands.entity(entity).push_children(&[image_bundle]);
    }
}
