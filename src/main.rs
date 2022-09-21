use bevy::{
    prelude::*,
    render::camera::ScalingMode
};
use bevy_rapier2d::prelude::*;

mod lib;
use crate::lib::{
    apply_phys,
    check_borders,
    GameState::{Playing, GameOver},
    GameState,
    game_over,
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

pub struct PlayerSprite(Handle<Image>);


fn main() {
    App::new()
    .add_state(GameState::Playing)
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
    .add_plugin(SpawnPlugin)
    .add_plugin(DebugPlugin)
    .add_system_set(SystemSet::on_update(GameState::Playing)
        .with_system(keyboard_input)
        .with_system(apply_phys)
        .with_system(check_borders)
    )
    .add_system_set(SystemSet::on_update(GameState::GameOver)
        .with_system(game_over)
    )
  .run();
}
