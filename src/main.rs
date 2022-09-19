use bevy::{
    prelude::*,
    render::camera::ScalingMode
};
use bevy_rapier2d::prelude::*;

mod lib;
use crate::lib::{
    apply_phys,
    check_borders,
};

use crate::spawn::Player;

mod input;
use input::keyboard_input;

mod spawn;
use spawn::SpawnPlugin;

mod debug;
use debug::DebugPlugin;

mod player;
use crate::player::{
    PlayerPlugin
};

const WWIDTH: f32 = 1280.0;
const WHEIGHT: f32 = 720.0;

pub const ROTATION_SPEED: f32 = 8.0;
pub const BOOST_FORCE: f32 = 4.00;
pub const MAX_SPEED: f32 = 5.0;

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
    .add_system(keyboard_input)
    .add_system(apply_phys)
    .add_system(check_borders)
    .add_plugin(PlayerPlugin)
    .add_plugin(SpawnPlugin)
    .add_plugin(DebugPlugin)
    .run();
}
