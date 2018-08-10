extern crate colored;

use utils::*;
use self::colored::Colorize;

use std::io::{self, BufReader, BufRead, Read};
use std::process::exit;

use stack::Stack;
use instruction::Instruction;

pub struct CheckerConfig {
    pub stack: StackN,
    pub debug_states: bool
}

pub fn check(config: CheckerConfig) -> ! {
    let mut stack = config.stack;
    let mut side_stack = Stack::default();

    let stack_display_width = stack.to_string().len();
    let debug_states = config.debug_states;

    let instructions = read_instructions(io::stdin());

    println!("Start: {}", stack.to_string().blue().on_yellow());

    let instr_count = instructions.fold(0, |c, i| {
        execute(&i, &mut stack, &mut side_stack);
        if debug_states {
            println!(
                "{:3} => {:width$} {} {}",
                i.to_string().purple(),
                stack.to_string(), "|".cyan(), side_stack,
                width=stack_display_width
            );
        }
        c + 1
    });

    let (status, sorted) = match stack.is_sorted() {
        true  => (0, "Yes".green()),
        false => (1, "No".red())
    };

    println!("End: {}", stack.to_string().on_green());
    println!("Sorted: {}", sorted);
    println!("Moves: {}", instr_count.to_string().cyan());
    let ratio = instr_count as f32 / stack.len() as f32;
    println!("Ratio: {:.5}", ratio.to_string().cyan());

    exit(status);
}

fn read_instructions(reader: impl Read) -> impl Iterator<Item = Instruction> {
    BufReader::new(reader).lines()
        .filter_map(|line_result| {
            let line = line_result
                .expect("Failed to read line");

            match line.trim() {
                ""    => None,
                instr => Some(instr.parse().expect("invalid instruction"))
            }
        })
}
