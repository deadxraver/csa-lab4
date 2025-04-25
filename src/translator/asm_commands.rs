#![allow(dead_code)]
const SHIFT: u8 = 8;
const OPCODE_MASK: u64 = 0x000F;
const OPERAND_MASK: u64 = 0x000F;
#[allow(dead_code)]
#[derive(Debug)]
pub enum Opcodes {
    PushImm = 0x00,
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

pub fn to_bytes(opcode: Opcodes) -> u8 {
    opcode as u8
}
