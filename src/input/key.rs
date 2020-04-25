use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct KeysRes(pub HashMap<Key, bool>);

impl KeysRes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn pressed(&self, key: Key) -> bool {
        match self.0.contains_key(&key) {
            true => self.0[&key],
            false => false
        }
    }

    pub fn press(&mut self, key: Key) {
        self.0.insert(key, true);
    }

    pub fn release(&mut self, key: Key) {
        self.0.insert(key, false);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Key {
    Unknown,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Space
}
