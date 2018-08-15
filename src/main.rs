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

    let Options { stack_type, command } = Options::from_args();

    let run_command = match stack_type {
        LinkedList => run_with_stack_type::<LLStack<N>>,
        VecDeque   => run_with_stack_type::<VDStack<N>>,
        Vec        => run_with_stack_type::<VecStack<N>>,
    };

    run_command(command)
}

fn run_with_stack_type<S: Stack<N>>(command: Command) {
    match command {
        Command::Check(config) => check::<S>(config),
        Command::Solve(config) => solve::<S>(config),
    }
}
