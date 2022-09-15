use bevy::{prelude::*, pbr::GlobalLightMeta};

pub struct SpriteList;


#[derive(Component)]
struct Collider;

#[derive(Bundle, Default)]
pub struct GameObject{
    #[bundle]
    pub sprite_bundle: SpriteBundle
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
            }
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
