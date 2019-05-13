use amethyst::{
    assets::{PrefabData, PrefabError, ProgressCounter},
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
pub struct Perception {
    pub range: f32,
    #[serde(skip)]
    pub entities: Vec<Entity>,
}

impl Component for Perception {
    type Storage = DenseVecStorage<Self>;
}
