use utils::StackN;
use instruction::Instruction;

use std::iter::{repeat, once};

mod dumb;
mod astar;

pub fn solve(stack: StackN) {
    // let instrs = dumb::DumbSolver::new(stack);
    let instrs = astar::Astar::new(stack);

    instrs.for_each(|i| println!("{}", i));
}

fn repeat_n<T: Clone>(t: T, n: usize) -> impl Iterator<Item = T> {
    repeat(t).take(n)
}

fn rotation(stack: &StackN, at: usize) -> (Instruction, usize) {
    use std::cmp::Ordering::Less;

    let mid = stack.len() / 2;

    match at.cmp(&mid) {
        Less => (Instruction::RotateA, at),
        _    => (Instruction::ReverseRotateA, stack.len() - at),
    }
}

fn sorted_rot(stack: &StackN) -> Option<(Instruction, usize)> {
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
