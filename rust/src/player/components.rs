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

impl DesiredMoveDirection {
    pub fn from_input(input: &PlayerInputState) -> Self {
        let direction = Vec3::new(input.horizontal_axis(), 0.0, input.depth_axis());

        if direction.length_squared() > 0.0 {
            Self(direction.normalize())
        } else {
            Self(Vec3::ZERO)
        }
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct VerticalVelocity(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charge {
    Low,
    Medium,
    Max,
}

impl Charge {
    const MEDIUM_THRESHOLD_SECONDS: f32 = 1.0;
    const MAX_THRESHOLD_SECONDS: f32 = 2.0;

    const LOW_MOVE_DAMPER: f32 = 0.65;
    const MEDIUM_MOVE_DAMPER: f32 = 0.25;
    const MAX_MOVE_DAMPER: f32 = 0.0;

    const LOW_JUMP_MULTIPLIER: f32 = 1.0;
    const MEDIUM_JUMP_MULTIPLIER: f32 = 1.5;
    const MAX_JUMP_MULTIPLIER: f32 = 2.1;

    pub fn from_elapsed_seconds(elapsed: f32) -> Self {
        if elapsed >= Self::MAX_THRESHOLD_SECONDS {
            Self::Max
        } else if elapsed >= Self::MEDIUM_THRESHOLD_SECONDS {
            Self::Medium
        } else {
            Self::Low
        }
    }

    pub fn movement_damper(self) -> f32 {
        match self {
            Self::Low => Self::LOW_MOVE_DAMPER,
            Self::Medium => Self::MEDIUM_MOVE_DAMPER,
            Self::Max => Self::MAX_MOVE_DAMPER,
        }
    }

    pub fn jump_velocity_multiplier(self) -> f32 {
        match self {
            Self::Low => Self::LOW_JUMP_MULTIPLIER,
            Self::Medium => Self::MEDIUM_JUMP_MULTIPLIER,
            Self::Max => Self::MAX_JUMP_MULTIPLIER,
        }
    }
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

impl JumpCharge {
    pub fn start(&mut self) {
        self.is_charging = true;
        self.elapsed = 0.0;
        self.current = Charge::Low;
        self.queued_jump = None;
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        self.elapsed += delta_seconds;
        self.current = Charge::from_elapsed_seconds(self.elapsed);
    }

    pub fn release(&mut self) {
        self.queued_jump = Some(self.current);
        self.is_charging = false;
        self.elapsed = 0.0;
    }

    pub fn cancel_queued_jump(&mut self) {
        self.queued_jump = None;
    }
}

#[derive(Component, Debug, Clone)]
pub struct MoveDamper(pub f32);

impl Default for MoveDamper {
    fn default() -> Self {
        Self(1.0)
    }
}

impl MoveDamper {
    pub fn reset(&mut self) {
        self.0 = 1.0;
    }

    pub fn apply_charge(&mut self, charge: Charge) {
        self.0 = charge.movement_damper();
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct PlayerInputState {

    // Horizontal Axis
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,

    // Vertical Axis
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub jump_released: bool,
}

impl PlayerInputState {
    pub fn set_movement(
        &mut self,
        move_left: bool,
        move_right: bool,
        move_forward: bool,
        move_backward: bool,
    ) {
        self.move_left = move_left;
        self.move_right = move_right;
        self.move_forward = move_forward;
        self.move_backward = move_backward;
    }

    pub fn set_jump(&mut self, pressed: bool, held: bool, released: bool) {
        self.jump_pressed = pressed;
        self.jump_held = held;
        self.jump_released = released;
    }

    pub fn horizontal_axis(&self) -> f32 {
        axis_value(self.move_right, self.move_left)
    }

    pub fn depth_axis(&self) -> f32 {
        axis_value(self.move_backward, self.move_forward)
    }
}

fn axis_value(positive: bool, negative: bool) -> f32 {
    match (positive, negative) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    }
}
