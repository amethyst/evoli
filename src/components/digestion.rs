use amethyst::ecs::{Component, DenseVecStorage, LazyUpdate, Read, ReadStorage, WriteStorage};
use amethyst_imgui::imgui;
use amethyst_inspector::Inspect;

#[derive(Default, Inspect, Clone)]
pub struct Digestion {
    // Points of fullness lost every second
    pub nutrition_burn_rate: f32,
}

impl Component for Digestion {
    type Storage = DenseVecStorage<Self>;
}

impl Digestion {
    pub fn new(nutrition_burn_rate: f32) -> Digestion {
        Digestion { nutrition_burn_rate }
    }
}

#[derive(Default, Debug, Inspect, Clone)]
pub struct Fullness {
    pub max: f32,
    pub value: f32,
}

impl Component for Fullness {
    type Storage = DenseVecStorage<Self>;
}

impl Fullness {
    pub fn new(initial: f32, max: f32) -> Fullness {
        Fullness { value: initial, max }
    }
}
