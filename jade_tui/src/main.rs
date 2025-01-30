use color_eyre::Result;
use jade_tui::user_interface::UserInterface;
fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = UserInterface::new().run(terminal);
    ratatui::restore();
    result
}
