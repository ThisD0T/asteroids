use bevy::{
    prelude::*,
    render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;

mod input;
use input::keyboard_input;

mod player;
use player::PlayerPlugin

const WWIDTH: f32 = 1280.0;
const WHEIGHT: f32 = 720.0;

mod lib;

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
    .add_startup_system_to_stage(StartupStage::PreStartup, gen_sprite_list)
    .add_system(keyboard_input)
    .run();
}

fn spawn_camera (mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::WindowSize;
    commands.spawn_bundle(camera);
}

fn gen_sprite_list(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let player_sprite: Handle<Image> = assets.load("player.png");

    commands.insert_resource(player_sprite)
}
