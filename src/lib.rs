use bevy::{prelude::*, pbr::GlobalLightMeta, ecs::query::WorldQuery};

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
) {
    for mut phys in object_phys.iter_mut() {
        for mut obj_transform in object_transform.iter_mut() {
            // messy garbage to work around an error I don't understand at all
            let mut temp_phys_vel = phys.velocity;
            let temp_phys_accel = phys.acceleration;

            temp_phys_vel = temp_phys_vel + temp_phys_accel;

            obj_transform.translation = obj_transform.translation + temp_phys_vel;

            phys.acceleration = Vec3::splat(0.0);
        }
    }
}
