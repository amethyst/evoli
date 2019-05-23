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
pub struct MatchVelocityRule {
    pub strength: f32,
}

impl Component for MatchVelocityRule {
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

#[derive(Default, Clone, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
pub struct AvoidRule {
    pub names: Vec<String>,
    pub strength: f32,
}

impl Component for AvoidRule {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct WorldBoundsRule {
    pub strength: f32,
}

impl Default for WorldBoundsRule {
    fn default() -> Self {
        WorldBoundsRule { strength: 10.0 }
    }
}

impl Component for WorldBoundsRule {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BoidsPrefabData {
    flocking: Option<FlockingRule>,
    match_velocity: Option<MatchVelocityRule>,
    avoid: Option<AvoidRule>,
    minimum_distance: Option<MinimumDistanceRule>,

    #[serde(skip)]
    wall_bounds: WorldBoundsRule,
}
