use amethyst::{core::transform::Transform, ecs::*, shrev::EventChannel};
use std::f32;

use crate::components::{combat::Health, creatures::Carcass, digestion::Fullness};
use crate::systems::spawner::CreatureSpawnEvent;

#[derive(Debug, Clone)]
pub struct CreatureDeathEvent {
    pub deceased: Entity,
}

pub struct StarvationSystem;

// Entities die if their fullness reaches zero (or less).
impl<'s> System<'s> for StarvationSystem {
    type SystemData = (
        ReadStorage<'s, Fullness>,
        Entities<'s>,
        Write<'s, EventChannel<CreatureDeathEvent>>,
    );

    fn run(&mut self, (fullnesses, entities, mut death_events): Self::SystemData) {
        for (fullness, entity) in (&fullnesses, &*entities).join() {
            if fullness.value < f32::EPSILON {
                death_events.single_write(CreatureDeathEvent { deceased: entity });
                let _ = entities.delete(entity);
            }
        }
    }
}

pub struct DeathByHealthSystem;

// Entities die if their health reaches zero (or less).
impl<'s> System<'s> for DeathByHealthSystem {
    type SystemData = (
        ReadStorage<'s, Health>,
        Entities<'s>,
        Write<'s, EventChannel<CreatureDeathEvent>>,
    );

    fn run(&mut self, (healths, entities, mut death_events): Self::SystemData) {
        for (health, entity) in (&healths, &*entities).join() {
            if health.value < f32::EPSILON {
                death_events.single_write(CreatureDeathEvent { deceased: entity });
                let _ = entities.delete(entity);
            }
        }
    }
}

#[derive(Default)]
pub struct CarcassSystem {
    death_reader_id: Option<ReaderId<CreatureDeathEvent>>,
}

impl<'s> System<'s> for CarcassSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, EventChannel<CreatureDeathEvent>>,
        Write<'s, EventChannel<CreatureSpawnEvent>>,
        Write<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Carcass>,
    );

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.death_reader_id = Some(
            world.fetch_mut::<EventChannel<CreatureDeathEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (entities, death_events, mut spawn_events, lazy_update, transforms, carcasses): Self::SystemData,
    ) {
        for event in death_events.read(self.death_reader_id.as_mut().unwrap()) {
            let mut deceased = BitSet::new();
            deceased.add(event.deceased.id());

            for (_, carcass, transform) in (&deceased, &carcasses, &transforms).join() {
                let creature_entity_builder =
                    lazy_update.create_entity(&entities).with(transform.clone());
                spawn_events.single_write(CreatureSpawnEvent {
                    creature_type: carcass.creature_type.clone(),
                    entity: creature_entity_builder.build(),
                });
            }
        }
    }
}
