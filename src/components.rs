pub use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct SpritePlaybackTimer(pub Timer);

#[derive(Component, Debug)]
pub struct ChangePlayerStateMessage {
    pub new_state: PlayerState,
}
