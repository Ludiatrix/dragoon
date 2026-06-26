use crate::prelude::*;

use godot::classes::CharacterBody3D;
use godot::prelude::{Vector3, godot_print};

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

#[derive(QueryData)]
pub struct PlayerSpawnDebug {
    pub move_speed: &'static MoveSpeed,
    pub jump_velocity: &'static JumpVelocity,
    pub gravity: &'static Gravity,
}

pub fn debug_player_spawned(players: Query<PlayerSpawnDebug, Added<Player>>) {
    for player in &players {
        godot_print!(
            "PlayerNode became ECS Player: speed={}, jump={}, gravity={}",
            player.move_speed.0,
            player.jump_velocity.0,
            player.gravity.0
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

        update_vertical_velocity(
            body.is_on_floor(),
            delta,
            player.jump_velocity.0,
            player.gravity.0,
            &mut player.vertical_velocity,
            &mut player.jump_charge,
        );

        let velocity = build_character_velocity(
            player.desired_direction.0,
            player.move_speed.0,
            player.move_damper.0,
            player.vertical_velocity.0,
        );

        body.set_velocity(velocity);
        body.move_and_slide();

        player.vertical_velocity.0 = body.get_velocity().y;
    }
}

fn update_vertical_velocity(
    is_on_floor: bool,
    delta: f32,
    base_jump_velocity: f32,
    gravity: f32,
    vertical_velocity: &mut VerticalVelocity,
    jump_charge: &mut JumpCharge,
) {
    if is_on_floor && vertical_velocity.0 < 0.0 {
        vertical_velocity.0 = 0.0;
    }

    if is_on_floor {
        if let Some(charge) = jump_charge.queued_jump.take() {
            vertical_velocity.0 = jump_velocity_for_charge(base_jump_velocity, charge);
        }
    } else {
        jump_charge.cancel_queued_jump();
        vertical_velocity.0 -= gravity * delta;
    }
}

fn build_character_velocity(
    desired_direction: Vec3,
    move_speed: f32,
    move_damper: f32,
    vertical_velocity: f32,
) -> Vector3 {
    let horizontal_velocity = desired_direction * move_speed * move_damper;

    Vector3::new(
        horizontal_velocity.x,
        vertical_velocity,
        horizontal_velocity.z,
    )
}

fn jump_velocity_for_charge(base_jump_velocity: f32, charge: Charge) -> f32 {
    base_jump_velocity * charge.jump_velocity_multiplier()
}
