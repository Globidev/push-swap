use super::*;

use std::collections::{VecDeque, HashSet};
use std::hash::{Hash, Hasher};

pub struct Astar {
    moves: Vec<Instruction>
}

impl Astar {
    pub fn new(stack: Stack<u32>) -> Self {
        Astar { moves: solve(stack).into_iter().rev().collect() }
    }
}

impl Iterator for Astar {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.moves.pop()
    }
}

#[derive(Debug, Clone)]
struct Node {
    pub a: Stack<u32>,
    pub b: Stack<u32>,
    instrs: Vec<Instruction>
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}

impl Eq for Node {}

fn solve(stack: Stack<u32>) -> Vec<Instruction> {
    let mut open_set = VecDeque::new();
    let mut closed_set = HashSet::new();
    open_set.push_back(Node { a: stack, b: Default::default(), instrs: Vec::new() });

    while !open_set.is_empty() {
        let node = open_set.pop_front().unwrap();

        if node.a.is_sorted() && node.b.len() == 0 {
            return node.instrs
        }

        closed_set.insert(node.clone());

        for neighbor in neighbors(node) {
            if closed_set.contains(&neighbor) {
                continue
            }

            open_set.push_back(neighbor)
        }
    }

    Vec::new()
}

fn neighbors(node: Node) -> impl Iterator<Item = Node> {
    use self::Instruction::*;

    const INSTRS: [Instruction; 11] = [
        PushA, PushB,
        SwapA, SwapB, SwapBoth,
        RotateA, RotateB, RotateBoth,
        ReverseRotateA, ReverseRotateB, ReverseRotateBoth,
    ];

    INSTRS.iter()
        .filter_map(move |i|
            match valid_instr(i, &node) {
                true  => Some(transform_instr(i, &node)),
                false => None
            }
        )
}

fn valid_instr(instr: &Instruction, node: &Node) -> bool {
    use self::Instruction::*;

    let last_instr = node.instrs.last();
    let a_len = node.a.len();
    let b_len = node.b.len();

    match instr {
        PushA => {
            let b_non_empty = b_len > 0;
            let last_not_push_b = last_instr
                .map(|i| i != &PushB)
                .unwrap_or(true);
            b_non_empty && last_not_push_b
        },
        PushB => {
            let a_non_empty = a_len > 0;
            let last_not_push_a = last_instr
                .map(|i| i != &PushA)
                .unwrap_or(true);
            a_non_empty && last_not_push_a
        },

        SwapA => {
            let last_not_swap = last_instr.map(|i| i != &SwapA && i != &SwapB && i != &SwapBoth).unwrap_or(true);
            let at_least_2 = a_len >= 2;
            last_not_swap && at_least_2
        },
        SwapB => {
            let last_not_swap = last_instr.map(|i| i != &SwapA && i != &SwapB && i != &SwapBoth).unwrap_or(true);
            let at_least_2 = b_len >= 2;
            last_not_swap && at_least_2
        },
        SwapBoth => {
            let last_not_swap = last_instr.map(|i| i != &SwapA && i != &SwapB && i != &SwapBoth).unwrap_or(true);
            let at_least_2 = a_len >= 2 && b_len >= 2;
            last_not_swap && at_least_2
        },

        RotateA => {
            let last_not_rotate = last_instr.map(|i| i != &ReverseRotateA && i != &ReverseRotateB && i != &ReverseRotateBoth).unwrap_or(true);
            let at_least_2 = a_len >= 2;
            last_not_rotate && at_least_2
        },
        RotateB => {
            let last_not_rotate = last_instr.map(|i| i != &ReverseRotateA && i != &ReverseRotateB && i != &ReverseRotateBoth).unwrap_or(true);
            let at_least_2 = b_len >= 2;
            last_not_rotate && at_least_2
        },
        RotateBoth => {
            let last_not_rotate = last_instr.map(|i|
                i != &ReverseRotateA &&
                i != &ReverseRotateB &&
                i != &ReverseRotateBoth &&
                i != &RotateA &&
                i != &RotateB
            ).unwrap_or(true);
            let at_least_2 = a_len >= 2 && b_len >= 2;
            last_not_rotate && at_least_2
        },

        ReverseRotateA => {
            let last_not_rotate = last_instr.map(|i| i != &RotateA && i != &RotateB && i != &RotateBoth).unwrap_or(true);
            let at_least_2 = a_len >= 2;
            last_not_rotate && at_least_2
        },
        ReverseRotateB => {
            let last_not_rotate = last_instr.map(|i| i != &RotateA && i != &RotateB && i != &RotateBoth).unwrap_or(true);
            let at_least_2 = b_len >= 2;
            last_not_rotate && at_least_2
        },
        ReverseRotateBoth => {
            let last_not_rotate = last_instr.map(|i|
                i != &RotateA &&
                i != &RotateB &&
                i != &RotateBoth &&
                i != &ReverseRotateA &&
                i != &ReverseRotateB
            ).unwrap_or(true);
            let at_least_2 = a_len >= 2 && b_len >= 2;
            last_not_rotate && at_least_2
        },
    }
}

fn transform_instr(instr: &Instruction, n: &Node) -> Node {
    use self::Instruction::*;

    let mut node = n.clone();
    node.instrs.push(instr.clone());

    match instr {
        PushA  => if let Some(x) = node.b.pop() { node.a.push(x) },
        PushB  => if let Some(x) = node.a.pop() { node.b.push(x) },

        SwapA    => node.a.swap(),
        SwapB    => node.b.swap(),
        SwapBoth => { node.a.swap(); node.b.swap() },

        RotateA    => node.a.rotate(),
        RotateB    => node.b.rotate(),
        RotateBoth => { node.a.rotate(); node.b.rotate() },

        ReverseRotateA    => node.a.reverse_rotate(),
        ReverseRotateB    => node.b.reverse_rotate(),
        ReverseRotateBoth => { node.a.reverse_rotate(); node.b.reverse_rotate(); }
    }

    node
}
