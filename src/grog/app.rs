use crate::grog::input::Input;
use crate::grog::ui::UI;

pub struct App {
    pub input: Input,
    pub ui: UI,
}

impl App {
    pub fn new() -> App {
        App {
            input: Input::new(),
            ui: UI::new(),
        }
    }
}