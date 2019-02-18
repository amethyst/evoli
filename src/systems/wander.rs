use amethyst::core::transform::Transform;
use amethyst::ecs::*;
use amethyst::renderer::*;

use crate::components::creatures;

pub struct WanderSystem;
impl<'s> System<'s> for WanderSystem {
    type SystemData = (
        WriteStorage<'s, creatures::Wander>,
        WriteStorage<'s, creatures::Movement>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, creatures::WanderBehaviorTag>,
        Write<'s, DebugLines>,
    );

    fn run(
        &mut self,
        (mut wanders, mut movements, locals, tag, mut debug_lines): Self::SystemData,
    ) {
        for (wander, movement, local, _) in (&mut wanders, &mut movements, &locals, &tag).join() {
            let position = local.translation();
            let future_position = position + movement.velocity * 0.5;

            let direction = wander.get_direction();
            let target = future_position + direction;
            wander.shake_angle();

            let desired_velocity = target - position;

            movement.velocity += desired_velocity * 0.3;

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

            wander.shake_angle();
        }
    }
}
