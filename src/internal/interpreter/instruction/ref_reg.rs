use super::super::super::{interpreter::Interpreter};
use crate::objects::{register::{Register, RegisterLocation, RegisterReference}, constant::Constant};

impl Interpreter {
    pub fn ref_registers(&mut self, dst_reg: &Register, dst_reg_ref: &RegisterReference, src_reg: &Register, src_reg_ref: &RegisterReference) {
        let Register(mut dst_index, mut dst_loc) = dst_reg;
        let mut dst_index_usize = dst_index as usize;

        if *dst_reg_ref == RegisterReference::Dereference {
            Register(dst_index, dst_loc) = self.dereference_register(dst_index_usize, &dst_loc);
            dst_index_usize = dst_index as usize;
        }

        match dst_loc {
            RegisterLocation::Global => {
                if *src_reg_ref == RegisterReference::Dereference {
                    self.global[dst_index_usize] = Some(Constant::Address(self.dereference_register(src_reg.0 as usize, &src_reg.1)));
                    return;
                }
                self.global[dst_index_usize] = Some(Constant::Address(src_reg.clone()));
            },
            RegisterLocation::Local => {
                if *src_reg_ref == RegisterReference::Dereference {
                    let register = Some(Constant::Address(self.dereference_register(src_reg.0 as usize, &src_reg.1))); let stack_frame = self.ref_stack_frame();
                    stack_frame.registers[dst_index_usize] = register;
                    return;
                }
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = Some(Constant::Address(src_reg.clone()));
            },
            _ => panic!("Can't reference anything other then a local or global register!"),
        }
    }
}