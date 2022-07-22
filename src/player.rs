pub use crate::prelude::*;

pub enum PlayerState {
    Idle,
    Walking,
    Jumping,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            texture: asset_server.load("rogue_solo_sprite.png"),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Collider::capsule_y(25.0, 25.0))
        .insert(GravityScale(5.0))
        .insert(Player {
            state: PlayerState::Idle,
        });
}
