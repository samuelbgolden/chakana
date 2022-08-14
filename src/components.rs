pub use crate::prelude::*;

// Other files can contain components; this file is just for those components that
// don't have an obvious home in a different file.

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
