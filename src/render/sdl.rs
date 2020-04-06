use std::collections::*;

use sdl2::*;
use sdl2::pixels::*;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::image::*;
use sdl2::rect::*;

use crate::render::*;
use crate::misc::vec::*;

struct SDLContext {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

struct SDLNode {
    name: String,
    pos: Vec2<i32>,
    dim: Vec2<u32>
}

pub struct SDLRenderer<'a> {
    sprite_cache: SpriteCache<'a>,
    queue: Vec<SDLNode>,
    cam_pos: Vec2<f32>,
    cam_screen: Vec2<u32>,
    cam_zoom: f32,
    context: SDLContext
}

impl<'a> Renderer for SDLRenderer<'a> {
    fn render(&mut self) {
        self.context.canvas.set_draw_color(Color::RGB(50, 50, 60));
        self.context.canvas.clear();

        for node in &self.queue {
            self.context.canvas.copy(self.sprite_cache.get(&node.name), None, Rect::new(node.pos.x, node.pos.y, node.dim.x, node.dim.y)).unwrap();
        }

        self.context.canvas.present();

        self.queue.clear();
    }

    fn add_sprite(&mut self, name: &str, file: &str) {
        // The texture can not outlive the creator as it is part of the same struct, so this should be safe
        let texture_creator = unsafe {
            &*(&self.context.texture_creator as *const TextureCreator<WindowContext>)
        };

        self.sprite_cache.insert(name.to_string(), texture_creator.load_texture(file).unwrap());
    }

    fn queue(&mut self, name: &str, pos: Vec2<f32>, dim: Vec2<f32>) {
        self.queue.push(SDLNode {
            name: name.to_string(),
            pos: (((pos - self.cam_pos + Vec2::new(0.0, dim.y)).inv_y() / 5.0 * Vec2::new(self.cam_zoom, self.cam_zoom) * Vec2::new(1.0, self.cam_screen.x as f32 / self.cam_screen.y as f32) + 1.0) / 2.0 * Vec2::new(self.cam_screen.x, self.cam_screen.y).convert()).convert(),
            dim: (dim * Vec2::new(self.cam_screen.x, self.cam_screen.y).convert() * Vec2::new(self.cam_zoom, self.cam_zoom) / 5.0 / 2.0 * Vec2::new(1.0, self.cam_screen.x as f32 / self.cam_screen.y as f32)).convert()
        })
    }

    fn camera(&mut self, pos: Vec2<f32>, zoom: f32, screen: Vec2<u32>) {
        self.cam_pos = pos;
        self.cam_zoom = zoom;
        self.cam_screen = screen;
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
            queue: Vec::new(),
            cam_pos: Vec2::new(0.0, 0.0),
            cam_screen: win_dim,
            cam_zoom: 1.0,
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
