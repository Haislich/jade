use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Widget},
    DefaultTerminal, Frame,
};

use crate::footer::Footer;

#[derive(Default)]
pub struct UserInterface {
    running: bool,
    // counter: u8,
}
impl UserInterface {
    /// Constructor of [`UserInterface`]
    pub fn new() -> Self {
        // default of bool is false
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
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            // (_, KeyCode::Left) => self.counter -= 1,
            _ => {}
        }
    }
    fn quit(&mut self) {
        self.running = false;
    }
}
// This allows to encapsulate code related to rendering only on one place.
impl Widget for &UserInterface {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // Render the border with instructions.
        let title = Line::from(" Jade ").bold().green().centered();
        let instructions =
            Line::from(vec![" Quit ".into(), "<Q / Ctrl-c / Esc> ".green().bold()]).centered();
        Block::bordered()
            .border_type(BorderType::Thick)
            // .border_type(BorderType::Rounded)
            .title(title)
            .title_bottom(instructions.centered())
            .render(area, buf);

        let main_layout = Layout::default()
            // Now we're setting the vertical spacing, as it's the default.
            .constraints(
                vec![
                    Constraint::Percentage(75), // Screen Space
                    Constraint::Percentage(22), // Logs
                    Constraint::Percentage(3),
                ], // Empty space
            )
            .split(area);

        // Render the footer
        let footer_layout = Layout::new(
            Direction::Horizontal,
            Constraint::from_percentages(vec![1, 98, 1]),
        )
        .split(main_layout[1]);

        Footer.render(footer_layout[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect};

    #[test]
    fn render() {
        let ui = UserInterface::default();
        // Creates a square buffer, simulates a square terminal.
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 50));
        // Write the rendered widget to the buffer.
        ui.render(buf.area, &mut buf);
        // This is a textual representation of what we expect to see.
        let mut _expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━━━━━━━━━━━ Quit <Q / Ctrl-c / Esc> ━━━━━━━━━━━━┛",
        ]);
        todo!()
    }
    #[test]
    fn test_default() {
        println!("{}", bool::default())
    }
}
