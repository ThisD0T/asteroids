use bevy::prelude::*;

use crate::lib::GameObject;

struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,

) {

    let player = commands.spawn_bundle(GameObject {
            custom_size: 20.0,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
            }
            texture: 
    });

}
