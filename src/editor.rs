use crate::{
    reader::{Command, Reader},
    render::Render,
};

pub struct Editor {
    pub reader: Reader,
    pub render: Render,
}

impl Editor {
    pub fn new(args: &Vec<String>) -> Self {
        let filename = if args.len() > 1 {
            args[1].clone()
        } else {
            String::from("")
        };

        return Editor {
            reader: Reader {},
            render: Render::new(filename),
        };
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        self.render.clear_screen();

        if self.render.filename.len() > 0 {
            self.render.output.open(&self.render.filename)?;
        }

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

mod editor {}
