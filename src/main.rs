use bevy::{
    prelude::*,
    render::camera::ScalingMode
};
use bevy_rapier2d::prelude::*;

mod lib;
use crate::input::{
    player_input
};

mod input;
use input::keyboard_input;

mod spawn;
use spawn::PlayerPlugin;

mod debug;
use debug::DebugPlugin;

const WWIDTH: f32 = 1280.0;
const WHEIGHT: f32 = 720.0;

pub struct PlayerSprite(Handle<Image>);

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(WindowDescriptor {
        title: "Rasteroids".to_string(),
        width: WWIDTH,
        height: WHEIGHT,
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_startup_system(spawn_camera)
    .add_system(keyboard_input)
    .add_system(player_input)
    .add_plugin(DebugPlugin)
    .run();
}

fn spawn_camera (mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::WindowSize;
    commands.spawn_bundle(camera);
}
