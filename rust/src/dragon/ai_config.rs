use crate::prelude::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::reflect::TypePath;
use godot::classes::ResourceLoader;
use godot::obj::{Gd, Singleton};
use godot::prelude::Resource as GodotBaseResource;
use std::fmt;

use super::components::Enemy;
use super::stats::EnemyStats;

#[derive(Asset, TypePath, Clone, Debug)]
pub struct EnemyAiConfig {
    pub wander_radius: f32,
    pub repick_secs: f32,
    pub stop_distance: f32,
    pub walk_speed: f32,
    pub fly_speed: f32,
    pub fly_y_min: f32,
    pub fly_y_max: f32,
    pub fly_chance: f32,
    pub play_area_half: f32,
    pub min_wander_distance: f32,
    pub turn_speed_rad: f32,
    pub anim_crossfade_secs: f32,
}

pub fn config_for<'a>(
    enemy: &Enemy,
    stats: &'a Assets<EnemyStats>,
    configs: &'a Assets<EnemyAiConfig>,
) -> Option<&'a EnemyAiConfig> {
    let stats = stats.get(&enemy.stats)?;
    configs.get(&stats.ai)
}

#[derive(Debug)]
pub enum EnemyAiConfigLoadError {
    ResourceLoadFailed(String),
    InvalidProperty(String),
}

impl fmt::Display for EnemyAiConfigLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceLoadFailed(msg) => write!(f, "failed to load enemy ai config: {msg}"),
            Self::InvalidProperty(msg) => write!(f, "invalid enemy ai config property: {msg}"),
        }
    }
}

impl std::error::Error for EnemyAiConfigLoadError {}

fn read_f32(resource: &Gd<GodotBaseResource>, name: &str) -> Result<f32, EnemyAiConfigLoadError> {
    resource
        .get(name)
        .try_to()
        .map_err(|_| EnemyAiConfigLoadError::InvalidProperty(name.into()))
}

fn enemy_ai_config_from_resource(
    resource: Gd<GodotBaseResource>,
) -> Result<EnemyAiConfig, EnemyAiConfigLoadError> {
    Ok(EnemyAiConfig {
        wander_radius: read_f32(&resource, "WanderRadius")?,
        repick_secs: read_f32(&resource, "RepickSecs")?,
        stop_distance: read_f32(&resource, "StopDistance")?,
        walk_speed: read_f32(&resource, "WalkSpeed")?,
        fly_speed: read_f32(&resource, "FlySpeed")?,
        fly_y_min: read_f32(&resource, "FlyYMin")?,
        fly_y_max: read_f32(&resource, "FlyYMax")?,
        fly_chance: read_f32(&resource, "FlyChance")?,
        play_area_half: read_f32(&resource, "PlayAreaHalf")?,
        min_wander_distance: read_f32(&resource, "MinWanderDistance")?,
        turn_speed_rad: read_f32(&resource, "TurnSpeedRad")?,
        anim_crossfade_secs: read_f32(&resource, "AnimCrossfadeSecs")?,
    })
}

#[derive(Default, TypePath)]
pub struct EnemyAiConfigAssetLoader;

impl AssetLoader for EnemyAiConfigAssetLoader {
    type Asset = EnemyAiConfig;
    type Settings = ();
    type Error = EnemyAiConfigLoadError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let godot_path = load_context.path().to_string();
        let path_gstring = godot::builtin::GString::from(&godot_path);

        let mut resource_loader = ResourceLoader::singleton();
        let resource = resource_loader.load(&path_gstring);

        match resource {
            Some(resource) => enemy_ai_config_from_resource(resource),
            None => Err(EnemyAiConfigLoadError::ResourceLoadFailed(format!(
                "Failed to load Godot resource: {godot_path}"
            ))),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["tres"]
    }
}
