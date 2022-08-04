use super::super::super::{interpreter::Interpreter};
use crate::objects::constant::{Constant, create_constant_double};
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

impl Interpreter {
    pub fn ref_register(&mut self, dst_reg: &Register, dst_reg_ref: &RegisterReference, src_reg: &Register, src_reg_ref: &RegisterReference) {
        let Register(dst_index, dst_loc) = dst_reg;
        match *dst_loc {
            RegisterLocation::ConstantPool => todo!(),
            RegisterLocation::Accumulator => todo!(),
            RegisterLocation::Global => todo!(),
            RegisterLocation::Local => todo!(),
        }
    }
}