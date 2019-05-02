use amethyst::core::Named;
use amethyst::ecs::{Component, DenseVecStorage, Entity, HashMapStorage, ReadStorage, VecStorage};
use amethyst::prelude::*;
use amethyst_imgui::imgui;
use amethyst_inspector::Inspect;
use std::time::Duration;

#[derive(Clone, Default, Inspect)]
pub struct Damage {
    // Points subtracted from target's health per hit
    pub damage: f32,
}

impl Component for Damage {
    type Storage = DenseVecStorage<Self>;
}

impl Damage {
    pub fn new(damage: f32) -> Damage {
        Damage { damage }
    }
}

///
///
///
#[derive(Clone, Default, Inspect)]
pub struct Speed {
    pub attacks_per_second: f32,
}

impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}

impl Speed {
    pub fn new(attacks_per_second: f32) -> Speed {
        Speed { attacks_per_second }
    }
}

///
///
///
// As long as the cooldown component is attached to an entity, that entity won't be able to attack.
#[derive(Debug, Clone, PartialEq, Eq, Default, Inspect)]
pub struct Cooldown {
    pub time_left: Duration,
}

impl Component for Cooldown {
    type Storage = DenseVecStorage<Self>;
}

impl Cooldown {
    pub fn new(time_left: Duration) -> Cooldown {
        Cooldown { time_left }
    }
}

///
///
///
// Indicate whether the entity is part of a faction. Factions are used to represent groups of
// entities that attack each other, see `HasFaction`. A faction is an entity of its own and might
// specify properties using components.
pub struct HasFaction {
    pub faction: Entity,
}

impl Component for HasFaction {
    type Storage = VecStorage<Self>;
}

impl HasFaction {
    pub fn new(faction: Entity) -> HasFaction {
        HasFaction { faction }
    }
}

impl<'a> amethyst_inspector::Inspect<'a> for HasFaction {
    type SystemData = (ReadStorage<'a, Self>,);

    fn inspect((storage,): &Self::SystemData, entity: amethyst::ecs::Entity, ui: &imgui::Ui<'_>) {
        let &HasFaction { faction } = if let Some(x) = storage.get(entity) {
            x
        } else {
            return;
        };
        ui.drag_int(
            imgui::im_str!("faction##has faction{:?}", entity),
            &mut (faction.id() as i32),
        )
        .build();
        ui.separator();
    }
}

///
///
///
#[derive(Default)]
// Store the faction entities this component's owner is hostile towards
pub struct FactionEnemies {
    pub enemies: Vec<Entity>,
}

impl Component for FactionEnemies {
    type Storage = HashMapStorage<Self>;
}

impl FactionEnemies {
    pub fn is_enemy(&self, other: &Entity) -> bool {
        self.enemies.contains(other)
    }
}

///
///
///
pub fn create_factions(world: &mut World) -> (Entity, Entity, Entity) {
    let plants = world
        .create_entity()
        .with(Named::new("Plants"))
        .with(FactionEnemies::default())
        .build();

    let herbivores = world
        .create_entity()
        .with(Named::new("Herbivores"))
        .with(FactionEnemies {
            enemies: vec![plants],
        })
        .build();

    let carnivores = world
        .create_entity()
        .with(Named::new("Carnivores"))
        .with(FactionEnemies {
            enemies: vec![herbivores],
        })
        .build();

    return (plants, herbivores, carnivores);
}
