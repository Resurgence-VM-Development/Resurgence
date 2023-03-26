use super::super::super::interpreter::Interpreter;
use crate::{ResurgenceError, create_new_trace};
use crate::objects::constant::{Constant, create_constant_double};
use crate::objects::register::{Register, RegisterLocation, RegisterReference};
use crate::objects::resurgence_error::ResurgenceErrorKind;

impl Interpreter {
    pub(crate) fn cpy_registers(&mut self, dst_reg: &Register, dst_reg_ref: &RegisterReference, src_reg: &Register, src_reg_ref: &RegisterReference) -> Result<(), ResurgenceError> {
        // Destination register
        let Register(mut dst_index, mut dst_loc) = dst_reg;
        let mut dst_index_usize = dst_index as usize;

        // Dereference the destination register if needed
        if *dst_reg_ref == RegisterReference::Dereference {
            Register(dst_index, dst_loc) = self.dereference_register(dst_index_usize, &dst_loc);
            dst_index_usize = dst_index as usize;
        }

        // Source register
        let Register(mut src_index, mut src_loc) = src_reg;
        let mut src_index_usize = src_index as usize;

        // Dereference the source register if needed
        if *src_reg_ref == RegisterReference::Dereference {
            Register(src_index, src_loc) = self.dereference_register(src_index_usize, &src_loc);
            src_index_usize = src_index as usize;
        }

        match (dst_loc, src_loc) {
            (RegisterLocation::Accumulator, RegisterLocation::Global) => {
                // Start doing the move if src_register is not None
                let src_register = self.cpy_global(src_index_usize); // take the value from global memory
                if let Err(mut err) = src_register {
                    create_new_trace!(err);
                    return Err(err);
                }
                match src_register.unwrap() {
                    Constant::Int(src_int) => {
                        self.accumulator = src_int as f64;
                    }
                    Constant::Double(src_double) => {
                        self.accumulator = src_double;
                    }
                    _ => {
                        let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Invalid copy to the accumulator!");
                        create_new_trace!(err);
                        return Err(err);
                    },
                }
        },
            (RegisterLocation::Accumulator, RegisterLocation::Local) => {
                let src_register = self.cpy_local(src_index_usize);
                match src_register {
                    Constant::Int(src_int) => {
                        self.accumulator = src_int as f64;
                    }
                    Constant::Double(src_double) => {
                        self.accumulator = src_double;
                    }
                    _ => {
                        let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Invalid copy to the accumulator!");
                        create_new_trace!(err);
                        return Err(err);
                    },
                }
            },

            (RegisterLocation::Global, RegisterLocation::ConstantPool) => {
                self.global[dst_index_usize] = Some(self.cpy_constant(src_index_usize));
            }
            (RegisterLocation::Global, RegisterLocation::Accumulator) => {
                self.global[dst_index_usize] = Some(create_constant_double(&self.accumulator));
            },
            (RegisterLocation::Global, RegisterLocation::Global) => {
                let src_reg = self.cpy_global(src_index_usize);
                if let Err(mut err) = src_reg {
                    create_new_trace!(err);
                    return Err(err);
                }
                self.global[dst_index_usize] = Some(src_reg.unwrap());
            },
            (RegisterLocation::Global, RegisterLocation::Local) => {
                self.global[dst_index_usize] = Some(self.cpy_local(src_index_usize));
            },

            (RegisterLocation::Local, RegisterLocation::ConstantPool) => {
                let constant = Some(self.cpy_constant(src_index_usize));
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = constant;
            }
            (RegisterLocation::Local, RegisterLocation::Accumulator) => {
                let accumulator = Some(create_constant_double(&self.accumulator));
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = accumulator;
            },
            (RegisterLocation::Local, RegisterLocation::Global) => {
                let global_value = self.cpy_global(src_index_usize);
                let stack_frame = self.ref_stack_frame();
                if let Err(mut err) = global_value {
                    create_new_trace!(err);
                    return Err(err);
                }
                stack_frame.registers[dst_index_usize] = Some(global_value.unwrap());
            },
            (RegisterLocation::Local, RegisterLocation::Local) => {
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = Some(stack_frame.cpy_register(src_index_usize));
            },

            _ => {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Invalid CPY operation!");
                create_new_trace!(err);
                return Err(err);
            }
        }
        Result::Ok(())
    }
}
