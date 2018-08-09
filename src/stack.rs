use std::collections::LinkedList;
use std::iter::FromIterator;
use std::fmt;

#[derive(Debug, Default)]
pub struct Stack<T>(LinkedList<T>);

impl<T> Stack<T> {
    pub fn push(&mut self, n: T) {
        self.0.push_front(n);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn swap(&mut self) {
        let mut it = self.0.iter_mut();

        if let (Some(f), Some(s)) = (it.next(), it.next()) {
            ::std::mem::swap(f, s)
        }
    }

    pub fn rotate(&mut self) {
        if let Some(front) = self.0.pop_front() {
            self.0.push_back(front)
        }
    }

    pub fn reverse_rotate(&mut self) {
        if let Some(back) = self.0.pop_back() {
            self.0.push_front(back)
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn raw(&self) -> &LinkedList<T> {
        &self.0
    }

    pub fn rotate_n(&mut self, n: usize)  {
        let mut high = self.0.split_off(n);
        ::std::mem::swap(&mut self.0, &mut high);
        self.0.append(&mut high)
    }

    pub fn reverse_rotate_n(&mut self, n: usize)  {
        let len = self.len();
        self.rotate_n(len - n)
    }
}

impl<T: PartialOrd> Stack<T> {
    pub fn is_sorted(&self) -> bool {
        self.0.iter().zip(self.0.iter().skip(1))
            .all(|(a, b)| a <= b)
    }

    pub fn minimum(&self) -> Option<(&T, usize)> {
        self.0.iter().zip(0..).fold(None, |min, (e, i)| {
            match min {
                Some((p, _)) if e < p => Some((e, i)),
                None                  => Some((e, i)),
                _                      => min
            }
        })
    }

    // pub fn maximum(&self) -> Option<(&T, usize)> {
    //     self.0.iter().zip(0..).fold(None, |min, (e, i)| {
    //         match min {
    //             Some((p, _)) if e > p => Some((e, i)),
    //             _                      => min
    //         }
    //     })
    // }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>
    {
        Stack(LinkedList::from_iter(iter))
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = self.0.iter();

        if let Some(n) = it.next() {
            write!(f, "{}", n)?;
            for n in it {
                write!(f, " {}", n)?
            }
        }

        Ok(())
    }
}
