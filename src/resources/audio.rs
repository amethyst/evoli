use amethyst::{
    assets::Loader,
    audio::{AudioSink, OggFormat, SourceHandle},
    ecs::prelude::World,
};

use std::iter::Cycle;
use std::vec::IntoIter;
use std::collections::HashMap;

const DAY_BACKGROUND_MUSIC: &'static str = "assets/day_ambient.ogg";
const NIGHT_BACKGROUND_MUSIC: &'static str = "assets/night_ambient.ogg";

#[derive(Default)]
pub struct Music {

    pub musics: HashMap<String, SourceHandle>,
    pub current_music: Option<String>,
}

impl Music {
    pub fn get_current(& self) -> Option<SourceHandle>{
        match &self.current_music {
            None => {None},
            Some(cur) => {self.musics.get(cur).cloned()},
        }

    }

    pub fn set_current(&mut self, new_music_name: Option<String>){
        self.current_music = new_music_name;
    }
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), (), &world.read_resource())
}

// Initialise audio in the world. This sets up the background music
pub fn initialise_audio(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25);



        let mut musics = HashMap::new();
        musics.insert("day".into(), load_audio_track(&loader, &world, DAY_BACKGROUND_MUSIC));
        musics.insert("night".into(), load_audio_track(&loader, &world, NIGHT_BACKGROUND_MUSIC));

        Music { musics: musics, current_music: Some("day".into()), }
    };

    // Add sounds to the world
    world.add_resource(music);
}
