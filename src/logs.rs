use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Widget, Wrap};

use crate::circular_buffer::CircularBuffer;

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
#[derive(Debug)]
pub struct LogMessage {
    log_level: LogLevel,
    message: String,
}
impl LogMessage {
    pub fn new<S: Into<String>>(log_level: LogLevel, message: S) -> Self {
        let message = message.into();
        Self { log_level, message }
    }
}
impl<'line> Into<Line<'line>> for LogMessage {
    fn into(self) -> Line<'line> {
        Line::from(vec![
            match self.log_level {
                LogLevel::Info => Span::styled("Info: ", Style::new().yellow().bold()),
                LogLevel::Warning => Span::styled("Warning: ", Style::new().green().bold()),
                LogLevel::Error => Span::styled("Error: ", Style::new().red().bold()),
            },
            Span::styled(self.message, Style::new()),
        ])
    }
}
impl<'line> Into<Line<'line>> for &'line LogMessage {
    fn into(self) -> Line<'line> {
        Line::from(vec![
            match self.log_level {
                LogLevel::Info => Span::styled("[Info]: ", Style::new().green().bold()),
                LogLevel::Warning => Span::styled("[Warning]: ", Style::new().yellow().bold()),
                LogLevel::Error => Span::styled("[Error]: ", Style::new().red().bold()),
            },
            Span::styled(self.message.as_str(), Style::new()),
        ])
    }
}

#[derive(Debug, Default)]
pub struct Logs {
    circular_buffer: CircularBuffer<LogMessage>,
}
impl Logs {
    pub fn new(circular_buffer: CircularBuffer<LogMessage>) -> Self {
        Self { circular_buffer }
    }
    pub fn append(&mut self, element: LogMessage) {
        self.circular_buffer.append(element);
    }
    pub fn len(&self) -> usize {
        self.circular_buffer.len()
    }
}
impl From<CircularBuffer<LogMessage>> for Logs {
    fn from(circular_buffer: CircularBuffer<LogMessage>) -> Self {
        Self { circular_buffer }
    }
}
impl<'line> Into<Text<'line>> for &'line mut Logs {
    fn into(self) -> Text<'line> {
        let mut result = Vec::with_capacity(self.circular_buffer.len());
        // println!("{}",)
        for log_message in &self.circular_buffer {
            result.push(log_message.into());
        }
        Text::from(result)
    }
}
impl Widget for &mut Logs {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title(Span::styled(
            " Logs ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));

        Paragraph::new(self)
            .wrap(Wrap { trim: false })
            .block(block)
            .render(area, buf);
    }
}
