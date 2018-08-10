use stack::Stack;
use instruction::Instruction;

pub type StackN = Stack<u32>;

pub fn execute(instruction: &Instruction, a: &mut StackN, b: &mut StackN) {
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
