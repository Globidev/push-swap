use super::*;

macro_rules! yield_from {
    ($x: expr) => { for e in $x { yield e } };
}

pub fn smart_insert<S: Stack<N>>(mut stack: S) -> impl Iterator<Item = Instruction> {
    use self::Instruction::*;

    IterGen(move || {
        let mut side_stack = S::default();

        while stack.len() > 0 {
            let a_len = stack.len();
            let b_len = side_stack.len();

            let (rot_a, rot_b, _) = {
                let neighboring_rotations = (-100..100).map(|delta| {
                    let x = stack.peek(delta);
                    let rot_a = if delta < 0 {
                        a_len - (delta % (a_len as isize)).abs() as usize - 1
                    } else {
                        delta as usize % a_len
                    };
                    let rot_b = match side_stack.insert_index(&x) {
                        Some(index) => index,
                        None        => side_stack.maximum()
                                        .map(|(_, idx)| idx)
                                        .unwrap_or(0)
                    };
                    (rot_a, rot_b, x)
                });

                let best_rotation = neighboring_rotations
                    .min_by(|(rot_a1, rot_b1, x1), (rot_a2, rot_b2, x2)| {
                        use std::cmp::{min, Ordering::Equal};

                        let (rot_cost1, rrot_cost1) = rotation_costs(a_len, b_len, *rot_a1, *rot_b1);
                        let (rot_cost2, rrot_cost2) = rotation_costs(a_len, b_len, *rot_a2, *rot_b2);

                        let min_cost1 = min(rot_cost1, rrot_cost1);
                        let min_cost2 = min(rot_cost2, rrot_cost2);

                        match min_cost1.cmp(&min_cost2) {
                            Equal => x1.cmp(x2),
                            ord   => ord
                        }
                    }).unwrap();

                best_rotation
            };
            let (rot_cost, rrot_cost) = rotation_costs(a_len, b_len, rot_a, rot_b);

            use std::cmp::min;
            // Optimize output
            match rot_cost <= rrot_cost {
                true => { // rotate
                    let rot_both = min(rot_a, rot_b);
                    let rot_a_only = rot_a.saturating_sub(rot_both);
                    let rot_b_only = rot_b.saturating_sub(rot_both);
                    for _ in 0..rot_both { yield RotateBoth }
                    for _ in 0..rot_a_only { yield RotateA }
                    for _ in 0..rot_b_only { yield RotateB }
                    yield PushB
                },
                false => { // rrotate
                    let rrot_a = a_len - rot_a;
                    let rrot_b = b_len - rot_b;
                    let rot_both = min(rrot_a, rrot_b);
                    let rot_a_only = rrot_a.saturating_sub(rot_both);
                    let rot_b_only = rrot_b.saturating_sub(rot_both);
                    for _ in 0..rot_both { yield RRotateBoth }
                    for _ in 0..rot_a_only { yield RRotateA }
                    for _ in 0..rot_b_only { yield RRotateB }
                    yield PushB
                }
            }

            stack.rotate_n(rot_a);
            let x = stack.pop().unwrap();
            side_stack.rotate_n(rot_b);
            side_stack.push(x);
        }

        if let Some((_, min_idx)) = side_stack.maximum() {
            let (instr, n) = shortest_rotation(&side_stack, min_idx);
            yield_from!(repeat_n(instr, n));
        }

        for _ in 0..side_stack.len() { yield PushA }
    })
}

fn shortest_rotation(stack: &impl Stack<N>, at: usize) -> (Instruction, usize) {
    use std::cmp::Ordering::Greater;

    let mid = stack.len() / 2;

    match at.cmp(&mid) {
        Greater => (Instruction::RRotateB, stack.len() - at),
        _       => (Instruction::RotateB, at),
    }
}

fn rotation_costs(a_len: usize, b_len: usize, rot_a: usize, rot_b: usize)
    -> (usize, usize)
{
    let rrot_a = a_len - rot_a;
    let rrot_b = b_len - rot_b;

    let rot_cost = (rot_a as isize - rot_b as isize).abs();
    let rrot_cost = (rrot_a as isize - rrot_b as isize).abs();

    (rot_cost as usize, rrot_cost as usize)
}
