use amethyst::{
    assets::{PrefabData, PrefabError, ProgressCounter},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use amethyst_inspector::Inspect;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Inspect, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
pub struct FlockingRule {
    pub strength: f32,
}

impl Component for FlockingRule {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Clone, Inspect, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
pub struct MinimumDistanceRule {
    pub minimum: f32,
    pub strength: f32,
}

impl Component for MinimumDistanceRule {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BoidsPrefabData {
    flocking: Option<FlockingRule>,
    minimum_distance: Option<MinimumDistanceRule>,
}
