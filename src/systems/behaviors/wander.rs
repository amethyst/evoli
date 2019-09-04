use amethyst::core::{math::Point3, transform::Transform, Time};
use amethyst::ecs::*;
use amethyst::renderer::{debug_drawing::DebugLinesComponent, palette::Srgba};

use crate::components::creatures;
use rand::{thread_rng, Rng};

pub struct WanderSystem;
impl<'s> System<'s> for WanderSystem {
    type SystemData = (
        WriteStorage<'s, creatures::Wander>,
        WriteStorage<'s, creatures::Movement>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut wanders, mut movements, locals, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        let mut rng = thread_rng();

        for (wander, movement, local) in (&mut wanders, &mut movements, &locals).join() {
            let position = local.translation();
            let future_position = position + movement.velocity * 0.5;

            let direction = wander.get_direction();
            let target = future_position + direction;

            let desired_velocity = target - position;

            movement.velocity += desired_velocity * delta_time;
            // Quick and dirty fix to keep entities from wandering into the ground if they target
            // an entity not on the same z-level as themselves.
            movement.velocity.z = 0.0;

            let change = 10.0;
            if rng.gen::<bool>() {
                wander.angle += change * delta_time; // Radians per second
            } else {
                wander.angle -= change * delta_time;
            }
        }
    }
}

pub struct DebugWanderSystem;
impl<'s> System<'s> for DebugWanderSystem {
    type SystemData = (
        ReadStorage<'s, creatures::Wander>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, creatures::Movement>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (wanders, transforms, movements, mut debug_lines_comps): Self::SystemData) {
        for (wander, transform, movement, db_comp) in
            (&wanders, &transforms, &movements, &mut debug_lines_comps).join()
        {
            let mut position = transform.global_matrix().column(3).xyz();
            position[2] += 0.5;
            let mut future_position = position + movement.velocity * 0.5;
            future_position[2] += 0.5;
            let direction = wander.get_direction();
            db_comp.add_line(
                Point3::from(position),
                Point3::from(future_position),
                Srgba::new(1.0, 0.05, 0.65, 1.0),
            );

            db_comp.add_direction(
                Point3::from(future_position),
                direction,
                Srgba::new(1.0, 0.05, 0.65, 1.0),
            );
        }
    }
}
