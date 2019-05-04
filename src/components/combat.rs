use amethyst::{
    assets::{PrefabData, PrefabError, ProgressCounter},
    core::Named,
    derive::PrefabData,
    ecs::{
        Component, DenseVecStorage, Entity, HashMapStorage, ReadStorage, VecStorage, WriteStorage,
    },
    prelude::*,
};
use amethyst_imgui::imgui;
use amethyst_inspector::Inspect;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Default, Debug, Inspect, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
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

#[derive(Default, Debug, Inspect, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
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
#[derive(Default, Debug, Inspect, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
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
#[derive(Default, Debug, PartialEq, Eq, Inspect, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
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

#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub enum Faction {
    Carnivore,
    Herbivore,
    Plant,
}

impl Component for Faction {
    type Storage = VecStorage<Self>;
}

impl Faction {
    pub fn is_enemy(&self, other: &Self) -> bool {
        match self {
            Faction::Carnivore => *other == Faction::Herbivore,
            Faction::Herbivore => *other == Faction::Plant,
            Faction::Plant => false,
        }
    }
}

///
///
///
// Indicate whether the entity is part of a faction. Factions are used to represent groups of
// entities that attack each other, see `HasFaction`. A faction is an entity of its own and might
// specify properties using components.
#[derive(Debug, Clone)]
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
// Store the faction entities this component's owner is hostile towards
#[derive(Default, Debug, Clone)]
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
    let plants = world.create_entity().with(Named::new("Plants")).build();

    let herbivores = world.create_entity().with(Named::new("Herbivores")).build();

    let carnivores = world.create_entity().with(Named::new("Carnivores")).build();

    return (plants, herbivores, carnivores);
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CombatPrefabData {
    health: Option<Health>,
    speed: Option<Speed>,
    damage: Option<Damage>,
    faction: Option<Faction>,
}
