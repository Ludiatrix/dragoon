use crate::prelude::*;

use super::components::Charge;

#[derive(Message, Debug, Clone, Copy)]
pub struct JumpChargeStarted {
    pub entity: Entity,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct JumpChargeChanged {
    pub entity: Entity,
    pub charge: Charge,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct JumpChargeReleased {
    pub entity: Entity,
    pub charge: Charge,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct JumpLaunched {
    pub entity: Entity,
    pub charge: Charge,
}