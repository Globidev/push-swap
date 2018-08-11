use std::iter::FromIterator;
use std::fmt::Display;
use std::hash::Hash;

pub mod linked_list;

pub trait Stack<T: PartialOrd>:
    Default + Clone + PartialEq +
    Display + Hash + FromIterator<T>
{
    // Main problem operations
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
    fn swap(&mut self);
    fn rotate(&mut self);
    fn rrotate(&mut self);

    // Helpers
    fn len(&self) -> usize;
    fn is_sorted(&self) -> bool;
    fn sorted_at(&self) -> Option<usize>;
    fn minimum(&self) -> Option<(&T, usize)>;

    fn rotate_n(&mut self, n: usize);
    fn rrotate_n(&mut self, n: usize) {
        let len = self.len();
        self.rotate_n(len - n)
    }

}
