use amethyst::ecs::{Component, DenseVecStorage, WriteStorage};

pub struct Digestion {
    // Points of fullness lost every second
    pub nutrition_burn_rate: f32,
}

impl Component for Digestion {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> amethyst_inspector::Inspect<'a> for Digestion {
    type UserData = &'a mut crate::UserData;
    const CAN_ADD: bool = true;

    fn inspect(
        storage: &mut WriteStorage<'_, Self>,
        entity: amethyst::ecs::Entity,
        ui: &amethyst_imgui::imgui::Ui<'_>,
        _user_data: Self::UserData,
    ) {
        let me = if let Some(x) = storage.get_mut(entity) {
            x
        } else {
            return;
        };
        ui.drag_float(
            amethyst_inspector::imgui::im_str!(
                "nutrition burn rate##digestion{}{}",
                entity.id(),
                entity.gen().id()
            ),
            &mut me.nutrition_burn_rate,
        )
        .build();
        ui.separator();
    }

    fn add(
        storage: &mut WriteStorage<'_, Self>,
        entity: amethyst::ecs::Entity,
        _user_data: Self::UserData,
    ) {
        storage.insert(entity, Digestion::new(0.)).unwrap();
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
    type UserData = &'a mut crate::UserData;
    const CAN_ADD: bool = true;

    fn inspect(
        storage: &mut WriteStorage<'_, Self>,
        entity: amethyst::ecs::Entity,
        ui: &amethyst_imgui::imgui::Ui<'_>,
        _user_data: Self::UserData,
    ) {
        let me = if let Some(x) = storage.get_mut(entity) {
            x
        } else {
            return;
        };
        ui.drag_float(
            amethyst_inspector::imgui::im_str!(
                "fullness value##fullness{}{}",
                entity.id(),
                entity.gen().id()
            ),
            &mut me.value,
        )
        .build();
        ui.drag_float(
            amethyst_inspector::imgui::im_str!(
                "fullness max##fullness{}{}",
                entity.id(),
                entity.gen().id()
            ),
            &mut me.max,
        )
        .build();
        ui.separator();
    }

    fn add(
        storage: &mut WriteStorage<'_, Self>,
        entity: amethyst::ecs::Entity,
        _user_data: Self::UserData,
    ) {
        storage.insert(entity, Fullness::new(0., 0.)).unwrap();
    }
}

impl Fullness {
    pub fn new(initial: f32, max: f32) -> Fullness {
        Fullness { value: initial, max }
    }
}
