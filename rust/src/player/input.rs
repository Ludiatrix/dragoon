use crate::prelude::*;
use godot::{classes::Input, obj::Singleton};

use super::components::*;

pub fn read_player_input(
    mut players: Query<(&mut PlayerInputState, &mut DesiredMoveDirection), With<Player>>,
) {
    let input = Input::singleton();

    let move_left = input.is_action_pressed("move_left");
    let move_right = input.is_action_pressed("move_right");
    let move_forward = input.is_action_pressed("move_forward");
    let move_backward = input.is_action_pressed("move_backward");

    let jump_pressed = input.is_action_just_pressed("jump");
    let jump_held = input.is_action_pressed("jump");
    let jump_released = input.is_action_just_released("jump");

    for (mut input_state, mut desired_direction) in &mut players {
        input_state.move_left = move_left;
        input_state.move_right = move_right;
        input_state.move_forward = move_forward;
        input_state.move_backward = move_backward;
        input_state.jump_pressed = jump_pressed;
        input_state.jump_held = jump_held;
        input_state.jump_released = jump_released;

        let x =
            if input_state.move_right { 1.0 } else { 0.0 }
            - if input_state.move_left { 1.0 } else { 0.0 };

        let z =
            if input_state.move_backward { 1.0 } else { 0.0 }
            - if input_state.move_forward { 1.0 } else { 0.0 };

        let direction = Vec3::new(x, 0.0, z);

        desired_direction.0 = if direction.length_squared() > 0.0 {
            direction.normalize()
        } else {
            Vec3::ZERO
        };
    }
}