use super::bundles::DragonNode;
use super::components::Enemy;
use super::stats::EnemyStats;
use crate::prelude::*;
use godot::classes::{CapsuleShape3D, CollisionShape3D, Node, Shape3D};
use godot::prelude::*;

const SPAWN_POSITIONS: [Vec3; 2] = [Vec3::new(-4.0, 3.0, 0.0), Vec3::new(4.0, 3.0, 0.0)];

#[derive(Component)]
pub struct EnemyModelSpawned;

#[derive(Component)]
pub struct EnemyModelFor(pub GodotNodeHandle);

pub fn spawn_dragons(mut spawned: Local<bool>, mut scene_tree: SceneTreeRef) {
    if *spawned {
        return;
    }

    let Some(mut root) = scene_tree.get().get_root() else {
        return;
    };

    for pos in SPAWN_POSITIONS {
        let mut dragon = DragonNode::new_alloc();
        dragon.set_lock_rotation_enabled(true);
        let mut collision = CollisionShape3D::new_alloc();
        let shape = CapsuleShape3D::new_gd();
        let _ = &collision.set_shape(&shape.upcast::<Shape3D>());
        let _ = &collision
            .clone()
            .upcast::<Node3D>()
            .set_position(Vector3::new(0.0, 1.0, 0.0));
        dragon.add_child(&collision);
        dragon.set_global_position(Vector3::new(pos.x, pos.y, pos.z));
        root.add_child(&dragon.upcast::<Node>());
    }

    *spawned = true;
}

pub fn assign_enemy_stats(
    mut enemies: Query<&mut Enemy, Added<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    for mut enemy in &mut enemies {
        enemy.stats = asset_server.load("enemies/stats_dragon.tres");
    }
}

pub fn assign_enemy_body(mut enemies: Query<(&mut Enemy, &GodotNodeHandle), Added<Enemy>>) {
    for (mut enemy, handle) in &mut enemies {
        enemy.body = *handle;
    }
}

pub fn spawn_enemy_models(
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy), Without<EnemyModelSpawned>>,
    enemy_stats: Res<Assets<EnemyStats>>,
) {
    for (entity, enemy) in &enemies {
        let Some(stats) = enemy_stats.get(&enemy.stats) else {
            continue;
        };

        commands.spawn((
            GodotScene::from_handle(stats.model.clone()).with_parent(enemy.body),
            Transform::default(),
            EnemyModelFor(enemy.body),
        ));
        commands.entity(entity).insert(EnemyModelSpawned);
    }
}

pub fn assign_enemy_model(
    new_models: Query<(&GodotNodeHandle, &EnemyModelFor), Added<GodotNodeHandle>>,
    mut enemies: Query<&mut Enemy>,
) {
    for (model_handle, parent) in &new_models {
        for mut enemy in &mut enemies {
            if enemy.body == parent.0 {
                enemy.model = *model_handle;
            }
        }
    }
}
