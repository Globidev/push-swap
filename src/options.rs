#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(short="c", long="check",
                conflicts_with="solve", required_unless="solve")]
    pub check: bool,
    #[structopt(short="s", long="solve",
                conflicts_with="check", required_unless="check")]
    pub solve: bool,
    #[structopt(short="d", long="debug-states",
                conflicts_with="solve")]
    pub debug_states: bool,
    // Positional
    pub raw_stack: Vec<u32>
}
