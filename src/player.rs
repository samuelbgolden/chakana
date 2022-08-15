pub use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Walking,
    Jumping,
}

pub fn spawn_player(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_server: Res<SpriteServer>,
) {
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: sprite_server.get_sprite_handle("player_idle").unwrap(),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        })
        .insert(
            sprite_server
                .get_sprite_anim("player_idle", PlaybackType::Repeat, &mut texture_atlases)
                .unwrap(),
        )
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Collider::capsule_y(20.0, 25.0))
        .insert(GravityScale(5.0))
        .insert(Player {
            state: PlayerState::Idle,
        });
}
