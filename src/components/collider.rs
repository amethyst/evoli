use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Circle {
    pub radius: f32,
}

impl Component for Circle {
    type Storage = DenseVecStorage<Self>;
}
