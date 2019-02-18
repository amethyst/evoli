use amethyst::{
    core::transform::Transform,
    core::Time,
    ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::*,
};

use crate::components::creatures;
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, creatures::Movement>,
        WriteStorage<'s, Transform>,
        Write<'s, DebugLines>,
        Read<'s, Time>,
    );

    fn run(&mut self, (movements, mut locals, mut debug_lines_resource, time): Self::SystemData) {
        for (movement, local) in (&movements, &mut locals).join() {
            local.translate_x(movement.velocity.x * time.delta_seconds());
            local.translate_y(movement.velocity.y * time.delta_seconds());

            let position = local.translation();

            debug_lines_resource.draw_direction(
                [position.x, position.y, position.z].into(),
                movement.velocity,
                [1.0, 0.05, 0.65, 1.0].into(),
            );
        }
    }
}
