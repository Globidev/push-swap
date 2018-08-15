#![feature(generators, generator_trait)]
#![feature(iterator_find_map)]

#[macro_use] extern crate structopt;

mod options;
mod stack;
mod instruction;
mod checker;
mod solver;
mod utils;

use options::*;
use checker::check;
use stack::*;
use solver::solve;
use utils::N;

fn main() {
    use StackType::*;
    use structopt::StructOpt;

    let opts = Options::from_args();

    match opts.stack_type {
        LinkedList => run_with_stack_type::<LLStack<N>>(opts.command),
        VecDeque   => run_with_stack_type::<VDStack<N>>(opts.command),
        Vec        => run_with_stack_type::<VecStack<N>>(opts.command),
    }
}

fn run_with_stack_type<S: Stack<N>>(command: Command) {
    match command {
        Command::Check(config) => check::<S>(config),
        Command::Solve(config) => solve::<S>(config),
    }
}
