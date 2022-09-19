use bevy::{
    prelude::*,
    render::camera::ScalingMode,
};

use bevy_inspector_egui::{
    Inspectable
};


use crate::{
    lib::{
        GameObject,
        PhysicsVars,
        PhysFlag,
        MAP_SIZE,
    },
    PlayerSprite,
};


const PLAYER_SPRITE: &str = "player.png";

#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component)]
struct Camera;

#[derive(Component)]
pub struct BorderFlag;

#[derive(Component)]
pub struct ParentFlag;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin{
    fn build (&self, app: &mut App) {
        app.add_startup_system(spawn_player)
        .add_startup_system(spawn_border);
        //app.add_startup_system(spawn_bg);
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    // I realize this is probably a train wreck of a way to handle resources
    let texture: Handle<Image> = assets.load(PLAYER_SPRITE);

    let camera = commands.spawn().id();

    commands.entity(camera)
        .insert_bundle(Camera2dBundle{
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize,
                ..Default::default()
            },
        ..Default::default()
        })
        .insert(Camera)
        .insert(Name::new("MainCam"));


    let player = commands.spawn().id();
    
    commands.entity(player)
        .insert_bundle(GameObject {
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
        .insert(PhysFlag)
        .insert(Name::new("Player"));

}

fn spawn_asteroid(
    mut commands: Commands,
    num_asteroids: u32,
) {

}

/*
fn spawn_bg(
    mut commands: Commands, 
    assets: Res<AssetServer>
) {
    let background_texture: Handle<Image> = assets.load("background.png");
    let mut background_elements = Vec::new();

    for _ in 0..50 {
        let star = commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(3.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new()
            }
            ..Default::default()
        },
        );
    }
}
*/

fn spawn_border (
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let background_texture= assets.load("boundary.png");

    let boundary = commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(MAP_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::splat(0.0),
                ..Default::default()
            },
            texture: background_texture,
            ..Default::default()
        }
        )
        .insert(Name::new("Boundary"))
        .insert(BorderFlag);
}
