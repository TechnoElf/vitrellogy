pub mod vec;

#[derive(Default, Debug)]
pub struct AppStateRes(pub AppState);

impl AppStateRes {
    pub fn new(state: AppState) -> Self {
        Self(state)
    }
}

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
