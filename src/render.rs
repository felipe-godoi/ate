use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{CursorShape, Hide, MoveTo, SetCursorShape, Show},
    execute, queue,
    terminal::{self, ClearType},
};

use crate::output::Output;

pub struct Render {
    pub writer: RenderWriter,
    pub output: Output,
}

impl Render {
    pub fn new() -> Self {
        return Render {
            writer: RenderWriter::new(),
            output: Output::new(),
        };
    }

    pub fn clear_screen(&mut self) {
        execute!(
            self.writer,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();
    }

    pub fn render(&mut self) -> Result<(), std::io::Error> {
        let x = self.output.cursor.x;
        let y = self.output.cursor.y;

        queue!(self.writer, Hide, MoveTo(0, 0))?;

        for row in self.output.document.rows.iter() {
            self.writer.content.push_str(&row.content);

            queue!(self.writer, terminal::Clear(ClearType::UntilNewLine))?;
            self.writer.content.push_str("\r\n");
        }

        queue!(self.writer, MoveTo(x, y), Show)?;

        self.writer.flush()
    }

    pub fn change_cursor_style(&mut self, style: CursorShape) -> Result<(), std::io::Error> {
        execute!(self.writer, SetCursorShape(style))
    }
}

pub struct RenderWriter {
    content: String,
}

impl RenderWriter {
    fn new() -> Self {
        return RenderWriter {
            content: String::new(),
        };
    }
}

impl io::Write for RenderWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

pub mod render {}
