use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{BitSet, Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
//use amethyst_inspector::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
pub struct Perception {
    pub range: f32,
}

impl Component for Perception {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Clone, Debug)]
pub struct DetectedEntities {
    pub entities: BitSet,
}

impl Component for DetectedEntities {
    type Storage = DenseVecStorage<Self>;
}
