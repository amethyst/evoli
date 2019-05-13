use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GlobalTransform {
    pub transform: Transform,
}

impl Component for GlobalTransform {
    type Storage = DenseVecStorage<Self>;
}
