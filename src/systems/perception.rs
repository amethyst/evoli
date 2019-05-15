use amethyst::{
    core::nalgebra::Vector4,
    core::transform::GlobalTransform,
    ecs::{Entities, Join, ReadStorage, System, Write, WriteStorage},
    renderer::DebugLines,
};

use crate::components::perception::Perception;

pub struct EntityDetectionSystem;

impl<'s> System<'s> for EntityDetectionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Perception>,
        ReadStorage<'s, GlobalTransform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (entities, mut perceptions, globals, mut debug_lines): Self::SystemData) {
        for (entity, mut perception, global) in (&entities, &mut perceptions, &globals).join() {
            perception.entities = Vec::new();
            let global_matrix: [[f32; 4]; 4] = global.clone().into();
            let pos = Vector4::from(global_matrix[3]).xyz();

            let sq_range = perception.range * perception.range;
            for (other_entity, other_global) in (&entities, &globals).join() {
                if entity == other_entity {
                    continue;
                }
                let other_global_matrix: [[f32; 4]; 4] = other_global.clone().into();
                let other_pos = Vector4::from(other_global_matrix[3]).xyz();

                if (pos - other_pos).norm_squared() < sq_range {
                    perception.entities.push(other_entity);
                    debug_lines.draw_line(
                        [pos[0], pos[1], 0.0].into(),
                        [other_pos[0], other_pos[1], 0.0].into(),
                        [1.0, 1.0, 0.0, 1.0].into(),
                    )
                }
            }
        }
    }
}
