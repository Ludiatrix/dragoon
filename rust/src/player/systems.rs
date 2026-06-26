use godot::global::godot_print;

use crate::prelude::*;

use super::components::*;

pub fn debug_player(
    players: Query<(&MoveSpeed, &JumpVelocity, &Gravity), With<Player>>,
) {
    for (speed, jump, gravity) in &players {
        godot_print!(
            "Player: speed={}, jump={}, gravity={}",
            speed.0,
            jump.0,
            gravity.0
        );
    }
}