use amethyst::{
    assets::{AssetPrefab, PrefabData, ProgressCounter},
    core::{math::Vector3, Named},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, NullStorage, WriteStorage},
    gltf::{GltfSceneAsset, GltfSceneFormat},
    Error,
};
//use amethyst_inspector::Inspect;

use serde::{Deserialize, Serialize};

use crate::components::{
    collider::Circle, combat::CombatPrefabData, digestion::DigestionPrefabData,
    perception::Perception,
};

pub type CreatureType = String;

// tag all creatures for when we need to run operations against everything
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct CreatureTag;
impl Component for CreatureTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct RicochetTag;

impl Component for RicochetTag {
    type Storage = NullStorage<Self>;
}

/// Entities tagged with this Component (and of course a Transform and Movement) will actively
/// avoid obstacles by steering away from them.
/// The world bounds currently (v0.2.0) are the only obstacles.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct AvoidObstaclesTag;

impl Component for AvoidObstaclesTag {
    type Storage = NullStorage<Self>;
}


/// Required on Topplegrass, this is what gives it its toppling animation.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct TopplegrassTag;

impl Component for TopplegrassTag {
    type Storage = NullStorage<Self>;
}

/// Gives this tag to any entity that is falling and should be affected by gravity.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct FallingTag;

impl Component for FallingTag {
    type Storage = NullStorage<Self>;
}

/// Entities tagged with this Component will despawn as soon as their position is outside the world bounds.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct DespawnWhenOutOfBoundsTag;

impl Component for DespawnWhenOutOfBoundsTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct IntelligenceTag;
impl Component for IntelligenceTag {
    type Storage = NullStorage<Self>;
}

///
///
///
#[derive(Clone, smart_default::SmartDefault, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Movement {
    #[default(Vector3::zeros())]
    pub velocity: Vector3<f32>,
    pub max_movement_speed: f32,
}
impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

///
///
///
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Wander {
    pub angle: f32,
    pub radius: f32,
}
impl Component for Wander {
    type Storage = DenseVecStorage<Self>;
}

impl Wander {
    pub fn get_direction(&self) -> Vector3<f32> {
        Vector3::new(
            self.radius * self.angle.cos(),
            self.radius * self.angle.sin(),
            0.0,
        )
    }
}

// This is the main prefab data for creatures.
// It defines all the components that a creature could have.
// In the prefab, it is not necessary to define all of them (due to Option).
// Only define the ones you want to add to your entity.
#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CreaturePrefabData {
    pub name: Option<Named>,
    creature_tag: Option<CreatureTag>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
    movement: Option<Movement>,
    wander: Option<Wander>,
    collider: Option<Circle>,
    digestion: Option<DigestionPrefabData>,
    combat: Option<CombatPrefabData>,
    intelligence_tag: Option<IntelligenceTag>,
    perception: Option<Perception>,
    ricochet_tag: Option<RicochetTag>,
    avoid_obstacles_tag: Option<AvoidObstaclesTag>,
    despawn_when_out_of_bounds_tag: Option<DespawnWhenOutOfBoundsTag>,
    topplegrass_tag: Option<TopplegrassTag>,
    falling_tag: Option<FallingTag>,
}
