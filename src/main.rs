use color_eyre::Result;
use jade_tui::user_interface::UserInterface;
// use ratatui::prelude::Backend;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    // let be = terminal.backend_mut();
    // be.hide_cursor()?;
    // be.
    let result = UserInterface::default().run(terminal);
    ratatui::restore();
    result
}
