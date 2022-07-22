pub use crate::prelude::*;

pub fn build_environment(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1000.0, 200.0)),
                ..default()
            },
            transform: Transform::from_xyz(0., -200., 0.),
            texture: asset_server.load("minimal_grass_flat_platform.png"),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(500., 100.));
}
