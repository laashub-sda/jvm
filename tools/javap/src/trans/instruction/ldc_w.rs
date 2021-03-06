#![allow(non_camel_case_types)]
use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Ldc_W;

impl Instruction for Ldc_W {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::ldc_w,
            icp: self.calc_cp_index_u16(codes, pc),
            wide: false,
        };

        (info, pc + 3)
    }
}
