use amethyst::ecs::{Component, DenseVecStorage, LazyUpdate, Read, ReadStorage, WriteStorage};
use amethyst_imgui::imgui;

pub struct Digestion {
    // Points of fullness lost every second
    pub nutrition_burn_rate: f32,
}

impl Component for Digestion {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> amethyst_inspector::Inspect<'a> for Digestion {
    type SystemData = (ReadStorage<'a, Self>, Read<'a, LazyUpdate>);
    const CAN_ADD: bool = true;

    fn inspect(
        (storage, lazy): &Self::SystemData,
        entity: amethyst::ecs::Entity,
        ui: &imgui::Ui<'_>,
    ) {
        let mut burn_rate = if let Some(x) = storage.get(entity) {
            x.nutrition_burn_rate
        } else {
            return;
        };
        ui.drag_float(
            imgui::im_str!("nutrition burn rate##digestion{:?}", entity,),
            &mut burn_rate,
        )
        .build();
        lazy.insert(entity, Digestion::new(burn_rate));
        ui.separator();
    }

    fn add((_storage, lazy): &Self::SystemData, entity: amethyst::ecs::Entity) {
        lazy.insert(entity, Digestion::new(0.));
    }
}

impl Digestion {
    pub fn new(nutrition_burn_rate: f32) -> Digestion {
        Digestion { nutrition_burn_rate }
    }
}

pub struct Fullness {
    pub max: f32,
    pub value: f32,
}

impl Component for Fullness {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> amethyst_inspector::Inspect<'a> for Fullness {
    type SystemData = (ReadStorage<'a, Self>, Read<'a, LazyUpdate>);
    const CAN_ADD: bool = true;

    fn inspect(
        (storage, lazy): &Self::SystemData,
        entity: amethyst::ecs::Entity,
        ui: &imgui::Ui<'_>,
    ) {
        let &Fullness { mut max, mut value } = if let Some(x) = storage.get(entity) {
            x
        } else {
            return;
        };
        ui.drag_float(
            imgui::im_str!("fullness value##fullness{:?}", entity,),
            &mut value,
        )
        .build();
        ui.drag_float(
            imgui::im_str!("fullness max##fullness{:?}", entity,),
            &mut max,
        )
        .build();
        lazy.insert(entity, Fullness::new(value, max));
        ui.separator();
    }

    fn add((_storage, lazy): &Self::SystemData, entity: amethyst::ecs::Entity) {
        lazy.insert(entity, Fullness::new(0., 0.));
    }
}

impl Fullness {
    pub fn new(initial: f32, max: f32) -> Fullness {
        Fullness { value: initial, max }
    }
}
