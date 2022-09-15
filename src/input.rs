use bevy::{
    prelude::*,
    input::{
        ButtonState,
        keyboard::KeyboardInput,
    }
    
};

use crate::spawn::Player;

pub fn keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
) {

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?}, ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
                println!("Key released: {:?}, ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}

pub fn player_input(
    mut keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut rotation_direction = 1.0;

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_direction = 1.0;
        println!("right pressed")
    }

    let rotation_vector= Vec4::new(0.0, 0.0, 0.0, 0.0);
    let rotation_quat= Quat::from_vec4(rotation_vector);

    println!("{}", rotation_direction);

    player_transform.rotate(rotation_quat);
}