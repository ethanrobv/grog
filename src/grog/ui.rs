use std::io;
use std::io::Write;
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

    pub fn set_cursor_pos(&self, x: usize, y: usize) {
        Term::stdout().move_cursor_to(x, y).unwrap()
    }

    pub fn get_terminal_window_size(&mut self) {
        (self.terminal_height, self.terminal_width) = Term::stdout().size()
    }

    pub fn draw_main_menu(&mut self) {
        let header_footer_row = "*".repeat(self.terminal_width as usize);
        let body_row = format!("*{}*", " ".repeat((self.terminal_width-2) as usize));

        let mut display_buffer = String::new();
        display_buffer.push_str(&header_footer_row);
        display_buffer.push('\n');
        for _ in 0..self.terminal_height - 2 {
            display_buffer.push_str(&body_row);
            display_buffer.push('\n');
        }
        display_buffer.push_str(&header_footer_row);
        self.current_display = display_buffer;
    }

    pub fn render(&self) {
        let mut term = Term::stdout();
        term.clear_screen().unwrap();
        term.write_all(self.current_display.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
}