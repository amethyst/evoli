use amethyst::{
    core::{
        nalgebra::Vector3,
        timing::Time,
        transform::components::{Parent, Transform},
    },
    ecs::*,
    shrev::EventChannel,
};

use rand::{thread_rng, RngCore};

use crate::{
    components::{
        creatures::{Movement, Wander},
        swarm::{SwarmBehavior, SwarmCenter},
    },
    systems::spawner::CreatureSpawnEvent,
};

#[derive(Default)]
pub struct SwarmSpawnSystem {
    swarm_timer: f32,
}

impl<'s> System<'s> for SwarmSpawnSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Write<'s, EventChannel<CreatureSpawnEvent>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, lazy_update, mut spawn_events, time): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        self.swarm_timer -= delta_seconds;
        if self.swarm_timer <= 0.0 {
            let mut swarm_entity_builder = lazy_update.create_entity(&entities);
            let mut rng = thread_rng();
            self.swarm_timer = 5.0f32;
            let x = (rng.next_u32() % 100) as f32 / 5.0 - 10.0;
            let y = (rng.next_u32() % 100) as f32 / 5.0 - 10.0;
            let mut transform = Transform::default();
            transform.set_xyz(x, y, 0.0);
            swarm_entity_builder = swarm_entity_builder.with(transform);
            let movement = Movement {
                velocity: Vector3::new(0.0, 0.0, 0.0),
                max_movement_speed: 0.8,
            };
            swarm_entity_builder = swarm_entity_builder.with(movement);
            let wander = Wander {
                radius: 1.0,
                angle: 0.0,
            };
            swarm_entity_builder = swarm_entity_builder.with(wander);
            let swarm_entity = swarm_entity_builder.build();

            let mut swarm_center = SwarmCenter::default();
            let nb_swarm_individuals = 2 + (rng.next_u32() % 5);

            for i in 0..nb_swarm_individuals {
                let mut swarmling_entity_builder = lazy_update.create_entity(&entities);
                let swarm_behavior = SwarmBehavior {
                    swarm_center: Some(swarm_entity),
                };
                swarmling_entity_builder = swarmling_entity_builder.with(swarm_behavior);
                let mut transform = Transform::default();
                let x = (rng.next_u32() % 100) as f32 / 100.0 - 0.5;
                let y = (rng.next_u32() % 100) as f32 / 100.0 - 0.5;
                transform.set_xyz(x, y, 0.02);
                transform.set_scale(0.2, 0.2, 1.0);
                let parent = Parent {
                    entity: swarm_entity,
                };
                swarmling_entity_builder = swarmling_entity_builder.with(transform).with(parent);
                let swarmling_entity = swarmling_entity_builder.build();
                swarm_center.entities.push(swarmling_entity);
                spawn_events.single_write(CreatureSpawnEvent {
                    creature_type: "Bee".to_string(),
                    entity: swarmling_entity,
                });
            }
            lazy_update.insert(swarm_entity, swarm_center);
        }
    }
}

#[derive(Default)]
pub struct SwarmBehaviorSystem {}

impl<'s> System<'s> for SwarmBehaviorSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadStorage<'s, SwarmCenter>,
        ReadStorage<'s, SwarmBehavior>,
    );

    fn run(&mut self, (entities, time, swarm_centers, swarm_behaviors): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
    }
}
