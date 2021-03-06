use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Ineg;

impl Instruction for Ineg {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::ineg,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
