use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    PushA, PushB,
    SwapA, SwapB, SwapBoth,
    RotateA, RotateB, RotateBoth,
    RRotateA, RRotateB, RRotateBoth,
}

#[derive(Debug)]
pub struct InvalidInstruction(String);

impl FromStr for Instruction {
    type Err = InvalidInstruction;

    fn from_str(as_string: &str) -> Result<Self, Self::Err> {
        use self::Instruction::*;

        match as_string {
            "pa"  => Ok(PushA),
            "pb"  => Ok(PushB),
            "sa"  => Ok(SwapA),
            "sb"  => Ok(SwapB),
            "ss"  => Ok(SwapBoth),
            "ra"  => Ok(RotateA),
            "rb"  => Ok(RotateB),
            "rr"  => Ok(RotateBoth),
            "rra" => Ok(RRotateA),
            "rrb" => Ok(RRotateB),
            "rrr" => Ok(RRotateBoth),
            _     => Err(InvalidInstruction(String::from(as_string)))
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        let as_string = match self {
            PushA => "pa",
            PushB => "pb",
            SwapA => "sa",
            SwapB => "sb",
            SwapBoth => "ss",
            RotateA => "ra",
            RotateB => "rb",
            RotateBoth => "rr",
            RRotateA => "rra",
            RRotateB => "rrb",
            RRotateBoth => "rrr",
        };

        write!(f, "{}", as_string)
    }
}
