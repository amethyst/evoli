use amethyst::{
    assets::{ PrefabData, PrefabError},
    ecs::{Component, DenseVecStorage, WriteStorage, Entity},
    derive::PrefabData,
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

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle { radius }
    }
}
