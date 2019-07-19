use amethyst::{
    core::{math::Point3, transform::Transform},
    ecs::{
        BitSet, Entities, Join, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage,
    },
    renderer::{debug_drawing::DebugLines, palette::Srgba},
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
        let mut count = 0;
        for (_,) in (&entities,).join() {
            count += 1;
        }
        println!("Total entities: {}", count);

        let mut count = 0;
        for (_, _) in (&entities, &transforms).join() {
            count += 1;
        }
        println!("Total entities with transform: {}", count);

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
            println!(
                "{} entities detected",
                (&nearby_entities).into_iter().count()
            );
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
            let pos = transform.global_matrix().column(3).xyz();
            for (other_transform, _) in (&transforms, &detected.entities).join() {
                let other_pos = other_transform.global_matrix().column(3).xyz();
                debug_lines.draw_line(
                    Point3::from(pos),
                    Point3::from(other_pos),
                    Srgba::new(1.0, 1.0, 0.0, 1.0),
                );
            }
        }
    }
}
