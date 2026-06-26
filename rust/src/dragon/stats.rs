use crate::prelude::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::reflect::TypePath;
use godot::classes::ResourceLoader;
use godot::obj::{Gd, Singleton};
use godot::prelude::Resource as GodotBaseResource;
use std::fmt;

#[derive(Asset, TypePath, Clone, Debug)]
pub struct EnemyStats {
    pub max_health: i32,
    pub model: Handle<GodotResource>,
}

#[derive(Debug)]
pub enum EnemyStatsLoadError {
    ResourceLoadFailed(String),
    InvalidProperty(String),
}

impl fmt::Display for EnemyStatsLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceLoadFailed(msg) => write!(f, "failed to load enemy stats: {msg}"),
            Self::InvalidProperty(msg) => write!(f, "invalid enemy stats property: {msg}"),
        }
    }
}

impl std::error::Error for EnemyStatsLoadError {}

fn enemy_stats_from_resource(
    resource: Gd<GodotBaseResource>,
    load_context: &mut LoadContext<'_>,
) -> Result<EnemyStats, EnemyStatsLoadError> {
    let max_health: i32 = resource
        .get("MaxHealth")
        .try_to()
        .map_err(|_| EnemyStatsLoadError::InvalidProperty("MaxHealth".into()))?;

    let model: Gd<GodotBaseResource> = resource
        .get("Model")
        .try_to()
        .map_err(|_| EnemyStatsLoadError::InvalidProperty("Model".into()))?;

    let path = model.get_path().to_string();
    if path.is_empty() {
        return Err(EnemyStatsLoadError::InvalidProperty(
            "Model resource has no path".into(),
        ));
    }

    let bevy_path = path.strip_prefix("res://").unwrap_or(&path).to_string();
    let model = load_context.load(bevy_path);

    Ok(EnemyStats { max_health, model })
}

#[derive(Default, TypePath)]
pub struct EnemyStatsAssetLoader;

impl AssetLoader for EnemyStatsAssetLoader {
    type Asset = EnemyStats;
    type Settings = ();
    type Error = EnemyStatsLoadError;

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
            Some(resource) => enemy_stats_from_resource(resource, load_context),
            None => Err(EnemyStatsLoadError::ResourceLoadFailed(format!(
                "Failed to load Godot resource: {godot_path}"
            ))),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["tres"]
    }
}
