use super::super::{interpreter::Interpreter, execution_engine::ExecutionEngine};
use crate::objects::{instruction::Instruction, stackframe::StackFrame, codeholder::CodeHolder};

impl ExecutionEngine for Interpreter {

    /// Execute Resurgence Instruction
    fn execute_instruction(&mut self, code_holder: &CodeHolder, start_index: usize)
    {
        let mut index = start_index;
        let CodeHolder(instruction_vec) = &*code_holder;

        loop {
            if index == instruction_vec.len() { break; }
            let operation = &instruction_vec[index];
            match &*operation {
                Instruction::Alloc(register_amount) => self.call_stack.push(StackFrame::from(*register_amount)), // Very simple operation
                Instruction::Free(block_amount) => {
                    for _ in 0..*block_amount {
                        self.call_stack.pop();
                    }
                },
                Instruction::Jump(jmp_amount) => {
                    index += *jmp_amount as usize;
                    continue;
                },

                Instruction::Call(func_index) => self.execute_instruction(code_holder, *func_index as usize),
                Instruction::ExtCall(_) => todo!(),
                
                Instruction::Mov(dst_reg, dst_reg_ref, src_reg, src_reg_ref) => self.mov_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref),
                Instruction::Cpy(dst_reg, dst_reg_ref, src_reg, src_reg_ref) => self.cpy_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref),
                Instruction::Ref(dst_reg, dst_reg_ref, src_reg, src_reg_ref) => self.ref_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref),

                Instruction::StackPush(_, _) => todo!(),
                Instruction::StackPop => todo!(),

                Instruction::Add(_, _, _) => todo!(),
                Instruction::Sub(_, _, _) => todo!(),
                Instruction::Mul(_, _, _) => todo!(),
                Instruction::Div(_, _, _) => todo!(),
                
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