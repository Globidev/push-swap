#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(long="stack-type", default_value="linked-list")]
    pub stack_type: StackType,

    #[structopt(subcommand)]
    pub command: Command
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "check")]
    Check(CheckConfig),
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

#[derive(StructOpt, Debug)]
pub struct CheckConfig {
    #[structopt(short="d", long="debug-states")]
    pub debug_states: bool,
    // Positional
    pub raw_stack: Vec<u32>,
}

#[derive(Debug)]
pub enum SolveStrategy {
    AStar, ParAStar, NaiveInsert, SmartInsert,
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
            "astar"        | "a*"    => Ok(SolveStrategy::AStar),
            "par-astar"    | "para*" => Ok(SolveStrategy::ParAStar),
            "naive-insert" | "naive" => Ok(SolveStrategy::NaiveInsert),
            "smart-insert" | "smart" => Ok(SolveStrategy::SmartInsert),
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
