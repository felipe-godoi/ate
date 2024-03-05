use crate::reader::Direction;

#[derive(Debug)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub max_x: u16,
    pub max_y: u16,
}

impl Cursor {
    pub fn new() -> Self {
        return Cursor {
            x: 0,
            y: 0,
            max_y: 0,
            max_x: 0,
        };
    }

    pub fn move_cursor(&mut self, direction: Direction, amount: u16) -> () {
        match direction {
            Direction::Up => {
                if self.y > amount {
                    self.y -= amount;
                } else {
                    self.y = 0;
                }
            }
            Direction::Down => {
                if self.y + amount < self.max_y {
                    self.y += amount;
                }
            }
            Direction::Left => {
                if self.x > amount {
                    self.x -= amount;
                } else {
                    self.x = 0;
                }
            }
            Direction::Right => {
                if self.x + amount < self.max_x {
                    self.x += amount;
                } else {
                    self.x = self.max_x;
                }
            }
        }
    }
}

mod cursor {}
