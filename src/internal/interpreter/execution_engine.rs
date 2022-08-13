use std::io::Error;

use super::super::{execution_engine::ExecutionEngine, interpreter::Interpreter};
use crate::objects::{instruction::Instruction, stackframe::StackFrame};

impl ExecutionEngine for Interpreter {
    /// Execute Resurgence Instructions
    fn execute_instruction(&mut self, start_index: usize) -> Result<(), Error> {
        self.resolve_imports()?;
        let mut index = start_index;
        let max_length = self.code_holder.instructions.len();
        while index != max_length {
            let operation = self.code_holder.instructions[index];
            match operation {
                Instruction::Alloc(ref register_amount) => {
                    self.call_stack.push(StackFrame::from(*register_amount))
                }
                Instruction::Free(ref block_amount) => {
                    for _ in 0..*block_amount {
                        self.call_stack.pop();
                    }
                }
                Instruction::Jump(ref jmp_amount) => {
                    index += *jmp_amount as usize;
                    continue;
                }

                Instruction::Call(ref func_index) => self.execute_instruction(*func_index as usize)?,
                Instruction::ExtCall(ref func_reg) => self.ext_call(*func_reg)?,

                Instruction::Mov(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => self.mov_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref)?,
                Instruction::Cpy(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => self.cpy_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref)?,
                Instruction::Ref(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => self.ref_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref)?,

                Instruction::StackPush(ref register, ref reference) => self.push_on_stack(register, reference),
                Instruction::StackPop => {
                    self.stack.pop();
                } // We have the braces around this call to make the Rust compiler happy

                Instruction::Add(ref dst_reg, ref reg_1, ref reg_2) => self.add(dst_reg, reg_1, reg_2),
                Instruction::Sub(ref dst_reg, ref reg_1, ref reg_2) => self.sub(dst_reg, reg_1, reg_2),
                Instruction::Mul(ref dst_reg, ref reg_1, ref reg_2) => self.mul(dst_reg, reg_1, reg_2),
                Instruction::Div(ref dst_reg, ref reg_1, ref reg_2) => self.div(dst_reg, reg_1, reg_2),

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

            // Increment i to advance to the next index
            index += 1;
        }
        Result::Ok(())
    }
}
