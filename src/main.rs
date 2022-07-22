mod components;
mod player_input;

mod prelude {
    pub use crate::components::*;
    pub use crate::player_input::*;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
}

use prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn create_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(100., 0., 0.),
            texture: asset_server.load("rogue_solo_sprite.png"),
            ..default()
        })
        .insert(Player {});
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(create_player)
            .add_system(handle_player_input);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
