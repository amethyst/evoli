use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng, RngCore,
};

use crate::{components::creatures::CreatureType, resources::prefabs::CreaturePrefabs};

#[derive(Debug, Clone)]
pub struct CreatureSpawnEvent {
    pub creature_type: CreatureType,
    pub position: (f32, f32),
}

impl Distribution<CreatureType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CreatureType {
        match rng.gen_range(0, 3) {
            0 => CreatureType::Carnivore,
            1 => CreatureType::Herbivore,
            _ => CreatureType::Plant,
        }
    }
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

    fn run(&mut self, (entities, spawn_events, prefabs, lazy_update): Self::SystemData) {
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

// For debugging purposes this system sends spawn events regularly
#[derive(Default)]
pub struct DebugSpawnTriggerSystem {
    timer_to_next_spawn: f32,
}

impl<'s> System<'s> for DebugSpawnTriggerSystem {
    type SystemData = (Write<'s, EventChannel<CreatureSpawnEvent>>, Read<'s, Time>);

    fn run(&mut self, (mut spawn_events, time): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        self.timer_to_next_spawn -= delta_seconds;
        if self.timer_to_next_spawn <= 0.0 {
            self.timer_to_next_spawn = 2.0;
            let mut rng = thread_rng();
            let x = (rng.next_u32() % 100) as f32 / 5.0 - 10.0;
            let y = (rng.next_u32() % 100) as f32 / 5.0 - 10.0;
            let creature_type: CreatureType = rand::random();

            spawn_events.single_write(CreatureSpawnEvent {
                creature_type: creature_type,
                position: (x, y),
            });
        }
    }
}
