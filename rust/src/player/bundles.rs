use crate::prelude::*;
use super::components::*;

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody3D), class_name(PlayerNode))]
pub struct PlayerBundle {
    pub player: Player,

    #[export_fields(value(export_type(f32), default(5.0)))]
    pub move_speed: MoveSpeed,

    #[export_fields(value(export_type(f32), default(8.0)))]
    pub jump_velocity: JumpVelocity,

    #[export_fields(value(export_type(f32), default(24.0)))]
    pub gravity: Gravity,

    pub desired_move_direction: DesiredMoveDirection,
    pub vertical_velocity: VerticalVelocity,
    pub input_state: PlayerInputState,
    pub move_damper: MoveDamper,
    pub jump_charge: JumpCharge,
}