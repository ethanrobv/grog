mod grog;

use std::time::Duration;
use grog::app::App;


fn main() {
    let mut app = App::new();
    //let term = Term::stdout();
    loop {
        app.input.get_key();
        app.ui.get_terminal_window_size();

        app.ui.draw_main_menu();
        app.ui.render();
        app.input.process_key_event();
        app.ui.set_cursor_pos(app.input.cursor_x_pos, app.input.cursor_y_pos);

        std::thread::sleep(Duration::from_millis(16));
    }
}
