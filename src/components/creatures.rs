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
#[derive(Clone, Copy, Debug, Default, PrefabData)]
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
    #[serde(skip)]
    tag: CreatureTag,

    pub name: Option<Named>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
    movement: Option<Movement>,
    wander: Option<Wander>,
    collider: Option<Circle>,
    digestion: Option<DigestionPrefabData>,
    combat: Option<CombatPrefabData>,
    intelligence_tag: Option<IntelligenceTag>,
    perception: Option<Perception>,
    ricochet_tag: Option<RicochetTag>,
}
