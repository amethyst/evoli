use amethyst::{core::Transform, ecs::*};

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use crate::components::creatures;
use crate::resources::world_bounds::*;

pub struct RicochetSystem;

impl<'s> System<'s> for RicochetSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, creatures::RicochetTag>,
        WriteStorage<'s, creatures::Movement>,
        Read<'s, WorldBounds>
        );

    fn run(&mut self, (transforms, ricochets,  mut movements, bounds): Self::SystemData) {
        for (local, ricochet, movement) in (&transforms, &ricochets, &mut movements).join() {
            if local.translation().x >= bounds.right  || local.translation().x <= bounds.left {
                movement.velocity.x = -movement.velocity.x;
            }
            
            if local.translation().y >= bounds.top || local.translation().y <= bounds.bottom {
                movement.velocity.y = -movement.velocity.y;
            }
        }
    }
}

