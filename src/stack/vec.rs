use std::iter::FromIterator;
use std::hash::Hash;
use std::fmt;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct VecStack<T>(Vec<T>);

impl<T> super::Stack<T> for VecStack<T>
where
    T: PartialOrd + fmt::Display + Default + Hash + Clone + Send + 'static
{
    fn push(&mut self, n: T) {
        self.0.insert(0, n)
    }

    fn pop(&mut self) -> Option<T> {
        match self.0.is_empty() {
            true => None,
            false => Some(self.0.remove(0))
        }
    }

    fn swap(&mut self) {
        let mut it = self.0.iter_mut();

        if let (Some(f), Some(s)) = (it.next(), it.next()) {
            ::std::mem::swap(f, s)
        }
    }

    fn rotate(&mut self) {
        if !self.0.is_empty() {
            let front = self.0.remove(0);
            self.0.push(front)
        }
    }

    fn rrotate(&mut self) {
        if let Some(back) = self.0.pop() {
            self.0.insert(0, back)
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_sorted(&self) -> bool {
        self.0.iter().zip(self.0.iter().skip(1))
            .all(|(a, b)| a <= b)
    }

    fn sorted_at(&self) -> Option<usize> {
        let mut pivot = None;

        let lefts = self.0.iter();
        let rights = self.0.iter().skip(1).chain(self.0.iter().take(1));
        let indexes = 1..;

        for ((a, b), i) in lefts.zip(rights).zip(indexes) {
            if a > b {
                if pivot.is_some() { return None }
                else { pivot = Some(i) }
            }
        }

        pivot
    }

    fn minimum(&self) -> Option<(&T, usize)> {
        self.0.iter().zip(0..).fold(None, |min, (e, i)| {
            match min {
                Some((p, _)) if e > p => min,
                _                     => Some((e, i))
            }
        })
    }

    fn rotate_n(&mut self, n: usize)  {
        let mut high = self.0.split_off(n);
        ::std::mem::swap(&mut self.0, &mut high);
        self.0.append(&mut high)
    }
}

impl<T> FromIterator<T> for VecStack<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>
    {
        VecStack(Vec::from_iter(iter))
    }
}

impl<T: fmt::Display + PartialOrd> fmt::Display for VecStack<T> {
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
