use amethyst::{
    assets::{AssetStorage, Loader},
    core::timing::Time,
    ecs::*,
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
        .with_position(-40.0, 20.0)
        .with_font(font_handle.clone())
        .with_font_size(24.0f32)
        .with_text_color([0.0f32, 0.0, 0.0, 1.0])
        .with_hover_text_color([0.2f32, 0.2f32, 0.2f32, 1.0f32])
        .with_press_text_color([0.5, 0.5, 0.5, 1.0])
        .build_from_world(world);
}

#[derive(Default)]
pub struct TimeControlSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl<'s> System<'s> for TimeControlSystem {
    type SystemData = (
        Read<'s, EventChannel<UiEvent>>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
        Write<'s, Time>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(res.fetch_mut::<EventChannel<UiEvent>>().register_reader());
    }

    fn run(&mut self, (ui_events, mut ui_texts, finder, mut time): Self::SystemData) {
        for event in ui_events.read(self.reader_id.as_mut().unwrap()) {
            if let Some(pause_button_entity) = finder.find("pause button") {
                if pause_button_entity == event.target {
                    match event.event_type {
                        UiEventType::ClickStart => {
                            let mut new_text = "Pause";
                            let current_time_scale = time.time_scale();
                            if current_time_scale == 0.0 {
                                time.set_time_scale(1.0);
                            } else {
                                time.set_time_scale(0.0);
                                new_text = "Play";
                            }
                            if let Some(button_entity) = finder.find("pause button_btn_txt") {
                                if let Some(text) = ui_texts.get_mut(button_entity) {
                                    text.text = new_text.to_string();
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
