use std::fs;

use crossterm::terminal;

use crate::{
    cursor::Cursor,
    document::{Document, Row},
    reader::Direction,
};

pub struct Output {
    pub document: Document,
    pub cursor: Cursor,
    pub screen_rows: u16,
    pub rows_offset: u16,
}

impl Output {
    pub fn new() -> Self {
        return Output {
            document: Document::new(),
            cursor: Cursor::new(),
            screen_rows: terminal::size().unwrap().1,
            rows_offset: 0,
        };
    }

    pub fn open(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let file_content = fs::read_to_string(filename)?;
        file_content.lines().for_each(|line| {
            let row = Row::new(line.to_string());
            self.document.add_row(row)
        });

        self.cursor.max_y = self.document.rows.len() as u16 - 1;
        self.cursor.max_x = self.document.get_row(self.cursor.y as usize).content.len() as u16;

        Ok(())
    }

    pub fn move_cursor(&mut self, direction: Direction, amount: u16) {
        match direction {
            Direction::Left => {
                if self.cursor.x == 0 && self.cursor.y > 0 {
                    self.cursor.max_x = self
                        .document
                        .get_row(self.cursor.y as usize - 1)
                        .content
                        .len() as u16;
                    self.cursor.move_cursor(Direction::Up, 1);
                    self.cursor.x = self.cursor.max_x;
                } else {
                    self.cursor.move_cursor(direction, amount);
                }
            }
            Direction::Right => {
                if self.cursor.x == self.cursor.max_x && self.cursor.y < self.cursor.max_y {
                    self.cursor.move_cursor(Direction::Down, 1);
                    self.cursor.x = 0;
                } else {
                    self.cursor.move_cursor(direction, amount);
                }
            }
            Direction::Down => {
                if self.cursor.y > self.screen_rows + self.rows_offset {
                    self.rows_offset += 1;
                }

                self.cursor.move_cursor(direction, amount);
            }
            Direction::Up => {
                if self.cursor.y < self.rows_offset && self.rows_offset > 0 {
                    self.rows_offset -= 1;
                }

                self.cursor.move_cursor(direction, amount)
            }
        }

        let row = self.document.get_row(self.cursor.y as usize);
        self.cursor.max_x = row.content.len() as u16;
        if self.cursor.x > self.cursor.max_x {
            self.cursor.x = self.cursor.max_x;
        }
    }

    pub fn update_row_position(&mut self, new_row: u16) {
        if new_row < self.document.rows.len() as u16 {
            self.cursor.y = new_row;
            self.cursor.max_y = self.document.rows.len() as u16 - 1;

            let row = self.document.get_row(self.cursor.y as usize);
            self.cursor.max_x = row.content.len() as u16;

            if self.cursor.x >= row.content.len() as u16 {
                self.cursor.x = if row.content.len() as u16 > 0 {
                    row.content.len() as u16 - 1
                } else {
                    0
                };
            }
        }
    }

    pub fn write(&mut self, c: char) {
        let row = self.document.get_row(self.cursor.y as usize);

        row.insert(self.cursor.x as usize, c);
        self.cursor.max_x = row.content.len() as u16;
        self.move_cursor(Direction::Right, 1);
    }

    pub fn backspace(&mut self) {
        if self.cursor.x == 0 {
            if self.cursor.y == 0 {
                return;
            }

            let row = self.document.get_row(self.cursor.y as usize);
            let new_row = row.content.clone();
            self.document.delete_row(self.cursor.y as usize);
            self.update_row_position(self.cursor.y - 1);
            let row_to_update = self.document.get_row(self.cursor.y as usize);
            self.cursor.x = row_to_update.content.len() as u16;
            row_to_update.content.push_str(&new_row);
        } else {
            let row = self.document.get_row(self.cursor.y as usize);

            row.delete((self.cursor.x - 1) as usize);
            self.move_cursor(Direction::Left, 1);
        }
    }

    pub fn delete(&mut self) {
        let row = self.document.get_row(self.cursor.y as usize);

        if self.cursor.x + 1 == row.content.len() as u16 {
            return;
        }

        row.delete((self.cursor.x + 1) as usize);
    }

    pub fn enter(&mut self) {
        if self.cursor.x == 0 {
            self.document.insert_row(self.cursor.y as usize);
            self.update_row_position(self.cursor.y + 1);
        } else {
            let row = self.document.get_row(self.cursor.y as usize);
            let new_row = row.content.split_off(self.cursor.x as usize);
            self.document.insert_row(self.cursor.y as usize + 1);
            self.document.get_row(self.cursor.y as usize + 1).content = new_row;
            self.cursor.x = 0;
            self.update_row_position(self.cursor.y + 1);
        }
    }
}

mod output {}
