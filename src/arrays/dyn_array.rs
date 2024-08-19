#[derive(Debug)]
pub struct DynamicArray<T, const N: usize> {
    buffer: Box<[T]>,
    len: usize,
    capacity: usize,
}

impl<T: Default + Clone + Copy, const N: usize> DynamicArray<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: vec![T::default(); N].into_boxed_slice(),
            len: 0,
            capacity: N,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.buffer.len() {
            self.grow();
        }

        self.buffer[self.len] = val;
        self.len += 1;
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            Some(&self.buffer[idx])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(std::mem::take(&mut self.buffer[self.len]))
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn grow(&mut self) {
        self.capacity *= 2;
        let mut new_buf = vec![T::default(); self.capacity].into_boxed_slice();
        new_buf[..self.len].copy_from_slice(&self.buffer[..self.len]);
        self.buffer = new_buf;
    }
}

impl<T: Default + Clone + Copy, const N: usize> Default for DynamicArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr: DynamicArray<i32, 5> = DynamicArray::new();
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 5);
    }

    #[test]
    fn test_push() {
        let mut arr: DynamicArray<i32, 2> = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);

        assert_eq!(arr.len(), 3);
        assert_eq!(arr.capacity(), 4);
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), Some(&3));
    }

    #[test]
    fn test_get() {
        let mut arr: DynamicArray<i32, 5> = DynamicArray::new();
        arr.push(10);
        arr.push(20);

        assert_eq!(arr.get(0), Some(&10));
        assert_eq!(arr.get(1), Some(&20));
        assert_eq!(arr.get(2), None);
    }

    #[test]
    fn test_pop() {
        let mut arr: DynamicArray<i32, 5> = DynamicArray::new();
        arr.push(1);
        arr.push(2);

        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.pop(), None);
    }

    #[test]
    fn test_grow() {
        let mut arr: DynamicArray<i32, 2> = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        assert_eq!(arr.capacity(), 2);
        arr.push(3);
        assert_eq!(arr.capacity(), 4);
        arr.push(4);
        arr.push(5);
        assert_eq!(arr.capacity(), 8);
    }

    #[test]
    fn test_default() {
        let arr: DynamicArray<i32, 10> = DynamicArray::default();
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 10);
    }

    #[test]
    fn test_push_pop_many() {
        let mut arr: DynamicArray<i32, 1> = DynamicArray::new();
        for i in 0..1000 {
            arr.push(i);
        }
        assert_eq!(arr.len(), 1000);
        for i in (0..1000).rev() {
            assert_eq!(arr.pop(), Some(i));
        }
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_get_out_of_bounds() {
        let mut arr: DynamicArray<i32, 5> = DynamicArray::new();
        arr.push(1);
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), None);
    }
}
