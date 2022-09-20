use std::time::Duration;

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    time::Stopwatch,
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
        AsteroidTimer,
        spawn_asteroid,
        PlayerStats,
        GameState,
        setup_text,
        Score,
    },
    PlayerSprite,
};

use crate::player::{
    BulletTimer,
};

#[derive(Component, Inspectable)]
pub struct Player;

const PLAYER_SPRITE: &str = "player.png";


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
        .add_startup_system(spawn_border)
        .add_startup_system(setup_text);
        app.add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(asteroid_spawn_sys)
        );
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
        .insert(AsteroidTimer{timer: Timer::from_seconds(10.0, false)})
        .insert(BulletTimer{stopwatch: Stopwatch::new()})
        .insert(Player)
        .insert(PhysFlag)
        .insert(PlayerStats{health: 3, fuel: 20.0})
        .insert(Score{score: 0})
        .insert(Name::new("Player"));

        let health_text = commands.spawn().id();

}

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
        })
        .insert(Name::new("Boundary"))
        .insert(BorderFlag);
}

fn make_timers () -> Timer {
    let timer = Timer::from_seconds(0.0, false);
    return timer
}

pub fn asteroid_spawn_sys (
    mut commands: Commands,
    mut player_query: Query<&mut AsteroidTimer, With<Player>>,
    time: Res<Time>,
    assets: Res<AssetServer>,
) {
    let mut player = player_query.single_mut();

    player.timer.tick(time.delta());
    if player.timer.just_finished() {
        spawn_asteroid(commands, 5, assets);
        player.timer.reset();
    }
}
