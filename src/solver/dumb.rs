use super::*;

pub struct DumbSolver {
    stack: Stack<u32>,
    pushed: usize,
    state: DumbSolverState
}

enum DumbSolverState {
    Done(Vec<Instruction>),
    Rotate(Vec<Instruction>),
    Looping,
}

impl DumbSolver {
    pub fn new(stack: Stack<u32>) -> Self {
        Self { stack, pushed: 0, state: DumbSolverState::Looping }
    }
}

impl Iterator for DumbSolver {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        use self::DumbSolverState::*;
        use self::Instruction::*;

        let next_state = match self.state {
            Done(ref mut leftover)   => return leftover.pop(),
            Rotate(ref mut leftover) => match leftover.pop() {
                item@Some(_) => return item,
                _            => Looping
            },
            Looping => match self.stack.minimum() {
                None => Done(repeat_n(PushA, self.pushed).collect()),
                Some((_, min_idx)) => match sorted_rot(&self.stack) {
                    Some((instr, n)) => {
                        let leftover_instrs = repeat_n(PushA, self.pushed)
                            .chain(repeat_n(instr, n));
                        Done(leftover_instrs.collect())
                    },
                    None => {
                        let (instr, n) = rotation(&self.stack, min_idx);

                        match instr {
                            RotateA => self.stack.rotate_n(n),
                            ReverseRotateA => self.stack.reverse_rotate_n(n),
                            _ => ()
                        };

                        self.stack.pop().unwrap_or_default();
                        let rotations_and_push = once(PushB)
                            .chain(repeat_n(instr, n));
                        self.pushed += 1;
                        Rotate(rotations_and_push.collect())
                    }
                }
            }
        };

        self.state = next_state;
        self.next()
    }
}
