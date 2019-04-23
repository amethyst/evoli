use amethyst::ecs::{Component, DenseVecStorage};

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
