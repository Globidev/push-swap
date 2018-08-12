extern crate crossbeam_deque;
extern crate smallvec;

use super::*;

use self::crossbeam_deque::{self as deque, Steal, Stealer};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use std::collections::{VecDeque, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

pub struct ParallelAstar {
    moves: VecDeque<Instruction>
}

impl ParallelAstar {
    pub fn new(n_threads: usize, stack: impl Stack<N>) -> Self {
        // We have the main worker thread + at least 1 stealer so the
        // amount of extra workers we can get is max(0, n_threads - 2)
        let extra_worker_count = n_threads.saturating_sub(2);

        Self {
            moves: solve(extra_worker_count, stack)
        }
    }
}

impl Iterator for ParallelAstar {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.moves.pop_front()
    }
}

type ClosedSet = Arc<RwLock<HashSet<u64>>>;
type Work<S> = smallvec::SmallVec<[(Node<S>, u64); 10]>;
type WorkSender<S> = mpsc::Sender<Work<S>>;

fn steal_work<S>(closed_set: ClosedSet, stealer: Stealer<Node<S>>, tx: WorkSender<S>)
where
    S: Stack<N>
{
    loop {
        if let Steal::Data(node) = stealer.steal() {
            let valid_neighbors = neighbors(node)
                .map(|n| { let h = hash(&n); (n, h) })
                .filter(|(_, h)| !closed_set.read().unwrap().contains(&h));

            if let Err(_) = tx.send(valid_neighbors.collect()) {
                return
            }
        }
    }
}

fn solve(extra_worker_count: usize, stack: impl Stack<N>)
    -> VecDeque<Instruction>
{
    let (worker, stealer) = deque::fifo();
    let (tx, rx) = mpsc::channel();
    let closed_set = ClosedSet::default();

    worker.push(Node { a: stack, ..Default::default() });

    for _ in 0..extra_worker_count {
        thread::spawn({
            let thread_set = closed_set.clone();
            let thread_stealer = stealer.clone();
            let thread_tx = tx.clone();
            move || steal_work(thread_set, thread_stealer, thread_tx)
        });
    }

    thread::spawn({
        let thread_set = closed_set.clone();
        move || steal_work(thread_set, stealer, tx)
    });

    let mut buff_nodes = Vec::with_capacity(512);
    let mut buff_hashes = Vec::with_capacity(512);
    let mut pushed = 1;

    while let Ok(nodes) = rx.recv() {
        pushed -= 1;

        for (node, h) in nodes {
            if node.b.len() == 0 && node.a.is_sorted() {
                return node.instrs.clone()
            }
            buff_nodes.push(node);
            buff_hashes.push(h);
        }

        if pushed == 0 || buff_nodes.len() >= 512 - 16 {
            match closed_set.write().unwrap() {
                mut closed_set => closed_set.extend(buff_hashes.drain(..))
            }
            pushed += buff_nodes.len();
            buff_nodes.drain(..).for_each(|node| {
                worker.push(node);
            })
        }
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
