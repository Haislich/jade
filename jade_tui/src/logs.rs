#![allow(unused, dead_code)]

use std::fmt::Display;

use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
// impl Into<String> for LogLevel {
//     fn into(self) -> String {
//         match self {
//             LogLevel::Info => String::from("Info: "),
//             LogLevel::Warning => String::from("Waning: "),
//             LogLevel::Error => String::from("Error: "),
//         }
//     }
// }
// impl Display for LogLevel {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             LogLevel::Info => write!(f, "Info"),
//             LogLevel::Warning => write!(f, "Waning"),
//             LogLevel::Error => write!(f, "Error"),
//         }
//     }
// }
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
                LogLevel::Info => Span::styled("Info: ", Style::new().yellow().bold()),
                LogLevel::Warning => Span::styled("Warning: ", Style::new().green().bold()),
                LogLevel::Error => Span::styled("Error: ", Style::new().red().bold()),
            },
            Span::styled(self.message.as_str(), Style::new()),
        ])
    }
}

#[derive(Debug)]
pub struct CircularBuffer<T> {
    pub inner: Vec<T>,
    last_idx: usize,
}
impl<T> Default for CircularBuffer<T> {
    fn default() -> Self {
        // Rather abitrary choice, but I assume that
        // the buffer keeps track of up to the last
        // 32 logs
        Self {
            inner: Vec::with_capacity(32),
            last_idx: 0,
        }
    }
}
impl<T> CircularBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            last_idx: 0,
        }
    }
    pub fn push(&mut self, element: T) {
        println!(
            "lat_idx {}, {}, {}",
            self.last_idx,
            self.inner.len(),
            self.inner.capacity()
        );

        if self.inner.len() == self.inner.capacity() {
            // println!("hehe");
            self.inner[self.last_idx] = element;
        } else {
            self.inner.push(element);
        }
        self.last_idx = (self.last_idx + 1) % self.inner.capacity();
    }
}
// impl<'line> Into<Vec<Line<'line>>> for CircularBuffer {
//     fn into(self) -> Vec<Line<'line>> {
//         let mut result = Vec::with_capacity(self.inner.capacity());
//         for log_message in self.inner {
//             result.push(log_message.into());
//         }
//         result
//     }
// }
impl<'line> Into<Text<'line>> for &'line CircularBuffer<LogMessage> {
    fn into(self) -> Text<'line> {
        let mut result = Vec::with_capacity(self.inner.capacity());
        for log_message in &self.inner {
            result.push(log_message.into());
        }
        Text::from(result)
    }
}
#[derive(Debug)]
pub struct Logs {
    pub data: CircularBuffer<LogMessage>,
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
        // .render(area, buf);
        let text: Text<'_> = (&self.data).into();
        Paragraph::new(text).block(block).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

    use super::*;
    fn print_logs(logs: &Logs) {
        for log in &logs.data.inner {
            println!("{:?}", log);
        }
    }
    #[test]
    fn creation() {
        eprintln!("ciao");
        let mut circular_buffer = CircularBuffer::with_capacity(3);
        circular_buffer.push(LogMessage::new(LogLevel::Info, "Questa e' una info"));
        circular_buffer.push(LogMessage::new(LogLevel::Warning, "Questo e' un warning"));
        circular_buffer.push(LogMessage::new(LogLevel::Error, "Questo e' un errore"));
        // let mut terminal = ratatui::init();
        let mut logs = Logs {
            data: circular_buffer,
        };
        // println!("{:?}", logs);
        // print_logs(&logs);
        loop {
            print_logs(&logs);
        }
        // let mut cnt = 0;
        // loop {
        //     // println!("{:?}", logs);
        //     if cnt == 20 {
        //         logs.data
        //             .push(LogMessage::new(LogLevel::Info, "Questa e' una info"));
        //     }
        //     if cnt == 40 {
        //         logs.data
        //             .push(LogMessage::new(LogLevel::Warning, "Questo e' un warning"));
        //     }
        //     if cnt == 60 {
        //         logs.data
        //             .push(LogMessage::new(LogLevel::Error, "Questo e' un errore"));
        //     }
        //     // terminal
        //     //     .draw(|frame| {
        //     //         frame.render_widget(&mut logs, frame.area());
        //     //     })
        //     //     .unwrap();
        //     // println!("{:?}", logs);
        //     // match event::read().unwrap() {
        //     //     Event::Key(key) if key.kind == KeyEventKind::Press => {
        //     //         match (key.modifiers, key.code) {
        //     //             (_, KeyCode::Esc | KeyCode::Char('q'))
        //     //             | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
        //     //                 break;
        //     //             }
        //     //             _ => {}
        //     //         }
        //     //     }
        //     //     _ => {}
        //     // }

        //     cnt = (cnt + 1) % 61;
        // }

        // ratatui::restore();
    }
}
