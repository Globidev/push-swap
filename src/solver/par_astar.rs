extern crate crossbeam_deque;
extern crate smallvec;

use super::*;
use super::utils::*;

use self::crossbeam_deque::{fifo as work_steal_fifo, Steal, Stealer};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use std::collections::{VecDeque, HashSet, vec_deque::IntoIter};

pub fn par_astar<S: Stack<N>>(extra_worker_count: usize)
    -> impl FnOnce(S) -> IntoIter<Instruction>
{
    move |stack| solve(extra_worker_count, stack).into_iter()
}

type ClosedSet = Arc<RwLock<HashSet<u64>>>;
// type Work<S> = smallvec::SmallVec<[(Node<S>, u64); 10]>;
type Work<S> = Vec<(Node<S>, u64)>;
type WorkSender<S> = mpsc::Sender<Work<S>>;

fn solve(extra_worker_count: usize, stack: impl Stack<N>)
    -> VecDeque<Instruction>
{
    let (open_set_worker, open_set_stealer) = work_steal_fifo();
    let (neighbors_tx, neighbors_rx) = mpsc::channel();
    let closed_set = ClosedSet::default();

    open_set_worker.push(Node { a: stack, ..Default::default() });
    let mut open_set_size = 1;

    // Spawning the work stealers (at least 1 + extras)
    for _ in 0..extra_worker_count {
        thread::spawn({
            let thread_set = closed_set.clone();
            let thread_stealer = open_set_stealer.clone();
            let thread_tx = neighbors_tx.clone();
            move || compute_neighbors(thread_set, thread_stealer, thread_tx)
        });
    }

    thread::spawn({
        let thread_set = closed_set.clone();
        move || compute_neighbors(thread_set, open_set_stealer, neighbors_tx)
    });

    // Process each batch of computed neighbors in the main thread
    // Buffer them to prevent excessive write locking on the closed set
    const NODE_BUFFER_SIZE: usize = 512;

    let mut buff_nodes = Vec::with_capacity(NODE_BUFFER_SIZE);
    let mut buff_hashes = Vec::with_capacity(NODE_BUFFER_SIZE);

    while let Ok(nodes) = neighbors_rx.recv() {
        open_set_size -= 1;

        // Check for end condition and buffer nodes
        for (node, hash) in nodes {
            if node.b.len() == 0 && node.a.is_sorted() {
                return node.instrs
            }
            buff_nodes.push(node);
            buff_hashes.push(hash);
        }

        // Process the buffer if there is no more work queued or if the buffer
        // is about to spill:
        // current len + (maximum neighbor count =~ 16) > buffer size
        if open_set_size == 0 || buff_nodes.len() + 16 > NODE_BUFFER_SIZE {
            closed_set.write().unwrap()
                .extend(buff_hashes.drain(..));

            open_set_size += buff_nodes.len();

            buff_nodes.drain(..).for_each(|node| {
                open_set_worker.push(node);
            })
        }
    }

    unreachable!("Stacks are always solvable")
}

fn compute_neighbors<S>(closed_set: ClosedSet, stealer: Stealer<Node<S>>, tx: WorkSender<S>)
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
