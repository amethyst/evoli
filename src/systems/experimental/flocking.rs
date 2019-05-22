use amethyst::{
    core::{nalgebra::Vector4, transform::GlobalTransform},
    ecs::{
        BitSet, Entities, Join, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage,
    },
};

use crate::components::{creatures::Movement, perception::DetectedEntities};

pub struct FlockingSystem;

impl<'s> System<'s> for FlockingSystem {
    type SystemData = (
        ReadStorage<'s, DetectedEntities>,
        ReadStorage<'s, GlobalTransform>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (detected_entities, globals, mut movements): Self::SystemData) {}
}
