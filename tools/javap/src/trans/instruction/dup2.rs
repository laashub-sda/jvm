use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Dup2;

impl Instruction for Dup2 {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::dup2,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
