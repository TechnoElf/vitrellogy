use std::time::Instant;

use sdl2::mixer::*;

use specs::*;

use vitrellogy_macro::DefaultConstructor;
use crate::render::{UIEvent, UIEventQueue};

pub struct SoundRes<'a> {
    _mixer: Sdl2MixerContext,
    audio_cache: Vec<(Chunk, f64)>,
    music_cache: Vec<Vec<(Music<'a>, f64)>>,
    frequency: u32,
    format: u32,
    channel_count: u32,
    current_music: Option<(MusicID, usize, Instant)>
}

// Do NOT, and I repeat, DO NOT attempt to send -
unsafe impl<'a> Send for SoundRes<'a> {}
// - OR SYNC!
unsafe impl<'a> Sync for SoundRes<'a> {}

#[allow(dead_code)]
impl<'a> SoundRes<'a> {
    pub fn new() -> Self {
        let mixer = init(InitFlag::all()).unwrap();
        open_audio(44100, DEFAULT_FORMAT, DEFAULT_CHANNELS, 512).unwrap();
        Music::set_volume(10);
        let spec = query_spec().unwrap();
        SoundRes {
            _mixer: mixer,
            audio_cache: Vec::new(),
            music_cache: Vec::new(),
            frequency: spec.0 as u32,
            format: spec.1 as u32,
            channel_count: spec.2 as u32,
            current_music: None
        }
    }

    pub fn load(&mut self, file: &str) -> SoundID {
        let audio = Chunk::from_file(file).unwrap();
        let len = self.chunk_length(&audio);
        self.audio_cache.push((audio, len));
        SoundID(self.audio_cache.len() - 1)
    }

    pub fn load_music(&mut self, files: &[&str]) -> MusicID {
        let mut layers: Vec<(Music, f64)> = Vec::new();
        for file in files {
            let audio = Chunk::from_file(file).unwrap();
            let len = self.chunk_length(&audio);
            let audio = Music::from_file(file).unwrap();
            layers.push((audio, len));
        }
        self.music_cache.push(layers);
        MusicID(self.music_cache.len() - 1)
    }

    pub fn play_music(&mut self, id: MusicID, layer: usize) {
        if self.current_music.is_some() {
            if id != self.current_music.as_ref().unwrap().0 {
                Music::fade_out(0).unwrap();

                self.music_cache[id.0][layer].0.fade_in_from_pos(-1, 0, 0.0).unwrap();
                self.current_music = Some((id, layer, Instant::now()));
            } else if layer != self.current_music.as_ref().unwrap().1 {
                Music::fade_out(0).unwrap();

                let mut time = self.current_music.as_ref().unwrap().2.elapsed().as_secs_f64() % self.music_cache[(self.current_music.as_ref().unwrap().0).0][self.current_music.as_ref().unwrap().1].1;
                if time >= self.music_cache[id.0][layer].1 {
                    time = 0.0;
                }

                self.music_cache[id.0][layer].0.fade_in_from_pos(-1, 0, time).unwrap();
                self.current_music.as_mut().unwrap().1 = layer;
            }
        } else {
            self.music_cache[id.0][layer].0.play(-1).unwrap();
            self.current_music = Some((id, layer, Instant::now()));
        }
    }

    pub fn play(&self, id: SoundID) {
        Channel::all().play(&self.audio_cache[id.0].0, 1).unwrap();
    }

    fn chunk_length(&self, chunk: &Chunk) -> f64 {
        let chunk_size = unsafe { (*chunk.raw).alen };
        let chunk_points = chunk_size / ((self.format & 0xff) / 8);
        let chunk_frames = chunk_points / self.channel_count;
        let chunk_length = chunk_frames as f64 / self.frequency as f64;
        chunk_length
    }
}

#[derive(DefaultConstructor)]
pub struct SoundSys(u8, MusicID);

impl<'a> System<'a> for SoundSys {
    type SystemData = (WriteExpect<'a, SoundRes<'static>>,
        Read<'a, UIEventQueue>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut sound, ui_events) = data;

        for event in ui_events.iter() {
            match event {
                UIEvent::ButtonPressed { id } => match id.as_ref() {
                    "sound" => {
                        sound.play_music(self.1, self.0 as usize);
                        self.0 += 1;
                        if self.0 > 3 {
                            self.0 = 0;
                        }
                    },
                    _ => ()
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct SoundID(usize);
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct MusicID(pub usize);
