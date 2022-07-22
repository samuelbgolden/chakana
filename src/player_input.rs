pub use crate::prelude::*;

const MOVEMENT_SPEED: f32 = 100.0;

pub fn handle_player_input(
    keyb_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_trans = query.iter_mut().nth(0).unwrap();

    for key in keyb_in.get_pressed() {
        match key {
            KeyCode::W => player_trans.translation.y += MOVEMENT_SPEED * time.delta_seconds(),
            KeyCode::A => player_trans.translation.x -= MOVEMENT_SPEED * time.delta_seconds(),
            KeyCode::S => player_trans.translation.y -= MOVEMENT_SPEED * time.delta_seconds(),
            KeyCode::D => player_trans.translation.x += MOVEMENT_SPEED * time.delta_seconds(),
            _ => println!("unmapped keycode: {:?}", key),
        }
    }
}
