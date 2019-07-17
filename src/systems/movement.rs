use amethyst::{core::transform::Transform, core::Time, ecs::*};

use crate::components::creatures::{CreatureTag, Movement};

pub struct MovementSystem;
impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Movement>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CreatureTag>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut movements, mut transforms, creature_tags, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (movement, transform) in (&mut movements, &mut transforms).join() {
            let magnitude = movement.velocity.magnitude();
            if magnitude > movement.max_movement_speed {
                movement.velocity = movement.velocity * (movement.max_movement_speed / magnitude);
            }
            transform.prepend_translation_x(movement.velocity.x * delta_time);
            transform.prepend_translation_y(movement.velocity.y * delta_time);
        }
        for (movement, transform, _) in (&mut movements, &mut transforms, &creature_tags).join() {
            let angle = movement.velocity.y.atan2(movement.velocity.x);
            transform.set_rotation_2d(angle);
        }
    }
}
