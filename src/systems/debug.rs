use amethyst::{
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::debug_drawing::DebugLinesComponent,
};

use crate::components::creatures::CreatureTag;

pub struct DebugSystem;
impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, DebugLinesComponent>,
        ReadStorage<'s, CreatureTag>,
    );

    fn run(&mut self, (entities, mut debug_lines_comps, tags): Self::SystemData) {
        for (entity, _) in (&entities, &tags).join() {
            match debug_lines_comps.get(entity) {
                Some(_) => (),
                None => {
                    debug_lines_comps
                        .insert(entity, DebugLinesComponent::new())
                        .expect("Unreachable");
                }
            }
        }
    }
}
