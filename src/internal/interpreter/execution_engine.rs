use std::io::{Error, ErrorKind};

use super::super::{execution_engine::ExecutionEngine, interpreter::Interpreter};
use crate::objects::{
    instruction::Instruction, stackframe::StackFrame,
};

impl ExecutionEngine for Interpreter {
    /// Execute Resurgence Instructions
    fn execute_instruction(&mut self, start_index: usize) -> Result<(), Error> {
        if !self.imports_resolved {
            let res = self.resolve_imports();
            if let Err(err) = res {
                return Err(err);
            }
            self.imports_resolved = true;
        }
        let mut index = start_index;
        let max_length = self.code_holder.instructions.len();
        while index != max_length {
            let operation = self.code_holder.instructions[index].take().unwrap();
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
                    let cloned_jmp_amount = *jmp_amount;
                    if index < max_length {
                        self.code_holder.instructions[index] = Some(operation);
                    }
                    index = (index as i64 + cloned_jmp_amount) as usize;
                    continue;
                }

                Instruction::Call(ref func_index) => self.execute_instruction(*func_index as usize)?,
                Instruction::ExtCall(ref func_reg) => self.ext_call(*func_reg)?,
                Instruction::Ret => {
                    if index < max_length {
                        self.code_holder.instructions[index] = Some(operation);
                    }
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
                    self.add(dst_reg, reg_1, reg_2);
                }
                Instruction::Sub(ref dst_reg, ref reg_1, ref reg_2) => {
                    self.sub(dst_reg, reg_1, reg_2);
                }
                Instruction::Mul(ref dst_reg, ref reg_1, ref reg_2) => {
                    self.mul(dst_reg, reg_1, reg_2);
                }
                Instruction::Div(ref dst_reg, ref reg_1, ref reg_2) => {
                    self.div(dst_reg, reg_1, reg_2);
                }
                Instruction::Mod(ref dst_reg, ref reg_1, ref reg_2) => {
                    self.modlo(dst_reg, reg_1, reg_2);
                }

                Instruction::Equal(ref reg_1, ref reg_2) => {
                    if self.equal(reg_1, reg_2) {
                        if index < max_length {
                            self.code_holder.instructions[index] = Some(operation);
                        }
                        index += 2;
                        continue;
                    }
                }
                Instruction::NotEqual(ref reg_1, ref reg_2) => {
                    if self.not_equal(reg_1, reg_2) {
                        if index < max_length {
                            self.code_holder.instructions[index] = Some(operation);
                        }
                        index += 2;
                        continue;
                    }
                }
                Instruction::Greater(ref reg_1, ref reg_2) => {
                    if self.greater_than(reg_1, reg_2) {
                        if index < max_length {
                            self.code_holder.instructions[index] = Some(operation);
                        }
                        index += 2;
                        continue;
                    }
                }
                Instruction::Less(ref reg_1, ref reg_2) => {
                    if self.less_than(reg_1, reg_2) {
                        if index < max_length {
                            self.code_holder.instructions[index] = Some(operation);
                        }
                        index += 2;
                        continue;
                    }
                }
                Instruction::GreaterEqual(ref reg_1, ref reg_2) => {
                    if self.greater_or_equal(reg_1, reg_2) {
                        if index < max_length {
                            self.code_holder.instructions[index] = Some(operation);
                        }
                        index += 2;
                        continue;
                    }
                }
                Instruction::LessEqual(ref reg_1, ref reg_2) => {
                    if self.less_or_equal(reg_1, reg_2) {
                        if index < max_length {
                            self.code_holder.instructions[index] = Some(operation);
                        }
                        index += 2;
                        continue;
                    }
                }
            }

            // Store instruction back into memory and increment index
            if index < max_length {
                self.code_holder.instructions[index] = Some(operation);
            }
            index += 1;
        }
        Result::Ok(())
    }

    // Execute an exported function.
    fn execute_function(&mut self, func_name: &str) -> Result<(), Error> {
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
