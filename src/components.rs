pub use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Component, Deref, DerefMut)]
pub struct SpritePlaybackTimer(pub Timer);

#[derive(Component, Debug)]
pub struct SpriteSheetRanges {
    pub curr_sprite: usize,
    pub curr_range: usize,
    pub ranges: Vec<(usize, usize)>,
}
