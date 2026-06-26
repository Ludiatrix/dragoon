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
