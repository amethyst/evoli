use crate::resources::prefabs::UiPrefabRegistry;
use crate::states::main_game::MainGameState;
use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
};

#[derive(Default)]
pub struct MenuState {
    // button entities are created in on_start() and destroyed in on_stop()
    // if there is an invalid Entity that could be assigned to these by default, that'd be better than using Option
    start_button: Option<Entity>,
    exit_button: Option<Entity>,
    root: Option<Entity>,

    pause_menu: bool,
}

impl MenuState {
    pub fn new(pause: bool) -> MenuState {
        MenuState {
            pause_menu: pause,
            ..Default::default()
        }
    }
}

const MENU_ID: &str = "menu";
const START_BUTTON_ID: &str = "start";
const EXIT_BUTTON_ID: &str = "exit";

// load the menu.ron prefab then instantiate it
// if the "start" button is clicked, goto MainGameState
// if the "exit" button is clicked, exit app
impl<'a> SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<GameData>) {
        // assume UiPrefab loading has happened in a previous state
        // look through the UiPrefabRegistry for the "menu" prefab and instantiate it
        let menu_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, MENU_ID);
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
        self.start_button = None;
        self.exit_button = None;
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.start_button {
                    if self.pause_menu {
                        Trans::Pop
                    } else {
                        Trans::Switch(Box::new(MainGameState::new(data.world)))
                    }
                } else if Some(target) == self.exit_button {
                    Trans::Quit
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
        if self.start_button.is_none() || self.exit_button.is_none() {
            data.world.exec(|ui_finder: UiFinder<'_>| {
                self.start_button = ui_finder.find(START_BUTTON_ID);
                self.exit_button = ui_finder.find(EXIT_BUTTON_ID);
            });
        }
        Trans::None
    }
}
