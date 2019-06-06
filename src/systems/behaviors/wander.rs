use amethyst::core::{
    transform::Transform,
    math::Point3,
    Time,
    Float
};
use amethyst::ecs::*;
use amethyst::renderer::{palette::{
    Alpha,
    Srgba,
},debug_drawing::DebugLines, *};

use crate::{
    components::creatures,
    utils::vector3_to_f32,
};
use rand::{thread_rng, Rng};

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
        let delta_time = time.delta_seconds();
        let mut rng = thread_rng();

        for (wander, movement, local) in (&mut wanders, &mut movements, &locals).join() {
            let position = vector3_to_f32(&local.translation());
            let future_position = position + movement.velocity * 0.5;

            let direction = wander.get_direction();
            let target = future_position + direction;

            let desired_velocity = target - position;

            movement.velocity += desired_velocity * delta_time;

            let change = 10.0;
            if rng.gen::<bool>() {
                wander.angle += change * delta_time; // Radians per second
            } else {
                wander.angle -= change * delta_time;
            }

            debug_lines.draw_line(
                Point3::from(position),
                Point3::from(future_position),
                Srgba::new(1.0, 0.05, 0.65, 1.0),
            );

            debug_lines.draw_direction(
                Point3::from(future_position),
                direction,
                Srgba::new(1.0, 0.05, 0.65, 1.0),
            );
        }
    }
}
