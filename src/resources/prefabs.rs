use std::collections::HashMap;

use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, RonFormat},
    ecs::World,
};

use crate::components::creatures::{CreaturePrefabData, CreatureType};

#[derive(Default)]
pub struct CreaturePrefabs {
    prefabs: HashMap<CreatureType, Handle<Prefab<CreaturePrefabData>>>,
}

impl CreaturePrefabs {
    pub fn insert(
        &mut self,
        creature_type: CreatureType,
        prefab_handle: Handle<Prefab<CreaturePrefabData>>,
    ) {
        self.prefabs.insert(creature_type, prefab_handle);
    }

    pub fn get_prefab(
        &self,
        creature_type: &CreatureType,
    ) -> Option<&Handle<Prefab<CreaturePrefabData>>> {
        self.prefabs.get(creature_type)
    }
}

// Here we load all prefabs for the different creatures in the game.
// These prefabs are then stored in a resource of type CreaturePrefabs that is used by the spawner system.
pub fn initialize_prefabs(world: &mut World) {
    let mut creature_prefabs = CreaturePrefabs::default();
    let (carnivore_prefab, herbivore_prefab, plant_prefab) =
        world.exec(|loader: PrefabLoader<'_, CreaturePrefabData>| {
            (
                loader.load("prefabs/carnivore.ron", RonFormat, (), ()),
                loader.load("prefabs/herbivore.ron", RonFormat, (), ()),
                loader.load("prefabs/plant.ron", RonFormat, (), ()),
            )
        });
    creature_prefabs.insert("Carnivore".to_string(), carnivore_prefab);
    creature_prefabs.insert("Herbivore".to_string(), herbivore_prefab);
    creature_prefabs.insert("Plant".to_string(), plant_prefab);
    world.add_resource(creature_prefabs);
}
