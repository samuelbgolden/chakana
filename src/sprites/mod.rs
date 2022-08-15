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

#[derive(Component, Clone, Debug)]
pub struct SpriteAnimation {
    pub playback: PlaybackType,
    /// Intended playback fps for this animations.
    pub fps: f32,
    /// Index of the currently displayed frame of the playing animation
    pub index: usize,
    /// Count of total sprites (max index for sheet accessing).
    pub length: usize,
    /// Timer for keeping track of when to advance the frame of the animation.
    pub timer: Timer,
    /// Asset handle for the texture atlas.
    pub handle: Handle<TextureAtlas>,
    /// Flag for managing the facing direction of the sprite (assume the resource originally faces to the right)
    pub direction: Option<SpriteDirection>,
}

pub fn handle_sprite_playback(
    time: Res<Time>,
    mut commands: Commands,
    mut anims_query: Query<(Entity, &mut SpriteAnimation, &mut TextureAtlasSprite)>,
) {
    for (e, mut anim, mut sprite) in anims_query.iter_mut() {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            if anim.index == (anim.length - 1) {
                match anim.playback {
                    PlaybackType::Once => {
                        commands.entity(e).remove::<SpriteAnimation>();
                        continue;
                    }
                    PlaybackType::Repeat => anim.index = 0,
                }
            } else {
                anim.index += 1;
            }
            sprite.index = anim.index;

            sprite.flip_x = match anim.direction {
                None => false,
                Some(SpriteDirection::Right) => false,
                Some(SpriteDirection::Left) => true,
            }
        }
    }
}
