use bevy::prelude::*;

use bevy_inspector_egui::{
    Inspectable
};


use crate::{
    lib::{
        GameObject,
        PhysicsVars,
        PhysFlag,
    },
    PlayerSprite,
};

const player_sprite: &str = "player.png";

#[derive(Component, Inspectable)]
pub struct Player;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin{
    fn build (&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    // I realize this is probably a train wreck of a way to handle resources
    let texture: Handle<Image> = assets.load(player_sprite);

    let player = commands.spawn_bundle(GameObject {
        sprite_bundle: SpriteBundle {
            sprite: Sprite{
                custom_size: Some(Vec2::new(13.0, 20.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            texture: texture,
            ..Default::default()
        },
        physics_vars: PhysicsVars {
            velocity: Vec3::splat(0.0),
            acceleration: Vec3::splat(0.0),
        }
    })
    .insert(Player)
    .insert(PhysFlag);

}
