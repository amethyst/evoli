use crate::day_night_cycle::DayNightCycleEvent;
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    audio,
    audio::{
        output::Output as AudioOutput, AudioSink, OggFormat, Source as AudioSource,
        SourceHandle as AudioSourceHandle,
    },
    ecs::*,
    shrev::{EventChannel, ReaderId},
};
use std::ops::DerefMut;

#[derive(Default)]
pub struct MusicSystem {
    day_night_cycle_event_subscription: Option<ReaderId<DayNightCycleEvent>>,
    day_music: Option<AudioSourceHandle>,
    night_music: Option<AudioSourceHandle>,
    audio_source: Option<AudioSourceHandle>,
    audio_sink: Option<AudioSink>,
}

type MusicSystemData<'s> = (
    Read<'s, EventChannel<DayNightCycleEvent>>,
    Read<'s, AssetStorage<AudioSource>>,
    Write<'s, AudioOutput>,
);

impl<'s> System<'s> for MusicSystem {
    type SystemData = MusicSystemData<'s>;

    fn setup(&mut self, res: &mut Resources) {
        info!("setup Music");

        if let Some(audio_output) = audio::output::default_output() {
            res.entry::<AudioOutput>().or_insert(audio_output);
        } else {
            error!("No audio output hardware detected")
        }

        // TODO #day_night #date_time as there is no system currently writing to this EventChannel, explicitly register it for now
        // once the DateTimeSystem is producing DayNightCycleEvents, this can be removed
        // TODO what? this doesn't parse?
        // res.insert(EventChannel<DayNightCycleEvent>::new());
        type DayNightEventChannel = EventChannel<DayNightCycleEvent>;
        res.entry::<DayNightEventChannel>()
            .or_insert(DayNightEventChannel::new());

        Self::SystemData::setup(res);

        self.day_night_cycle_event_subscription = Some(
            res.fetch_mut::<EventChannel<DayNightCycleEvent>>()
                .register_reader(),
        );

        res.entry::<ProgressCounter>()
            .or_insert(ProgressCounter::default());
        let mut progress = res.fetch_mut::<ProgressCounter>();
        let loader = res.fetch::<Loader>();
        let audio_source_storage = res.fetch::<AssetStorage<AudioSource>>();
        self.day_music = Some(loader.load(
            "assets/ambient.ogg",
            OggFormat,
            progress.deref_mut(),
            &audio_source_storage,
        ));
        // this music courtesy of https://community.amethyst.rs/u/jakob_t_r
        // via this post https://community.amethyst.rs/t/evoli-mvp-implementation-tracker/537/6
        self.night_music = Some(loader.load(
            "assets/ambient_night.ogg",
            OggFormat,
            progress.deref_mut(),
            &audio_source_storage,
        ));
    }

    fn run(
        &mut self,
        (day_night_cycle_event_channel, audio_source_storage, audio_output): Self::SystemData,
    ) {
        // watch for day_night_cycle events and change music to match time of day
        day_night_cycle_event_channel
            .read(
                self.day_night_cycle_event_subscription
                    .as_mut()
                    .expect("not subscribed to DayNightCycleEvents"),
            )
            .for_each(|event| {
                // stop the current audio
                if let Some(audio_sink) = self.audio_sink.as_ref() {
                    audio_sink.stop();
                }
                self.audio_sink = None;

                // choose new audio source appropriate for the event
                self.audio_source = match event {
                    DayNightCycleEvent::GoodMorning => self.day_music.clone(),
                    DayNightCycleEvent::GoodNight => self.night_music.clone(),
                };
            });

        // we will create and start a new audio_sink whenever the current one becomes invalid,
        // either by finishing normally or by being explicitly stopped
        let new_sink_needed = if let Some(audio_sink) = self.audio_sink.as_ref() {
            audio_sink.empty()
        } else {
            true
        };

        // if it is deemed that new audio should start, play the current audio source
        if new_sink_needed {
            self.audio_sink = self
                .audio_source
                .as_ref()
                .and_then(|audio_source| audio_source_storage.get(audio_source))
                .and_then(|audio_source| {
                    let audio_sink = AudioSink::new(&audio_output);
                    audio_sink.append(audio_source).ok().and(Some(audio_sink))
                });
        };
    }
}
