#[derive(StructOpt, Debug)]
pub enum Options {
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

use std::str::FromStr;

impl FromStr for SolveStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "astar"     => Ok(SolveStrategy::AStar),
            "dumb"      => Ok(SolveStrategy::Dumb),
            "par-astar" => Ok(SolveStrategy::ParallelAStar),
            invalid => Err(String::from(invalid))
        }
    }
}
