use amethyst::{core::transform::Transform, core::Time, ecs::*};

use crate::components::creatures;

pub struct MovementSystem;
impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, creatures::Movement>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut movements, mut transforms, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (movement, transform) in (&mut movements, &mut transforms).join() {
            if movement.velocity.magnitude() > movement.max_movement_speed {
                movement.velocity = movement.velocity.normalize() * movement.max_movement_speed;
            }
            let angle = movement.velocity.y.atan2(movement.velocity.x);

            transform.translate_x(movement.velocity.x * delta_time);
            transform.translate_y(movement.velocity.y * delta_time);
            transform.set_rotation_euler(0.0, 0.0, angle);
        }
    }
}
