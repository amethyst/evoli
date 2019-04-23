use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct Digestion {
    // Points of fullness lost every second
    pub nutrition_burn_rate: f32,
}

impl Component for Digestion {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> amethyst_inspector::Inspect<'a> for Digestion {
    type UserData = &'a mut crate::UserData;

    fn inspect(
        &mut self,
        entity: amethyst::ecs::Entity,
        ui: &amethyst_inspector::imgui::Ui<'_>,
        _user_data: Self::UserData,
    ) {
        ui.drag_float(
            amethyst_inspector::imgui::im_str!(
                "nutrition burn rate##digestion{}{}",
                entity.id(),
                entity.gen().id()
            ),
            &mut self.nutrition_burn_rate,
        )
        .build();
        ui.separator();
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

    fn inspect(
        &mut self,
        entity: amethyst::ecs::Entity,
        ui: &amethyst_inspector::imgui::Ui<'_>,
        _user_data: Self::UserData,
    ) {
        ui.drag_float(
            amethyst_inspector::imgui::im_str!(
                "fullness value##fullness{}{}",
                entity.id(),
                entity.gen().id()
            ),
            &mut self.value,
        )
        .build();
        ui.drag_float(
            amethyst_inspector::imgui::im_str!(
                "fullness max##fullness{}{}",
                entity.id(),
                entity.gen().id()
            ),
            &mut self.max,
        )
        .build();
        ui.separator();
    }
}

impl Fullness {
    pub fn new(initial: f32, max: f32) -> Fullness {
        Fullness { value: initial, max }
    }
}
