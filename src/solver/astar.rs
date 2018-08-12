use super::*;
use super::utils::*;

use std::collections::{VecDeque, HashSet};

pub struct Astar {
    moves: VecDeque<Instruction>
}

impl Astar {
    pub fn new(stack: impl Stack<N>) -> Self {
        Self { moves: solve(stack) }
    }
}

impl Iterator for Astar {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.moves.pop_front()
    }
}

fn solve(stack: impl Stack<N>) -> VecDeque<Instruction> {
    let mut open_set = VecDeque::new();
    let mut closed_set = HashSet::new();

    open_set.push_back(Node { a: stack, ..Default::default() });

    while let Some(node) = open_set.pop_front() {
        if node.b.len() == 0 && node.a.is_sorted() {
            return node.instrs
        }

        closed_set.insert(hash(&node));

        let valid_neighbors = neighbors(node)
            .filter(|n| !closed_set.contains(&hash(n)));

        open_set.extend(valid_neighbors);
    }

    unreachable!("Stacks are always solvable")
}
