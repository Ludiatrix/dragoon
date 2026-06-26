use super::components::Enemy;
use crate::prelude::*;

#[derive(Bundle, GodotNode, Default)]
#[godot_node(base(RigidBody3D), class_name(DragonNode))]
pub struct DragonBundle {
    pub enemy: Enemy,
}
