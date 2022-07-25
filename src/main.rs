mod components;
mod environment;
mod player;
mod player_input;
mod sprites;

mod prelude {
    pub use crate::components::*;
    pub use crate::environment::*;
    pub use crate::player::*;
    pub use crate::player_input::*;
    pub use crate::sprites::*;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use std::collections::HashMap;
}

#[allow(unused_imports)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::system::exit_on_esc_system;
use prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(spawn_player)
            .add_startup_system(build_environment)
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
