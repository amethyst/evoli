use crate::{resources::prefabs::UiPrefabRegistry, utils::hierarchy_util};
use amethyst::{
    core::transform::components::Parent,
    ecs::{join::Join, Entity},
    input::{Button, InputHandler, StringBindings},
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder, UiText, UiTransform},
};

#[derive(Default)]
pub struct ControlsState {
    // button entities are created in on_start() and destroyed in on_stop()
    // if there is an invalid Entity that could be assigned to these by default, that'd be better than using Option
    done_button: Option<Entity>,
    root: Option<Entity>,
    rows: Vec<Entity>,
}

impl ControlsState {
    fn fill_rows(&mut self, world: &mut World) {
        // add UiText widgets describing the controls as specified in the InputHandler resource
        let input_bindings = {
            let input_handler = world.read_resource::<InputHandler<StringBindings>>();
            let mut bindings = input_handler
                .bindings
                .actions()
                .map(|action| {
                    (
                        action.clone(),
                        format!(
                            "{:?}",
                            input_handler
                                .bindings
                                .action_bindings(action)
                                .collect::<Vec<&[Button]>>()
                        ),
                    )
                })
                .collect::<Vec<(String, String)>>();
            bindings.sort();
            bindings
        };
        let y = -150.; // start below our title label
        let y_step = -40.;
        (
            &world.entities(),
            &mut world.write_storage::<UiTransform>(),
            &mut world.write_storage::<UiText>(),
        )
            .join()
            .for_each(|(entity, transform, label)| {
                self.rows
                    .iter()
                    .zip(&input_bindings)
                    .enumerate()
                    .filter_map(|(index, (&row, binding))| {
                        if row == entity {
                            Some((index, binding))
                        } else {
                            None
                        }
                    })
                    .for_each(|(index, (action, buttons))| {
                        label.text = format!("{} : {}", action, buttons);
                        info!("{}", label.text);
                        transform.local_y = y + y_step * (index as f32);
                    })
            });
    }
}

const CONTROLS_ID: &str = "controls";
const CONTROLS_ROW_ID: &str = "controls_row";
const DONE_BUTTON_ID: &str = "done button";

// load the controls.ron prefab then instantiate it
// if the "done" button is clicked, pop ControlsState
impl<'a> SimpleState for ControlsState {
    fn on_start(&mut self, data: StateData<GameData>) {
        // assume UiPrefab loading has happened in a previous state
        // look through the UiPrefabRegistry for the "controls" prefab and instantiate it
        let controls_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, CONTROLS_ID);
        if let Some(controls_prefab) = controls_prefab {
            self.root = Some(data.world.create_entity().with(controls_prefab).build());

            // find the prefab for our row widget
            // create a row for each binding in the InputHandler and parent them to our ui root
            let row_prefab = data
                .world
                .read_resource::<UiPrefabRegistry>()
                .find(data.world, CONTROLS_ROW_ID);
            if let Some(row_prefab) = row_prefab {
                let binding_count = data
                    .world
                    .read_resource::<InputHandler<StringBindings>>()
                    .bindings
                    .actions()
                    .count();
                let root = self.root.unwrap();
                // TODO is there a more clever way to do this with data.world.create_iter()?
                self.rows = (0..binding_count)
                    .map(|_row_index| {
                        data.world
                            .create_entity()
                            .with(row_prefab.clone())
                            .with(Parent::new(root))
                            .build()
                    })
                    .collect::<Vec<Entity>>();
            }
        }

        // update the world to finish building all of our ui entities
        data.data.update(&data.world);

        // look up the done button to make checking its status efficient
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.done_button = ui_finder.find(DONE_BUTTON_ID);
        });

        // fill the row widgets with data
        self.fill_rows(data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root {
            hierarchy_util::delete_hierarchy(root, data.world)
                .expect("failed to delete all controls overlay widgets");
        };
        self.root = None;
        self.done_button = None;
        self.rows.clear();
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.done_button {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        Trans::None
    }
}
