use amethyst::{
    core::{
        nalgebra::{Vector3, Vector4},
        transform::GlobalTransform,
        Named, Time,
    },
    ecs::{
        BitSet, Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteExpect,
        WriteStorage,
    },
};

use crate::{
    components::{boids::*, creatures::Movement, perception::DetectedEntities},
    resources::world_bounds::WorldBounds,
};

pub struct FlockingSystem;

impl<'s> System<'s> for FlockingSystem {
    type SystemData = (
        ReadStorage<'s, Named>,
        ReadStorage<'s, FlockingRule>,
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, GlobalTransform>,
        WriteStorage<'s, Movement>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (names, flocking_rules, detected_entities, globals, mut movements, time): Self::SystemData,
    ) {
        let delta_time = time.delta_seconds();
        for (name, rule, detected, global, mut movement) in (
            &names,
            &flocking_rules,
            &detected_entities,
            &globals,
            &mut movements,
        )
            .join()
        {
            let pos = Vector4::from(global.as_ref()[3]).xyz();
            let mut average_position = pos;
            let mut count = 1;
            for (other_name, other_global, _) in (&names, &globals, &detected.entities).join() {
                if other_name.name == name.name {
                    let other_pos = Vector4::from(other_global.as_ref()[3]).xyz();
                    average_position += other_pos;
                    count += 1;
                }
            }
            if count >= 2 {
                average_position /= count as f32;
                let diff_vector = (average_position - pos);
                movement.velocity += delta_time * rule.strength * diff_vector;
            }
        }
    }
}

pub struct MatchVelocitySystem;

impl<'s> System<'s> for MatchVelocitySystem {
    type SystemData = (
        ReadStorage<'s, Named>,
        ReadStorage<'s, MatchVelocityRule>,
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, GlobalTransform>,
        WriteStorage<'s, Movement>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (names, velocity_rules, detected_entities, globals, mut movements, time): Self::SystemData,
    ) {
        let delta_time = time.delta_seconds();
        for (name, rule, detected, global, movement) in (
            &names,
            &velocity_rules,
            &detected_entities,
            &globals,
            &movements,
        )
            .join()
        {
            let pos = Vector4::from(global.as_ref()[3]).xyz();
            let mut average_velocity = Vector3::new(0.0, 0.0, 0.0);
            let mut count = 0;
            for (other_name, other_global, other_movement, _) in
                (&names, &globals, &movements, &detected.entities).join()
            {
                if other_name.name == name.name {
                    average_velocity += other_movement.velocity;
                    count += 1;
                }
            }
        }
    }
}

pub struct MinimumDistanceSystem;

impl<'s> System<'s> for MinimumDistanceSystem {
    type SystemData = (
        ReadStorage<'s, Named>,
        ReadStorage<'s, MinimumDistanceRule>,
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, GlobalTransform>,
        WriteStorage<'s, Movement>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (names, min_distances, detected_entities, globals, mut movements, time): Self::SystemData,
    ) {
        let delta_time = time.delta_seconds();
        for (name, min_distance, detected, global, mut movement) in (
            &names,
            &min_distances,
            &detected_entities,
            &globals,
            &mut movements,
        )
            .join()
        {
            let sq_min_dist = min_distance.minimum * min_distance.minimum;
            let pos = Vector4::from(global.as_ref()[3]).xyz();
            let mut total_diff = Vector3::new(0.0, 0.0, 0.0);
            for (other_name, other_global, _) in (&names, &globals, &detected.entities).join() {
                let other_pos = Vector4::from(other_global.as_ref()[3]).xyz();
                let diff = pos - other_pos;
                let dist = diff.norm_squared();
                if dist < sq_min_dist {
                    total_diff += diff;
                }
            }
            movement.velocity += delta_time * min_distance.strength * total_diff;
        }
    }
}

pub struct AvoidSystem;

impl<'s> System<'s> for AvoidSystem {
    type SystemData = (
        ReadStorage<'s, Named>,
        ReadStorage<'s, AvoidRule>,
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, GlobalTransform>,
        WriteStorage<'s, Movement>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (names, avoid_rules, detected_entities, globals, mut movements, time): Self::SystemData,
    ) {
        let delta_time = time.delta_seconds();
        for (name, rule, detected, global, mut movement) in (
            &names,
            &avoid_rules,
            &detected_entities,
            &globals,
            &mut movements,
        )
            .join()
        {
            let pos = Vector4::from(global.as_ref()[3]).xyz();
            let mut average_position = Vector3::new(0.0, 0.0, 0.0);
            let mut count = 0;
            for (other_name, other_global, _) in (&names, &globals, &detected.entities).join() {
                if rule.names.contains(&(&*other_name.name).to_string()) {
                    let other_pos = Vector4::from(other_global.as_ref()[3]).xyz();
                    average_position += other_pos;
                    count += 1;
                }
            }
            if count >= 1 {
                average_position /= count as f32;
                let diff_vector = (average_position - pos);
                movement.velocity -= delta_time * rule.strength * diff_vector;
            }
        }
    }
}

pub struct WorldBoundsSystem;

impl<'s> System<'s> for WorldBoundsSystem {
    type SystemData = (
        Read<'s, WorldBounds>,
        ReadStorage<'s, WorldBoundsRule>,
        ReadStorage<'s, GlobalTransform>,
        WriteStorage<'s, Movement>,
        Read<'s, Time>,
    );

    fn run(&mut self, (bounds, bounds_rules, globals, mut movements, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (rule, global, mut movement) in (&bounds_rules, &globals, &mut movements).join() {
            let pos = Vector4::from(global.as_ref()[3]).xyz();

            if pos[0] < bounds.left {
                movement.velocity[0] += delta_time * rule.strength;
            }
            if pos[0] > bounds.right {
                movement.velocity[0] -= delta_time * rule.strength;
            }
            if pos[1] < bounds.bottom {
                movement.velocity[1] += delta_time * rule.strength;
            }
            if pos[1] > bounds.top {
                movement.velocity[1] -= delta_time * rule.strength;
            }
        }
    }
}
