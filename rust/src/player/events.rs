use bevy::ecs::message::Message;
use crate::player::components::Charge;

// Jump Held Message
#[derive(Message)]
pub(crate) struct OnJumpHeld(Charge);

// Jump Release Message
#[derive(Message)]
pub(crate) struct OnJumpReleased(f32);