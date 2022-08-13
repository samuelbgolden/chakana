use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct SpriteSheetDataset {
    pub dataset: BTreeMap<String, SpriteSheetData>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpriteSheetData {
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

impl SpriteSheetDataset {
    pub fn load(path: &str) -> Self {
        println!("{}", path);
        let file = File::open(path).expect("Failed opening file");
        from_reader(file).expect("Unable to load spritesheet data")
    }

    pub fn print_dataset(&self) {
        for (k, v) in self.dataset.iter() {
            println!(
                "{}: {}x{} tiles of {}x{} px @ {} fps from '{}'",
                k, v.rows, v.cols, v.height, v.width, v.fps, v.filepath
            );
        }
    }
}

pub fn handle_sprite_playback(
    time: Res<Time>,
    mut player_query: Query<(&mut SpritePlaybackTimer, &mut TextureAtlasSprite, &Player)>,
) {
    // handle player sprite
    for (mut timer, mut sprite, player) in player_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index += 1;
        }
        //sprite.
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
