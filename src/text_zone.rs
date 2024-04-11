use std::usize;

#[derive(Clone)]
pub(crate) struct TextContent {
    content: Vec<Vec<char>>,
    cursor: (usize, usize),
}

impl TextContent {
    pub fn new() -> Self {
        TextContent {
            content: {
                let mut c = Vec::new();
                c.push(Vec::new());
                c
            },
            cursor: (0, 0),
        }
    }

    pub fn from_string(text: String) -> Self {
        let mut res = Self::new();
        for line in text.lines() {
            res.content.push(line.chars().collect());
        }
        res
    }

    pub fn get_text(&self) -> Vec<String> {
        self.content.iter().map(|l| l.iter().collect()).collect()
    }

    pub fn get_string(&self) -> String {
        self.get_text().join("\n")
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
        self.content.is_empty() || self.content[0].is_empty()
    }

    pub fn line_count(&self) -> usize {
        self.content.len()
    }

    pub fn longest_line_length(&self) -> usize {
        self.content.iter().map(|l| l.len()).max().unwrap()
    }

    pub fn _current_line_length(&self) -> usize {
        let (l, _) = self.cursor;
        self.content[l].len()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.line_count(), self.longest_line_length())
    }

    pub fn append(&mut self, text: String) {
        let text = text.chars().collect();
        if self.content.is_empty() {
            self.content.push(text);
            return;
        }
        let (l, mut c) = self.cursor;
        if self.cursor.1 >= self.content[l].len() {
            self.snap_cursor_end_of_line();
            c = self.cursor.1;
        }
        let (left, right) = self.content[l].split_at(c);
        let new = [left, &text, right].concat();
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
        let (l, mut c) = self.cursor;
        if c > self.content[l].len() {
            self.snap_cursor_end_of_line();
            c = self.cursor.1;
        }
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

    pub fn set_cursor(&mut self, cursor: (usize, usize)) {
        let (mut l, mut c) = cursor;
        if l >= self.content.len() {
            l = self.content.len() - 1;
            c = self.content[l].len();
        }
        if c > self.content[l].len() {
            c = self.content[l].len();
        }
        self.cursor = (l, c);
    }

    pub fn remove(&mut self) {
        let (l, mut c) = self.cursor;
        if c > self.content[l].len() {
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
            let removed_line = self.content[l + 1].clone();
            self.content[l].extend(removed_line);
            self.content.remove(l + 1);
        } else {
            self.content[l].remove(c - 1);
            self.cursor.1 -= 1;
        }
    }

    pub fn move_line_up(&mut self) {
        let (l, _) = self.cursor;
        if l == 0 {
            return;
        }
        let line = self.content.remove(l);
        self.content.insert(l - 1, line);
        self.cursor.0 -= 1;
    }

    pub fn move_line_down(&mut self) {
        let (l, _) = self.cursor;
        if l >= self.content.len() - 1 {
            return;
        }
        let line = self.content.remove(l);
        self.content.insert(l + 1, line);
        self.cursor.0 += 1;
    }

    pub fn new_line(&mut self) {
        let (l, _) = self.cursor;
        self.content.insert(l + 1, Vec::new());
        self.cursor = (l + 1, 0);
    }

    pub fn break_line(&mut self) {
        let (l, c) = self.cursor;
        if c >= self.content[l].len() {
            self.new_line();
            return;
        }
        let right = self.content[l].split_off(c);
        self.content.insert(l + 1, right);
        self.move_cursor_down(1);
        self.snap_cursor_start_of_line();
    }
}
