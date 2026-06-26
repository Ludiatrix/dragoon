use bevy::{ecs::message::MessageReader, log::info};
use crate::player::events::*;

pub fn debug_jump_charge_events(
    mut started: MessageReader<JumpChargeStarted>,
    mut changed: MessageReader<JumpChargeChanged>,
    mut released: MessageReader<JumpChargeReleased>,
) {
    for event in started.read() {
        info!("Jump charge started: {:?}", event.entity);
    }

    for event in changed.read() {
        info!("Jump charge changed: {:?}", event.charge);
    }

    for event in released.read() {
        info!("Jump charge released: {:?}", event.charge);
    }
}