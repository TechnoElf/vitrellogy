use std::fmt::Debug;
use std::collections::HashMap;
use std::any::{Any, TypeId};

use num_traits::cast::{NumCast, cast};

use nalgebra::Vector2;

use specs::{FlaggedStorage, VecStorage, Component};

use vitrellogy_macro::DefaultConstructor;

pub struct StateRes(HashMap<String, Box<dyn State>>);

pub trait State: Any + Send + Sync + 'static {
    fn get_type(&self) -> TypeId;
}

impl<T: Any + Send + Sync + 'static> State for T {
    fn get_type(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

#[allow(dead_code)]
impl StateRes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<T: State>(&mut self, name: &str, val: T) {
        self.0.insert(name.to_string(), Box::new(val));
    }

    pub fn get<T: State>(&self, name: &str) -> Option<&T> {
        self.0.get(name).map(Box::as_ref).and_then(|r| if r.get_type() == TypeId::of::<T>() {
            unsafe {
                Some(&*(r as *const dyn State as *const T))
            }
        } else {
            None
        })
    }
}

#[derive(Debug)]
pub enum AppState {
    Running,
    Stopping
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

#[macro_export]
macro_rules! event_queue {
    ($queue_name:ident : pub $type:tt $name:ident {$($element:tt)*}) => {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub $type $name {
            $($element)+
        }

        #[allow(dead_code)]
        #[derive(Default, Clone, Debug)]
        pub struct $queue_name {
            q: std::vec::Vec<$name>,
        }

        #[allow(dead_code)]
        impl $queue_name {
            pub fn new() -> Self {
                Self {
                    q: std::vec::Vec::new()
                }
            }

            pub fn push(&mut self, item: $name) {
                self.q.push(item);
            }

            pub fn iter(&self) -> std::slice::Iter<$name> {
                self.q.iter()
            }

            fn clear(&mut self) {
                self.q.clear();
            }
        }
    };
}
