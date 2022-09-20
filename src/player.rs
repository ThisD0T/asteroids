use bevy::{
    prelude::*,
    input::keyboard::KeyboardInput,
    time::Stopwatch,
};

use crate::lib::{
    PhysicsVars,
    GameObject,
    PhysFlag,
    bullet_collision_check,
    BulletCollider,
    player_health,
    GameState,
    update_health_text,
    score_text,
    PlayerFuelStopwatch,
    fuel_cycle,
    update_fuel_text,
};
use crate::spawn::{
    Player,
    BorderFlag,
    ParentFlag,
};
use crate::{
    BOOST_FORCE,
    ROTATION_SPEED,
};

#[derive(Component)]
pub struct BulletTimer {
    pub stopwatch: Stopwatch,
}

#[derive(Component)]
pub struct DeathTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Bullet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(camera_follow)
            .with_system(player_input)
            .with_system(player_boost)
            .with_system(player_shoot)
            .with_system(bullet_timeout)
            .with_system(bullet_collision_check)
            .with_system(player_health)
            .with_system(score_text)
            .with_system(fuel_cycle)
            .with_system(update_fuel_text)
        );

        app.add_system(update_health_text);
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

pub fn player_shoot (
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut query: Query<&mut Transform, With<Player>>,
    mut time_query: Query<&mut BulletTimer, With<Player>>,
    mut phys_query: Query<&mut PhysicsVars, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut timer = time_query.single_mut();

    timer.stopwatch.tick(time.delta());

    if timer.stopwatch.elapsed_secs() > 1.3 {
        if keyboard_input.pressed(KeyCode::Space) {

            let player_transform = query.single_mut();
            let player_phys = phys_query.single_mut();

            let mut bullet = commands.spawn().id();

            let bullet_size = Vec2::splat(5.0);
            let bullet_pos = player_transform;
            let bullet_texture = assets.load("bullet.png");


            commands.entity(bullet)
                .insert_bundle(GameObject{
                    sprite_bundle: SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(bullet_size),
                                ..Default::default()
                        },

                        transform: * bullet_pos,
                        texture: bullet_texture,
                        ..Default::default()
                    },
                    physics_vars: PhysicsVars {
                        velocity: bullet_pos.rotation * (Vec3::Y * BOOST_FORCE) + player_phys.velocity,
                        ..Default::default()
                    }
                })
                .insert(PhysFlag)
                .insert(DeathTimer{timer: Timer::from_seconds(2.0, false)})
                .insert(Bullet)
                .insert(BulletCollider);
            timer.stopwatch.reset();
        }
    }
}

fn bullet_timeout(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut DeathTimer, &mut Transform), With<DeathTimer>>,
    time: Res<Time>,
) {

    for (entity, mut timer_query, mut bullet_transform) in bullet_query.iter_mut() {

        let mut timer = timer_query;

        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(entity).despawn();
        }
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
