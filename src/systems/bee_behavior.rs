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
use std::f32;

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

            for _ in 0..nb_swarm_individuals {
                let mut swarmling_entity_builder = lazy_update.create_entity(&entities);
                let swarm_behavior = SwarmBehavior {
                    swarm_center: Some(swarm_entity),
                    attraction: 0.5f32,
                    deviation: 0.5f32,
                };
                swarmling_entity_builder = swarmling_entity_builder.with(swarm_behavior);
                let mut transform = Transform::default();
                let x = (rng.next_u32() % 100) as f32 / 100.0 - 0.5;
                let y = (rng.next_u32() % 100) as f32 / 100.0 - 0.5;
                transform.set_xyz(x, y, 0.0);
                transform.set_scale(0.3, 0.3, 1.0);
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
pub struct SwarmCenterSystem {}

impl<'s> System<'s> for SwarmCenterSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        WriteStorage<'s, SwarmCenter>,
        ReadStorage<'s, SwarmBehavior>,
    );

    fn run(&mut self, (entities, time, mut swarm_centers, swarm_behaviors): Self::SystemData) {
        let delta_seconds = time.delta_seconds();

        for (entity, mut swarm_center) in (&entities, &mut swarm_centers).join() {
            swarm_center.entities = swarm_center
                .entities
                .iter()
                .filter(|swarmling_entity| !(swarm_behaviors.get(**swarmling_entity).is_none()))
                .cloned()
                .collect();
            if swarm_center.entities.len() == 0 {
                entities
                    .delete(entity)
                    .expect("unreachable, the entity has been used just before");
            }
        }
    }
}

#[derive(Default)]
pub struct SwarmBehaviorSystem {}

impl<'s> System<'s> for SwarmBehaviorSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, SwarmCenter>,
        WriteStorage<'s, SwarmBehavior>,
        WriteStorage<'s, Movement>,
    );

    fn run(
        &mut self,
        (entities, time, transforms, swarm_centers, mut swarm_behaviors, mut movements): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let mut rng = thread_rng();

        for (transform, mut swarm_behavior, mut movement) in
            (&transforms, &mut swarm_behaviors, &mut movements).join()
        {
            let position = transform.translation();

            let center_x = (rng.next_u32() % 100) as f32 / 500.0 - 0.1;
            let center_y = (rng.next_u32() % 100) as f32 / 500.0 - 0.1;
            let attraction_center = Vector3::new(center_x, center_y, 0.0);
            let center_pull = swarm_behavior.attraction * 15.0 * (attraction_center - position);

            let current_velocity = movement.velocity;
            let mut side_direction = Vector3::new(current_velocity[1], -current_velocity[0], 0.0);
            if !(side_direction.norm_squared() < f32::EPSILON) {
                side_direction = side_direction.normalize();
            }
            let side_deviation_force = swarm_behavior.deviation * 8.0 * side_direction;

            let delta_velocity = delta_seconds * (center_pull + side_deviation_force);
            movement.velocity = movement.velocity + delta_velocity;
        }
    }
}
