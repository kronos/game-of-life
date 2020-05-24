mod vector;

pub use self::{
    vector::Vector,
};

pub trait Storage {
    fn new(size: usize) -> Self;
    fn get(&self, index: usize) -> bool;
    fn set(&mut self, index: usize, value: bool);
    fn switch(&mut self);
}