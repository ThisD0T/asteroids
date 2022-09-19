use bevy::{
    prelude::*,
    input::keyboard::KeyboardInput,
};

use crate::lib::PhysicsVars;
use crate::spawn::{
    Player,
    BorderFlag,
    ParentFlag,
};
use crate::{
    BOOST_FORCE,
    ROTATION_SPEED,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_follow);
        app.add_system(player_input);
        app.add_system(player_boost);
    }
}


pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
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

    let rotation = rotation_direction * ROTATION_SPEED * time.delta_seconds();

    player_transform.rotate_local_z(rotation);
}

pub fn player_boost(
    mut query: Query<&mut PhysicsVars, &Player>,
    mut transform_query: Query<&mut Transform,  &Player>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_phys = query.single_mut();
    let transform = transform_query.single_mut();
    let mut boost_vector = Vec3::splat(0.0);

    // some kind of black magic here
    boost_vector = transform.rotation * (Vec3::Y * BOOST_FORCE);

    if keyboard_input.pressed(KeyCode::W) {
        player_phys.acceleration += boost_vector * time.delta_seconds();
    }
}

pub fn camera_follow(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, &Camera>,
) {
    let player_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    //let mut new_transform = player_transform;
    //new_transform.rotation = Quat::from_vec4(Vec4::splat(0.0));

    camera_transform.translation = player_transform.translation;
    //println!("{}", camera_transform.rotation);
}
