use crate::prelude::*;

use bevy::ecs::query::QueryData;
use godot::classes::CharacterBody3D;
use godot::prelude::{godot_print, Vector3};

use super::components::*;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerMovement {
    pub move_speed: &'static MoveSpeed,
    pub jump_velocity: &'static JumpVelocity,
    pub gravity: &'static Gravity,
    pub desired_direction: &'static DesiredMoveDirection,
    pub move_damper: &'static MoveDamper,
    pub vertical_velocity: &'static mut VerticalVelocity,
    pub jump_charge: &'static mut JumpCharge,
    pub node_handle: &'static GodotNodeHandle,
}


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
    mut players: Query<PlayerMovement, With<Player>>,
) {
    let delta = physics_delta.delta_seconds;

    for mut player in &mut players {
        let Some(mut body) = godot.try_get::<CharacterBody3D>(*player.node_handle) else {
            godot_print!("Player entity does not have a valid CharacterBody3D handle.");
            continue;
        };

        let is_on_floor = body.is_on_floor();

        if is_on_floor && player.vertical_velocity.0 < 0.0 {
            player.vertical_velocity.0 = 0.0;
        }

        if is_on_floor {
            if let Some(charge) = player.jump_charge.queued_jump.take() {
                player.vertical_velocity.0 =
                    jump_velocity_for_charge(player.jump_velocity.0, charge);
            }
        } else {
            player.jump_charge.queued_jump = None;
            player.vertical_velocity.0 -= player.gravity.0 * delta;
        }

        let horizontal_velocity =
            player.desired_direction.0 * player.move_speed.0 * player.move_damper.0;

        body.set_velocity(Vector3::new(
            horizontal_velocity.x,
            player.vertical_velocity.0,
            horizontal_velocity.z,
        ));

        body.move_and_slide();

        player.vertical_velocity.0 = body.get_velocity().y;
    }
}

fn jump_velocity_for_charge(base_jump_velocity: f32, charge: Charge) -> f32 {
    match charge {
        Charge::Low => base_jump_velocity,
        Charge::Medium => base_jump_velocity * 1.5,
        Charge::Max => base_jump_velocity * 2.1,
    }
}