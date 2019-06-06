use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
//use amethyst_inspector::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Digestion {
    // Points of fullness lost every second
    pub nutrition_burn_rate: f32,
}

impl Component for Digestion {
    type Storage = DenseVecStorage<Self>;
}

impl Digestion {
    pub fn new(nutrition_burn_rate: f32) -> Digestion {
        Digestion {
            nutrition_burn_rate,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Fullness {
    pub max: f32,
    pub value: f32,
}

impl Component for Fullness {
    type Storage = DenseVecStorage<Self>;
}

impl Fullness {
    pub fn new(initial: f32, max: f32) -> Fullness {
        Fullness {
            value: initial,
            max,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Nutrition {
    // nutritional value of the entity
    pub value: f32,
}

impl Component for Nutrition {
    type Storage = DenseVecStorage<Self>;
}

impl Nutrition {
    pub fn new(value: f32) -> Nutrition {
        Nutrition { value }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DigestionPrefabData {
    fullness: Fullness,
    digestion: Digestion,
    nutrition: Nutrition,
}
