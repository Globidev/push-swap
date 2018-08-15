use super::*;
use super::utils::*;

use std::collections::{VecDeque, HashSet};

pub fn astar(stack: impl Stack<N>) -> impl Iterator<Item = Instruction> {
    let mut open_set = VecDeque::new();
    let mut closed_set = HashSet::new();

    open_set.push_back(Node { a: stack, ..Default::default() });

    while let Some(node) = open_set.pop_front() {
        if node.b.len() == 0 && node.a.is_sorted() {
            return node.instrs.into_iter()
        }

        closed_set.insert(hash(&node));

        let valid_neighbors = neighbors(node)
            .filter(|n| !closed_set.contains(&hash(n)));

        open_set.extend(valid_neighbors);
    }

    unreachable!("Stacks are always solvable")
}
