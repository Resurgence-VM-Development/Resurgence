use super::super::{interpreter::Interpreter, execution_engine::ExecutionEngine};
use crate::objects::{bytecode::ByteCode, stackframe::StackFrame};

impl ExecutionEngine for Interpreter {
    fn execute_bytecode(&mut self)
    {
        let mut i = 0;
        loop {
            if i == self.bytecode.len() {
                break;
            }
            let operation = &self.bytecode[i];
            match *operation {
                ByteCode::Alloc(register_amount) => self.stack.push(StackFrame::from(register_amount)),
                ByteCode::Free(block_amount) => {
                    for _ in 0..block_amount {
                        self.stack.pop();
                    }
                },
                ByteCode::Jump(jmp_amount) => {
                    i += jmp_amount as usize;
                    continue;
                },

                ByteCode::Mov(_, _, _, _) => todo!(),
                ByteCode::Cpy(_, _, _, _) => todo!(),
                ByteCode::Ref(_, _, _, _) => todo!(),

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
            i += 1;
        }
    }
}