extern crate num_cpus;

use utils::*;
use stack::Stack;
use instruction::Instruction;
use options::{SolveConfig, SolveStrategy};

mod utils;

mod astar;
mod par_astar;
mod naive_insert;
mod smart_insert;

use self::astar::astar;
use self::naive_insert::naive_insert;
use self::smart_insert::smart_insert;
use self::par_astar::par_astar;

pub fn solve<S: Stack<N>>(config: SolveConfig) {
    let SolveConfig { strategy, par_threads, raw_stack } = config;
    let stack = raw_stack.into_iter().collect::<S>();

    match strategy {
        SolveStrategy::AStar       => solve_with(astar, stack),
        SolveStrategy::NaiveInsert => solve_with(naive_insert, stack),
        SolveStrategy::SmartInsert => solve_with(smart_insert, stack),
        SolveStrategy::ParAStar    => {
            let n_threads = par_threads.unwrap_or_else(num_cpus::get);
            solve_with(par_astar(n_threads), stack)
        }
    }
}

fn solve_with<S, Solver, Solution>(solver: Solver, stack: S)
where
    S: Stack<N>,
    Solver: FnOnce(S) -> Solution,
    Solution: Iterator<Item = Instruction>
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
