use std::io;
use std::io::Write;
use console::Term;

pub struct UI {
    current_display: String,
}

impl UI {
    pub fn new() -> UI {
        UI {
            current_display: String::new(),
        }
    }

    pub fn set_cursor_pos(&self, x: usize, y: usize) {
        Term::stdout().move_cursor_to(x, y).unwrap()
    }

    pub fn draw_main_menu(&mut self) {
        let (terminal_height, terminal_width) = {
            let (h, w) = Term::stdout().size();
            (h as usize, w as usize)
        };
        let header_footer_row = "*".repeat(terminal_width);
        let body_row = format!("*{}*", " ".repeat(terminal_width - 2));

        let mut display_buffer = String::new();
        display_buffer.push_str(&header_footer_row);
        display_buffer.push('\n');
        for _ in 0..terminal_height - 2 {
            display_buffer.push_str(&body_row);
            display_buffer.push('\n');
        }
        display_buffer.push_str(&header_footer_row);
        if display_buffer != self.current_display {
            self.render(display_buffer);
        }
    }

    pub fn render(&mut self, buffer: String) {
        let mut term = Term::stdout();
        term.hide_cursor().unwrap();
        term.clear_screen().unwrap();
        term.write_all(buffer.as_bytes()).unwrap();
        term.show_cursor().unwrap();
        io::stdout().flush().unwrap();
        self.current_display = buffer;
    }
}