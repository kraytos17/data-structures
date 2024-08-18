#[derive(Debug)]
pub struct BitArray {
    bits: Vec<u64>,
    size: usize,
}

impl BitArray {
    pub fn new(size: usize) -> Self {
        let elem_num = (size + 63) / 64;
        Self {
            bits: vec![0; elem_num],
            size,
        }
    }

    #[inline(always)]
    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        if idx >= self.size {
            panic!("Index out of bounds");
        }

        (idx / 64, idx % 64)
    }

    pub fn set(&mut self, idx: usize) {
        let (block, offset) = self.idx_to_pos(idx);
        self.bits[block] |= 1 << offset;
    }

    pub fn get(&self, idx: usize) -> bool {
        let (block, offset) = self.idx_to_pos(idx);
        (self.bits[block] & (1 << offset)) != 0
    }

    pub fn clear(&mut self, idx: usize) {
        let (block, offset) = self.idx_to_pos(idx);
        self.bits[block] &= !(1 << offset);
    }

    pub fn toggle(&mut self, idx: usize) {
        let (block, offset) = self.idx_to_pos(idx);
        self.bits[block] ^= 1 << offset;
    }

    pub fn invert(&mut self) {
        for block in &mut self.bits {
            *block = !*block;
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        let mut bits = Vec::with_capacity(self.bits.len());
        for i in 0..self.bits.len() {
            bits.push(self.bits[i] | other.bits[i]);
        }

        Self {
            bits,
            size: self.size,
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        let mut bits = Vec::with_capacity(self.bits.len());
        for i in 0..self.bits.len() {
            bits.push(self.bits[i] & other.bits[i]);
        }

        Self {
            bits,
            size: self.size,
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        let mut bits = Vec::with_capacity(self.bits.len());
        for i in 0..self.bits.len() {
            bits.push(self.bits[i] & !other.bits[i]);
        }

        Self {
            bits,
            size: self.size,
        }
    }

    pub fn complement(&self) -> Self {
        let mut bits = Vec::with_capacity(self.bits.len());
        for i in 0..self.bits.len() {
            bits.push(!self.bits[i]);
        }

        Self {
            bits,
            size: self.size,
        }
    }

    pub fn iter_bits<F>(&self, mut f: F)
    where
        F: FnMut(usize, bool),
    {
        for (i, &word) in self.bits.iter().enumerate() {
            let mut word = word;
            let base_idx = i * 64;
            for bit in 0..64 {
                let lsb = (word & 1) != 0;
                let bit_idx = base_idx + bit;
                if bit_idx < self.size {
                    f(bit_idx, lsb);
                } else {
                    break;
                }

                word >>= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bit_array = BitArray::new(100);
        assert_eq!(bit_array.size, 100);
        assert_eq!(bit_array.bits.len(), (100 + 63) / 64);
    }

    #[test]
    fn test_set_and_get() {
        let mut bit_array = BitArray::new(100);
        bit_array.set(10);
        assert!(bit_array.get(10));
        assert!(!bit_array.get(11));
    }

    #[test]
    fn test_clear() {
        let mut bit_array = BitArray::new(100);
        bit_array.set(10);
        bit_array.clear(10);
        assert!(!bit_array.get(10));
    }

    #[test]
    fn test_toggle() {
        let mut bit_array = BitArray::new(100);
        bit_array.toggle(10);
        assert!(bit_array.get(10));
        bit_array.toggle(10);
        assert!(!bit_array.get(10));
    }

    #[test]
    fn test_invert() {
        let mut bit_array = BitArray::new(64);
        bit_array.set(0);
        bit_array.invert();
        assert!(!bit_array.get(0));
        assert!(bit_array.get(1));
    }

    #[test]
    fn test_union() {
        let mut a = BitArray::new(64);
        let mut b = BitArray::new(64);
        a.set(0);
        b.set(1);
        let c = a.union(&b);
        assert!(c.get(0));
        assert!(c.get(1));
        assert!(!c.get(2));
    }

    #[test]
    fn test_intersection() {
        let mut a = BitArray::new(64);
        let mut b = BitArray::new(64);
        a.set(0);
        a.set(1);
        b.set(1);
        let c = a.intersection(&b);
        assert!(!c.get(0));
        assert!(c.get(1));
    }

    #[test]
    fn test_difference() {
        let mut a = BitArray::new(64);
        let mut b = BitArray::new(64);
        a.set(0);
        a.set(1);
        b.set(1);
        let c = a.difference(&b);
        assert!(c.get(0));
        assert!(!c.get(1));
    }

    #[test]
    fn test_complement() {
        let mut a = BitArray::new(64);
        a.set(0);
        let b = a.complement();
        assert!(!b.get(0));
        assert!(b.get(1));
    }

    #[test]
    fn test_iter_bits() {
        let mut bit_array = BitArray::new(64);
        bit_array.set(0);
        bit_array.set(1);
        let mut bits: Vec<(usize, bool)> = Vec::new();
        bit_array.iter_bits(|idx, value| bits.push((idx, value)));
        assert_eq!(bits.len(), 64);
        assert_eq!(bits[0], (0, true));
        assert_eq!(bits[1], (1, true));
        assert_eq!(bits[2], (2, false));
    }

    #[test]
    fn test_empty_bit_array() {
        let bit_array = BitArray::new(0);
        assert_eq!(bit_array.size, 0);
        assert!(bit_array.bits.is_empty());
    }

    #[test]
    fn test_large_bit_array() {
        let mut bit_array = BitArray::new(1_000_000);
        bit_array.set(999_999);
        assert!(bit_array.get(999_999));
        bit_array.clear(999_999);
        assert!(!bit_array.get(999_999));
    }

    #[test]
    fn test_boundary_conditions() {
        let mut bit_array = BitArray::new(128);
        bit_array.set(127);
        assert!(bit_array.get(127));
        bit_array.clear(127);
        assert!(!bit_array.get(127));
    }
}
