use amethyst::{
    core::transform::{ParentHierarchy, Transform},
    ecs::{BitSet, Entities, Join, prelude::ComponentEvent,ReadExpect, ReadStorage, System, WriteStorage},
    shrev::{ReaderId},
};

use crate::components::global_transform::GlobalTransform;

#[derive(Default)]
pub struct GlobalTransformSystem {
    reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for GlobalTransformSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, ParentHierarchy>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, GlobalTransform>,
    );

    fn run(&mut self, (entities, hierarchy, mut transforms, mut global_transforms): Self::SystemData) {
        match self.reader_id {
            Some(_) => (),
            None => self.reader_id = Some(transforms.channel().register_reader()),
        };
        let mut modified = BitSet::new();
        let mut inserted = BitSet::new();
        {
            let events = transforms.channel()
                .read(&mut self.reader_id.as_mut().unwrap());
            for event in events {
                match event {
                    ComponentEvent::Modified(id) => { modified.add(*id); },
                    ComponentEvent::Inserted(id) => { inserted.add(*id); },
                    _ => { },
                };
            }
        }

        for (entity, transform, _) in (&entities, &transforms, &inserted).join() {
            let mut current_transform = transform.clone();
            let mut current_entity = entity;
            while let Some(parent_entity) = hierarchy.parent(current_entity) {
                current_transform = match transforms.get(parent_entity) {
                    Some(mut t) => t.clone().concat(&current_transform).clone(),
                    None => current_transform,
                };
                current_entity = parent_entity;
            }
            let global_transform = GlobalTransform {
                transform: current_transform
            };
            global_transforms.insert(entity, global_transform);
        }

        for (entity, transform, _) in (&entities, &transforms, &modified).join() {

        }






    }
}
