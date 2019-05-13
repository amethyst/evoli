use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::perception::Perception;

pub struct EntityDetectionSystem;

impl<'s> System<'s> for EntityDetectionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Perception>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut perceptions, transforms): Self::SystemData) {
        for (entity, mut perception) in (&entities, &mut perceptions).join() {
            println!("{:?}", entity);
        }
    }
}
