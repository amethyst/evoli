use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Clone, Debug, Default)]
pub struct SwarmCenter {
    pub entities: Vec<Entity>,
}

impl Component for SwarmCenter {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Default)]
pub struct SwarmBehavior {
    pub swarm_center: Option<Entity>,

    pub attraction: f32,
    pub deviation: f32,
}

impl Component for SwarmBehavior {
    type Storage = DenseVecStorage<Self>;
}
