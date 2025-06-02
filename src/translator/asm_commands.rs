#![allow(dead_code)]
const SHIFT: u8 = 8;
const OPCODE_MASK: u64 = 0x000F;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Opcodes {
    PushImm = 0x00, // push immediate value to stack
    Add,            // stack.push(stack.pop() + stack.pop())
    Sub,            // stack.push(stack.pop() - stack.pop())
    Mul,            // stack.push(stack.pop() * stack.pop())
    Div,            // stack.push(stack.pop() / stack.pop())
    Pop1,           // stack.pop()
    Pop2,           // stack2.pop()
    Dup,            // stack.push(stack.top())
    To1,            // stack.push(stack2.pop())
    To2,            // stack2.push(stack.pop())
    Jmp,            // jump to address represented as imm val
    BIf,            // branch if top of the stack is non-zero
    Store,          // не помню че я хотел этим сказать
    Call,           // call function
    Ret,            // return from function
    Halt,           // halt the machine
    And,            // bitwise AND
    Or,             // bitwise OR
    Xor,            // bitwise XOR
    Not,            // bitwise NOT
    Eq,             // stack.pop() == stack.pop() ? -1 : 0
    PushInd,        // вот это и ниже тоже не помню что изначально должно было значить потом поменяю
    StoreInd,
}

fn shift_left(num: u64) -> u32 {
    (num << SHIFT) as u32
}

pub fn to_bytes(opcode: Opcodes) -> u8 {
    opcode as u8
}
