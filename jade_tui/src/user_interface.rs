#![allow(unused, dead_code)]
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Widget},
    DefaultTerminal, Frame,
};

use crate::{logs::Logs, screen::Screen};

#[derive(Default)]
pub struct UserInterface {
    running: bool,
    logs: u8,
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

        let [_, screen_space, logs_space, _] = Layout::default()
            // Now we're setting the vertical spacing, as it's the default.
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Percentage(78), // Screen Space
                    Constraint::Percentage(20), // Logs
                    Constraint::Min(1),
                ], // Empty space
            )
            .areas(area);

        // Render the footer
        let [_, logs_space, _] = Layout::new(
            Direction::Horizontal,
            // Constraint::from_percentages([1, 98, 1]),
            [
                Constraint::Min(1),
                Constraint::Percentage(98),
                Constraint::Min(1),
            ],
        )
        .areas(logs_space);
        // Logs.render(logs_space, buf);

        // Render the screen
        let [_, screen_space, _] = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Min(1),
                Constraint::Percentage(98),
                Constraint::Min(1),
            ],
        )
        .areas(screen_space);
        Screen.render(screen_space, buf);
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
