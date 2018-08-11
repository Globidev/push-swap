#[macro_use]
extern crate structopt;

mod options;
mod stack;
mod instruction;
mod checker;
mod solver;
mod utils;

use options::{Options, SolveStrategy};
use structopt::StructOpt;
use checker::*;
use stack::linked_list::LLStack;
use solver::{astar, dumb};

fn main() {
    match Options::from_args() {
        Options::Check { debug_states, raw_stack } => {
            let app_config = CheckerConfig {
                stack: raw_stack.into_iter().collect::<LLStack<_>>(),
                debug_states
            };
            check(app_config);
        },

        Options::Solve { strategy, raw_stack } => {
            let stack = raw_stack.into_iter().collect::<LLStack<_>>();

            match strategy {
                SolveStrategy::AStar => solve(astar::Astar::new, stack),
                SolveStrategy::Dumb => solve(dumb::DumbSolver::new, stack),
            }
        }
    }
}


fn solve<Solver, Stack, Solution>(solver: Solver, stack: Stack)
where
    Stack: stack::Stack<utils::N>,
    Solver: FnOnce(Stack) -> Solution,
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
