use console::Term;

pub struct UI {
    terminal_height: u16,
    terminal_width: u16,

    current_display: String,
}

impl UI {
    pub fn new() -> UI {
        let (terminal_height, terminal_width) = Term::stdout().size();

        UI {
            terminal_width,
            terminal_height,
            current_display: String::new(),
        }
    }

    pub fn get_terminal_window_size(&mut self) {
        (self.terminal_height, self.terminal_width) = Term::stdout().size()
    }

    pub fn draw_main_menu(&self) {
        let header_footer_row = "*".repeat(self.terminal_width as usize);
        let body_row = format!("*{}*", " ".repeat(self.terminal_width as usize));

        Term::stdout().clear_screen().unwrap();
        Term::stdout().write_line(&header_footer_row).unwrap();
        for _ in 0..self.terminal_height - 2 {
            Term::stdout().write_line(&body_row).unwrap();
        }
        Term::stdout().write_line(&header_footer_row).unwrap();
    }
}