pub use crate::prelude::*;

const APPLIED_VEL: f32 = 300.0;
const JUMP_VEL: f32 = 435.0;

pub struct InputPollTimer(pub Timer);

pub fn handle_player_input(
    keyb_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<InputPollTimer>,
    mut query0: Query<(
        &mut Velocity,
        &mut Transform,
        &mut GravityScale,
        &mut Player,
    )>,
    mut debug_context: ResMut<DebugRenderContext>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (mut player_vel, mut player_trans, mut player_grav, mut player) =
            query0.iter_mut().nth(0).unwrap();
        let mut new_x_vel: f32 = 0.0;
        let mut new_y_vel: f32 = player_vel.linvel.y;

        // basic movement
        if keyb_in.pressed(KeyCode::A) {
            new_x_vel -= APPLIED_VEL;
        }
        if keyb_in.pressed(KeyCode::D) {
            new_x_vel += APPLIED_VEL;
        }
        if keyb_in.just_pressed(KeyCode::Space) || keyb_in.just_pressed(KeyCode::W) {
            if (new_y_vel - 0.0).abs() < 0.1 {
                new_y_vel = JUMP_VEL;
            }
        }

        // fast fall
        if keyb_in.just_pressed(KeyCode::S) {
            player_grav.0 *= 2.0;
        }

        player_vel.linvel = Vec2::new(new_x_vel, new_y_vel);

        // changing animation (TEMPORARY)
        if keyb_in.just_pressed(KeyCode::Key2) {
            player.state = PlayerState::Idle;
        }
        if keyb_in.just_pressed(KeyCode::Key3) {
            player.state = PlayerState::Walking;
        }
        if keyb_in.just_pressed(KeyCode::Key4) {
            player.state = PlayerState::Jumping;
        }

        if keyb_in.just_pressed(KeyCode::R) {
            player_trans.translation = Vec3::new(0.0, 0.0, 0.0);
            player_grav.0 = 5.0;
        }

        // dev shortcuts
        if keyb_in.just_pressed(KeyCode::Slash) {
            debug_context.enabled = !debug_context.enabled
        }
    }
}
