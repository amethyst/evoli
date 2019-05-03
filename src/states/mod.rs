pub mod main_game;
pub mod paused;

use amethyst::derive;

use amethyst::{
    core::EventReader,
    ui::UiEvent,
    input::InputEvent,
    renderer::Event,
    ecs::{Read, Resources, SystemData},
    shrev::{EventChannel, ReaderId},
};

#[derive(Clone, EventReader)]
#[reader(CustomStateEventReader)]
pub enum CustomStateEvent {
    Window(Event),
    Ui(UiEvent),
    Input(InputEvent<String>),
}

//#[derive(Default)]
//pub struct CustomStateEventReader {
//    window_reader_id: Option<ReaderId<WindowEvent>>,
//    ui_reader_id: Option<ReaderId<UiEvent>>,
//    input_reader_id: Option<ReaderId<InputEvent<String>>>,
//}
//
//impl<'r> EventReader<'r> for CustomStateEventReader {
//    type SystemData = (
//    Read<'r, EventChannel<WindowEvent>>,
//    Read<'r, EventChannel<UiEvent>>,
//    Read<'r, EventChannel<InputEvent<String>>>,
//    );
//
//    type Event = CustomStateEvent;
//
//    fn read(&mut self, (window_events, ui_events, input_events): Self::SystemData, events: &mut Vec<Self::Event>) {
//        println!("Reading events");
//        for event in window_events.read(self.window_reader_id.as_mut().unwrap()) {
//            events.push(CustomStateEvent::Window(event.clone()));
//            println!("Add window event.");
//        }
//        for event in ui_events.read(self.ui_reader_id.as_mut().unwrap()) {
//            events.push(CustomStateEvent::Ui(event.clone()));
//            println!("Add ui event.");
//        }
//        for event in input_events.read(self.input_reader_id.as_mut().unwrap()) {
//            events.push(CustomStateEvent::Input(event.clone()));
//            println!("Add input event.");
//        }
//    }
//
//    fn setup(&mut self, res: &mut Resources) {
//        Self::SystemData::setup(res);
//        self.window_reader_id = Some(
//            res.fetch_mut::<EventChannel<WindowEvent>>()
//                .register_reader(),
//        );
//        self.ui_reader_id = Some(
//            res.fetch_mut::<EventChannel<UiEvent>>()
//                .register_reader(),
//        );
//        self.input_reader_id = Some(
//            res.fetch_mut::<EventChannel<InputEvent<String>>>()
//                .register_reader(),
//        );
//    }
//}
