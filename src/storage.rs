mod bitvector;
mod vector;

pub use self::{bitvector::BitVector, vector::Vector};

pub trait Storage {
    fn new(size: usize) -> Self;
    fn get(&self, index: usize) -> bool;
    fn set(&mut self, index: usize, value: bool);
    fn switch(&mut self);
}
