use amethyst::{
    assets::{PrefabData, PrefabError},
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Perception {
    pub range: f32,
}

impl Component for Perception {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Clone, Debug)]
pub struct DetectedEntities {
    pub entities: Vec<Entity>,
}

impl Component for DetectedEntities {
    type Storage = DenseVecStorage<Self>;
}
