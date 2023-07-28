use bevy::prelude::{
    Resource, DefaultPlugins, PreStartup, Startup,
    App, Update
};

use crate::{
    update::{
        TextInputResource, History, ImageIdenifier, MyMaterialResource, 
        text_input, handle_return_key, update_window_and_input_system
    }, 
    setup::{
        FontResource, 
        load_font, setup, EntityDatabase
    }
};

#[derive(Resource)]
pub struct Terminal {
    font_path: String,
    font_size: f32,
}
impl Terminal {
    pub fn new() -> Self { 
        Terminal::default()
    }
    pub fn font_path(&self) -> &str {
        self.font_path.as_str()
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn run(&self) {
        App::new()
            .init_resource::<Terminal>()
            .add_plugins(DefaultPlugins)
            .add_systems(PreStartup, load_font)
            .add_systems(Startup, setup)
            .add_systems(Update, text_input)
            .add_systems(Update, handle_return_key)
            .add_systems(Update, update_window_and_input_system)
            .init_resource::<TextInputResource>()
            .init_resource::<EntityDatabase>()
            .init_resource::<History>()
            .init_resource::<FontResource>()
            .init_resource::<ImageIdenifier>()
            .init_resource::<MyMaterialResource>()
            .run();
    }
}
impl Default for Terminal {
    fn default() -> Self { 
        Terminal {
            font_path: "FiraCode/FiraCodeNerdFontMono-Bold.ttf".to_string(),
            font_size: 20.0,
        }   
    }
}