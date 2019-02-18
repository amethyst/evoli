use amethyst::{core::Transform, ecs::*};

use crate::resources::world_bounds::*;

pub struct EnforceBoundsSystem;

impl<'s> System<'s> for EnforceBoundsSystem {
    type SystemData = (WriteStorage<'s, Transform>, Read<'s, WorldBounds>);

    fn run(&mut self, (mut locals, bounds): Self::SystemData) {
        for local in (&mut locals).join() {
            if local.translation().x > bounds.right {
                local.translation_mut().x = bounds.right;
            } else if local.translation().x < bounds.left {
                local.translation_mut().x = bounds.left;
            }

            if local.translation().y > bounds.top {
                local.translation_mut().y = bounds.top;
            } else if local.translation().y < bounds.bottom {
                local.translation_mut().y = bounds.bottom;
            }
        }
    }
}
