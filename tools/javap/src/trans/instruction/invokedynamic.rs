use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Invokedynamic;

impl Instruction for Invokedynamic {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::invokedynamic,
            icp: self.calc_cp_index_u16(codes, pc),
            wide: false,
        };

        (info, pc + 5)
    }
}
