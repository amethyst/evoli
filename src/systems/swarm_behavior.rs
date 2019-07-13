use amethyst::{
    core::{
        math::Vector3,
        timing::Time,
        transform::components::{Parent, Transform},
    },
    ecs::*,
    shrev::EventChannel,
};

use rand::{thread_rng, Rng};
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
            let mut rng = thread_rng();
            self.swarm_timer = 10.0f32;
            let mut swarm_entity_builder = lazy_update.create_entity(&entities);
            let x = rng.gen_range(-10.0, 10.0);
            let y = rng.gen_range(-10.0, 10.0);
            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 2.0);
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
            let nb_swarm_individuals = rng.gen_range(3, 10);
            for _ in 0..nb_swarm_individuals {
                let mut swarmling_entity_builder = lazy_update.create_entity(&entities);
                let swarm_behavior = SwarmBehavior {
                    swarm_center: Some(swarm_entity),
                    attraction: 0.5f32,
                    deviation: 0.5f32,
                };
                swarmling_entity_builder = swarmling_entity_builder.with(swarm_behavior);
                let mut transform = Transform::default();
                let x = rng.gen_range(-1.0, 1.0);
                let y = rng.gen_range(-1.0, 1.0);
                transform.set_translation_xyz(x, y, 0.0);
                transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
                let parent = Parent {
                    entity: swarm_entity,
                };
                let movement = Movement {
                    velocity: Vector3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0),
                    max_movement_speed: 5.0,
                };
                swarmling_entity_builder = swarmling_entity_builder
                    .with(transform)
                    .with(parent)
                    .with(movement);
                let swarmling_entity = swarmling_entity_builder.build();
                swarm_center.entities.push(swarmling_entity);
                spawn_events.single_write(CreatureSpawnEvent {
                    creature_type: "Ixie".to_string(),
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

    fn run(&mut self, (entities, _time, mut swarm_centers, swarm_behaviors): Self::SystemData) {
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
        ReadStorage<'s, SwarmBehavior>,
        WriteStorage<'s, Movement>,
    );

    fn run(
        &mut self,
        (_entities, time, transforms, _swarm_centers, swarm_behaviors, mut movements): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();

        // avoid divide-by-zero when delta_seconds is zero
        if delta_seconds <= f32::EPSILON {
            return;
        }
        let time_step = 0.01;
        let iterations = (delta_seconds / time_step) as u32 + 1;
        for (transform, swarm_behavior, mut movement) in
            (&transforms, &swarm_behaviors, &mut movements).join()
        {
            let original_position = transform.translation();
            let mut current_position = original_position.clone();
            let mut current_velocity = movement.velocity.clone();
            let pull_factor = 10.0;
            let side_factor = 5.0;
            for t in 0..iterations {
                let iter_step = time_step.min(delta_seconds - time_step * t as f32);
                let center_pull = if current_position.norm_squared() > 0.16 {
                    swarm_behavior.attraction * pull_factor * (-current_position)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                };
                let mut side_direction =
                    Vector3::new(current_velocity[1], -current_velocity[0], 0.0);
                if !(side_direction.norm_squared() < f32::EPSILON) {
                    side_direction = side_direction.normalize();
                }
                let side_deviation_force = swarm_behavior.deviation * side_factor * side_direction;
                let delta_velocity = iter_step * (center_pull + side_deviation_force);
                current_velocity = current_velocity + delta_velocity;
                let speed = current_velocity.norm();
                if speed > movement.max_movement_speed {
                    current_velocity *= movement.max_movement_speed / speed;
                }
                current_position = current_position + iter_step * current_velocity;
            }
            movement.velocity = (current_position - original_position) / delta_seconds;
        }
    }
}
