use bevy::core::Zeroable;

pub use crate::prelude::*;

const APPLIED_VEL: f32 = 2.5;

pub struct InputPollTimer(pub Timer);

pub fn handle_player_input(
    keyb_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<InputPollTimer>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut player_vel = query.iter_mut().nth(0).unwrap();
        let mut new_x_vel: f32 = player_vel.linvel.x;
        let mut new_y_vel: f32 = player_vel.linvel.y;

        for key in keyb_in.get_pressed() {
            match key {
                KeyCode::W => new_y_vel += APPLIED_VEL,
                KeyCode::A => new_x_vel -= APPLIED_VEL,
                KeyCode::S => new_y_vel -= APPLIED_VEL,
                KeyCode::D => new_x_vel += APPLIED_VEL,
                _ => println!("unmapped keycode: {:?}", key),
            }
        }

        player_vel.linvel = Vec2::new(new_x_vel, new_y_vel);
        //player_vel.angvel = 0.;
    }
}
