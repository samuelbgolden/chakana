//use crate::prelude::*;

/*
pub fn update_player_state(
    mut query_player: Query<&mut Player, &mut >,
    query_messages: Query<&ChangePlayerStateMessage>,
) {
    let mut player = query_player.iter_mut().nth(0).unwrap();

    for msg in query_messages.iter_mut() {
        // check if this is a state change or a redundant message
        if player.state == msg.new_state {
            break;
        }

    }
    query_messages.for_each_mut(|msg| player.state = msg.new_state);
}
*/
