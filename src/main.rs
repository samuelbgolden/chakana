mod components;
mod environment;
mod physics;
mod player;
mod player_input;
mod player_state;
mod sprites;

mod prelude {
    pub use crate::components::*;
    pub use crate::environment::*;
    pub use crate::physics::*;
    pub use crate::player::*;
    pub use crate::player_input::*;
    pub use crate::sprites::*;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::geometry::*;
    pub use bevy_rapier2d::math::*;
    pub use bevy_rapier2d::pipeline::*;
    pub use bevy_rapier2d::plugin::*;
    pub use bevy_rapier2d::render::*;
    pub use std::collections::HashMap;
}

#[allow(unused_imports)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::system::exit_on_esc_system;
use prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_server = SpriteServer::build_from_file(
        "assets/sprite_sheet_dataset.ron",
        &asset_server,
        &mut texture_atlases,
    );
    sprite_server.print_dataset();

    commands.insert_resource(sprite_server);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(build_environment)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player)
            .insert_resource(InputPollTimer(Timer::from_seconds(0.01, true)))
            .add_system(handle_player_input)
            .add_system(handle_sprite_playback);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "CHAKANA".to_string(),
            width: 2000.0,
            height: 1000.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(75.0))
        .add_plugin({
            let mut rdrp = RapierDebugRenderPlugin::default();
            rdrp.depth_test = false;
            rdrp
        })
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(GamePlugin)
        .add_system(exit_on_esc_system)
        .run();
}
