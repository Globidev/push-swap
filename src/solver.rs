use stack::Stack;
use instruction::Instruction;

use std::iter::{repeat, once};

pub fn solve(stack: Stack<u32>) {
    let instrs = DumbSolver::new(stack);

    instrs.for_each(|i| println!("{}", i));
}

struct DumbSolver {
    stack: Stack<u32>,
    pushed: usize,
    state: DumbSolverState
}

enum DumbSolverState {
    Done(Vec<Instruction>),
    Rotate(Vec<Instruction>),
    Looping,
}

impl DumbSolver {
    pub fn new(stack: Stack<u32>) -> Self {
        Self { stack, pushed: 0, state: DumbSolverState::Looping }
    }
}

impl Iterator for DumbSolver {
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
                Some((_, min_idx)) => match sorted_rot(&self.stack) {
                    Some((instr, n)) => {
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

fn rotation(stack: &Stack<u32>, at: usize) -> (Instruction, usize) {
    use std::cmp::Ordering::Less;

    let mid = stack.len() / 2;

    match at.cmp(&mid) {
        Less => (Instruction::RotateA, at),
        _    => (Instruction::ReverseRotateA, stack.len() - at),
    }
}

fn sorted_rot(stack: &Stack<u32>) -> Option<(Instruction, usize)> {
    let mut pivot = None;
    let raw = stack.raw();

    for ((a, b), i) in raw.iter().zip(raw.iter().skip(1)).zip(1..) {
        if a > b {
            if pivot.is_some() { return None }
            else { pivot = Some(i) }
        }
    }

    pivot.map(|i| rotation(stack, i))
}
