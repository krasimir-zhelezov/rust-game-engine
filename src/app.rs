use crate::state::State;

pub struct App {
    pub state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}