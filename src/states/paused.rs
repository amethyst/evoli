use amethyst::{
    input::InputEvent,
    prelude::*,
    shrev::{EventChannel, ReaderId},
};

pub struct PausedState {
    input_event_reader_id: Option<ReaderId<InputEvent<String>>>,
}

impl Default for PausedState {
    fn default() -> Self {
        PausedState {
            input_event_reader_id: None,
        }
    }
}

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        {
            let mut input_event_channel = data
                .world
                .write_resource::<EventChannel<InputEvent<String>>>();
            self.input_event_reader_id = Some(input_event_channel.register_reader());
        }
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        {
            // We re-register the ReaderId when switching back to the state to avoid reading events
            // that happened when the state was inactive.
            let mut input_event_channel = data
                .world
                .write_resource::<EventChannel<InputEvent<String>>>();
            self.input_event_reader_id = Some(input_event_channel.register_reader());
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let input_event_channel = data
            .world
            .read_resource::<EventChannel<InputEvent<String>>>();
        for event in input_event_channel.read(self.input_event_reader_id.as_mut().unwrap()) {
            match event {
                InputEvent::ActionPressed(action_name) => match action_name.as_ref() {
                    "Pause" => return Trans::Pop,
                    _ => (),
                },
                _ => (),
            }
        }
        Trans::None
    }
}
