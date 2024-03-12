use std::usize;

pub(crate) struct TextContent {
    content: Vec<String>,
    cursor: (usize, usize),
}

impl TextContent {
    pub fn new() -> Self {
        TextContent {
            content: {
                let mut c = Vec::new();
                c.push(String::new());
                c
            },
            cursor: (0, 0),
        }
    }

    pub fn get_text(&self) -> Vec<String> {
        self.content.clone()
    }

    pub fn get_string(&self) -> String {
        self.content.join("\n")
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        let (l, c) = self.cursor.clone();
        if c >= self.content[l].len() {
            (l, self.content[l].len())
        } else {
            (l, c)
        }
    }

    pub fn _empty(&self) -> bool {
        self.content.len() == 0 && self.content[0].len() == 0
    }

    pub fn append(&mut self, text: String) {
        if self.content.len() == 0 {
            self.content.push(text);
            return;
        }
        let (l, mut c) = self.cursor;
        if self.cursor.1 >= self.content[l].len() {
            self.snap_cursor_end_of_line();
            c = self.cursor.1;
        }
        let (left, right) = self.content[l].split_at(c);
        let new = format!("{}{}{}", left, text, right);
        self.content[l] = new;
        self.move_cursor_right(text.len());
    }

    pub fn snap_cursor_end_of_line(&mut self) {
        let (l, _) = self.cursor;
        let c = self.content[l].len();
        self.cursor = (l, c);
    }

    pub fn snap_cursor_start_of_line(&mut self) {
        let (l, _) = self.cursor;
        self.cursor = (l, 0);
    }

    pub fn move_cursor_up(&mut self, n: usize) {
        let (l, c) = self.cursor;
        let l = if l < n { 0 } else { l - n };
        self.cursor = (l, c);
    }

    pub fn move_cursor_down(&mut self, n: usize) {
        let (l, c) = self.cursor;
        let l = {
            let l = l + n;
            if l >= self.content.len() {
                self.content.len() - 1
            } else {
                l
            }
        };
        self.cursor = (l, c);
    }

    pub fn move_cursor_right(&mut self, n: usize) {
        let (l, c) = self.cursor;
        let no_more_characters = c >= self.content[l].len() && l == self.content.len() - 1;
        let end_of_line = c >= self.content[l].len();
        if no_more_characters {
        } else if end_of_line {
            self.move_cursor_down(1);
            self.snap_cursor_start_of_line();
        } else {
            let c = c + n;
            self.cursor = (l, c);
        }
    }

    pub fn move_cursor_left(&mut self, n: usize) {
        let (l, c) = self.cursor;
        let no_more_characters = c == 0 && l == 0;
        let beginning_of_line = c < n;
        if no_more_characters {
        } else if beginning_of_line {
            self.move_cursor_up(1);
            self.snap_cursor_end_of_line()
        } else {
            self.cursor = (l, c - n);
        };
    }

    pub fn remove(&mut self) {
        let (l, mut c) = self.cursor;
        if c >= self.content[l].len() {
            self.snap_cursor_end_of_line();
            c = self.cursor.1;
        }
        let nothing_to_remove = c == 0 && l == 0;
        let begining_of_line = c == 0;
        if nothing_to_remove {
        } else if begining_of_line {
            self.move_cursor_up(1);
            self.snap_cursor_end_of_line();
            let (l, _) = self.cursor;
            self.content[l] = format!("{}{}", self.content[l], self.content[l + 1]);
            self.content.remove(l + 1);
        } else {
            self.content[l].remove(c - 1);
            self.move_cursor_left(1);
        }
    }

    pub fn new_line(&mut self) {
        let (l, c) = self.cursor;
        let right = if c < self.content[l].len() {
            self.content[l].split_off(c)
        } else {
            String::new()
        };
        self.content.insert(l + 1, right);
        self.move_cursor_down(1);
        self.snap_cursor_start_of_line();
    }
}
