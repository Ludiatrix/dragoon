use crate::prelude::*;

use godot::{classes::Input, obj::Singleton};

use super::components::*;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerInput {
    pub state: &'static mut PlayerInputState,
    pub desired_direction: &'static mut DesiredMoveDirection,
}

struct InputSnapshot {
    move_left: bool,
    move_right: bool,
    move_forward: bool,
    move_backward: bool,
    jump_pressed: bool,
    jump_held: bool,
    jump_released: bool,
}

impl InputSnapshot {
    fn from_godot() -> Self {
        let input = Input::singleton();

        Self {
            move_left: input.is_action_pressed("move_left"),
            move_right: input.is_action_pressed("move_right"),
            move_forward: input.is_action_pressed("move_forward"),
            move_backward: input.is_action_pressed("move_backward"),
            jump_pressed: input.is_action_just_pressed("jump"),
            jump_held: input.is_action_pressed("jump"),
            jump_released: input.is_action_just_released("jump"),
        }
    }
}

pub fn read_player_input(mut players: Query<PlayerInput, With<Player>>) {
    let input = InputSnapshot::from_godot();

    for mut player in &mut players {
        player.state.set_movement(
            input.move_left,
            input.move_right,
            input.move_forward,
            input.move_backward,
        );

        player
            .state
            .set_jump(input.jump_pressed, input.jump_held, input.jump_released);

        *player.desired_direction = DesiredMoveDirection::from_input(&player.state);
    }
}
