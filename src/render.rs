pub mod sdl;

use serde::{Serialize, Deserialize};

use nalgebra::Vector2;

use specs::{prelude::*, Component, DenseVecStorage};

use vitrellogy_macro::DefaultConstructor;
use crate::physics::TransformCom;
use crate::render::sdl::SDLRenderImpl;
use crate::input::MouseRes;
use crate::misc::{Convertable, Vector};

#[derive(Debug, DefaultConstructor)]
pub struct CameraRes {
    pub pos: Vector,
    pub zoom: f32,
    pub screen: Vector2<u32>
}

impl Default for CameraRes {
    fn default() -> Self {
        Self {
            pos: Vector::new(0.0, 0.0),
            zoom: 1.0,
            screen: Vector2::new(800, 600)
        }
    }
}

#[derive(DefaultConstructor)]
pub struct RenderSys {
    renderer: SDLRenderImpl<'static>
}

impl<'a> System<'a> for RenderSys {
    type SystemData = (Write<'a, UIEventQueue>,
        Read<'a, CameraRes>,
        Read<'a, MouseRes>,
        ReadStorage<'a, TransformCom>,
        ReadStorage<'a, SpriteCom>,
        ReadStorage<'a, TextCom>,
        ReadStorage<'a, ButtonUICom>,
        ReadStorage<'a, TextUICom>,
        ReadStorage<'a, StartVerticalGroupCom>,
        ReadStorage<'a, StartHorizontalGroupCom>,
        ReadStorage<'a, EndGroupCom>,
        ReadStorage<'a, ConstraintCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut events, camera, mouse, transforms, sprites, texts, buttons, text_labels, v_group_start, h_group_start, group_end, constraints) = data;

        self.renderer.pre();

        for (sprite, transform) in (&sprites, &transforms).join() {
            self.renderer.render(&sprite.name, transform.pos, sprite.dim, camera.pos, camera.zoom, camera.screen);
        }

        for (text, transform) in (&texts, &transforms).join() {
            self.renderer.write(&text.text, &text.font, transform.pos, text.dim, camera.pos, camera.zoom, camera.screen);
        }

        events.clear();
        let mut container: Vec<(Vector2<u32>, Vector2<u32>, Vector2<u32>, bool)> = Vec::new();
        container.push((Vector2::new(0, 0), camera.screen, Vector2::new(0, 0), true));
        for (constraint, button, text_label, vgs, hgs, ge) in (&constraints, (&buttons).maybe(), (&text_labels).maybe(), (&v_group_start).maybe(), (&h_group_start).maybe(), (&group_end).maybe()).join() {
            let size = Vector2::new(constraint.x_size.as_pixels(container.last().unwrap().1.x), constraint.y_size.as_pixels(container.last().unwrap().1.y));
            let pos = Vector2::new(constraint.x_pos.as_pixels(size.x, container.last().unwrap().1.x) + container.last().unwrap().0.x, constraint.y_pos.as_pixels(size.y, container.last().unwrap().1.y) + container.last().unwrap().0.y);

            match vgs {
                Some(_) => {
                    container.push((pos, size, Vector2::new(0, 0), true));
                    continue;
                },
                None => ()
            }

            match hgs {
                Some(_) => {
                    container.push((pos, size, Vector2::new(0, 0), false));
                    continue;
                },
                None => ()
            }

            match ge {
                Some(_) => {
                    container.pop();
                    continue;
                },
                None => ()
            }
            
            container.last_mut().unwrap().2 += pos;

            match button {
                Some(button) => {
                    match mouse.0 {
                        Some(m) if container.last_mut().unwrap().2.x < m.x && m.x < container.last_mut().unwrap().2.x + size.x && container.last_mut().unwrap().2.y < m.y && m.y < container.last_mut().unwrap().2.y + size.y => {
                            events.push(UIEvent::ButtonPressed { id: button.element_name.clone() });
                            self.renderer.render_ss(&button.sprite_pressed, container.last_mut().unwrap().2.convert(), size, camera.screen);
                        },
                        Some(_) | None => self.renderer.render_ss(&button.sprite, container.last_mut().unwrap().2.convert(), size, camera.screen)

                    }
                },
                None => ()
            }

            match text_label {
                Some(text_label) => {
                    self.renderer.write_ss(&text_label.text, &text_label.font, container.last_mut().unwrap().2.convert(), size, camera.screen);
                },
                None => ()
            }

            if container.last_mut().unwrap().3 {
                container.last_mut().unwrap().2.y += size.y;
                container.last_mut().unwrap().2.x -= pos.x;
            } else {
                container.last_mut().unwrap().2.x += size.x;
                container.last_mut().unwrap().2.y -= pos.y;
            }
        }

        self.renderer.post();
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct SpriteCom {
    pub name: String,
    pub dim: Vector
}

impl SpriteCom {
    pub fn new(name: &str, dim: Vector) -> Self {
        Self {
            name: name.to_string(),
            dim: dim
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TextCom {
    pub text: String,
    pub font: String,
    pub dim: Vector
}

impl TextCom {
    pub fn new(text: &str, font: &str, dim: Vector) -> Self {
        Self {
            text: text.to_string(),
            font: font.to_string(),
            dim: dim
        }
    }
}

event_queue! {
    UIEventQueue: pub enum UIEvent {
        ButtonPressed { id: String }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TextUICom {
    pub text: String,
    pub font: String
}

impl TextUICom {
    pub fn new(text: &str, font: &str) -> Self {
        Self {
            text: text.to_string(),
            font: font.to_string()
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct ButtonUICom {
    pub sprite: String,
    pub sprite_pressed: String,
    pub element_name: String
}

impl ButtonUICom {
    pub fn new(sprite: &str, sprite_pressed: &str, element_name: &str) -> Self {
        Self {
            sprite: sprite.to_string(),
            sprite_pressed: sprite_pressed.to_string(),
            element_name: element_name.to_string()
        }
    }
}

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct ConstraintCom {
    x_pos: PositionConstraint,
    y_pos: PositionConstraint,
    x_size: SizeConstraint,
    y_size: SizeConstraint
}

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct StartVerticalGroupCom;

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct StartHorizontalGroupCom;

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct EndGroupCom;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum PositionConstraint {
    Start,
    Center,
    End,
    StartPixelOffset(u32)
}

impl PositionConstraint {
    fn as_pixels(&self, size: u32, container_size: u32) -> u32 {
        match self {
            PositionConstraint::Start => 0,
            PositionConstraint::Center => (container_size / 2) - (size / 2),
            PositionConstraint::End => container_size - size,
            PositionConstraint::StartPixelOffset(offset) => offset.clone()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum SizeConstraint {
    Proportion(f32),
    Pixels(u32),
    NegativePixels(u32)
}

impl SizeConstraint {
    fn as_pixels(&self, container_size: u32) -> u32 {
        match self {
            SizeConstraint::Proportion(proportion) => (proportion * container_size as f32).floor() as u32,
            SizeConstraint::Pixels(pixels) => pixels.clone(),
            SizeConstraint::NegativePixels(pixels) => container_size - pixels.clone()
        }
    }
}

pub fn register(world: &mut World) {
    world.insert(CameraRes::default());
    world.register::<SpriteCom>();
    world.register::<TextCom>();
    world.register::<ConstraintCom>();
    world.register::<TextUICom>();
    world.register::<ButtonUICom>();
    world.register::<StartVerticalGroupCom>();
    world.register::<StartHorizontalGroupCom>();
    world.register::<EndGroupCom>();
}
