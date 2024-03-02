use crate::{
    reader::{Command, Reader},
    render::Render,
};

pub struct Editor {
    pub reader: Reader,
    pub render: Render,
}

impl Editor {
    pub fn new() -> Self {
        return Editor {
            reader: Reader {},
            render: Render::new(),
        };
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        self.render.clear_screen();

        loop {
            self.render.render()?;

            match self.reader.run()? {
                Command::Quit => break,
                Command::Move(direction) => self.render.output.move_cursor(direction, 1),
                Command::Write(c) => self.render.output.write(c),
                Command::Backspace => {
                    self.render.output.backspace();
                }
                Command::Delete => {
                    self.render.output.delete();
                }
                Command::Enter => {
                    self.render.output.enter();
                }
            }
        }

        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.render
            .change_cursor_style(crossterm::cursor::CursorShape::Line)
            .unwrap();
    }
}

mod editor {}
