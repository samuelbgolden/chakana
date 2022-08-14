use crate::prelude::*;
pub use loader::SpriteServer;
use serde::Deserialize;
mod loader;

// for keeping track of sprite flipping when facing L/R
#[derive(Clone, Debug, Deserialize)]
pub enum SpriteDirection {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum PlaybackType {
    Once,
    Repeat,
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct SpritePlaybackTimer(pub Timer);

#[derive(Component, Clone, Debug)]
pub struct SpriteAnimation {
    pub playback: PlaybackType,
    /// Intended playback fps for this animations.
    pub fps: f32,
    /// Index of the currently displayed frame of the playing animation
    pub index: usize,
    /// Count of total sprites (max index for sheet accessing).
    pub length: usize,
    /// Asset handle for the texture atlas.
    pub handle: Handle<TextureAtlas>,
    /// Flag for managing the facing direction of the sprite (assume the resource originally faces to the right)
    pub direction: Option<SpriteDirection>,
}

pub fn handle_sprite_playback(
    time: Res<Time>,
    mut player_query: Query<(&mut SpritePlaybackTimer, &mut TextureAtlasSprite, &Player)>,
) {
    // handle player sprite
    /*
    for (mut timer, mut sprite, ss_map, player) in player_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index += 1;
            todo!();
        }
    }
    */
}
