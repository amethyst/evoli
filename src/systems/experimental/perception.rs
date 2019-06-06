use amethyst::{
    core::{math::Vector4, transform::Transform},
    ecs::{
        BitSet, Entities, Join, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage,
    },
    renderer::debug_drawing::DebugLines,
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
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, perceptions, mut detected_entities, grid, transforms): Self::SystemData,
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

        for (entity, perception, mut detected, transform) in
            (&entities, &perceptions, &mut detected_entities, &transforms).join()
        {
            detected.entities = Vec::new();
            let nearby_entities = grid.query(transform, perception.range);
            let pos = Vector4::from(transform.global_matrix()[3]).xyz();
            let sq_range = perception.range * perception.range;
            let mut nearby_entities_bitset = BitSet::new();
            for other_entity in &nearby_entities {
                if entity == *other_entity {
                    continue;
                }
                nearby_entities_bitset.add(other_entity.id());
            }
            for (other_entity, other_transform, _) in
                (&entities, &transforms, &nearby_entities_bitset).join()
            {
                let other_pos = Vector4::from(other_transform.global_matrix()[3]).xyz();
                if (pos - other_pos).norm_squared() < sq_range {
                    detected.entities.add(other_entity.id());
                }
            }
        }
    }
}

pub struct SpatialGridSystem;

impl<'s> System<'s> for SpatialGridSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteExpect<'s, SpatialGrid>,
    );

    fn run(&mut self, (entities, transforms, mut spatial_grid): Self::SystemData) {
        spatial_grid.reset();
        for (entity, transform) in (&entities, &transforms).join() {
            spatial_grid.insert(entity, transform);
        }
    }
}

pub struct DebugEntityDetectionSystem;

impl<'s> System<'s> for DebugEntityDetectionSystem {
    type SystemData = (
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (detected_entities, transforms, mut debug_lines): Self::SystemData) {
        for (detected, transform) in (&detected_entities, &transforms).join() {
            let pos = Vector4::from(transform.global_matrix()[3]).xyz();
            for (other_transform, _) in (&transforms, &detected.entities).join() {
                let other_pos = Vector4::from(other_transform.global_matrix()[3]).xyz();
                debug_lines.draw_line(
                    [pos[0], pos[1], 0.0].into(),
                    [other_pos[0], other_pos[1], 0.0].into(),
                    [1.0, 1.0, 0.0, 1.0].into(),
                );
            }
        }
    }
}
