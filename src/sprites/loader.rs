use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct SpriteSheetDataset {
    pub dataset: HashMap<String, SpriteSheetMetadata>,
}

pub struct SpriteServer {
    sprite_map: HashMap<String, (Handle<TextureAtlas>, SpriteSheetMetadata)>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpriteSheetMetadata {
    /// Path to the original resource file relative to assets folder.
    pub filepath: String,
    /// Intended playback fps for this animations.
    pub fps: f32,
    /// Rows in the sheet.
    pub rows: usize,
    /// Columns in the sheet.
    pub cols: usize,
    /// Height of each sprite in pixels.
    pub height: usize,
    /// Width of each sprite in pixels.
    pub width: usize,
    /// Whether or not the sprite faces a direction (left, right, or none)
    pub direction: Option<SpriteDirection>,
}

impl SpriteServer {
    /// Load the sprite metadata from the passed RON file path, load them with the AssetServer,
    /// and store weak handles for all loaded assets.
    pub fn build_from_file(
        path: &str,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let file = File::open(path).expect("Failed opening file");
        let metadataset: SpriteSheetDataset =
            from_reader(file).expect("Unable to load spritesheet metadata");
        let mut sprites: HashMap<String, (Handle<TextureAtlas>, SpriteSheetMetadata)> =
            HashMap::new();
        for (name, md) in metadataset.dataset.iter() {
            let handle =
                load_texture_atlas_from_sprite_sheet_metadata(md, asset_server, texture_atlases);
            sprites.insert(name.to_string(), (handle.clone_weak(), md.clone()));
        }
        Self {
            sprite_map: sprites,
        }
    }

    /// Get a strong handle to the texture atlas mapped to the given name.
    pub fn get_sprite_handle(&self, name: &str) -> Option<Handle<TextureAtlas>> {
        if let Some((handle, _)) = self.sprite_map.get(name) {
            Some(handle.clone())
        } else {
            None
        }
    }

    /// Create SpriteAnimation from name associated with loaded metadata.
    pub fn get_sprite_anim(
        &self,
        name: &str,
        playback: PlaybackType,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Option<SpriteAnimation> {
        if let Some((handle, metadata)) = self.sprite_map.get(name) {
            let mut h = handle.clone();
            h.make_strong(texture_atlases);
            Some(SpriteAnimation {
                playback,
                fps: metadata.fps,
                index: 0,
                length: metadata.rows * metadata.cols,
                timer: Timer::from_seconds(1.0 / metadata.fps, true),
                direction: metadata.direction.clone(),
                handle: h,
            })
        } else {
            None
        }
    }

    /// Print all the loaded sprite sheets and their metadata.
    pub fn print_dataset(&self) {
        for (name, (_, md)) in self.sprite_map.iter() {
            println!(
                "{}: {}x{} tiles of {}x{} px @ {} fps from '{}'",
                name, md.rows, md.cols, md.height, md.width, md.fps, md.filepath
            );
        }
    }
}

/// Loads texture atlas directly from a given file path.
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

/// Loads texture atlas from a SpriteSheetMetadata object
pub fn load_texture_atlas_from_sprite_sheet_metadata(
    metadata: &SpriteSheetMetadata,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(&metadata.filepath);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(metadata.width as f32, metadata.height as f32),
        metadata.cols,
        metadata.rows,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    texture_atlas_handle
}
