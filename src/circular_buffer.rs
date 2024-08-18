use std::iter::FromIterator;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");

        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.size == self.capacity {
            self.tail = (self.tail + 1) % self.capacity;
        } else {
            self.size += 1;
        }

        if self.buffer.len() < self.capacity {
            self.buffer.push(item);
        } else {
            self.buffer[self.head] = item;
        }

        self.head = (self.head + 1) % self.capacity;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let item = std::mem::replace(&mut self.buffer[self.tail], unsafe { std::mem::zeroed() });
        self.tail = (self.tail + 1) % self.capacity;
        self.size -= 1;

        Some(item)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            Some(&self.buffer[self.tail])
        }
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.size).map(move |i| {
            let index = (self.tail + i) % self.capacity;
            &self.buffer[index]
        })
    }
}

impl<T> Default for CircularBuffer<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            capacity: 0,
            head: 0,
            tail: 0,
            size: 0,
        }
    }
}

impl<T> FromIterator<T> for CircularBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let capacity = iter.size_hint().0;
        let mut buffer = Self::new(capacity);
        buffer.extend(iter);

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        assert_eq!(buffer.capacity(), 5);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
    }

    #[test]
    fn test_push_and_pop() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.len(), 3);
        assert!(!buffer.is_empty());
        assert!(buffer.is_full());

        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), None);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(buffer.peek(), Some(&1));
        buffer.push(3);
        assert_eq!(buffer.peek(), Some(&1));

        buffer.pop();
        assert_eq!(buffer.peek(), Some(&2));
        buffer.pop();
        assert_eq!(buffer.peek(), Some(&3));
        buffer.pop();
        assert_eq!(buffer.peek(), None);
    }

    #[test]
    fn test_clear() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.clear();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
    }

    #[test]
    fn test_extend() {
        let mut buffer = CircularBuffer::new(5);
        buffer.extend(vec![1, 2, 3]);
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), None);

        buffer.extend(vec![4, 5, 6, 7]);
        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.pop(), Some(4));
        assert_eq!(buffer.pop(), Some(5));
        assert_eq!(buffer.pop(), Some(6));
        assert_eq!(buffer.pop(), Some(7));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_iter() {
        let mut buffer = CircularBuffer::new(5);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        let collected: Vec<&i32> = buffer.iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);

        buffer.push(4);
        buffer.push(5);
        let collected: Vec<&i32> = buffer.iter().collect();
        assert_eq!(collected, vec![&3, &4, &5]);
    }

    #[test]
    fn test_from_iterator() {
        let mut buffer: CircularBuffer<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(buffer.capacity(), 5);
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), Some(4));
        assert_eq!(buffer.pop(), Some(5));
        assert_eq!(buffer.pop(), None);
    }
}
