use std::{thread::sleep, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use jade_tui::{
    circular_buffer::CircularBuffer,
    logs::{LogLevel, LogMessage, Logs},
};

fn main() {
    let mut circular_buffer = CircularBuffer::with_capacity(20);
    circular_buffer.append(LogMessage::new(LogLevel::Info, "Questa e' una info"));
    let mut terminal = ratatui::init();
    let mut logs = Logs::from(circular_buffer);
    let mut cnt = 0;
    loop {
        logs.append(LogMessage::new(
            LogLevel::Info,
            format!("Questa e' una info,{}", cnt),
        ));

        terminal
            .draw(|frame| {
                frame.render_widget(&mut logs, frame.area());
            })
            .unwrap();
        if event::poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match (key.modifiers, key.code) {
                        (_, KeyCode::Esc | KeyCode::Char('q'))
                        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        cnt = (cnt + 1) % 10;
        sleep(Duration::from_millis(500));
    }

    ratatui::restore();
}
