mod components;
mod environment;
mod player_input;

mod prelude {
    pub use crate::components::*;
    pub use crate::environment::*;
    pub use crate::player_input::*;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
}

use prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            texture: asset_server.load("rogue_solo_sprite.png"),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::ball(32.))
        .insert(Player {});
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(create_player)
            .add_startup_system(build_environment)
            .insert_resource(InputPollTimer(Timer::from_seconds(0.1, true)))
            .add_system(handle_player_input);
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(GamePlugin)
        .run();
}
