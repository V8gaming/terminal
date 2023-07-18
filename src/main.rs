use crate::setup::setup;
use bevy::prelude::*;
use update::{text_input, History,TextInputResource, update_window_and_input_system, handle_return_key, MyMaterialResource, ImageIdenifier};
use setup::{FontResource, load_font};
mod update;
mod basic;
mod pty;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreStartup, load_font)
        .add_systems(Startup, setup)
        .add_systems(Update, text_input)
        .add_systems(Update, handle_return_key)
        .add_systems(Update, update_window_and_input_system)
        .init_resource::<TextInputResource>()
        .init_resource::<History>()
        .init_resource::<FontResource>()
        .init_resource::<ImageIdenifier>()
        .init_resource::<MyMaterialResource>()
        .run();
}