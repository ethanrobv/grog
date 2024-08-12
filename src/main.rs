mod grog;
use grog::app::App;


fn main() {
    let mut app = App::new();
    loop {
        app.ui.get_terminal_window_size();
        app.ui.draw_main_menu();

        app.input.get_key();
        app.input.process_events();
    }
}
