use stack::Stack;
use instruction::Instruction;

pub type N = u32;

pub fn execute<S>(instr: &Instruction, a: &mut S, b: &mut S)
where
    S: Stack<N> + Sized
{
    use self::Instruction::*;

    match instr {
        PushA  => if let Some(n) = b.pop() { a.push(n) },
        PushB  => if let Some(n) = a.pop() { b.push(n) },

        SwapA    => a.swap(),
        SwapB    => b.swap(),
        SwapBoth => { a.swap(); b.swap() },

        RotateA    => a.rotate(),
        RotateB    => b.rotate(),
        RotateBoth => { a.rotate(); b.rotate() },

        RRotateA    => a.rrotate(),
        RRotateB    => b.rrotate(),
        RRotateBoth => { a.rrotate(); b.rrotate(); }
    }
}
