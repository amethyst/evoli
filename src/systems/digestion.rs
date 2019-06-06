use amethyst::renderer::{
    palette::Srgba,
    debug_drawing::DebugLines
};
use amethyst::{core::transform::ParentHierarchy, core::Time, core::Transform, ecs::*};
use std::f32;

use crate::components::digestion::{Digestion, Fullness};

pub struct DigestionSystem;

impl<'s> System<'s> for DigestionSystem {
    type SystemData = (
        ReadStorage<'s, Digestion>,
        WriteStorage<'s, Fullness>,
        Read<'s, Time>,
    );

    fn run(&mut self, (digestions, mut fullnesses, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (digestion, fullness) in (&digestions, &mut fullnesses).join() {
            let burned = digestion.nutrition_burn_rate * delta_time;
            let new_value = fullness.value - burned;
            fullness.value = new_value;
        }
    }
}

pub struct StarvationSystem;

// Entities die if their fullness reaches zero (or less).
impl<'s> System<'s> for StarvationSystem {
    type SystemData = (ReadStorage<'s, Fullness>, Entities<'s>);

    fn run(&mut self, (fullnesses, entities): Self::SystemData) {
        for (fullness, entity) in (&fullnesses, &*entities).join() {
            if fullness.value < f32::EPSILON {
                let _ = entities.delete(entity);
            }
        }
    }
}

pub struct DebugFullnessSystem;

impl<'s> System<'s> for DebugFullnessSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Fullness>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, ParentHierarchy>,
        Write<'s, DebugLines>,
    );

    fn run(
        &mut self,
        (entities, fullnesses, locals, hierarchy, mut debug_lines): Self::SystemData,
    ) {
        for (entity, fullness, local) in (&entities, &fullnesses, &locals).join() {
            let pos = local.global_matrix();
            debug_lines.draw_line(
                [pos[(3,0)].as_f32(), pos[(3,1)].as_f32(), 0.0].into(),
                [pos[(3,0)].as_f32() + fullness.value / 100.0, pos[(3,1)].as_f32(), 0.0].into(),
                Srgba::new(0.0, 1.0, 0.0, 1.0),
            )
        }
    }
}
