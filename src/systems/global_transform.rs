use amethyst::{
    core::transform::{ParentHierarchy, Transform},
    ecs::{ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::components::global_transform::GlobalTransform;

pub struct GlobalTransformSystem;

impl<'s> System<'s> for GlobalTransformSystem {
    type SystemData = (
        ReadExpect<'s, ParentHierarchy>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, GlobalTransform>,
    );

    fn run(&mut self, (hierarchy, transforms, mut global_transforms): Self::SystemData) {}
}
