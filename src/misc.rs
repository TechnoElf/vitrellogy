pub mod vec;

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
