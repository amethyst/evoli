use amethyst::{
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

use std::f32::consts::PI;

use crate::{
    components::creatures::{CreatureTag, CreatureType},
    resources::prefabs::CreaturePrefabs,
};

#[derive(Debug, Clone)]
pub struct CreatureSpawnEvent {
    pub creature_type: String,
    pub entity: Entity,
}

struct CreatureTypeDistribution {
    creature_type: CreatureType,
}

impl Distribution<CreatureTypeDistribution> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CreatureTypeDistribution {
        match rng.gen_range(0, 3) {
            0 => CreatureTypeDistribution {
                creature_type: "Herbivore".to_string(),
            },
            1 => CreatureTypeDistribution {
                creature_type: "Carnivore".to_string(),
            },
            _ => CreatureTypeDistribution {
                creature_type: "Plant".to_string(),
            },
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

    fn run(&mut self, (_entities, spawn_events, prefabs, lazy_update): Self::SystemData) {
        for event in spawn_events.read(self.spawn_reader_id.as_mut().unwrap()) {
            if let Some(creature_prefab) = prefabs.get_prefab(&event.creature_type) {
                lazy_update.insert(event.entity, creature_prefab.clone());
                lazy_update.insert(event.entity, CreatureTag::default());
            }
        }
    }
}

//
//
// For debugging purposes this system sends spawn events regularly
#[derive(Default)]
pub struct DebugSpawnTriggerSystem {
    timer_to_next_spawn: f32,
}

impl<'s> System<'s> for DebugSpawnTriggerSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Write<'s, EventChannel<CreatureSpawnEvent>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, lazy_update, mut spawn_events, time): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        self.timer_to_next_spawn -= delta_seconds;
        if self.timer_to_next_spawn <= 0.0 {
            let mut creature_entity_builder = lazy_update.create_entity(&entities);
            self.timer_to_next_spawn = 1.5;
            let mut rng = thread_rng();
            let x = rng.gen_range(-5.0f32, 5.0f32);
            let y = rng.gen_range(-5.0f32, 5.0f32);
            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.02);
            let CreatureTypeDistribution { creature_type }: CreatureTypeDistribution =
                rand::random();
            if creature_type == "Carnivore" || creature_type == "Herbivore" {
                transform.set_scale(Vector3::new(0.4, 0.4, 0.4));
            }
            if creature_type == "Plant" {
                let scale = rng.gen_range(0.8f32, 1.2f32);
                let rotation = rng.gen_range(0.0f32, PI);
                transform.set_translation_z(0.01);
                transform.set_scale(Vector3::new(scale, scale, scale));
                transform.set_rotation_euler(0.0, 0.0, rotation);
            }
            creature_entity_builder = creature_entity_builder.with(transform);
            spawn_events.single_write(CreatureSpawnEvent {
                creature_type,
                entity: creature_entity_builder.build(),
            });
        }
    }
}
