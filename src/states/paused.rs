use amethyst::{input::InputEvent, prelude::*};

pub struct PausedState {}

impl Default for PausedState {
    fn default() -> Self {
        PausedState {}
    }
}

impl SimpleState for PausedState {
    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(_) => (), // Events related to the window and inputs.
            StateEvent::Ui(_) => (),     // Ui event. Button presses, mouse hover, etc...
            StateEvent::Input(input_event) => match input_event {
                InputEvent::ActionPressed(action_name) => match action_name.as_ref() {
                    "TogglePause" => return Trans::Pop,
                    _ => (),
                },
                _ => (),
            },
        };
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        Trans::None
    }
}
