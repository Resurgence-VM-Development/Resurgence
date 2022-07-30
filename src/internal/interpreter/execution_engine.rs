use super::super::{interpreter::Interpreter, execution_engine::ExecutionEngine};
use crate::objects::constant::{Constant, create_constant_double};
use crate::objects::{instruction::Instruction, stackframe::StackFrame, codeholder::CodeHolder};
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

impl Interpreter {
    // fn get_global(index: usize, ref_type: RegisterReference) -> Constant {

    // }
}

impl ExecutionEngine for Interpreter {

    /// Execute Resurgence Instruction
    fn execute_Instruction(&mut self, code_holder: &CodeHolder, start_index: usize)
    {
        let mut index = start_index;
        let CodeHolder(Instruction_vec) = &*code_holder;

        loop {
            if index == Instruction_vec.len() {
                break;
            }
            let operation = &Instruction_vec[index];
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

                Instruction::Call(func_index) => {
                    self.execute_Instruction(code_holder, *func_index as usize); // Do recursive call
                },
                Instruction::ExtCall(_) => todo!(),

                Instruction::Mov(dst_reg, dst_reg_ref, src_reg, src_reg_ref) => {
                    let Register(dst_index, dst_loc) = dst_reg;
                    let Register(src_index, src_loc) = src_reg;

                    match (dst_loc, src_loc) {
                        (RegisterLocation::Accumulator, RegisterLocation::Global) => {

                        },
                        (RegisterLocation::Accumulator, RegisterLocation::Local) => todo!(),

                        (RegisterLocation::Global, RegisterLocation::Accumulator) => todo!(),
                        (RegisterLocation::Global, RegisterLocation::Global) => todo!(),
                        (RegisterLocation::Global, RegisterLocation::Local) => todo!(),

                        (RegisterLocation::Local, RegisterLocation::Accumulator) => todo!(),
                        (RegisterLocation::Local, RegisterLocation::Global) => todo!(),
                        (RegisterLocation::Local, RegisterLocation::Local) => todo!(),

                        _ => panic!("Invalid Mov operation!")
                    }
                },
                Instruction::Cpy(_, _, _, _) => todo!(),
                Instruction::Ref(_, _, _, _) => todo!(),

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