use amethyst::{
    core::{math::Point3, transform::Transform},
    ecs::{BitSet, Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};

use crate::components::{
    creatures::CreatureTag,
    perception::{DetectedEntities, Perception},
};
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

        for (perception, mut detected, transform) in
            (&perceptions, &mut detected_entities, &transforms).join()
        {
            detected.entities = BitSet::new();
            let nearby_entities = grid.query(transform, perception.range);
            let pos = transform.global_matrix().column(3).xyz();
            let sq_range = perception.range * perception.range;
            for (other_entity, other_transform, _) in
                (&entities, &transforms, &nearby_entities).join()
            {
                let other_pos = other_transform.global_matrix().column(3).xyz();
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
        ReadStorage<'s, CreatureTag>,
        WriteExpect<'s, SpatialGrid>,
    );

    fn run(&mut self, (entities, transforms, tags, mut spatial_grid): Self::SystemData) {
        spatial_grid.reset();
        for (entity, transform, _) in (&entities, &transforms, &tags).join() {
            spatial_grid.insert(entity, transform);
        }
    }
}

pub struct DebugEntityDetectionSystem;

impl<'s> System<'s> for DebugEntityDetectionSystem {
    type SystemData = (
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (detected_entities, transforms, mut debug_lines_comps): Self::SystemData) {
        for (detected, transform, debug_comp) in
            (&detected_entities, &transforms, &mut debug_lines_comps).join()
        {
            let mut pos = transform.global_matrix().column(3).xyz();
            pos[2] += 0.3;
            for (other_transform, _) in (&transforms, &detected.entities).join() {
                let mut other_pos = other_transform.global_matrix().column(3).xyz();
                other_pos[2] += 0.3;
                debug_comp.add_line(
                    Point3::from(pos),
                    Point3::from(other_pos),
                    Srgba::new(1.0, 1.0, 0.0, 1.0),
                );
            }
        }
    }
}
