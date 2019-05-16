use amethyst::{
    core::{nalgebra::Vector4, transform::GlobalTransform},
    ecs::{
        Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage,
    },
    renderer::DebugLines,
};

use crate::components::perception::Perception;
use crate::resources::{debug::DebugConfig, spatial_grid::SpatialGrid};

pub struct EntityDetectionSystem;

impl<'s> System<'s> for EntityDetectionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Perception>,
        ReadExpect<'s, SpatialGrid>,
        ReadStorage<'s, GlobalTransform>,
        Write<'s, DebugLines>,
        Read<'s, DebugConfig>,
    );

    fn run(
        &mut self,
        (entities, mut perceptions, grid, globals, mut debug_lines, debug_config): Self::SystemData,
    ) {
        for (entity, mut perception, global) in (&entities, &mut perceptions, &globals).join() {
            perception.entities = Vec::new();
            let nearby_entities = grid.query(global, perception.range);
            println!("{}", perception.entities.len());
            let pos = Vector4::from(global.as_ref()[3]).xyz();
            let sq_range = perception.range * perception.range;

            for other_entity in nearby_entities {
                if entity == other_entity {
                    continue;
                }
                let other_global = globals.get(other_entity).unwrap();
                let other_pos = Vector4::from(other_global.as_ref()[3]).xyz();
                if (pos - other_pos).norm_squared() < sq_range {
                    perception.entities.push(other_entity);
                    if debug_config.visible {
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
