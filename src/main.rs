#[macro_use]
extern crate structopt;

mod options;
mod stack;
mod instruction;
mod checker;
mod solver;

use stack::Stack;
use options::Options;
use structopt::StructOpt;
use checker::*;

fn main() {
    let opts = Options::from_args();

    let stack = opts.raw_stack
        .into_iter()
        .collect::<Stack<_>>();

    if opts.check {
        let app_config = CheckerConfig { stack, debug_states: opts.debug_states };
        check(app_config);
    } else if opts.solve {
        solver::solve(stack);
    }
}
