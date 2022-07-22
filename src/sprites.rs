use crate::prelude::*;

/*
pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let player_handle: TextureAtlas = TextureAtlas::from_grid(
        asset_server.load("rogue_spritesheet_calciumtrice.png"),
        Vec2::new(32.0, 32.0),
        10,
        10,
    );
    for player_entity in player_query.iter_mut() {
        commands.entity(player_entity).insert(player_handle.han);
    }
}
*/

pub fn handle_sprite_playback(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<(
        &mut SpritePlaybackTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Player,
    )>,
) {
    // set player sprite
    for (mut timer, mut sprite, texture_atlas_handle, player) in player_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let playback_length = 10;
            sprite.index = sprite.index % playback_length;

            match player.state {
                PlayerState::TempB => sprite.index += playback_length * 0,
                PlayerState::Idle => sprite.index += playback_length * 1,
                PlayerState::Walking => sprite.index += playback_length * 2,
                PlayerState::Jumping => sprite.index += playback_length * 3,
                PlayerState::TempA => sprite.index += playback_length * 4,
            }
            //let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            //println!("Setting player texture to {}/{}", sprite.index, texture_atlas.textures.len());
            sprite.index = sprite.index + 1;
        }
    }
}
