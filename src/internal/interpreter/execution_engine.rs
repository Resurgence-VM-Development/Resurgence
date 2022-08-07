use super::super::{interpreter::Interpreter, execution_engine::ExecutionEngine};
use crate::objects::{instruction::Instruction, stackframe::StackFrame, codeholder::CodeHolder, register::{self, Register, RegisterReference}};

impl ExecutionEngine for Interpreter {

    /// Execute Resurgence Instruction
    fn execute_instruction(&mut self, code_holder: &CodeHolder, start_index: usize)
    {
        let CodeHolder(instruction_vec) = &*code_holder;
        
        let mut index = start_index; let max_length = instruction_vec.len();
        while index != max_length {
            let operation = &instruction_vec[index];
            match &*operation {
                Instruction::Alloc(ref register_amount) => self.call_stack.push(StackFrame::from(*register_amount)), // Very simple operation
                Instruction::Free(ref block_amount) => {
                    for _ in 0..*block_amount {
                        self.call_stack.pop();
                    }
                },
                Instruction::Jump(ref jmp_amount) => {
                    index += *jmp_amount as usize;
                    continue;
                },

                Instruction::Call(ref func_index) => self.execute_instruction(code_holder, *func_index as usize),
                Instruction::ExtCall(_) => todo!(),
                
                Instruction::Mov(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => self.mov_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref),
                Instruction::Cpy(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => self.cpy_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref),
                Instruction::Ref(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => self.ref_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref),

                Instruction::StackPush(ref register, ref reference) => self.push_on_stack(register, reference),
                Instruction::StackPop => {self.stack.pop();}, // We have the braces around this call to make the Rust compiler happy

                Instruction::Add(ref dst_reg, ref reg_1, ref reg_2) => self.add(dst_reg, reg_1, reg_2),
                Instruction::Sub(ref dst_reg, ref reg_1, ref reg_2) => self.sub(dst_reg, reg_1, reg_2),
                Instruction::Mul(ref dst_reg, ref reg_1, ref reg_2) => self.mul(dst_reg, reg_1, reg_2),
                Instruction::Div(ref dst_reg, ref reg_1, ref reg_2) => self.div(dst_reg, reg_1, reg_2),
                
                Instruction::Equal(_, _) => todo!(),
                Instruction::NotEqual(_, _) => todo!(),
                Instruction::Greater(_, _) => todo!(),
                Instruction::Less(_, _) => todo!(),
                Instruction::GreaterEqual(_, _) => todo!(),
                Instruction::LessEqual(_, _) => todo!(),
            }

            // Increment i to advance to the next index
            index += 1;
        }
    }
}