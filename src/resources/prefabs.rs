use std::collections::HashMap;
use std::fs::read_dir;

use crate::components::creatures::CreaturePrefabData;
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::World,
    ui::{UiLoader, UiPrefab},
    utils::application_root_dir,
};

#[derive(Default)]
pub struct UiPrefabRegistry {
    pub prefabs: Vec<Handle<UiPrefab>>,
}

impl UiPrefabRegistry {
    pub fn find(&self, world: &World, name: &str) -> Option<Handle<UiPrefab>> {
        let storage = world.read_resource::<AssetStorage<UiPrefab>>();
        self.prefabs.iter().find_map(|handle| {
            if storage
                .get(handle)?
                .entities()
                .next()?
                .data()?
                .0 // transform is 0th element of UiPrefab tuple
                .as_ref()?
                .id
                == name
            {
                Some(handle.clone())
            } else {
                None
            }
        })
    }
}

#[derive(Default)]
pub struct CreaturePrefabs {
    prefabs: HashMap<String, Handle<Prefab<CreaturePrefabData>>>,
}

impl CreaturePrefabs {
    pub fn insert(
        &mut self,
        creature_type: String,
        prefab_handle: Handle<Prefab<CreaturePrefabData>>,
    ) {
        self.prefabs.insert(creature_type, prefab_handle);
    }

    pub fn get_prefab(&self, creature_type: &str) -> Option<&Handle<Prefab<CreaturePrefabData>>> {
        self.prefabs.get(creature_type)
    }

    pub fn get_prefabs(&self) -> &HashMap<String, Handle<Prefab<CreaturePrefabData>>> {
        &self.prefabs
    }

    pub fn set_prefabs(&mut self, prefabs: HashMap<String, Handle<Prefab<CreaturePrefabData>>>) {
        self.prefabs = prefabs;
    }
}

fn make_name(subdirectory: &str, entry: &std::fs::DirEntry) -> String {
    let path_buffer = entry.path();
    let filename = path_buffer.file_name().unwrap();
    format!("{}{}", subdirectory, filename.to_str().unwrap())
}

// Here we load all prefabs for the different creatures in the game.
// These prefabs are then stored in a resource of type CreaturePrefabs that is used by the spawner system.
// At initialization time, we put temporary keys for the prefabs since they're not loaded yet.
// When their loading is finished, we read the name of the entity inside to change the keys. This is done in the update_prefabs function.
pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();
    // load ui prefabs
    {
        let mut ui_prefab_registry = UiPrefabRegistry::default();
        let prefab_dir_path = application_root_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            + "/resources/prefabs/ui";
        let prefab_iter = read_dir(prefab_dir_path).unwrap();
        ui_prefab_registry.prefabs = prefab_iter
            .map(|prefab_dir_entry| {
                world.exec(|loader: UiLoader<'_>| {
                    loader.load(
                        make_name("prefabs/ui/", &prefab_dir_entry.unwrap()),
                        &mut progress_counter,
                    )
                })
            })
            .collect::<Vec<Handle<UiPrefab>>>();
        world.add_resource(ui_prefab_registry);
    }

    // load creature prefabs
    {
        let prefab_iter = {
            let prefab_dir_path = application_root_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                + "/resources/prefabs/creatures";
            let prefab_iter = read_dir(prefab_dir_path).unwrap();
            prefab_iter.map(|prefab_dir_entry| {
                world.exec(|loader: PrefabLoader<'_, CreaturePrefabData>| {
                    loader.load(
                        make_name("prefabs/creatures/", &prefab_dir_entry.unwrap()),
                        RonFormat,
                        &mut progress_counter,
                    )
                })
            })
        };

        let mut creature_prefabs = CreaturePrefabs::default();
        for (count, prefab) in prefab_iter.enumerate() {
            creature_prefabs.insert("temp_prefab_".to_string() + &count.to_string(), prefab);
        }
        world.add_resource(creature_prefabs);
    }

    progress_counter
}

// Once the prefabs are loaded, this function is called to update the ekeys in the CreaturePrefabs struct.
// We use the Named component of the entity to determine which key to use.
pub fn update_prefabs(world: &mut World) {
    let updated_prefabs = {
        let creature_prefabs = world.read_resource::<CreaturePrefabs>();
        let prefabs = creature_prefabs.get_prefabs();
        let mut prefab_resource =
            world.write_resource::<AssetStorage<Prefab<CreaturePrefabData>>>();
        let mut new_prefabs = HashMap::new();
        for (_key, handle) in prefabs.iter() {
            if let Some(prefab) = prefab_resource.get_mut(handle) {
                if let Some(prefab_data) = prefab.entity(0) {
                    let name = prefab_data
                        .data()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        .name
                        .to_string();
                    new_prefabs.insert(name, handle.clone());
                }
            }
        }
        new_prefabs
    };
    world
        .write_resource::<CreaturePrefabs>()
        .set_prefabs(updated_prefabs);
}
