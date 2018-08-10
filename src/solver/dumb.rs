use std::iter::{repeat, once};

use super::*;

pub struct DumbSolver<S> {
    stack: S,
    pushed: usize,
    state: DumbSolverState
}

enum DumbSolverState {
    Done(Vec<Instruction>),
    Rotate(Vec<Instruction>),
    Looping,
}

impl<S> DumbSolver<S> {
    pub fn new(stack: S) -> Self {
        Self { stack, pushed: 0, state: DumbSolverState::Looping }
    }
}

impl<S> Iterator for DumbSolver<S>
where
    S: Stack<N>
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        use self::DumbSolverState::*;
        use self::Instruction::*;

        let next_state = match self.state {
            Done(ref mut leftover)   => return leftover.pop(),
            Rotate(ref mut leftover) => match leftover.pop() {
                item@Some(_) => return item,
                _            => Looping
            },
            Looping => match self.stack.minimum() {
                None => Done(repeat_n(PushA, self.pushed).collect()),
                Some((_, min_idx)) => match self.stack.sorted_at() {
                    Some(i) => {
                        let (instr, n) = rotation(&self.stack, i);
                        let leftover_instrs = repeat_n(PushA, self.pushed)
                            .chain(repeat_n(instr, n));
                        Done(leftover_instrs.collect())
                    },
                    None => {
                        let (instr, n) = rotation(&self.stack, min_idx);

                        match instr {
                            RotateA => self.stack.rotate_n(n),
                            ReverseRotateA => self.stack.reverse_rotate_n(n),
                            _ => ()
                        };

                        self.stack.pop().unwrap_or_default();
                        let rotations_and_push = once(PushB)
                            .chain(repeat_n(instr, n));
                        self.pushed += 1;
                        Rotate(rotations_and_push.collect())
                    }
                }
            }
        };

        self.state = next_state;
        self.next()
    }
}

fn repeat_n<T: Clone>(t: T, n: usize) -> impl Iterator<Item = T> {
    repeat(t).take(n)
}

fn rotation(stack: &impl Stack<N>, at: usize) -> (Instruction, usize) {
    use std::cmp::Ordering::Less;

    let mid = stack.len() / 2;

    match at.cmp(&mid) {
        Less => (Instruction::RotateA, at),
        _    => (Instruction::ReverseRotateA, stack.len() - at),
    }
}
