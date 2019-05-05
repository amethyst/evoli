use amethyst::ecs::error::Error;
use amethyst::{
    assets::{PrefabData, PrefabError, PrefabLoader, ProgressCounter, RonFormat},
    core::Named,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, HashMapStorage, Read, Write, WriteStorage},
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

/// Indicate whether the entity is part of a faction. Factions are used to represent groups of
/// entities that attack each other, see `HasFaction`. A faction is an entity of its own and might
/// specify properties using components.
/// The type is generic because we use `HasFaction<Entity>` as a component and `HasFaction<String>` for the prefab.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct HasFaction<T> {
    pub faction: T,
}

impl Component for HasFaction<Entity> {
    type Storage = DenseVecStorage<Self>;
}

/// A custom PrefabData implementation because we are referencing entities in `HasFaction<Entity>`.
/// The prefab references the target entity using a name. The factions are stored using a lookup table.
/// This custom trait implementation will look up the name in the table and assign the correct faction entity
/// to the creature.
impl<'a> PrefabData<'a> for HasFaction<String> {
    type SystemData = (WriteStorage<'a, HasFaction<Entity>>, Read<'a, Factions>);
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<Self::Result, Error> {
        let faction = (system_data.1).0.get(&self.faction);
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
/// The type is generic because we use `FactionEnemies<Entity>` as a component and `FactionEnemies<String>` for the prefab.
#[derive(Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FactionEnemies<T> {
    pub enemies: Vec<T>,
}

impl Component for FactionEnemies<Entity> {
    type Storage = HashMapStorage<Self>;
}

impl<T> FactionEnemies<T> {
    pub fn is_enemy(&self, other: &T) -> bool
    where
        T: PartialEq,
    {
        self.enemies.contains(other)
    }
}

impl<'a> PrefabData<'a> for FactionEnemies<String> {
    type SystemData = (
        Write<'a, Factions>,
        WriteStorage<'a, FactionEnemies<Entity>>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<Self::Result, Error> {
        let ref factions = (system_data.0).0;
        let enemies: Vec<Entity> = self
            .enemies
            .iter()
            .map(|enemy| {
                let faction = factions.get(enemy);
                if faction.is_none() {
                    error!("Failed to load faction {:?}", enemy);
                }
                faction
            })
            .filter(|faction| faction.is_some())
            .map(|faction| *faction.unwrap())
            .collect();
        system_data
            .1
            .insert(entity, FactionEnemies { enemies })
            .expect("unreachable: we are inserting");
        Ok(())
    }
}

// Prefab data for the factions. The prefab will populate the faction lookup table.
impl<'a> PrefabData<'a> for FactionPrefabData {
    type SystemData = (
        <Named as PrefabData<'a>>::SystemData,
        <FactionEnemies<String> as PrefabData<'a>>::SystemData,
        // We can't access Factions here, because Factions is already in use by `FactionEnemies<String>::SystemData`.
        // As a workaround we use `Write` in `FactionEnemies<String>::SystemData.0` instead of `Read`
        // Write<'a, Factions>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<Self::Result, Error> {
        let (ref mut named, ref mut factions_enemies) = system_data;

        // Update our faction lookup table
        if let Some(ref name) = self.name {
            (factions_enemies.0).0.insert(name.name.to_string(), entity);
        }
        self.name
            .add_to_entity(entity, named, entities)
            .expect("unreachable");
        self.faction_enemies
            .add_to_entity(entity, factions_enemies, entities)
            .expect("unreachable");
        Ok(())
    }
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct FactionPrefabData {
    name: Option<Named>,
    faction_enemies: Option<FactionEnemies<String>>,
}

#[derive(Default)]
pub struct Factions(HashMap<String, Entity>);

// The factions are stored inside the Ron file in a sorted way. They can only define
// factions as enemies that are on top of their definition. For example, 'Plants' cannot define 'Herbivores' as their enemies
// because 'Herbivores' is defined after 'Plants'.
pub fn load_factions(world: &mut World) {
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, FactionPrefabData>| {
        loader.load("prefabs/factions.ron", RonFormat, (), ())
    });

    world.create_entity().with(prefab_handle.clone()).build();
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
