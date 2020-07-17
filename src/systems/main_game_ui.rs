use amethyst::{
    ecs::*,
    input::{InputEvent, StringBindings},
    shrev::{EventChannel, ReaderId},
    ui::*,
};

pub struct ButtonInfo {
    pub name: &'static str,
    pub text: &'static str,
    pub action: &'static str,
}

const DEFAULT_BUTTON: ButtonInfo = ButtonInfo {
    name: "",
    text: "",
    action: "",
};

impl Default for &ButtonInfo {
    fn default() -> Self {
        &DEFAULT_BUTTON
    }
}

// centralize the button strings here (used in both button creation and response logic)
// NOTE name and text will need to be kept in alignment with main_game.ron; action will need to be kept in alignment with input.ron
pub const MENU_BUTTON: ButtonInfo = ButtonInfo {
    name: "menu button",
    text: "Menu",
    action: "Menu",
};
pub const PAUSE_BUTTON: ButtonInfo = ButtonInfo {
    name: "pause button",
    text: "Pause",
    action: "TogglePause",
};
pub const SLOW_DOWN_BUTTON: ButtonInfo = ButtonInfo {
    name: "slow down button",
    text: "Slow Down",
    action: "SlowDown",
};
pub const SPEED_UP_BUTTON: ButtonInfo = ButtonInfo {
    name: "speed up button",
    text: "Speed Up",
    action: "SpeedUp",
};

pub const BUTTON_INFOS: [&ButtonInfo; 4] = [
    &MENU_BUTTON,
    &PAUSE_BUTTON,
    &SLOW_DOWN_BUTTON,
    &SPEED_UP_BUTTON,
];

#[derive(Default)]
struct Button {
    info: &'static ButtonInfo,
    entity: Option<Entity>,
}

#[derive(Default)]
pub struct MainGameUiSystem {
    ui_reader_id: Option<ReaderId<UiEvent>>,
    input_reader_id: Option<ReaderId<InputEvent<StringBindings>>>,
    buttons: Vec<Button>,
    pause_button_text: Option<Entity>,
}

// implementation-specific, hidden way of producing the name of a button's child text widget, given the button name
fn make_ui_text_name(button_name: &str) -> String {
    format!("{}_btn_txt", button_name)
}

impl<'s> MainGameUiSystem {
    fn find_ui_elements(&mut self, finder: &UiFinder) {
        if self.buttons.is_empty() {
            self.buttons = BUTTON_INFOS
                .iter()
                .map(|info| Button {
                    info,
                    entity: finder.find(info.name),
                })
                .collect::<Vec<Button>>();
            self.pause_button_text = finder.find(&make_ui_text_name(PAUSE_BUTTON.name));
        }
    }

    // translate ui button clicks into input actions for registered buttons
    fn translate_click(
        &self,
        clicked: Entity,
        input_events: &mut Write<'s, EventChannel<InputEvent<StringBindings>>>,
    ) {
        if let Some(button) = self
            .buttons
            .iter()
            .find(|button| button.entity == Some(clicked))
        {
            input_events.single_write(InputEvent::ActionPressed(button.info.action.to_string()));
        }
    }

    fn handle_action(&self, action: &str, ui_texts: &mut WriteStorage<'s, UiText>) {
        // only one action handled right now; change to 'match' when we handle more
        if action != PAUSE_BUTTON.action {
            return;
        }

        // toggle text between 'Play' and 'Pause' depending on what the next click will do
        const PAUSE_TEXT: &str = PAUSE_BUTTON.text;
        const PLAY_TEXT: &str = "Play";
        if let Some(text_entity) = self.pause_button_text {
            if let Some(ui_text) = ui_texts.get_mut(text_entity) {
                if ui_text.text == PAUSE_TEXT {
                    ui_text.text = PLAY_TEXT.to_string();
                } else if ui_text.text == PLAY_TEXT {
                    ui_text.text = PAUSE_TEXT.to_string();
                }
            }
        }
    }
}

impl<'s> System<'s> for MainGameUiSystem {
    type SystemData = (
        UiFinder<'s>,
        Read<'s, EventChannel<UiEvent>>,
        WriteStorage<'s, UiText>,
        Write<'s, EventChannel<InputEvent<StringBindings>>>,
    );

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        self.ui_reader_id = Some(world.fetch_mut::<EventChannel<UiEvent>>().register_reader());
        self.input_reader_id = Some(
            world.fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (ui_finder, ui_events, mut ui_texts, mut input_events): Self::SystemData) {
        self.find_ui_elements(&ui_finder);

        ui_events
            .read(self.ui_reader_id.as_mut().unwrap())
            // filter for Clicks; change to 'match' when other UiEventType variants need to be handled
            .filter(|event| event.event_type == UiEventType::Click)
            .for_each(|event| self.translate_click(event.target, &mut input_events));

        input_events
            .read(self.input_reader_id.as_mut().unwrap())
            .for_each(|event| {
                // change from if-let to match when more InputEvent variants need to be handled
                if let InputEvent::ActionPressed(action_name) = event {
                    self.handle_action(action_name, &mut ui_texts);
                }
            });
    }
}
