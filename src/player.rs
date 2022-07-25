pub use crate::prelude::*;

#[derive(Debug)]
pub enum PlayerState {
    Idle,
    Walking,
    Jumping,
    TempA,
    TempB,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas_handle = load_texture_atlas(
        "rogue_spritesheet_calciumtrice.png",
        Vec2::new(32.0, 32.0),
        10,
        10,
        asset_server,
        texture_atlases,
    );
    let texture_atlas_ranges = SpriteSheetRanges {
        curr_sprite: 0,
        curr_range: 0,
        ranges: vec![(0, 10), (10, 20), (20, 30), (30, 40), (40, 50)],
    };

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        })
        .insert(texture_atlas_ranges)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Collider::capsule_y(20.0, 25.0))
        .insert(GravityScale(5.0))
        .insert(Player {
            state: PlayerState::Idle,
        })
        .insert(SpritePlaybackTimer(Timer::from_seconds(0.15, true)));
}
