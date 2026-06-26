use super::stats::EnemyStats;
use crate::prelude::*;
use godot::obj::InstanceId;

#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub stats: Handle<EnemyStats>,
    pub body: GodotNodeHandle,
    pub model: GodotNodeHandle,
}

impl Default for Enemy {
    fn default() -> Self {
        let invalid = GodotNodeHandle::from_instance_id(InstanceId::from_i64(1));
        Self {
            stats: Handle::default(),
            body: invalid,
            model: invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonAiState {
    Idle,
    Moving,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonLocomotion {
    Walk,
    Fly,
}

#[derive(Component, Debug, Clone)]
pub struct DragonAi {
    pub state: DragonAiState,
    pub locomotion: DragonLocomotion,
    pub target: Vec3,
    pub repick_timer: Timer,
}

impl Default for DragonAi {
    fn default() -> Self {
        Self {
            state: DragonAiState::Idle,
            locomotion: DragonLocomotion::Walk,
            target: Vec3::ZERO,
            repick_timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}
