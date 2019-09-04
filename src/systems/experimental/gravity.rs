use amethyst::{core::timing::Time, ecs::*};

use crate::{components::creatures::FallingTag, components::creatures::Movement};

/// Acceleration due to gravity.
const GRAVITY: f32 = 4.0;

/// Applies the force of gravity on all entities with the FallingTag.
/// Will remove the tag if an entity has reached the ground again.
#[derive(Default)]
pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        WriteStorage<'s, Movement>,
        ReadStorage<'s, FallingTag>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut movements, falling_tags, time): Self::SystemData) {
        for (movement, _) in (&mut movements, &falling_tags).join() {
            //TODO: Add terminal velocity cap on falling speed.
            movement.velocity.z -= GRAVITY * time.delta_seconds();
        }
    }
}
