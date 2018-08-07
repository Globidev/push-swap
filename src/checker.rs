extern crate colored;

use self::colored::Colorize;

use std::io::{self, BufReader, BufRead};
use std::process::exit;

use stack::Stack;
use instruction::Instruction;

pub struct CheckerConfig {
    pub stack: Stack<u32>,
    pub debug_states: bool
}

pub fn check(config: CheckerConfig) {
    let mut stack = config.stack;
    let mut side_stack = Stack::default();

    let instructions = read_instructions();

    let stack_size = stack.len();
    let debug_states = config.debug_states;

    println!("Start: {}", stack.to_string().blue().on_yellow());

    let mut count = 0;
    instructions.for_each(|i| {
        execute(&i, &mut stack, &mut side_stack);
        if debug_states {
            println!(
                "{:3} => {:width$} {} {}",
                i.to_string().purple(),
                stack.to_string(), "|".cyan(), side_stack,
                width=stack_size * 2 - 1
            );
        }
        count += 1;
    });

    let (status, sorted) = match stack.is_sorted() {
        true  => (0, "yes".green()),
        false => (1, "no".red())
    };

    println!("End:   {} in {} moves", stack.to_string().on_green(), count.to_string().cyan());
    println!("Sorted: {}", sorted);

    exit(status);
}

fn read_instructions() -> impl Iterator<Item = Instruction> {
    BufReader::new(io::stdin()).lines()
        .filter_map(|line_result| {
            let line = line_result
                .expect("Failed to read line");

            match line.trim() {
                ""    => None,
                instr => Some(instr.parse().expect("invalid instruction"))
            }
        })
}

fn execute(instruction: &Instruction, a: &mut Stack<u32>, b: &mut Stack<u32>) {
    use self::Instruction::*;

    match instruction {
        PushA  => if let Some(n) = b.pop() { a.push(n) },
        PushB  => if let Some(n) = a.pop() { b.push(n) },

        SwapA    => a.swap(),
        SwapB    => b.swap(),
        SwapBoth => { a.swap(); b.swap() },

        RotateA    => a.rotate(),
        RotateB    => b.rotate(),
        RotateBoth => { a.rotate(); b.rotate() },

        ReverseRotateA    => a.reverse_rotate(),
        ReverseRotateB    => b.reverse_rotate(),
        ReverseRotateBoth => { a.reverse_rotate(); b.reverse_rotate(); }
    }
}
