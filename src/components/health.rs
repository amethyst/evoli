use amethyst::ecs::{Component, DenseVecStorage, ReadStorage};
use amethyst_imgui::imgui;
use amethyst_inspector::Inspect;

#[derive(Clone, Default, Inspect)]
pub struct Health {
    pub max_health: f32,
    pub value: f32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

impl Health {
    pub fn new(max_health: f32) -> Health {
        Health {
            max_health,
            value: max_health,
        }
    }
}
