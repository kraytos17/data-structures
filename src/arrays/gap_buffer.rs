#[derive(Debug)]
pub struct GapBuffer {
    buffer: Vec<char>,
    cursor: usize,
    gap_end: usize,
}

const MIN_BUF_SIZE: usize = 1024;
const SHRINK_THRESHOLD: usize = MIN_BUF_SIZE;

impl GapBuffer {
    pub fn new(capacity: usize) -> Self {
        let init_size = std::cmp::max(capacity, MIN_BUF_SIZE);
        let mut buffer = Vec::with_capacity(init_size);
        buffer.resize(init_size, '\0');

        Self {
            cursor: 0,
            gap_end: init_size,
            buffer,
        }
    }

    pub fn move_cursor(&mut self, pos: usize) {
        let max_pos = self.used_size();
        let new_pos = pos.min(max_pos);

        if new_pos < self.cursor {
            self.move_left(self.cursor - new_pos);
        } else if new_pos > self.cursor {
            self.move_right(new_pos - self.cursor);
        }
    }

    fn move_left(&mut self, distance: usize) {
        if distance == 0 || distance > self.cursor {
            return;
        }
        let start = self.cursor - distance;
        self.buffer
            .copy_within(start..self.cursor, self.gap_end - distance);

        self.cursor -= distance;
        self.gap_end -= distance;
    }

    fn move_right(&mut self, distance: usize) {
        if distance == 0 || self.cursor + distance > self.gap_end {
            return;
        }
        let start = self.gap_end;
        self.buffer
            .copy_within(self.cursor..start, self.cursor + distance);

        self.cursor += distance;
        self.gap_end += distance;
    }

    pub fn insert(&mut self, c: char) {
        if self.cursor == self.gap_end {
            self.grow();
        }
        self.buffer[self.cursor] = c;
        self.cursor += 1;
    }

    pub fn delete(&mut self) {
        if self.cursor < self.gap_end && self.gap_end < self.buffer.len() {
            self.gap_end += 1;
        }
        if self.used_size() < self.buffer.len() / 4 && self.buffer.len() > SHRINK_THRESHOLD {
            self.shrink();
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.cursor.saturating_sub(1);
            if self.gap_end < self.buffer.len() {
                self.gap_end = self.gap_end.saturating_add(1);
            }

            if self.used_size() < self.buffer.len() / 4 && self.buffer.len() > SHRINK_THRESHOLD {
                self.shrink();
            }
        }
    }

    fn grow(&mut self) {
        let new_size = self.buffer.len() * 2;
        self.resize_buffer(new_size);
    }

    fn shrink(&mut self) {
        let used_size = self.used_size();
        let new_size = std::cmp::max(used_size * 2, MIN_BUF_SIZE);
        if new_size < self.buffer.len() {
            self.resize_buffer(new_size);
        }
    }

    fn resize_buffer(&mut self, new_size: usize) {
        let used_size = self.used_size();
        let mut new_buffer = Vec::with_capacity(new_size);

        new_buffer.extend(self.buffer.iter().take(self.cursor).cloned());
        new_buffer.extend(std::iter::repeat('\0').take(new_size.saturating_sub(used_size)));
        new_buffer.extend(self.buffer.iter().skip(self.gap_end).cloned());

        self.buffer = new_buffer;
        self.gap_end = std::cmp::min(self.cursor + (new_size - used_size), new_size);
    }

    pub fn extract_text(&self) -> String {
        let mut result = String::with_capacity(self.used_size());
        result.extend(self.buffer.iter().take(self.cursor));
        result.extend(self.buffer.iter().skip(self.gap_end));
        
        result
    }

    #[inline]
    fn used_size(&self) -> usize {
        self.cursor + (self.buffer.len() - self.gap_end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let gap_buffer = GapBuffer::new(10);
        assert_eq!(gap_buffer.buffer.capacity(), MIN_BUF_SIZE);
        assert_eq!(gap_buffer.cursor, 0);
        assert_eq!(gap_buffer.gap_end, MIN_BUF_SIZE);
    }

    #[test]
    fn test_insert() {
        let mut gap_buffer = GapBuffer::new(4);
        gap_buffer.insert('a');
        gap_buffer.insert('b');
        gap_buffer.insert('c');
        gap_buffer.insert('d');

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "abcd");
    }

    #[test]
    fn test_cursor_move_left() {
        let mut gap_buffer = GapBuffer::new(4);
        gap_buffer.insert('a');
        gap_buffer.insert('b');
        gap_buffer.insert('c');
        gap_buffer.move_cursor(1);
        gap_buffer.insert('x');

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "axbc");
    }

    #[test]
    fn test_cursor_move_right() {
        let mut gap_buffer = GapBuffer::new(4);
        gap_buffer.insert('a');
        gap_buffer.insert('b');
        gap_buffer.insert('c');
        gap_buffer.move_cursor(1);
        gap_buffer.move_cursor(3);
        gap_buffer.insert('x');

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "abcx");
    }

    #[test]
    fn test_delete() {
        let mut gap_buffer = GapBuffer::new(4);
        gap_buffer.insert('a');
        gap_buffer.insert('b');
        gap_buffer.insert('c');
        gap_buffer.insert('d');
        gap_buffer.move_cursor(2);
        gap_buffer.delete();

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "abd");
    }

    #[test]
    fn test_backspace() {
        let mut gap_buffer = GapBuffer::new(4);
        gap_buffer.insert('a');
        gap_buffer.insert('b');
        gap_buffer.insert('c');
        gap_buffer.backspace();

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "ab");
    }

    #[test]
    fn test_grow() {
        let mut gap_buffer = GapBuffer::new(2);
        gap_buffer.insert('a');
        gap_buffer.insert('b');
        gap_buffer.insert('c');
        gap_buffer.insert('d');
        gap_buffer.insert('e');

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "abcde");
        assert!(gap_buffer.buffer.capacity() > 4);
    }

    #[test]
    fn test_shrink() {
        let mut gap_buffer = GapBuffer::new(8);
        gap_buffer.insert('a');
        gap_buffer.insert('b');
        gap_buffer.insert('c');
        gap_buffer.insert('d');
        gap_buffer.backspace();
        gap_buffer.backspace();
        gap_buffer.backspace();

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "a");
        assert_eq!(gap_buffer.buffer.capacity(), MIN_BUF_SIZE);
    }

    #[test]
    fn test_extract_text() {
        let mut gap_buffer = GapBuffer::new(10);
        gap_buffer.insert('h');
        gap_buffer.insert('e');
        gap_buffer.insert('l');
        gap_buffer.insert('l');
        gap_buffer.insert('o');
        gap_buffer.move_cursor(2);
        gap_buffer.insert('y');

        let extracted_text = gap_buffer.extract_text();
        assert_eq!(extracted_text, "heyllo");
    }

    #[test]
    fn test_move_cursor_beyond_bounds() {
        let mut gap_buffer = GapBuffer::new(10);
        gap_buffer.insert('h');
        gap_buffer.insert('e');
        gap_buffer.insert('l');
        gap_buffer.insert('l');
        gap_buffer.insert('o');

        gap_buffer.move_cursor(usize::MAX);
        assert_eq!(gap_buffer.cursor, 5);

        gap_buffer.move_cursor(0);
        assert_eq!(gap_buffer.cursor, 0);

        gap_buffer.move_cursor(usize::MAX);
        assert_eq!(gap_buffer.cursor, 5);
    }
}
