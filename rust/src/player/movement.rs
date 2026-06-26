use crate::prelude::*;

use godot::classes::CharacterBody3D;
use godot::prelude::{godot_print, Vector3};

use super::components::*;

pub fn debug_player_spawned(
    players: Query<(&MoveSpeed, &JumpVelocity, &Gravity), Added<Player>>,
) {
    for (speed, jump, gravity) in &players {
        godot_print!(
            "PlayerNode became ECS Player: speed={}, jump={}, gravity={}",
            speed.0,
            jump.0,
            gravity.0
        );
    }
}

pub fn move_player(
    mut godot: GodotAccess,
    physics_delta: Res<PhysicsDelta>,
    mut players: Query<(
        &MoveSpeed,
        &JumpVelocity,
        &Gravity,
        &PlayerInputState,
        &DesiredMoveDirection,
        &mut VerticalVelocity,
        &GodotNodeHandle,
    ), With<Player>>,
) {
    let delta = physics_delta.delta_seconds;

    for (
        move_speed,
        jump_velocity,
        gravity,
        input_state,
        desired_direction,
        mut vertical_velocity,
        node_handle,
    ) in &mut players {
        let Some(mut body) = godot.try_get::<CharacterBody3D>(*node_handle) else {
            godot_print!("Player entity does not have a valid CharacterBody3D handle.");
            continue;
        };

        let is_on_floor = body.is_on_floor();

        if is_on_floor && vertical_velocity.0 < 0.0 {
            vertical_velocity.0 = 0.0;
        }

        if input_state.jump && is_on_floor {
            vertical_velocity.0 = jump_velocity.0;
        }

        if !is_on_floor {
            vertical_velocity.0 -= gravity.0 * delta;
        }

        let horizontal_velocity = desired_direction.0 * move_speed.0;

        let velocity = Vector3::new(
            horizontal_velocity.x,
            vertical_velocity.0,
            horizontal_velocity.z,
        );

        body.set_velocity(velocity);
        body.move_and_slide();

        vertical_velocity.0 = body.get_velocity().y;
    }
}