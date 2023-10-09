use macroquad::audio::{self, Sound, PlaySoundParams};
use std::collections::HashMap;

pub struct Audio {
    sounds: HashMap<&'static str, Sound>,
}

impl Audio {
    pub async fn new() -> Self {
        let mut sounds: HashMap<&'static str, Sound> = HashMap::new();
        sounds.insert("bg", audio::load_sound_from_bytes(include_bytes!("res/bg.wav")).await.unwrap());
        sounds.insert("gunshot", audio::load_sound_from_bytes(include_bytes!("res/gunshot.wav")).await.unwrap());
        sounds.insert("equip", audio::load_sound_from_bytes(include_bytes!("res/equip.wav")).await.unwrap());
        sounds.insert("door", audio::load_sound_from_bytes(include_bytes!("res/door.wav")).await.unwrap());
        sounds.insert("flesh", audio::load_sound_from_bytes(include_bytes!("res/flesh.wav")).await.unwrap());
        sounds.insert("lock", audio::load_sound_from_bytes(include_bytes!("res/lock.wav")).await.unwrap());
        sounds.insert("reload", audio::load_sound_from_bytes(include_bytes!("res/reload.wav")).await.unwrap());
        sounds.insert("empty shot", audio::load_sound_from_bytes(include_bytes!("res/empty-shot.wav")).await.unwrap());
        sounds.insert("trash", audio::load_sound_from_bytes(include_bytes!("res/trash.wav")).await.unwrap());
        sounds.insert("siren", audio::load_sound_from_bytes(include_bytes!("res/siren.wav")).await.unwrap());

        Self { sounds }
    }

    pub fn play_sound(&self, name: &str) {
        audio::play_sound(self.sounds.get(name).unwrap(), PlaySoundParams { looped: false, volume: 1. });
    }

    pub fn loop_sound(&self, name: &str) {
        audio::play_sound(self.sounds.get(name).unwrap(), PlaySoundParams { looped: true, volume: 1. });
    }

    pub fn stop_sound(&self, name: &str) {
        audio::stop_sound(self.sounds.get(name).unwrap());
    }
}
