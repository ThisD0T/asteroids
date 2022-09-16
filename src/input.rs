use bevy::{
    prelude::*,
    input::{
        ButtonState,
        keyboard::KeyboardInput,
    },
};

use crate::{
    spawn::Player,
    ROTATION_SPEED,

};

pub fn keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
) {

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                //println!("Key press: {:?}, ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
                //println!("Key released: {:?}, ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}

