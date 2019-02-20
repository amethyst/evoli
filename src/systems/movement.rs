use amethyst::{core::transform::Transform, core::Time, ecs::*};

use crate::components::creatures;
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, creatures::Movement>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut movements, mut locals, time): Self::SystemData) {
        for (movement, local) in (&mut movements, &mut locals).join() {
            if movement.velocity.magnitude() > movement.max_movement_speed {
                movement.velocity = movement.velocity.normalize() * movement.max_movement_speed;
            }

            local.translate_x(movement.velocity.x * time.fixed_seconds());
            local.translate_y(movement.velocity.y * time.fixed_seconds());
        }
    }
}
