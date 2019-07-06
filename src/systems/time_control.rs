use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::*,
    input::InputEvent,
    shrev::{EventChannel, ReaderId},
    ui::*,
};

#[derive(Default)]
pub struct TimeControlSystem {
    ui_reader_id: Option<ReaderId<UiEvent>>,
    input_reader_id: Option<ReaderId<InputEvent<String>>>,

    pause_button: Option<UiButton>,
    speed_up_button: Option<UiButton>,
    slow_down_button: Option<UiButton>,
}

impl<'s> System<'s> for TimeControlSystem {
    type SystemData = (
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

        let font_handle = {
            let loader = res.fetch::<Loader>();
            let font_storage = res.fetch::<AssetStorage<FontAsset>>();
            loader.load(
                "assets/fonts/OpenSans-Regular.ttf",
                TtfFormat,
                (),
                &font_storage,
            )
        };
        let button_resources = UiButtonBuilderResources::<(), u32>::fetch(&res);
        self.pause_button = Some(
            UiButtonBuilder::<(), u32>::new("Pause")
                .with_id(0u32)
                .with_anchor(Anchor::BottomRight)
                .with_size(80.0, 36.0)
                .with_position(-255.0, 20.0)
                .with_font(font_handle.clone())
                .with_font_size(24.0f32)
                .with_text_color([0.0f32, 0.0, 0.0, 1.0])
                .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
                .with_press_text_color([0.5, 0.5, 0.5, 1.0])
                .build(button_resources)
                .1,
        );

        let button_resources = UiButtonBuilderResources::<(), u32>::fetch(&res);
        self.slow_down_button = Some(
            UiButtonBuilder::<(), u32>::new("Slow Down")
                .with_id(1u32)
                .with_anchor(Anchor::BottomRight)
                .with_size(100.0, 36.0)
                .with_position(-160.0, 20.0)
                .with_font(font_handle.clone())
                .with_font_size(24.0f32)
                .with_text_color([0.0f32, 0.0, 0.0, 1.0])
                .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
                .with_press_text_color([0.5, 0.5, 0.5, 1.0])
                .build(button_resources)
                .1,
        );

        let button_resources = UiButtonBuilderResources::<(), u32>::fetch(&res);
        self.speed_up_button = Some(
            UiButtonBuilder::<(), u32>::new("Speed Up")
                .with_id(2u32)
                .with_anchor(Anchor::BottomRight)
                .with_size(100.0, 36.0)
                .with_position(-55.0, 20.0)
                .with_font(font_handle.clone())
                .with_font_size(24.0f32)
                .with_text_color([0.0f32, 0.0, 0.0, 1.0])
                .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
                .with_press_text_color([0.5, 0.5, 0.5, 1.0])
                .build(button_resources)
                .1,
        );
    }

    fn run(&mut self, (ui_events, mut ui_texts, mut input_events): Self::SystemData) {
        for event in ui_events.read(self.ui_reader_id.as_mut().unwrap()) {
            match event.event_type {
                UiEventType::Click => {
                    let action_to_send = {
                        if event.target == self.pause_button.as_ref().unwrap().image_entity {
                            "TogglePause"
                        } else if event.target
                            == self.speed_up_button.as_ref().unwrap().image_entity
                        {
                            "SpeedUp"
                        } else if event.target
                            == self.slow_down_button.as_ref().unwrap().image_entity
                        {
                            "SlowDown"
                        } else {
                            ""
                        }
                    };
                    if action_to_send != "" {
                        input_events
                            .single_write(InputEvent::ActionPressed(action_to_send.to_string()));
                    }
                }
                _ => (),
            }
        }

        for event in input_events.read(self.input_reader_id.as_mut().unwrap()) {
            match event {
                InputEvent::ActionPressed(action_name) => match action_name.as_ref() {
                    "TogglePause" => {
                        let pause_button_text_entity =
                            self.pause_button.as_ref().unwrap().text_entity;
                        if let Some(text) = ui_texts.get_mut(pause_button_text_entity) {
                            if text.text == "Pause" {
                                text.text = "Play".to_string();
                            } else if text.text == "Play" {
                                text.text = "Pause".to_string();
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
