use crate::prelude::*;

#[derive(Component, Default, Debug, Clone)]
pub struct Player;

#[derive(Component, Debug, Clone)]
pub struct MoveSpeed(pub f32);

impl Default for MoveSpeed {
    fn default() -> Self {
        Self(5.0)
    }
}

#[derive(Component, Debug, Clone)]
pub struct JumpVelocity(pub f32);

impl Default for JumpVelocity {
    fn default() -> Self {
        Self(8.0)
    }
}

#[derive(Component, Debug, Clone)]
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(24.0)
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct DesiredMoveDirection(pub Vec3);

#[derive(Component, Default, Debug, Clone)]
pub struct VerticalVelocity(pub f32);

#[derive(Component, Default, Debug, Clone)]
pub struct PlayerInputState {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,
    pub jump: bool,
}