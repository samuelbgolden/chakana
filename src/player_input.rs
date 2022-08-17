pub use crate::prelude::*;

pub struct InputPollTimer(pub Timer);

pub fn handle_player_input(
    keyb_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<InputPollTimer>,
    mut query0: Query<(&mut SemiImplicitEulerPhysics, &mut Transform, &mut Player)>,
    mut debug_context: ResMut<DebugRenderContext>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut phys, mut trans, mut player) in query0.iter_mut() {
            // basic movement
            if keyb_in.pressed(KeyCode::A) {
                phys.accel.x -= 10.0;
            }
            if keyb_in.pressed(KeyCode::D) {
                phys.accel.x += 10.0;
            }
            if keyb_in.just_pressed(KeyCode::Space) || keyb_in.just_pressed(KeyCode::W) {
                phys.accel.y += 50.0;
            }

            // dev shortcuts
            if keyb_in.just_pressed(KeyCode::Slash) {
                debug_context.enabled = !debug_context.enabled
            }

            if keyb_in.just_pressed(KeyCode::R) {
                phys.stop();
                trans.translation.x = 0.0;
                trans.translation.y = 0.0;
            }
        }
    }
}
