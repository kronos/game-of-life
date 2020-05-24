use crate::storage::Storage;

pub struct Vector {
    data: Vec<bool>,
    size: usize,
    offset: usize,
}

impl Storage for Vector {
    fn new(size: usize) -> Self {
        return Vector {
            data: vec![false; size*2],
            offset: 0,
            size
        }
    }

    fn get(&self, index: usize) -> bool {
        self.data[self.offset + index]
    }

    fn set(&mut self, index: usize, value: bool) {
        self.data[self.size - self.offset + index] = value
    }

    fn switch(&mut self) {
        self.offset = self.size - self.offset
    }
}
