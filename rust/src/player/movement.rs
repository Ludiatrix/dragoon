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
        &DesiredMoveDirection,
        &MoveDamper,
        &mut VerticalVelocity,
        &mut JumpCharge,
        &GodotNodeHandle,
    ), With<Player>>,
) {
    let delta = physics_delta.delta_seconds;

    for (
        move_speed,
        jump_velocity,
        gravity,
        desired_direction,
        move_damper,
        mut vertical_velocity,
        mut jump_charge,
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

        if is_on_floor {
            if let Some(charge) = jump_charge.queued_jump.take() {
                vertical_velocity.0 = jump_velocity_for_charge(jump_velocity.0, charge);
            }
        } else {
            jump_charge.queued_jump = None;
        }

        if !is_on_floor {
            vertical_velocity.0 -= gravity.0 * delta;
        }

        let horizontal_velocity =
            desired_direction.0 * move_speed.0 * move_damper.0;

        body.set_velocity(Vector3::new(
            horizontal_velocity.x,
            vertical_velocity.0,
            horizontal_velocity.z,
        ));

        body.move_and_slide();

        vertical_velocity.0 = body.get_velocity().y;
    }
}

fn jump_velocity_for_charge(base_jump_velocity: f32, charge: Charge) -> f32 {
    match charge {
        Charge::Low => base_jump_velocity,
        Charge::Medium => base_jump_velocity * 1.5,
        Charge::Max => base_jump_velocity * 2.1,
    }
}