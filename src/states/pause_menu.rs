use crate::resources::prefabs::UiPrefabRegistry;
use crate::states::menu::MenuState;
use amethyst::{
    ecs::Entity,
    prelude::*,
    shrev::EventChannel,
    ui::{UiEvent, UiEventType, UiFinder},
    TransEvent,
};

#[derive(Default)]
pub struct PauseMenuState {
    // button entities are created in on_start() and destroyed in on_stop()
    // if there is an invalid Entity that could be assigned to these by default, that'd be better than using Option
    resume_button: Option<Entity>,
    exit_to_main_menu_button: Option<Entity>,
    root: Option<Entity>,
}

const PAUSE_MENU_ID: &str = "pause_menu";
const RESUME_BUTTON_ID: &str = "resume";
const EXIT_TO_MAIN_MENU_BUTTON_ID: &str = "exit_to_main_menu";

// load the pause_menu.ron prefab then instantiate it
// if the "resume" button is clicked, goto MainGameState
// if the "exit_to_main_menu" button is clicked, remove the pause and main game states and go to MenuState.
impl<'a> SimpleState for PauseMenuState {
    fn on_start(&mut self, data: StateData<GameData>) {
        // assume UiPrefab loading has happened in a previous state
        // look through the UiPrefabRegistry for the "menu" prefab and instantiate it
        let menu_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, PAUSE_MENU_ID);
        if let Some(menu_prefab) = menu_prefab {
            self.root = Some(data.world.create_entity().with(menu_prefab).build());
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
        self.resume_button = None;
        self.exit_to_main_menu_button = None;
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.resume_button {
                    Trans::Pop
                } else if Some(target) == self.exit_to_main_menu_button {
                    let mut state_transition_event_channel = data
                        .world
                        .write_resource::<EventChannel<TransEvent<GameData, StateEvent>>>();
                    state_transition_event_channel.single_write(Box::new(|| Trans::Pop));
                    state_transition_event_channel
                        .single_write(Box::new(|| Trans::Switch(Box::new(MenuState::default()))));
                    Trans::None
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        // once deferred creation of the root ui entity finishes, look up buttons
        if self.resume_button.is_none() || self.exit_to_main_menu_button.is_none() {
            data.world.exec(|ui_finder: UiFinder<'_>| {
                self.resume_button = ui_finder.find(RESUME_BUTTON_ID);
                self.exit_to_main_menu_button = ui_finder.find(EXIT_TO_MAIN_MENU_BUTTON_ID);
            });
        }
        Trans::None
    }
}
