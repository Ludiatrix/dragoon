use super::ai_config::{config_for, EnemyAiConfig};
use super::components::{DragonAi, DragonAiState, DragonLocomotion, Enemy};
use super::stats::EnemyStats;
use crate::prelude::*;
use godot::classes::{RandomNumberGenerator, RigidBody3D};
use godot::obj::InstanceId;
use godot::prelude::*;

fn shortest_angle_delta(from: f32, to: f32) -> f32 {    let mut delta = to - from;
    while delta > std::f32::consts::PI {
        delta -= std::f32::consts::TAU;
    }
    while delta < -std::f32::consts::PI {
        delta += std::f32::consts::TAU;
    }
    delta
}

pub fn pick_dragon_targets(    time: Res<Time>,
    mut godot: GodotAccess,
    stats: Res<Assets<EnemyStats>>,
    configs: Res<Assets<EnemyAiConfig>>,
    mut dragons: Query<(&Enemy, &mut DragonAi)>,
) {
    let invalid = GodotNodeHandle::from_instance_id(InstanceId::from_i64(1));
    let mut rng = RandomNumberGenerator::new_gd();

    for (enemy, mut ai) in &mut dragons {
        if ai.state != DragonAiState::Idle {
            continue;
        }

        let Some(config) = config_for(enemy, &stats, &configs) else {
            continue;
        };

        if !ai.repick_timer.tick(time.delta()).just_finished() {
            continue;
        }

        let Some(body) = godot.try_get::<RigidBody3D>(enemy.body) else {
            continue;
        };

        if enemy.body == invalid {
            continue;
        }

        let pos = body.get_global_position();
        let origin = Vec3::new(pos.x, pos.y, pos.z);

        let angle = rng.randf_range(0.0, std::f32::consts::TAU);
        let distance = rng.randf_range(config.min_wander_distance, config.wander_radius);
        let offset = Vec3::new(angle.cos() * distance, 0.0, angle.sin() * distance);

        ai.locomotion = if rng.randf() < config.fly_chance {
            DragonLocomotion::Fly
        } else {
            DragonLocomotion::Walk
        };

        let target_y = match ai.locomotion {
            DragonLocomotion::Walk => 0.0,
            DragonLocomotion::Fly => rng.randf_range(config.fly_y_min, config.fly_y_max),
        };

        ai.target = Vec3::new(
            (origin.x + offset.x).clamp(-config.play_area_half, config.play_area_half),
            target_y,
            (origin.z + offset.z).clamp(-config.play_area_half, config.play_area_half),
        );
        ai.state = DragonAiState::Moving;
    }
}

pub fn move_dragons(
    mut godot: GodotAccess,
    physics_delta: Res<PhysicsDelta>,
    stats: Res<Assets<EnemyStats>>,
    configs: Res<Assets<EnemyAiConfig>>,
    mut dragons: Query<(&Enemy, &mut DragonAi)>,
) {
    let invalid = GodotNodeHandle::from_instance_id(InstanceId::from_i64(1));
    let delta = physics_delta.delta_seconds;

    for (enemy, mut ai) in &mut dragons {
        if enemy.body == invalid {
            continue;
        }

        let Some(config) = config_for(enemy, &stats, &configs) else {
            continue;
        };

        let Some(mut body) = godot.try_get::<RigidBody3D>(enemy.body) else {
            continue;
        };

        let pos = body.get_global_position();
        let current = Vec3::new(pos.x, pos.y, pos.z);

        let to_target = ai.target - current;
        let arrived = to_target.length() < config.stop_distance;

        if arrived && ai.state == DragonAiState::Moving {
            ai.state = DragonAiState::Idle;
            ai.repick_timer = Timer::from_seconds(config.repick_secs, TimerMode::Once);
        }

        body.set_gravity_scale(match ai.locomotion {
            DragonLocomotion::Walk => 1.0,
            DragonLocomotion::Fly => 0.0,
        });

        let mut target_velocity = match ai.state {
            DragonAiState::Idle => Vec3::ZERO,
            DragonAiState::Moving => {
                let direction = to_target.normalize_or_zero();
                let speed = match ai.locomotion {
                    DragonLocomotion::Walk => config.walk_speed,
                    DragonLocomotion::Fly => config.fly_speed,
                };
                direction * speed
            }
        };
        if ai.locomotion == DragonLocomotion::Walk {
            target_velocity.y = body.get_linear_velocity().y;
        }
        body.set_linear_velocity(Vector3::new(
            target_velocity.x,
            target_velocity.y,
            target_velocity.z,
        ));

        if target_velocity.length_squared() > f32::EPSILON {
            let target_yaw = target_velocity.x.atan2(target_velocity.z);
            let rotation = body.get_rotation();
            let delta_yaw = shortest_angle_delta(rotation.y, target_yaw)
                .clamp(-config.turn_speed_rad * delta, config.turn_speed_rad * delta);
            body.set_rotation(Vector3::new(rotation.x, rotation.y + delta_yaw, rotation.z));
        }
    }
}
