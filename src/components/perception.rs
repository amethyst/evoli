use amethyst::{
    assets::{PrefabData, PrefabError},
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use amethyst_inspector::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Inspect, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
pub struct Perception {
    pub range: f32,

    #[serde(skip)]
    #[inspect(skip)]
    pub entities: Vec<Entity>,
}

impl Component for Perception {
    type Storage = DenseVecStorage<Self>;
}
