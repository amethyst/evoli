use amethyst::core::transform::Transform;
use amethyst::core::Time;
use amethyst::ecs::*;
use amethyst::renderer::*;

use crate::components::creatures;

pub struct WanderSystem;
impl<'s> System<'s> for WanderSystem {
    type SystemData = (
        WriteStorage<'s, creatures::Wander>,
        WriteStorage<'s, creatures::Movement>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut wanders, mut movements, locals, mut debug_lines, time): Self::SystemData,
    ) {
        for (wander, movement, local) in (&mut wanders, &mut movements, &locals).join() {
            let position = local.translation();
            let future_position = position + movement.velocity * 0.5;

            let direction = wander.get_direction();
            let target = future_position + direction;

            let desired_velocity = target - position;
            let turn_rate = 10.0;

            movement.velocity += desired_velocity * turn_rate * time.fixed_seconds();

            let change = 10.0;
            if rand::random() {
                wander.angle += change * time.fixed_seconds(); // Radians per second
            } else {
                wander.angle -= change * time.fixed_seconds();
            }

            debug_lines.draw_line(
                [position.x, position.y, position.z].into(),
                [future_position.x, future_position.y, future_position.z].into(),
                [1.0, 0.05, 0.65, 1.0].into(),
            );

            debug_lines.draw_direction(
                [future_position.x, future_position.y, future_position.z].into(),
                [direction.x, direction.y, direction.z].into(),
                [1.0, 0.05, 0.65, 1.0].into(),
            );
        }
    }
}
