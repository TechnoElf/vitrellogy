use std::collections::HashMap;

use sdl2::*;
use sdl2::pixels::*;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::image::*;
use sdl2::rect::*;

use crate::render::Renderer;
use crate::misc::vec::Vec2;

struct SDLContext {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

pub struct SDLRenderer<'a> {
    sprite_cache: SpriteCache<'a>,
    context: SDLContext
}

impl<'a> Renderer for SDLRenderer<'a> {
    fn render(&mut self, sprite_name: &str, sprite_pos: Vec2<f32>, sprite_dim: Vec2<f32>, cam_pos: Vec2<f32>, cam_zoom: f32, cam_screen: Vec2<u32>) {
        // Camera transformation
        let pos = (((sprite_pos - cam_pos + Vec2::new(0.0, sprite_dim.y)).inv_y() / 5.0 * Vec2::new(cam_zoom, cam_zoom) * Vec2::new(1.0, cam_screen.x as f32 / cam_screen.y as f32) + 1.0) / 2.0 * Vec2::new(cam_screen.x, cam_screen.y).convert()).convert();
        let dim = (sprite_dim * Vec2::new(cam_screen.x, cam_screen.y).convert() * Vec2::new(cam_zoom, cam_zoom) / 5.0 / 2.0 * Vec2::new(1.0, cam_screen.x as f32 / cam_screen.y as f32)).convert();

        self.context.canvas.copy(self.sprite_cache.get(sprite_name), None, Rect::new(pos.x, pos.y, dim.x, dim.y)).unwrap();
    }

    fn pre(&mut self) {
        self.context.canvas.set_draw_color(Color::RGB(50, 50, 60));
        self.context.canvas.clear();
    }

    fn post(&mut self) {
        self.context.canvas.present();
    }

    fn add_sprite(&mut self, name: &str, file: &str) {
        // The texture can not outlive the creator as it is part of the same struct, so this should be safe
        let texture_creator = unsafe {
            &*(&self.context.texture_creator as *const TextureCreator<WindowContext>)
        };

        self.sprite_cache.insert(name.to_string(), texture_creator.load_texture(file).unwrap());
    }
}

impl SDLRenderer<'_> {
    pub fn init<'a>(sdl_context: &'a Sdl, win_dim: Vec2<u32>) -> SDLRenderer<'a> {
        let _sdl_image = sdl2::image::init(InitFlag::PNG).unwrap();

        let video = sdl_context.video().unwrap();

        let window = video.window("Vitrellogy", win_dim.x, win_dim.y)
            .position_centered()
            .resizable()
            .build().unwrap();

        let mut canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let texture_creator = canvas.texture_creator();

        let context = SDLContext {
            canvas: canvas,
            texture_creator: texture_creator
        };

        SDLRenderer {
            sprite_cache: SpriteCache::new(),
            context: context,
        }
    }
}

struct SpriteCache<'a> {
    cache: HashMap<String, Texture<'a>>
}

impl<'a> SpriteCache<'a> {
    fn new() -> Self {
        SpriteCache {
            cache: HashMap::new()
        }
    }

    fn insert(&mut self, name: String, texture: Texture<'a>) {
        self.cache.insert(name, texture);
    }

    fn get(&self, name: &str) -> &Texture<'a> {
        match self.cache.contains_key(name) {
            true => &self.cache[name],
            false => panic!("sprite \"{}\" not found in cache", name)
        }
    }
}
