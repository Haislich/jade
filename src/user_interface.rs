#![allow(unused, dead_code)]
use color_eyre::Result;
use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::EnterAlternateScreen,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Widget},
    DefaultTerminal, Frame,
};

use crate::{
    logs::{LogLevel, LogMessage, Logs},
    screen::Screen,
};

#[derive(Default)]
pub struct UserInterface {
    running: bool,
    logs: Logs,
}
impl UserInterface {
    /// Application main loop.
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        terminal.hide_cursor()?;
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
            _ => {}
        }
    }
    fn quit(&mut self) {
        self.running = false;
    }
}
// This allows to encapsulate code related to rendering only on one place.
impl Widget for &mut UserInterface {
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

        // Render the logs
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
        self.logs.append(LogMessage::new(
            LogLevel::Info,
            format!(
                "Screen space area {}, width {}, height {}",
                screen_space.area(),
                screen_space.width,
                screen_space.height,
            ),
        ));
        self.logs.append(LogMessage::new(
            LogLevel::Info,
            format!(
                "Screen Size{}, x {}, y {}",
                screen_space.as_size(),
                screen_space.x,
                screen_space.y
            ),
        ));
        self.logs.append(LogMessage::new(
            LogLevel::Info,
            format!(
                "Area {}, center_x {}, center_y {}",
                screen_space.area(),
                screen_space.x + screen_space.width,
                screen_space.y + screen_space.height
            ),
        ));
        self.logs.render(logs_space, buf);
    }
}
