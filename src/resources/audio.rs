use amethyst::{
    assets::Loader,
    audio::{AudioSink, OggFormat, SourceHandle},
    ecs::prelude::World,
};

use std::iter::Cycle;
use std::vec::IntoIter;

const BACKGROUND_MUSIC: &'static [&'static str] = &["assets/ambient.ogg"];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

// Initialise audio in the world. This sets up the background music
pub fn initialise_audio(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25);

        let music = BACKGROUND_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, &file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        Music { music }
    };

    // Add sounds to the world
    world.add_resource(music);
}
