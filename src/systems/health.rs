use amethyst::{core::transform::Transform, ecs::*, renderer::debug_drawing::DebugLines};
use std::f32;

use crate::components::combat::Health;

pub struct DeathByHealthSystem;

// Entities die if their health reaches zero (or less).
impl<'s> System<'s> for DeathByHealthSystem {
    type SystemData = (ReadStorage<'s, Health>, Entities<'s>);

    fn run(&mut self, (healths, entities): Self::SystemData) {
        for (health, entity) in (&healths, &*entities).join() {
            if health.value < f32::EPSILON {
                let _ = entities.delete(entity);
            }
        }
    }
}

#[derive(Default)]
pub struct DebugHealthSystem {}

impl<'s> System<'s> for DebugHealthSystem {
    type SystemData = (
        ReadStorage<'s, Health>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (healths, transforms, mut debug_lines): Self::SystemData) {
        for (health, transform) in (&healths, &transforms).join() {
            let pos: [f32; 4] = transform.global_matrix()[3];
            debug_lines.draw_line(
                [pos[0], pos[1] + 0.5, 0.0].into(),
                [pos[0] + health.value / 100.0, pos[1] + 0.5, 0.0].into(),
                [0.0, 1.0, 0.0, 1.0].into(),
            )
        }
    }
}
