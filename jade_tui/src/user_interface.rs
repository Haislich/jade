use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{style::Stylize, text::Line, widgets::Block, DefaultTerminal, Frame};

#[derive(Default)]
pub struct UserInterface {
    running: bool,
}
impl UserInterface {
    /// Constructor of [`UserInterface`]
    pub fn new() -> Self {
        Self::default()
    }
    /// Application main loop.
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            // Tell the terminal to refresh its frame.
            // In order to do so call self.draw(frame)
            // to actually render the content.
            // If an error occurs propagate the error.
            terminal.draw(|frame: &mut Frame<'_>| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }
    /// Draw the current `frame` to screen.
    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from(" Jade ").bold().green().centered();
        let border = Block::bordered()
            .title(title)
            .title_bottom("Press `Esc`, `Ctrl-c` or `q` to exit");
        frame.render_widget(border, frame.area())
    }
    /// Handle the incoming events.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }
    fn quit(&mut self) {
        self.running = false;
    }
}
