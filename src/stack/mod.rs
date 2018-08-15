use std::iter::FromIterator;
use std::fmt::Display;
use std::hash::Hash;

pub trait Stack<T: PartialOrd>:
    Default + Clone + PartialEq +
    Display + Hash + FromIterator<T> +
    Send + 'static
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
    fn maximum(&self) -> Option<(&T, usize)>;

    fn rotate_n(&mut self, n: usize);

    fn insert_index(&self, t: &T) -> Option<usize>;
    fn peek(&self, n: isize) -> &T;
}

mod linked_list;
mod vec_deque;
mod vec;

pub use self::linked_list::LLStack;
pub use self::vec_deque::VDStack;
pub use self::vec::VecStack;
