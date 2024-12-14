#[derive(Debug)]
pub struct CircularBuffer<T, const N: usize> {
    buffer: [T; N],
    head: usize,
    tail: usize,
    size: usize,
}

impl<T: Default + Copy, const N: usize> CircularBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [T::default(); N],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.size == N {
            self.tail = (self.tail + 1) % N;
        } else {
            self.size += 1;
        }

        self.buffer[self.head] = item;
        self.head = (self.head + 1) % N;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let item = self.buffer[self.tail];
        self.tail = (self.tail + 1) % N;
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

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == N
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.size).map(move |i| {
            let index = (self.tail + i) % N;
            &self.buffer[index]
        })
    }
}

impl<T: Default + Copy, const N: usize> Default for CircularBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + Copy, const N: usize> FromIterator<T> for CircularBuffer<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut buffer = Self::new();
        buffer.extend(iter);
        buffer
    }
}

impl<T: Default + Copy, const N: usize> Extend<T> for CircularBuffer<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut buffer = CircularBuffer::<i32, 3>::new();
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_push_overflow() {
        let mut buffer = CircularBuffer::<i32, 3>::new();
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), Some(4));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut buffer = CircularBuffer::<i32, 3>::new();
        buffer.push(1);
        buffer.push(2);

        assert_eq!(buffer.peek(), Some(&1));

        buffer.push(3);
        assert_eq!(buffer.peek(), Some(&1));

        buffer.pop();
        assert_eq!(buffer.peek(), Some(&2));
    }

    #[test]
    fn test_clear() {
        let mut buffer = CircularBuffer::<i32, 3>::new();
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.clear();

        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
    }

    #[test]
    fn test_capacity() {
        let buffer = CircularBuffer::<i32, 5>::new();
        assert_eq!(buffer.capacity(), 5);
    }

    #[test]
    fn test_iter() {
        let mut buffer = CircularBuffer::<i32, 5>::new();
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);
        buffer.push(5);

        let collected: Vec<&i32> = buffer.iter().collect();
        assert_eq!(collected, vec![&1, &2, &3, &4, &5]);

        buffer.pop();
        buffer.push(6);

        let collected: Vec<&i32> = buffer.iter().collect();
        assert_eq!(collected, vec![&2, &3, &4, &5, &6]);
    }

    #[test]
    fn test_from_iterator() {
        let buffer: CircularBuffer<i32, 4> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.iter().cloned().collect::<Vec<i32>>(), vec![1, 2, 3]);

        let buffer: CircularBuffer<i32, 4> = vec![1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(buffer.len(), 4);
        assert_eq!(
            buffer.iter().cloned().collect::<Vec<i32>>(),
            vec![2, 3, 4, 5]
        );
    }
}
