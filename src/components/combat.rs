use amethyst::ecs::error::Error;
use amethyst::{
    assets::{PrefabData, PrefabError, ProgressCounter},
    core::Named,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, HashMapStorage, Read, WriteStorage},
    prelude::*,
};
use amethyst_inspector::Inspect;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

///
///
///
// Indicate whether the entity is part of a faction. Factions are used to represent groups of
// entities that attack each other, see `HasFaction`. A faction is an entity of its own and might
// specify properties using components.
#[derive(Default, Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct HasFaction<T> {
    pub faction: T,
}

impl Component for HasFaction<Entity> {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> PrefabData<'a> for HasFaction<String> {
    type SystemData = (
        WriteStorage<'a, HasFaction<Entity>>,
        Read<'a, HashMap<String, Entity>>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<Self::Result, Error> {
        let faction = system_data.1.get(&self.faction);
        if let Some(f) = faction {
            system_data
                .0
                .insert(entity, HasFaction { faction: *f })
                .expect("Unreachable: we are inserting now.");
            return Ok(());
        }

        error!("Failed to load faction data");
        Ok(())
    }
}

// Store the faction entities this component's owner is hostile towards
#[derive(Default, Debug, PartialEq, Eq, Clone)]
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

pub fn create_factions(world: &mut World) {
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

    let mut factions = HashMap::new();
    factions.insert("Plants".to_string(), plants);
    factions.insert("Herbivores".to_string(), herbivores);
    factions.insert("Carnivores".to_string(), carnivores);
    world.add_resource(factions);
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CombatPrefabData {
    health: Option<Health>,
    speed: Option<Speed>,
    damage: Option<Damage>,
    has_faction: Option<HasFaction<String>>,
}
