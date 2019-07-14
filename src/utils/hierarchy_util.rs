use amethyst::{
    core::transform::ParentHierarchy,
    ecs::{error::WrongGeneration, Entity},
    prelude::*,
};
use std::iter;

// delete the specified root entity and all of its descendents as specified
// by the Parent component and maintained by the ParentHierarchy resource
pub fn delete_hierarchy(root: Entity, world: &mut World) -> Result<(), WrongGeneration> {
    let entities = {
        iter::once(root)
            .chain(
                world
                    .read_resource::<ParentHierarchy>()
                    .all_children_iter(root),
            )
            .collect::<Vec<Entity>>()
    };
    world.delete_entities(&entities)
}
