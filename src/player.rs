use bevy::{
    prelude::*,
    input::keyboard::KeyboardInput,
};

use crate::lib::PhysicsVars;
use crate::spawn::Player;
use crate::{
    BOOST_FORCE,
    ROTATION_SPEED,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_input);
        app.add_system(player_boost);
    }
}


pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut rotation_direction: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_direction -= 1.0;
    } else if keyboard_input.pressed(KeyCode::D) {
        rotation_direction -= 1.0;
    } 
    if keyboard_input.pressed(KeyCode::A) {
        rotation_direction += 1.0;
    } else if keyboard_input.pressed(KeyCode::Left) {
        rotation_direction += 1.0;
    }

    let rotation = rotation_direction * ROTATION_SPEED;

    player_transform.rotate_local_z(rotation);
}

pub fn player_boost(
    mut query: Query<&mut PhysicsVars, &Player>,
    mut transform_query: Query<&mut Transform,  &Player>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut player_phys = query.single_mut();
    let transform = transform_query.single_mut();

    let mut boost_vector = Vec3::splat(0.0);
    boost_vector = Vec3::normalize(player_phys.velocity);
    boost_vector *= BOOST_FORCE;

    println!("pos: {}, rot: {}", transform.translation, transform.rotation);

    if keyboard_input.pressed(KeyCode::W) {
        player_phys.acceleration += boost_vector;
    }
}