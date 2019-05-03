use amethyst::{
    input::InputEvent,
    prelude::*,
};

use crate::states::{
    CustomStateEvent,
};

pub struct PausedState {
}

impl Default for PausedState {
    fn default() -> Self {
        PausedState {
        }
    }
}

impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for PausedState {
    fn handle_event(&mut self, _data: StateData<GameData<'a, 'a>>, event: CustomStateEvent) -> Trans<GameData<'a, 'a>, CustomStateEvent> {
        match event {
            CustomStateEvent::Window(_) => (), // Events related to the window and inputs.
            CustomStateEvent::Ui(_) => (), // Ui event. Button presses, mouse hover, etc...
            CustomStateEvent::Input(input_event) => {
                match input_event {
                    InputEvent::ActionPressed(action_name) => {
                        match action_name.as_ref() {
                            "TogglePause" => return Trans::Pop,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            },
        };
        Trans::None
    }

    fn update(&mut self, data: StateData<'_, GameData<'a, 'a>>) -> Trans<GameData<'a, 'a>, CustomStateEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}
