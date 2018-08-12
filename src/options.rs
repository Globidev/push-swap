#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(long="stack-type", default_value="vecdeque")]
    pub stack_type: StackType,

    #[structopt(subcommand)]
    pub command: Command
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "check")]
    Check {
        #[structopt(short="d", long="debug-states")]
        debug_states: bool,
        // Positional
        raw_stack: Vec<u32>,
    },
    #[structopt(name = "solve")]
    Solve {
        #[structopt(short="s", long="strategy", default_value="dumb")]
        strategy: SolveStrategy,

        #[structopt(short="p", long="par-threads")]
        par_threads: Option<usize>,
        // Positional
        raw_stack: Vec<u32>
    }
}

#[derive(Debug)]
pub enum SolveStrategy {
    AStar, Dumb, ParallelAStar
}

#[derive(Debug)]
pub enum StackType {
    LinkedList, VecDeque, Vec
}

use std::str::FromStr;

impl FromStr for SolveStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "astar"     | "a*"  => Ok(SolveStrategy::AStar),
            "dumb"              => Ok(SolveStrategy::Dumb),
            "par-astar" | "pa*" => Ok(SolveStrategy::ParallelAStar),
            invalid => Err(String::from(invalid))
        }
    }
}

impl FromStr for StackType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "linked-list" | "ll" => Ok(StackType::LinkedList),
            "vec-deque"   | "vd" => Ok(StackType::VecDeque),
            "vec"         | "v"  => Ok(StackType::Vec),
            invalid => Err(String::from(invalid))
        }
    }
}
