pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn new() -> Self {
        return Document { rows: vec![] };
    }

    pub fn get_row(&mut self, at: usize) -> Option<&mut Row> {
        if at >= self.rows.len() {
            return None;
        }

        Some(&mut self.rows[at])
    }

    pub fn delete_row(&mut self, at: usize) {
        self.rows.remove(at);
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn insert_row(&mut self, at: usize) -> &mut Row {
        let new_row = Row::new(String::new());

        self.rows.insert(at, new_row);
        self.rows.last_mut().unwrap()
    }
}

pub struct Row {
    pub content: String,
}

impl Row {
    pub fn new(content: String) -> Self {
        return Row { content };
    }

    pub fn insert(&mut self, at: usize, c: char) -> () {
        self.content.insert(at, c);
    }

    pub fn delete(&mut self, at: usize) {
        self.content.remove(at);
    }
}

mod document {}
