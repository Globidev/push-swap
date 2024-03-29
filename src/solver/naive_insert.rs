use super::*;

macro_rules! yield_from {
    ($x: expr) => { for e in $x { yield e } };
}

pub fn naive_insert(mut stack: impl Stack<N>)
    -> impl Iterator<Item = Instruction>
{
    use self::Instruction::*;

    IterGen(move || {
        let mut pushed = 0;

        while let Some((_, min_idx)) = stack.minimum() {
            // If the stack is sorted, relative to a pivot, we rotate to
            // said pivot and end
            if let Some(rot_idx) = stack.sorted_at() {
                let (instr, n) = shortest_rotation(&stack, rot_idx);
                yield_from!(repeat_n(instr, n));
                break
            }
            // Otherwise we rotate to the next minimum value and push
            let (instr, n) = shortest_rotation(&stack, min_idx);

            stack.rotate_n(min_idx);
            drop(stack.pop());

            yield_from!(repeat_n(instr, n));
            yield PushB;

            pushed += 1;
        }

        for _ in 0..pushed { yield PushA }
    })
}

fn shortest_rotation(stack: &impl Stack<N>, at: usize) -> (Instruction, usize) {
    use std::cmp::Ordering::Less;

    let mid = stack.len() / 2;

    match at.cmp(&mid) {
        Less => (Instruction::RotateA, at),
        _    => (Instruction::RRotateA, stack.len() - at),
    }
}
