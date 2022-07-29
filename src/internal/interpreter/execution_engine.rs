use super::super::{interpreter::Interpreter, execution_engine::ExecutionEngine};
use crate::objects::{bytecode::ByteCode, stackframe::StackFrame, codeholder::CodeHolder};

impl ExecutionEngine for Interpreter {
    fn execute_bytecode(&mut self, code_holder: &CodeHolder, start_index: usize)
    {
        let mut index = start_index;
        let CodeHolder(bytecode_vec) = &*code_holder;

        loop {
            if index == bytecode_vec.len() {
                break;
            }
            let operation = &bytecode_vec[index];
            match *operation {
                ByteCode::Alloc(register_amount) => self.call_stack.push(StackFrame::from(register_amount)), // Very simple operation
                ByteCode::Free(block_amount) => {
                    for _ in 0..block_amount {
                        self.call_stack.pop();
                    }
                },
                ByteCode::Jump(jmp_amount) => {
                    index += jmp_amount as usize;
                    continue;
                },

                ByteCode::Call(func_index) => {
                    self.execute_bytecode(code_holder, func_index as usize); // Do recursive call
                },
                ByteCode::CCall(_) => todo!(),

                ByteCode::Mov(_, _, _, _) => todo!(),
                ByteCode::Cpy(_, _, _, _) => todo!(),
                ByteCode::Ref(_, _, _, _) => todo!(),

                ByteCode::StackPush(_, _) => todo!(),
                ByteCode::StackPop => todo!(),

                ByteCode::Add(_, _, _) => todo!(),
                ByteCode::Sub(_, _, _) => todo!(),
                ByteCode::Mul(_, _, _) => todo!(),
                ByteCode::Div(_, _, _) => todo!(),
                
                ByteCode::Equal(_, _) => todo!(),
                ByteCode::NotEqual(_, _) => todo!(),
                ByteCode::Greater(_, _) => todo!(),
                ByteCode::Less(_, _) => todo!(),
                ByteCode::GreaterEqual(_, _) => todo!(),
                ByteCode::LessEqual(_, _) => todo!(),
            }

            // Increment i to advance to the next index
            index += 1;
        }
    }
}