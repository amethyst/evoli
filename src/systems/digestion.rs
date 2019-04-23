use amethyst::{core::Transform, core::Time, ecs::*};
use amethyst::renderer::*;
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
        for (digestion, fullness) in (&digestions, &mut fullnesses).join() {
            let burned = digestion.nutrition_burn_rate * time.fixed_seconds();
            let new_value = fullness.value - burned;
            fullness.value = if burned < f32::EPSILON { 0.0 } else { new_value };
        }
    }
}

pub struct StarvationSystem;

// Entities die if their fullness reaches zero (or less).
impl<'s> System<'s> for StarvationSystem {
    type SystemData = (
        ReadStorage<'s, Fullness>,
        Entities<'s>,
    );

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
        ReadStorage<'s, Fullness>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (fullnesses, locals, mut debug_lines): Self::SystemData) {
        for (fullness, local) in (&fullnesses, &locals).join() {
            let pos = local.translation();
            debug_lines.draw_line(
                [pos.x, pos.y, 0.0].into(),
                [pos.x + fullness.value / 100.0, pos.y, 0.0].into(),
                [0.0, 1.0, 0.0, 1.0].into(),
            )
        }
    }
}
