use std::collections::{VecDeque, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

use super::*;

#[derive(Default, Debug, Clone)]
pub struct Node<S> {
    pub a: S,
    pub b: S,
    pub instrs: VecDeque<Instruction>
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

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

pub fn neighbors<S: Stack<N>>(node: Node<S>) -> impl Iterator<Item = Node<S>> {
    use self::Instruction::*;
    type ValidateInstruction = fn(usize, usize, &Instruction) -> bool;

    // Local helper: deliberately using a raw for loop because inputs will
    // always be statically known and using iterators prevent LLVM from
    // optimizing correctly: https://godbolt.org/g/uZzEKx
    fn instr_among(instr: &Instruction, set: &[Instruction]) -> bool {
        for set_instr in set {
            if instr == set_instr {
                return true
            }
        }
        false
    }

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

fn transform_instr<S: Stack<N>>(instr: &Instruction, n: &Node<S>) -> Node<S> {
    let mut node = n.clone();

    node.instrs.push_back(instr.clone());

    execute(instr, &mut node.a, &mut node.b);

    node
}
