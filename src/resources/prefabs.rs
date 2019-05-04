use std::collections::HashMap;

use amethyst::assets::{Handle, Prefab};

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
