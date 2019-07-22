use amethyst::{
    core::transform::Transform,
    ecs::*,
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};
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
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (healths, transforms, mut debug_lines_comps): Self::SystemData) {
        for (health, transform, db_comp) in (&healths, &transforms, &mut debug_lines_comps).join() {
            let pos = transform.global_matrix().column(3).xyz();
            db_comp.add_line(
                [pos[0], pos[1] + 0.5, pos[2] + 0.5].into(),
                [pos[0] + health.value / 100.0, pos[1] + 0.5, pos[2] + 0.5].into(),
                Srgba::new(0.0, 1.0, 0.0, 1.0),
            )
        }
    }
}
