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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charge {
    Low,
    Medium,
    Max,
}

#[derive(Component, Debug, Clone)]
pub struct JumpCharge {
    pub is_charging: bool,
    pub elapsed: f32,
    pub current: Charge,
    pub queued_jump: Option<Charge>,
}

impl Default for JumpCharge {
    fn default() -> Self {
        Self {
            is_charging: false,
            elapsed: 0.0,
            current: Charge::Low,
            queued_jump: None,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct MoveDamper(pub f32);

impl Default for MoveDamper {
    fn default() -> Self {
        Self(1.0)
    }
}


#[derive(Component, Default, Debug, Clone)]
pub struct PlayerInputState {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,

    pub jump_pressed: bool,
    pub jump_held: bool,
    pub jump_released: bool,
}