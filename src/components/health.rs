use amethyst::ecs::{Component, DenseVecStorage, ReadStorage};
use amethyst_imgui::imgui;

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

impl<'a> amethyst_inspector::Inspect<'a> for Health {
    type SystemData = (ReadStorage<'a, Self>,);

    fn inspect(
        (storage,): &Self::SystemData,
        entity: amethyst::ecs::Entity,
        ui: &imgui::Ui<'_>,
    ) {
        let &Health {
            mut value,
            mut max_health,
        } = if let Some(x) = storage.get(entity) {
            x
        } else {
            return;
        };
        ui.drag_float(imgui::im_str!("max health##health{:?}", entity), &mut max_health)
            .build();
        ui.drag_float(imgui::im_str!("value##health{:?}", entity), &mut value)
            .build();
        ui.separator();
    }
}
