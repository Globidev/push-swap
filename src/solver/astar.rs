use super::*;

use std::collections::{VecDeque, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

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

#[derive(Default, Debug, Clone)]
struct Node<S> {
    pub a: S,
    pub b: S,
    instrs: VecDeque<Instruction>
}

impl<S: Stack<N>> PartialEq for Node<S> {
    fn eq(&self, other: &Node<S>) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl<S: Stack<N>> Hash for Node<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}

impl<S: Stack<N>> Eq for Node<S> {}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

fn neighbors<S: Stack<N>>(node: Node<S>) -> impl Iterator<Item = Node<S>> {
    use self::Instruction::*;
    type ValidateInstruction = fn(usize, usize, &Instruction) -> bool;

    const INSTRS: [(Instruction, ValidateInstruction); 11] = [
        (RotateBoth, |a_len, b_len, instr|
            a_len >= 2 && b_len >= 2
                && !instr_among(instr, &[RRotateA, RRotateB, RRotateBoth, RotateA, RotateB])
        ),
        (RotateA, |a_len, _, instr|
            a_len >= 2 && !instr_among(instr, &[RRotateA, RRotateB, RRotateBoth])
        ),
        (RotateB, |_, b_len, instr|
            b_len >= 2 && !instr_among(instr, &[RRotateA, RRotateB, RRotateBoth])
        ),
        (RRotateBoth, |a_len, b_len, instr|
            a_len >= 2 && b_len >= 2
                && !instr_among(instr, &[RotateA, RotateB, RotateBoth, RRotateA, RRotateB])
        ),
        (RRotateA, |a_len, _, instr|
            a_len >= 2 && !instr_among(instr, &[RotateA, RotateB, RotateBoth])
        ),
        (RRotateB, |_, b_len, instr|
            b_len >= 2 && !instr_among(instr, &[RotateA, RotateB, RotateBoth])
        ),
        (SwapBoth, |a_len, b_len, instr|
            a_len >= 2 && b_len >= 2 && !instr_among(instr, &[SwapA, SwapB, SwapBoth])
        ),
        (SwapA, |a_len, _, instr|
            a_len >= 2 && !instr_among(instr, &[SwapA, SwapB, SwapBoth])
        ),
        (SwapB, |_, b_len, instr|
            b_len >= 2 && !instr_among(instr, &[SwapA, SwapB, SwapBoth])
        ),
        (PushA, |_, b_len, instr|
            b_len > 0 && instr != &PushB
        ),
        (PushB, |a_len, _, instr|
            a_len >= 2 && instr != &PushA
        ),
    ];

    let a_len = node.a.len();
    let b_len = node.b.len();
    let last_instr = node.instrs.back().cloned().unwrap_or(PushB);

    INSTRS.iter()
        .filter(move |(_, valid_instr)| valid_instr(a_len, b_len, &last_instr))
        .map(move |(instr, _)| transform_instr(instr, &node))
}

fn instr_among(instr: &Instruction, set: &[Instruction]) -> bool {
    for set_instr in set {
        if instr == set_instr {
            return true
        }
    }
    false
}

fn transform_instr<S: Stack<N>>(instr: &Instruction, n: &Node<S>) -> Node<S> {
    let mut node = n.clone();

    node.instrs.push_back(instr.clone());

    execute(instr, &mut node.a, &mut node.b);

    node
}
