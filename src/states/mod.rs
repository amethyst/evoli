pub mod main_game;
pub mod paused;

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
