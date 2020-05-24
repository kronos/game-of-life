use crate::storage::Storage;

struct Bits {
    data: Vec<u8>,
}

impl Bits {
    fn new(size: usize) -> Self {
        let mut vector_size = size / 8;
        if size % 8 > 0 {
            vector_size += 1;
        }

        return Bits {
            data: vec![0; vector_size],
        };
    }

    #[inline(always)]
    fn get(&self, index: usize) -> bool {
        let offset = (index % 8) as u8;
        (self.data[index / 8] & (1u8 << offset)) == (1u8 << offset)
    }

    #[inline(always)]
    fn set(&mut self, index: usize, value: bool) {
        let mask = 1u8 << (index % 8) as u8;
        if value {
            self.data[index / 8] |= mask;
        } else {
            self.data[index / 8] &= !mask;
        }
    }
}

pub struct BitVector {
    data: Bits,
    size: usize,
    offset: usize,
}

impl Storage for BitVector {
    fn new(size: usize) -> Self {
        return BitVector {
            data: Bits::new(size * 2),
            offset: 0,
            size,
        };
    }

    fn get(&self, index: usize) -> bool {
        self.data.get(self.offset + index)
    }

    fn set(&mut self, index: usize, value: bool) {
        self.data.set(self.size - self.offset + index, value)
    }

    fn switch(&mut self) {
        self.offset = self.size - self.offset
    }
}
