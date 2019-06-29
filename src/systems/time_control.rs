use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::*,
    input::InputEvent,
    shrev::{EventChannel, ReaderId},
    ui::*,
};

struct ButtonInfo {
    name: &'static str,
    text: &'static str,
    action: &'static str,
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
// if/when the ui creation moves to a .ron, these strings will need to be kept in alignment with that file
const PAUSE_BUTTON: ButtonInfo = ButtonInfo {
    name: "pause button",
    text: "Pause",
    action: "TogglePause",
};
const SLOW_DOWN_BUTTON: ButtonInfo = ButtonInfo {
    name: "slow down button",
    text: "Slow Down",
    action: "SlowDown",
};
const SPEED_UP_BUTTON: ButtonInfo = ButtonInfo {
    name: "speed up button",
    text: "Speed Up",
    action: "SpeedUp",
};

pub fn create_time_control_ui(world: &mut World) {
    world.add_resource(AssetStorage::<FontAsset>::new());
    let font_handle = {
        let loader = world.write_resource::<Loader>();
        let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
        loader.load(
            "assets/fonts/OpenSans-Regular.ttf",
            TtfFormat,
            (),
            (),
            &font_storage,
        )
    };

    // TODO move widget specification to time_control.ron
    let _ = UiButtonBuilder::new(PAUSE_BUTTON.name, PAUSE_BUTTON.text)
        .with_anchor(Anchor::BottomRight)
        .with_size(80.0, 36.0)
        .with_position(-255.0, 20.0)
        .with_font(font_handle.clone())
        .with_font_size(24.0f32)
        .with_text_color([0.0f32, 0.0, 0.0, 1.0])
        .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
        .with_press_text_color([0.5, 0.5, 0.5, 1.0])
        .build_from_world(world);

    let _ = UiButtonBuilder::new(SLOW_DOWN_BUTTON.name, SLOW_DOWN_BUTTON.text)
        .with_anchor(Anchor::BottomRight)
        .with_size(100.0, 36.0)
        .with_position(-160.0, 20.0)
        .with_font(font_handle.clone())
        .with_font_size(24.0f32)
        .with_text_color([0.0f32, 0.0, 0.0, 1.0])
        .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
        .with_press_text_color([0.5, 0.5, 0.5, 1.0])
        .build_from_world(world);

    let _ = UiButtonBuilder::new(SPEED_UP_BUTTON.name, SPEED_UP_BUTTON.text)
        .with_anchor(Anchor::BottomRight)
        .with_size(100.0, 36.0)
        .with_position(-55.0, 20.0)
        .with_font(font_handle.clone())
        .with_font_size(24.0f32)
        .with_text_color([0.0f32, 0.0, 0.0, 1.0])
        .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
        .with_press_text_color([0.5, 0.5, 0.5, 1.0])
        .build_from_world(world);
}

#[derive(Default)]
struct Button {
    info: &'static ButtonInfo,
    entity: Option<Entity>,
}

#[derive(Default)]
pub struct TimeControlSystem {
    ui_reader_id: Option<ReaderId<UiEvent>>,
    input_reader_id: Option<ReaderId<InputEvent<String>>>,
    buttons: Vec<Button>,
    pause_button_text: Option<Entity>,
}

// implementation-specific, hidden way of producing the name of a button's child text widget, given the button name
fn make_ui_text_name(button_name: &str) -> String {
    format!("{}_btn_txt", button_name)
}

impl<'s> TimeControlSystem {
    fn find_ui_elements(&mut self, finder: &UiFinder) {
        if !self.buttons.is_empty() {
            return;
        }
        self.buttons.push(Button {
            info: &PAUSE_BUTTON,
            entity: finder.find(PAUSE_BUTTON.name),
        });
        self.buttons.push(Button {
            info: &SPEED_UP_BUTTON,
            entity: finder.find(SPEED_UP_BUTTON.name),
        });
        self.buttons.push(Button {
            info: &SLOW_DOWN_BUTTON,
            entity: finder.find(SLOW_DOWN_BUTTON.name),
        });
        self.pause_button_text = finder.find(&make_ui_text_name(PAUSE_BUTTON.name));
    }

    // translate ui button clicks into input actions for registered buttons
    fn translate_click(
        &self,
        clicked: Entity,
        input_events: &mut Write<'s, EventChannel<InputEvent<String>>>,
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
        // only one action handled right now; change to match when we handle more
        if action != PAUSE_BUTTON.action {
            return;
        }
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

impl<'s> System<'s> for TimeControlSystem {
    type SystemData = (
        UiFinder<'s>,
        Read<'s, EventChannel<UiEvent>>,
        WriteStorage<'s, UiText>,
        Write<'s, EventChannel<InputEvent<String>>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.ui_reader_id = Some(res.fetch_mut::<EventChannel<UiEvent>>().register_reader());
        self.input_reader_id = Some(
            res.fetch_mut::<EventChannel<InputEvent<String>>>()
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
