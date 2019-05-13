use amethyst::{
    core::transform::Transform,
    ecs::{Entities, ReadStorage, System, WriteStorage},
};

use crate::components::perception::{DetectedEntities, Perception};

pub struct EntityDetectionSystem;

impl<'s> System<'s> for EntityDetectionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Perception>,
        WriteStorage<'s, DetectedEntities>,
    );

    fn run(&mut self, (entities, perceptions, mut detected_entities): Self::SystemData) {}
}
