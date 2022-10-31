use std::io::{Error, ErrorKind};

use super::super::{execution_engine::ExecutionEngine, interpreter::Interpreter};
use crate::objects::{
    instruction::Instruction, stackframe::StackFrame,
};

impl ExecutionEngine for Interpreter {
    /// Execute Resurgence Instructions
    fn execute_instruction(&mut self, start_index: usize) -> Result<(), Error> {
        if !self.seal.untampered_runtime {
           return Err(Error::new(ErrorKind::PermissionDenied, "Runtime has been tampered with"));
        }
        let mut index = start_index;
        let max_length = self.code_holder.instructions.len();
        while index < max_length {
            let operation = self.code_holder.instructions[index].take().unwrap();
            let ins_index = index;
            // To encourage the compiler to optimze extra bounds checks
            assert!(ins_index < max_length);
            match operation {
                Instruction::Alloc(ref register_amount) => {
                    self.call_stack.push(StackFrame::from(*register_amount))
                }
                Instruction::FrameAlloc(ref register_amount) => {
                    let stackframe = self.call_stack.last_mut().unwrap();
                    for _ in 0..*register_amount {
                        stackframe.registers.push(Option::None);
                    }
                }
                Instruction::Free(ref block_amount) => {
                    for _ in 0..*block_amount {
                        self.call_stack.pop();
                    }
                }
                Instruction::FrameFree(ref register_amount) => {
                    let stackframe = self.call_stack.last_mut().unwrap();
                    for _ in 0..*register_amount {
                        stackframe.registers.pop();
                    }
                }
                Instruction::Jump(ref jmp_amount) => {
                    index = (index as i64 + jmp_amount) as usize;
                    self.code_holder.instructions[ins_index] = Some(operation);
                    continue;
                }

                Instruction::Call(ref func_index) => {
                    let res = self.execute_instruction(*func_index as usize);
                    if let Err(err) = res {
                        return Err(err);
                    }
                },
                Instruction::ExtCall(ref func_reg) => {
                    let res = self.ext_call(*func_reg);
                    if let Err(err) = res {
                        return Err(err);
                    }
                },
                Instruction::Ret => {
                    self.code_holder.instructions[ins_index] = Some(operation);
                    return Result::Ok(());
                },

                Instruction::Mov(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => {
                    let res = self.mov_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }
                Instruction::Cpy(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => {
                    let res = self.cpy_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }
                Instruction::Ref(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => {
                    let res = self.ref_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }

                Instruction::StackPush(ref register, ref reference) => {
                    self.push_on_stack(register, reference)
                }
                Instruction::StackMov(ref register, ref reference) => {
                    let res = self.stack_mov(register, reference);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }
                Instruction::StackPop => {
                    self.stack.pop();
                } // We have the braces around this call to make the Rust compiler happy

                Instruction::Add(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.add(dst_reg, reg_1, reg_2);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }
                Instruction::Sub(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.sub(dst_reg, reg_1, reg_2);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }
                Instruction::Mul(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.mul(dst_reg, reg_1, reg_2);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }
                Instruction::Div(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.div(dst_reg, reg_1, reg_2);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }
                Instruction::Mod(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.modlo(dst_reg, reg_1, reg_2);
                    if let Err(err) = res {
                        return Err(err);
                    }
                }

                Instruction::Equal(ref reg_1, ref reg_2) => {
                    if self.equal(reg_1, reg_2) {
                        index += 1;
                    }
                }
                Instruction::NotEqual(ref reg_1, ref reg_2) => {
                    if self.not_equal(reg_1, reg_2) {
                        index += 1;
                    }
                }
                Instruction::Greater(ref reg_1, ref reg_2) => {
                    if self.greater_than(reg_1, reg_2) {
                        index += 1;
                    }
                }
                Instruction::Less(ref reg_1, ref reg_2) => {
                    if self.less_than(reg_1, reg_2) {
                        index += 1;
                    }
                }
                Instruction::GreaterEqual(ref reg_1, ref reg_2) => {
                    if self.greater_or_equal(reg_1, reg_2) {
                        index += 1;
                    }
                }
                Instruction::LessEqual(ref reg_1, ref reg_2) => {
                    if self.less_or_equal(reg_1, reg_2) {
                        index += 1;
                    }
                }
            }

            // Store instruction back into memory and increment index
            self.code_holder.instructions[ins_index] = Some(operation);
            index += 1;
        }
        Result::Ok(())
    }

    // Execute an exported function.
    fn execute_function(&mut self, func_name: &str) -> Result<(), Error> {
        self.seal.set_runtime();
        match self.code_holder.exports.get(func_name) {
            Some(inst) => self.execute_instruction(*inst as usize),
            None => {
                Err(Error::new(
                    ErrorKind::Other,
                    format!("Function {} does not exist", func_name),
                ))
            }
        }
    }
}
