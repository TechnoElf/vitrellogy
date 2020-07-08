use std::fmt::Debug;

use num_traits::cast::{NumCast, cast};

use nalgebra::Vector2;

use specs::{FlaggedStorage, VecStorage, Component};

use vitrellogy_macro::DefaultConstructor;

#[derive(Default, Debug, DefaultConstructor)]
pub struct AppStateRes(pub AppState);

#[derive(Debug)]
pub enum AppState {
    Running,
    Stopping
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Running
    }
}

#[derive(Debug, Clone, DefaultConstructor)]
pub struct TransformCom {
    pub pos: Vector2<f32>,
}

impl Component for TransformCom {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

pub trait Convertable<U: NumCast + Debug + Copy + PartialEq + 'static> {
    fn convert(self) -> Vector2<U>;
}

impl<T: NumCast + Debug + Copy + PartialEq + 'static, U: NumCast + Debug + Copy + PartialEq + 'static> Convertable<U> for Vector2<T> {
    fn convert(self) -> Vector2<U> {
        Vector2::new(cast(self.x).unwrap(), cast(self.y).unwrap())
    }
}
