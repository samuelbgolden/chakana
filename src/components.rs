pub use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Component, Deref, DerefMut)]
pub struct SpritePlaybackTimer(pub Timer);
