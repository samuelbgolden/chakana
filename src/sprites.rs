use crate::prelude::*;
//use ron::de::from_reader;
use serde::Deserialize;
//use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct SpriteSheetDataset {
    pub dataset: Vec<SpriteSheetData>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpriteSheetData {
    // name to be referenced when retrieving sheet
    pub name: String,
    // path to the original resource file relative to assets folder
    pub filepath: String,
    // intended playback fps for these animations
    pub fps: f32,
    // rows in the sheet
    pub rows: i32,
    // columns in the sheet
    pub cols: i32,
    // height of each sprite in pixels
    pub height: i32,
    // width of each sprite in pixels
    pub width: i32,
}

pub fn handle_sprite_playback(
    time: Res<Time>,
    mut player_query: Query<(
        &mut SpritePlaybackTimer,
        &mut TextureAtlasSprite,
        &mut SpriteSheetRanges,
        &Player,
    )>,
) {
    // handle player sprite
    for (mut timer, mut sprite, mut ranges, player) in player_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            // move to next sprite in the current animation (sprite range)
            ranges.curr_sprite += 1;

            // check if we need to wrap around to the start of the animation
            if let Some((start, end)) = ranges.ranges.get(ranges.curr_range) {
                if *end <= ranges.curr_sprite {
                    ranges.curr_sprite = *start;
                }
            }

            // change the current animation if the player state has changed
            let new_sprite_range: usize = match player.state {
                PlayerState::TempB => 0,
                PlayerState::Idle => 1,
                PlayerState::Walking => 2,
                PlayerState::Jumping => 3,
                PlayerState::TempA => 4,
            };
            if new_sprite_range != ranges.curr_range {
                if let Some((start, _)) = ranges.ranges.get(new_sprite_range) {
                    ranges.curr_sprite = *start;
                    ranges.curr_range = new_sprite_range;
                } else {
                    warn!("Invalid sprite range {} for player atlas", new_sprite_range);
                }
            }

            // set the sprite
            sprite.index = ranges.curr_sprite;
        }
    }
}

pub fn load_texture_atlas(
    asset_path: &str,
    tile_size: Vec2,
    cols: usize,
    rows: usize,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(asset_path);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, cols, rows);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    texture_atlas_handle
}
