use amethyst::{
    assets::{AssetStorage, Loader},
    core::{timing::Time, transform::Transform},
    ecs::*,
    shrev::{EventChannel, ReaderId},
    ui::*,
};

use crate::{components::creatures::CreatureType, resources::prefabs::CreaturePrefabs};

use rand::{thread_rng, Rng, RngCore};

#[derive(Debug, Clone)]
pub struct CreatureSpawnEvent {
    pub creature_type: CreatureType,
    pub position: (f32, f32),
}

#[derive(Default)]
pub struct CreatureSpawnerSystem {
    spawn_reader_id: Option<ReaderId<CreatureSpawnEvent>>,
}

impl<'s> System<'s> for CreatureSpawnerSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, EventChannel<CreatureSpawnEvent>>,
        Read<'s, CreaturePrefabs>,
        Write<'s, LazyUpdate>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.spawn_reader_id = Some(
            res.fetch_mut::<EventChannel<CreatureSpawnEvent>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (entities, spawn_events, prefabs, mut lazy_update): Self::SystemData) {
        for event in spawn_events.read(self.spawn_reader_id.as_mut().unwrap()) {
            let mut transform = Transform::default();
            transform.set_xyz(event.position.0, event.position.1, 0.02);
            transform.set_scale(0.5, 0.5, 1.0);
            let creature_prefab = prefabs.get_prefab(&event.creature_type);
            match creature_prefab {
                Some(prefab) => {
                    lazy_update
                        .create_entity(&entities)
                        .with(prefab.clone())
                        .with(transform)
                        .build();
                }
                None => (),
            }
        }
    }
}
