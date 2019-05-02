use amethyst::{
    assets::{AssetStorage, Loader},
    core::timing::Time,
    ecs::*,
    input::InputEvent,
    shrev::{EventChannel, ReaderId},
    ui::*,
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

    let _ = UiButtonBuilder::new("pause button", "Pause")
        .with_anchor(Anchor::BottomRight)
        .with_size(80.0, 36.0)
        .with_position(-255.0, 20.0)
        .with_font(font_handle.clone())
        .with_font_size(24.0f32)
        .with_text_color([0.0f32, 0.0, 0.0, 1.0])
        .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
        .with_press_text_color([0.5, 0.5, 0.5, 1.0])
        .build_from_world(world);

    let _ = UiButtonBuilder::new("slow down button", "Slow Down")
        .with_anchor(Anchor::BottomRight)
        .with_size(100.0, 36.0)
        .with_position(-160.0, 20.0)
        .with_font(font_handle.clone())
        .with_font_size(24.0f32)
        .with_text_color([0.0f32, 0.0, 0.0, 1.0])
        .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
        .with_press_text_color([0.5, 0.5, 0.5, 1.0])
        .build_from_world(world);

    let _ = UiButtonBuilder::new("speed up button", "Speed Up")
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
pub struct TimeControlSystem {
    ui_reader_id: Option<ReaderId<UiEvent>>,
    input_reader_id: Option<ReaderId<InputEvent<String>>>,
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

    fn run(&mut self, (finder, ui_events, mut ui_texts, mut input_events): Self::SystemData) {

        for event in ui_events.read(self.ui_reader_id.as_mut().unwrap()) {

            let mut handle_button = |button_name: &str, action_name: &str| {
                if let Some(button_entity) = finder.find(button_name) {
                    if button_entity == event.target {
                        match event.event_type {
                            UiEventType::Click => {
                                input_events
                                    .single_write(InputEvent::ActionPressed(action_name.to_string()));
                            }
                            _ => (),
                        }
                    }
                }
            };
            handle_button("pause button", "TogglePause");
            handle_button("speed up button", "SpeedUp");
            handle_button("slow down button", "SlowDown");
        }

        for event in input_events.read(self.input_reader_id.as_mut().unwrap()) {
            match event {
                InputEvent::ActionPressed(action_name) => {
                    if let Some(button_entity) = finder.find("pause button_btn_txt") {
                        if let Some(text) = ui_texts.get_mut(button_entity) {
                            match action_name.as_ref() {
                                "TogglePause" => {
                                    if text.text == "Pause" {
                                        text.text = "Play".to_string();
                                    } else if text.text == "Play" {
                                        text.text = "Pause".to_string();
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
