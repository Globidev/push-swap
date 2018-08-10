use super::*;

use std::collections::{VecDeque, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

pub struct Astar {
    moves: Vec<Instruction>
}

impl Astar {
    pub fn new(stack: impl Stack<N>) -> Self {
        let moves = solve(stack)
            .into_iter()
            .rev()
            .collect();

        Self { moves }
    }
}

impl Iterator for Astar {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.moves.pop()
    }
}

fn solve(stack: impl Stack<N>) -> Vec<Instruction> {
    let mut open_set = VecDeque::new();
    let mut closed_set = HashSet::new();

    open_set.push_back(Node {
        a: stack,
        ..Default::default()
    });

    while let Some(node) = open_set.pop_front() {
        if node.a.is_sorted() && node.b.len() == 0 {
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
    instrs: Vec<Instruction>
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

    const INSTRS: [Instruction; 11] = [
        ReverseRotateBoth, ReverseRotateA, ReverseRotateB,
        RotateBoth, RotateA, RotateB,
        SwapBoth, SwapA, SwapB,
        PushA, PushB,
    ];

    INSTRS.iter()
        .filter_map(move |i|
            match valid_instr(i, &node) {
                true  => Some(transform_instr(i, &node)),
                false => None
            }
        )
}

fn valid_instr<S: Stack<N>>(instr: &Instruction, node: &Node<S>) -> bool {
    use self::Instruction::*;

    let last_instr = node.instrs.last().unwrap_or(&PushB);
    let a_len = node.a.len();
    let b_len = node.b.len();

    match instr {
        PushA => {
            let b_non_empty = b_len > 0;
            let last_not_push_b = last_instr != &PushB;
            b_non_empty && last_not_push_b
        },
        PushB => {
            let a_at_least_2 = a_len >= 2;
            let last_not_push_a = last_instr != &PushA;
            a_at_least_2 && last_not_push_a
        },

        SwapA => {
            let last_not_swap =
                last_instr != &SwapA &&
                last_instr != &SwapB &&
                last_instr != &SwapBoth;
            let at_least_2 = a_len >= 2;
            last_not_swap && at_least_2
        },
        SwapB => {
            let last_not_swap =
                last_instr != &SwapA &&
                last_instr != &SwapB &&
                last_instr != &SwapBoth;
            let at_least_2 = b_len >= 2;
            last_not_swap && at_least_2
        },
        SwapBoth => {
            let last_not_swap =
                last_instr != &SwapA &&
                last_instr != &SwapB &&
                last_instr != &SwapBoth;
            let at_least_2 = a_len >= 2 && b_len >= 2;
            last_not_swap && at_least_2
        },

        RotateA => {
            let last_not_rotate =
                last_instr != &ReverseRotateA &&
                last_instr != &ReverseRotateB &&
                last_instr != &ReverseRotateBoth;
            let at_least_2 = a_len >= 2;
            last_not_rotate && at_least_2
        },
        RotateB => {
            let last_not_rotate =
                last_instr != &ReverseRotateA &&
                last_instr != &ReverseRotateB &&
                last_instr != &ReverseRotateBoth;
            let at_least_2 = b_len >= 2;
            last_not_rotate && at_least_2
        },
        RotateBoth => {
            let last_not_rotate =
                last_instr != &ReverseRotateA &&
                last_instr != &ReverseRotateB &&
                last_instr != &ReverseRotateBoth &&
                last_instr != &RotateA &&
                last_instr != &RotateB;
            let at_least_2 = a_len >= 2 && b_len >= 2;
            last_not_rotate && at_least_2
        },

        ReverseRotateA => {
            let last_not_rotate =
                last_instr != &RotateA &&
                last_instr != &RotateB &&
                last_instr != &RotateBoth;
            let at_least_2 = a_len >= 2;
            last_not_rotate && at_least_2
        },
        ReverseRotateB => {
            let last_not_rotate =
                last_instr != &RotateA &&
                last_instr != &RotateB &&
                last_instr != &RotateBoth;
            let at_least_2 = b_len >= 2;
            last_not_rotate && at_least_2
        },
        ReverseRotateBoth => {
            let last_not_rotate =
                last_instr != &RotateA &&
                last_instr != &RotateB &&
                last_instr != &RotateBoth &&
                last_instr != &ReverseRotateA &&
                last_instr != &ReverseRotateB;
            let at_least_2 = a_len >= 2 && b_len >= 2;
            last_not_rotate && at_least_2
        },
    }
}

fn transform_instr<S: Stack<N>>(instr: &Instruction, n: &Node<S>) -> Node<S> {
    let mut node = n.clone();

    node.instrs.push(instr.clone());

    execute(instr, &mut node.a, &mut node.b);

    node
}
