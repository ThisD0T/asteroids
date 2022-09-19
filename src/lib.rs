use bevy::{prelude::*, pbr::GlobalLightMeta, ecs::query::WorldQuery};

pub const MAP_SIZE: f32 = 1500.0;
pub const BOUNDARY_BOUNCE_MULT: f32 = 0.15;

pub struct SpriteList;

#[derive(Component)]
struct Collider;

#[derive(Component)]
pub struct PhysFlag;

#[derive(Component, Default)]
pub struct PhysicsVars{
    pub velocity: Vec3,
    pub acceleration: Vec3,
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
    mut object_phys: Query<&mut PhysicsVars, &PhysFlag>,
    mut object_transform: Query<&mut Transform, &PhysFlag>,
    time: Res<Time>,
) {
    for mut phys in object_phys.iter_mut() {
        for mut obj_transform in object_transform.iter_mut() {
            // messy garbage to work around an error I don't understand at all
            let mut temp_phys_vel = phys.velocity;
            let temp_phys_accel = phys.acceleration;

            temp_phys_vel = temp_phys_vel + temp_phys_accel;
            temp_phys_vel = Vec3::clamp(temp_phys_vel, Vec3::splat(-11.0), Vec3::splat(11.0));

            phys.velocity = temp_phys_vel;
            obj_transform.translation = obj_transform.translation + phys.velocity;

            phys.acceleration = Vec3::splat(0.0);
        }
    }
}

pub fn check_borders(
    mut object_phys_query: Query<&mut PhysicsVars, &PhysFlag>,
    mut object_transform_query: Query<&mut Transform, &PhysFlag>,
) {
    for mut phys in object_phys_query.iter_mut() {
        for mut obj_transform in object_transform_query.iter_mut() {
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
}

