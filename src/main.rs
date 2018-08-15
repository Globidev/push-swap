#![feature(generators, generator_trait)]
#![feature(iterator_find_map)]

#[macro_use] extern crate structopt;
extern crate num_cpus;

mod options;
mod stack;
mod instruction;
mod checker;
mod solver;
mod utils;

use options::*;
use checker::*;
use stack::*;
use solver::*;
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

        Command::Solve { strategy, par_threads, raw_stack } => {
            let stack = raw_stack.into_iter().collect::<S>();

            match strategy {
                SolveStrategy::AStar       => solve(astar, stack),
                SolveStrategy::NaiveInsert => solve(naive_insert, stack),
                SolveStrategy::SmartInsert => solve(smart_insert, stack),
                SolveStrategy::ParAStar    => {
                    let n_threads = par_threads.unwrap_or_else(num_cpus::get);
                    solve(par_astar(n_threads), stack)
                }
            }
        }
    }
}

fn solve<S, Solver, Solution>(solver: Solver, stack: S)
where
    S: Stack<N>,
    Solver: FnOnce(S) -> Solution,
    Solution: Iterator<Item = instruction::Instruction>
{
    use std::fmt::Write;

    let buffer_size = 4096;
    let output_buffer = String::with_capacity(buffer_size);

    let remaining_output = solver(stack)
        .fold(output_buffer, |mut buff, instr| {
            if buff.len() >= buffer_size - 16 {
                print!("{}", buff);
                buff.clear();
            }
            writeln!(&mut buff, "{}", instr).unwrap();
            buff
        });

    print!("{}", remaining_output);
}
