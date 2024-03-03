use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Command {
    Quit,
    Move(Direction),
    Write(char),
    Backspace,
    Delete,
    Enter,
}

pub struct Reader {}

impl Reader {
    pub fn run(&self) -> crossterm::Result<Command> {
        loop {
            if let Ok(true) = crossterm::event::poll(Duration::from_millis(500)) {
                match crossterm::event::read()? {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::CONTROL,
                    }) => {
                        break;
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Up,
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Move(Direction::Up)),
                    Event::Key(KeyEvent {
                        code: KeyCode::Down,
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Move(Direction::Down)),
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Move(Direction::Left)),
                    Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Move(Direction::Right)),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Write(c)),
                    Event::Key(KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Backspace),
                    Event::Key(KeyEvent {
                        code: KeyCode::Delete,
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Delete),
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                    }) => return Ok(Command::Enter),
                    _ => {}
                }
            }
        }

        Ok(Command::Quit)
    }
}

pub mod reader {}
