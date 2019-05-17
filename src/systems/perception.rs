use amethyst::{
    core::{nalgebra::Vector4, transform::GlobalTransform},
    ecs::{Entities, Join, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage},
    renderer::DebugLines,
};

use crate::components::perception::{DetectedEntities, Perception};
use crate::resources::spatial_grid::SpatialGrid;

pub struct EntityDetectionSystem;

impl<'s> System<'s> for EntityDetectionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Perception>,
        WriteStorage<'s, DetectedEntities>,
        ReadExpect<'s, SpatialGrid>,
        ReadStorage<'s, GlobalTransform>,
    );

    fn run(
        &mut self,
        (entities, perceptions, mut detected_entities, grid, globals): Self::SystemData,
    ) {
        for (entity, _) in (&entities, &perceptions).join() {
            match detected_entities.get(entity) {
                Some(_) => (),
                None => {
                    detected_entities
                        .insert(entity, DetectedEntities::default())
                        .expect("Unreachable, we just tested the entity exists");
                }
            }
        }

        for (entity, perception, mut detected, global) in
            (&entities, &perceptions, &mut detected_entities, &globals).join()
        {
            detected.entities = Vec::new();
            let nearby_entities = grid.query(global, perception.range);
            let pos = Vector4::from(global.as_ref()[3]).xyz();
            let sq_range = perception.range * perception.range;

            for other_entity in nearby_entities {
                if entity == other_entity {
                    continue;
                }
                let other_global = globals.get(other_entity).unwrap();
                let other_pos = Vector4::from(other_global.as_ref()[3]).xyz();
                if (pos - other_pos).norm_squared() < sq_range {
                    detected.entities.push(other_entity);
                }
            }
        }
    }
}

pub struct SpatialGridSystem;

impl<'s> System<'s> for SpatialGridSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, GlobalTransform>,
        WriteExpect<'s, SpatialGrid>,
    );

    fn run(&mut self, (entities, globals, mut spatial_grid): Self::SystemData) {
        spatial_grid.reset();
        for (entity, global) in (&entities, &globals).join() {
            spatial_grid.insert(entity, global);
        }
    }
}

pub struct DebugEntityDetectionSystem;

impl<'s> System<'s> for DebugEntityDetectionSystem {
    type SystemData = (
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, GlobalTransform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (detected_entities, globals, mut debug_lines): Self::SystemData) {
        for (detected, global) in (&detected_entities, &globals).join() {
            let pos = Vector4::from(global.as_ref()[3]).xyz();
            for other_entity in &detected.entities {
                let other_global = globals.get(*other_entity).unwrap();
                let other_pos = Vector4::from(other_global.as_ref()[3]).xyz();
                debug_lines.draw_line(
                    [pos[0], pos[1], 0.0].into(),
                    [other_pos[0], other_pos[1], 0.0].into(),
                    [1.0, 1.0, 0.0, 1.0].into(),
                );
            }
        }
    }
}
