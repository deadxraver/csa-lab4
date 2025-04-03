#![allow(dead_code)]
const SHIFT: u8 = 8;
const OPCODE_MASK: u64 = 0x0FF0;
const OPERAND_MASK: u64 = 0x000F;

pub enum Opcodes {
    PushImm0 = 0x00,
    PushImm1,
    PushImm2,
    PushImm3,
    Add,
    Sub,
    Mul,
    Div,
    Pop1,
    Pop2,
    Dup,
    To1,
    To2,
    Jmp,
    BIf,
    Store,
    Call,
    Ret,
    Halt,
    And,
    Or,
    Xor,
    Not,
    Eq,
    PushInd,
    StoreInd,
}

fn shift_left(num: u64) -> u32 {
    (num << SHIFT) as u32
}

fn shift_right(num: u64) -> u32 {
    (num >> SHIFT) as u32
}

pub struct MachineInstruction {
    opcode: Opcodes,
    operand: u8,
}

/*
* FIXME: cast number to enum
pub fn from_bytes(instruction: u32) -> MachineInstruction {
    MachineInstruction { opcode: Opcodes { shift_right(instruction as u64 & OPCODE_MASK) }, operand: (instruction as u64 & OPERAND_MASK) as u8 }
}
*/

pub fn to_bytes(instruction: MachineInstruction) -> u32 {
    instruction.operand as u32 + shift_left(instruction.opcode as u64)
}
