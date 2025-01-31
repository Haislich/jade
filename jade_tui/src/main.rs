use std::{thread::sleep, time::Duration};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use jade_tui::{
    logs::{CircularBuffer, LogLevel, LogMessage, Logs},
    // user_interface::UserInterface,
};

fn main() -> Result<()> {
    // color_eyre::install()?;
    // let terminal = ratatui::init();
    // let result = UserInterface::new().run(terminal);
    // ratatui::restore();
    // result
    // not_main()
    let mut cnt = 0;
    let mut circular_buffer = CircularBuffer::with_capacity(3);
    circular_buffer.push(1);
    circular_buffer.push(2);
    circular_buffer.push(3);

    loop {
        sleep(Duration::from_millis(1000));
        if cnt == 0 {
            circular_buffer.push(1)
        }
        if cnt == 1 {
            circular_buffer.push(2)
        }
        if cnt == 2 {
            circular_buffer.push(3)
        }
        println!("{:?}", circular_buffer);
        cnt = (cnt + 1) % 3;
        println!("{}", cnt)
    }
}

fn not_main() -> Result<()> {
    let mut circular_buffer = CircularBuffer::with_capacity(3);
    circular_buffer.push(LogMessage::new(LogLevel::Info, "Questa e' una info"));
    circular_buffer.push(LogMessage::new(LogLevel::Warning, "Questo e' un warning"));
    circular_buffer.push(LogMessage::new(LogLevel::Error, "Questo e' un errore"));
    let mut terminal = ratatui::init();
    let mut logs = Logs {
        data: circular_buffer,
    };
    let mut cnt = 0;
    loop {
        println!("{:?}", logs);
        if cnt == 20 {
            logs.data
                .push(LogMessage::new(LogLevel::Info, "Questa e' una info"));
        }
        if cnt == 40 {
            logs.data
                .push(LogMessage::new(LogLevel::Warning, "Questo e' un warning"));
        }
        if cnt == 60 {
            logs.data
                .push(LogMessage::new(LogLevel::Error, "Questo e' un errore"));
        }
        terminal.draw(|frame| {
            frame.render_widget(&mut logs, frame.area());
        })?;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    break;
                }
                _ => {}
            },
            _ => {}
        }

        cnt = (cnt + 1) % 61;
        println!("{}", cnt)
    }

    ratatui::restore();
    Ok(())
}
