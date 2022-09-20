use bevy::{prelude::*, pbr::GlobalLightMeta, ecs::query::WorldQuery, time::Stopwatch};

use std::sync::Arc;
use std::time::Duration;

use rand;
use rand::{
    Rng,
    thread_rng,
};

use bevy_inspector_egui::{
    Inspectable,
};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Playing,
    GameOver,
}

pub const HEALTH_SIZE: f32 = 20.0;

pub const MAX_SPEED: f32 = 18.0;
pub const MAP_SIZE: f32 = 1500.0;
pub const BOUNDARY_BOUNCE_MULT: f32 = 0.15;

pub const ASTEROID_SPEED: f32 = 0.5;

pub struct SpriteList;

#[derive(Component)]
pub struct AsteroidSize{
    size: f32
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
pub struct AsteroidCollider;

#[derive(Component)]
pub struct BulletCollider;

#[derive(Component)]
pub struct PhysFlag;

#[derive(Component)]
pub struct HealthText;

#[derive(Component, Default)]
pub struct PhysicsVars{
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

#[derive(Component)]
pub struct AsteroidTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct PlayerStats {
    pub health: u32,
    pub fuel: f32,
}

#[derive(Bundle, Default)]
pub struct GameObject{
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub physics_vars: PhysicsVars,
}


impl GameObject{
    fn new(
        assets: Res<AssetServer>,
        custom_size: Option<Vec2>,
        transform: Transform,
        image: String,
) -> GameObject{
        let image = assets.load(&image);
        GameObject {
            sprite_bundle: SpriteBundle {

                sprite: Sprite {
                custom_size: custom_size,
                ..Default::default()
                },
                transform: transform,
                texture: image,
                ..Default::default()
            },
            physics_vars: PhysicsVars {
                velocity: Vec3::splat(0.0),
                acceleration: Vec3::splat(0.0),
            },
        }
    }
}

// this is convoluded
pub fn gen_player_sprite(
    assets: Res<AssetServer>,
) -> Vec<Handle<Image>> {
    let player_sprite: Handle<Image> = assets.load("player.png");
    let mut res_list = Vec::new();
    res_list.push(player_sprite);
    return res_list;
}

pub fn apply_phys(
    mut object_phys: Query<(&mut PhysicsVars, &mut Transform), &PhysFlag>,
    //mut object_transform: Query<&mut Transform, &PhysFlag>,
    time: Res<Time>,
) {
    for (mut phys, mut obj_transform) in &mut object_phys {
        
        // messy garbage to work around an error I don't understand at all
        let mut temp_phys_vel = phys.velocity;
        let temp_phys_accel = phys.acceleration;

        temp_phys_vel = temp_phys_vel + temp_phys_accel;
        temp_phys_vel = Vec3::clamp(temp_phys_vel, Vec3::splat(-MAX_SPEED), Vec3::splat(MAX_SPEED));

        phys.velocity = temp_phys_vel;
        obj_transform.translation = obj_transform.translation + phys.velocity;

        phys.acceleration = Vec3::splat(0.0);
    }
}

pub fn check_borders(
    mut phys_obj_query: Query<(&mut PhysicsVars, &mut Transform), Without<AsteroidTimer>>,
    mut player_query: Query<(&mut PhysicsVars, &mut Transform), With<AsteroidTimer>>,
) {
    for (mut phys, mut obj_transform) in &mut phys_obj_query {

        if obj_transform.translation.x > MAP_SIZE/2.0 {
            obj_transform.translation.x = MAP_SIZE/2.0;
            phys.velocity.x *= -1.0;
        } else if obj_transform.translation.x < -MAP_SIZE/2.0 {
            obj_transform.translation.x = -MAP_SIZE/2.0;
            phys.velocity.x *= -1.0;
        }

        if obj_transform.translation.y > MAP_SIZE/2.0 {
            obj_transform.translation.y = MAP_SIZE/2.0;
            phys.velocity.y *= -1.0;
        } else if obj_transform.translation.y < -MAP_SIZE/2.0 {
            obj_transform.translation.y = -MAP_SIZE/2.0;
            phys.velocity.y *= -1.0;
        }
    }

    for (mut phys, mut obj_transform) in &mut player_query {

        if obj_transform.translation.x > MAP_SIZE/2.0 {
            obj_transform.translation.x = MAP_SIZE/2.0;
            phys.velocity.x *= -BOUNDARY_BOUNCE_MULT;
        } else if obj_transform.translation.x < -MAP_SIZE/2.0 {
            obj_transform.translation.x = -MAP_SIZE/2.0;
            phys.velocity.x *= -BOUNDARY_BOUNCE_MULT;
        }

        if obj_transform.translation.y > MAP_SIZE/2.0 {
            obj_transform.translation.y = MAP_SIZE/2.0;
            phys.velocity.y *= -BOUNDARY_BOUNCE_MULT;
        } else if obj_transform.translation.y < -MAP_SIZE/2.0 {
            obj_transform.translation.y = -MAP_SIZE/2.0;
            phys.velocity.y *= -BOUNDARY_BOUNCE_MULT;
        }
    }
}

fn make_timers () -> Timer {
    let timer = Timer::from_seconds(0.0, false);
    return timer
}

pub fn spawn_asteroid(
    mut commands: Commands,
    num_asteroids: u32,
    assets: Res<AssetServer>,
) {

    let mut rng = rand::thread_rng();

    for asteroid in 0..num_asteroids {

    let asteroid_pos = Vec3::new(rng.gen_range(-MAP_SIZE..MAP_SIZE)/2.0, rng.gen_range(-MAP_SIZE..MAP_SIZE)/2.0, 0.0);
    let asteroid_texture: Handle<Image> = assets.load("asteroid.png");
    let asteroid_size_float = rng.gen_range(15.0..30.0);
    let asteroid_size = Vec2::splat(asteroid_size_float);

    commands.spawn_bundle(GameObject{
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(asteroid_size),
                ..Default::default()
            },
            transform: Transform {
                translation: asteroid_pos,
                ..Default::default()
            },
            texture: asteroid_texture,
            ..Default::default()
            },
            ..Default::default()
        })
        .insert(PhysicsVars{
            velocity: Vec3::new(rng.gen_range(-ASTEROID_SPEED..ASTEROID_SPEED), rng.gen_range(-ASTEROID_SPEED..ASTEROID_SPEED), 0.0),
            ..Default::default()
        })
        .insert(PhysFlag)
        .insert(AsteroidCollider)
        .insert(AsteroidSize{size: asteroid_size_float});
    };
    
}

pub fn bullet_collision_check(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &Transform, &AsteroidSize), Without<BulletCollider>>,
    mut bullet_query: Query<&mut Transform, With<BulletCollider>>,
) {
    for (entity, mut asteroid_transform, mut asteroid_size) in asteroid_query.iter_mut() {
        for mut transform in bullet_query.iter_mut() {
            if Vec3::distance(transform.translation, asteroid_transform.translation) < asteroid_size.size{
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn player_health(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &Transform, &AsteroidSize), Without<PlayerStats>>,
    mut player_query: Query<(Entity, &mut Transform, &mut PlayerStats), Without<BulletCollider>>,
    assets: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    let player_texture: Handle<Image> = assets.load("player.png");

    let (entity, player_transform, mut player_stats) = player_query.single_mut();

    for (asteroid_entity, asteroid_transform, asteroid_size) in asteroid_query.iter_mut() {
        if Vec3::distance(player_transform.translation, asteroid_transform.translation) < asteroid_size.size {
            commands.entity(asteroid_entity).despawn();
            player_stats.health -= 1;
            println!("{}", player_stats.health);
            if player_stats.health < 1 {
                state.set(GameState::GameOver).expect("Failed to change states");
            }
        }
    }
}

pub fn setup_text (
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let health_text = commands.spawn().id();
    commands.entity(health_text)
        .insert_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Health: ",
                    TextStyle {
                        font: assets.load("LemonMilk.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: assets.load("LemonMilk.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(HealthText);

}

pub fn update_health_text(
    mut query: Query<&mut Text, With<HealthText>>,
    mut player_query: Query<&mut PlayerStats, With<PhysFlag>>,
) {
    let mut text = query.single_mut();
    let player_stats = player_query.single_mut();
    let health = player_stats.health;

    text.sections[1].value = format!("{health}");
}

/*
pub fn player_health_text(
    mut commands: Commands,

)
*/