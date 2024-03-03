pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn new() -> Self {
        return Document {
            rows: vec![Row::new(String::new())],
        };
    }

    pub fn get_row(&mut self, at: usize) -> &mut Row {
        &mut self.rows[at]
    }

    pub fn delete_row(&mut self, at: usize) {
        self.rows.remove(at);
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn insert_row(&mut self, at: usize) {
        self.rows.insert(at, Row::new(String::new()));
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
