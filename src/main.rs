use std::io;

use ratatui::DefaultTerminal;

// TODO

fn main() {
    fn main() -> io::Result<()> {
        let mut terminal = ratatui::init();
        let app_result = App::default().run(&mut terminal);
        ratatui::restore();
        app_result
    }
}
