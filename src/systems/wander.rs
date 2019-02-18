use amethyst::core::transform::Transform;
use amethyst::core::Time;
use amethyst::ecs::*;

use crate::components::creatures;

pub struct WanderSystem;
impl<'s> System<'s> for WanderSystem {
    type SystemData = (
        WriteStorage<'s, creatures::Movement>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, creatures::WanderBehaviorTag>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut movements, locals, tag, time): Self::SystemData) {
        for (movement, local, _) in (&mut movements, &locals, &tag).join() {
            let position = local.translation();
            let gibberish_x = time.absolute_time_seconds() as f32 + position.y;
            let gibberish_y = time.absolute_time_seconds() as f32 + position.x;

            movement.velocity.x += gibberish_x.cos() * time.delta_seconds();
            movement.velocity.y += gibberish_y.sin() * time.delta_seconds();
        }
    }
}
