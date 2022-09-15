use bevy::{prelude::*, pbr::GlobalLightMeta};

#[derive(Component)]
struct Collider;

#[derive(Bundle, Default)]
pub struct GameObject{
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
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

