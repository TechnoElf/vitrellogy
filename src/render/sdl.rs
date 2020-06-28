use std::collections::HashMap;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{Sdl2ImageContext, InitFlag, LoadTexture};
use sdl2::rect::Rect;
use sdl2::ttf::{Sdl2TtfContext, Font};

use crate::render::Renderer;
use crate::misc::vec::Vec2;

#[allow(dead_code)]
struct SDLContext {
    canvas: Canvas<Window>,
    image: Sdl2ImageContext,
    font: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
}

pub struct SDLRenderer<'a> {
    sprite_cache: SpriteCache<'a>,
    font_cache: FontCache<'a>,
    context: SDLContext
}

impl<'a> Renderer for SDLRenderer<'a> {
    fn render(&mut self, sprite_name: &str, sprite_pos: Vec2<f32>, sprite_dim: Vec2<f32>, cam_pos: Vec2<f32>, cam_zoom: f32, cam_screen: Vec2<u32>) {
        // Camera transformation
        let pos = (((sprite_pos - cam_pos + Vec2::new(0.0, sprite_dim.y)).inv_y() / 5.0 * Vec2::new(cam_zoom, cam_zoom) * Vec2::new(1.0, cam_screen.x as f32 / cam_screen.y as f32) + 1.0) / 2.0 * Vec2::new(cam_screen.x, cam_screen.y).convert()).convert();
        let dim = (sprite_dim * Vec2::new(cam_screen.x, cam_screen.y).convert() * Vec2::new(cam_zoom, cam_zoom) / 5.0 / 2.0 * Vec2::new(1.0, cam_screen.x as f32 / cam_screen.y as f32)).convert();

        self.context.canvas.copy(self.sprite_cache.get(sprite_name), None, Rect::new(pos.x, pos.y, dim.x, dim.y)).unwrap();
    }

    fn write(&mut self, text: &str, font: &str, text_pos: Vec2<f32>, text_dim: Vec2<f32>, cam_pos: Vec2<f32>, cam_zoom: f32, cam_screen: Vec2<u32>) {
        let (font, color) = self.font_cache.get(font);

        let text_surface = font.render(text).blended(color.clone()).unwrap();
        let (w, h) = text_surface.size();
        let text_texture = self.context.texture_creator.create_texture_from_surface(text_surface).unwrap();

        // Camera transformation
        let text_dim = Vec2::new(text_dim.y * (w as f32 / h as f32), text_dim.y);
        let pos = (((text_pos - cam_pos + Vec2::new(0.0, text_dim.y)).inv_y() / 5.0 * Vec2::new(cam_zoom, cam_zoom) * Vec2::new(1.0, cam_screen.x as f32 / cam_screen.y as f32) + 1.0) / 2.0 * Vec2::new(cam_screen.x, cam_screen.y).convert()).convert();
        let dim = (text_dim * Vec2::new(cam_screen.x, cam_screen.y).convert() * Vec2::new(cam_zoom, cam_zoom) / 5.0 / 2.0 * Vec2::new(1.0, cam_screen.x as f32 / cam_screen.y as f32)).convert();

        self.context.canvas.copy(&text_texture, None, Rect::new(pos.x, pos.y, dim.x, dim.y)).unwrap();
    }

    fn render_ss(&mut self, sprite_name: &str, sprite_pos: Vec2<u32>, sprite_dim: Vec2<u32>, cam_screen: Vec2<u32>) {
        let pos = Vec2::new(sprite_pos.x, cam_screen.y - (sprite_pos.y + sprite_dim.y)).convert();
        let dim = sprite_dim;

        self.context.canvas.copy(self.sprite_cache.get(sprite_name), None, Rect::new(pos.x, pos.y, dim.x, dim.y)).unwrap();
    }

    fn write_ss(&mut self, text: &str, font: &str, text_pos: Vec2<u32>, text_dim: Vec2<u32>, cam_screen: Vec2<u32>) {
        let (font, color) = self.font_cache.get(font);

        let text_surface = font.render(text).blended(color.clone()).unwrap();
        let (w, h) = text_surface.size();
        let text_texture = self.context.texture_creator.create_texture_from_surface(text_surface).unwrap();

        let text_dim = Vec2::new((text_dim.y as f32 * (w as f32 / h as f32)) as u32, text_dim.y);
        let pos = Vec2::new(text_pos.x, cam_screen.y - (text_pos.y + text_dim.y)).convert();
        let dim = text_dim;

        self.context.canvas.copy(&text_texture, None, Rect::new(pos.x, pos.y, dim.x, dim.y)).unwrap();
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

    fn add_font(&mut self, name: &str, file: &str, size: u16, red: u8, green: u8, blue: u8) {
        // The font can not outlive the creator as it is part of the same struct, so this should be safe
        let font_context = unsafe {
            &*(&self.context.font as *const Sdl2TtfContext)
        };

        self.font_cache.insert(name.to_string(), font_context.load_font(file, size).unwrap(), Color::RGB(red, green, blue));
    }
}

impl SDLRenderer<'_> {
    pub fn init<'a>(sdl_context: &'a Sdl, win_dim: Vec2<u32>) -> SDLRenderer<'a> {
        let sdl_image_context = sdl2::image::init(InitFlag::PNG).unwrap();
        let sdl_font_context = sdl2::ttf::init().unwrap();

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
            image: sdl_image_context,
            font: sdl_font_context,
            texture_creator: texture_creator
        };

        SDLRenderer {
            sprite_cache: SpriteCache::new(),
            font_cache: FontCache::new(),
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

struct FontCache<'a> {
    cache: HashMap<String, (Font<'a, 'static>, Color)>
}

impl<'a> FontCache<'a> {
    fn new() -> Self {
        FontCache {
            cache: HashMap::new()
        }
    }

    fn insert(&mut self, name: String, font: Font<'a, 'static>, color: Color) {
        self.cache.insert(name, (font, color));
    }

    fn get(&self, name: &str) -> &(Font<'a, 'static>, Color) {
        match self.cache.contains_key(name) {
            true => &self.cache[name],
            false => panic!("font \"{}\" not found in cache", name)
        }
    }
}
