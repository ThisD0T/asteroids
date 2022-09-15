use bevy::prelude::*;

use bevy_inspector_egui::{
    Inspectable
};


use crate::{
    lib::{
        GameObject,
    },
    PlayerSprite,
};

const player_sprite: &str = "player.png";

#[derive(Component, Inspectable)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
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
        }
    })
    .insert(Player);

}
