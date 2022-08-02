use super::super::{interpreter::Interpreter, execution_engine::ExecutionEngine};
use crate::objects::constant::{Constant, create_constant_double};
use crate::objects::{instruction::Instruction, stackframe::StackFrame, codeholder::CodeHolder};
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

impl ExecutionEngine for Interpreter {

    /// Execute Resurgence Instruction
    fn execute_Instruction(&mut self, code_holder: &CodeHolder, start_index: usize)
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

                Instruction::Call(func_index) => {
                    self.execute_Instruction(code_holder, *func_index as usize); // Do recursive call
                },
                Instruction::ExtCall(_) => todo!(),
                
                Instruction::Mov(dst_reg, dst_reg_ref, src_reg, src_reg_ref) => {
                    let Register(dst_index, dst_loc) = dst_reg; let dst_index_usize = *dst_index as usize;
                    let Register(src_index, src_loc) = src_reg; let src_index_usize = *src_index as usize;

                    match (dst_loc, src_loc) {
                        (RegisterLocation::Accumulator, RegisterLocation::Global) => {
                            let src_register = self.global[src_index_usize].take(); // take the value from global memory
                            
                            // Start doing the move if src_register is not None
                            if let Some(src_val) = src_register {
                                match src_val {
                                    Constant::Int(src_int) => {
                                        self.accumulator = src_int as f64;
                                    }
                                    Constant::Double(src_double) => {
                                        self.accumulator = src_double;
                                    }
                                    _ => panic!("Invalid move to accumulator register!"),
                                }
                            } else {
                                panic!("Segmentation Fault!")
                            }
                        },
                        (RegisterLocation::Accumulator, RegisterLocation::Local) => {
                            let stack_frame = self.call_stack.last_mut();
                            if let Some(src_stack_frame) = stack_frame {
                                let src_register = src_stack_frame.registers[src_index_usize].take();
                                if let Some(src_val) = src_register {
                                    match src_val {
                                        Constant::Int(src_int) => {
                                            self.accumulator = src_int as f64;
                                        }
                                        Constant::Double(src_double) => {
                                            self.accumulator = src_double;
                                        }
                                        _ => panic!("Invalid move to accumulator register!"),
                                    }
                                }
                            } else {
                                panic!("Segmentation Fault!")
                            }
                        },

                        (RegisterLocation::Global, RegisterLocation::Accumulator) => {
                            self.global[dst_index_usize] = Some(create_constant_double(&self.accumulator));
                        },
                        (RegisterLocation::Global, RegisterLocation::Global) => {
                            self.global[dst_index_usize] = self.global[src_index_usize].take();
                        },
                        (RegisterLocation::Global, RegisterLocation::Local) => {
                            let stack_frame = self.call_stack.last_mut();
                            if let Some(src_stack_frame) = stack_frame { 
                                self.global[dst_index_usize] = src_stack_frame.registers[src_index_usize].take();
                            }
                        },

                        (RegisterLocation::Local, RegisterLocation::Accumulator) => {
                            let stack_frame = self.call_stack.last_mut();
                            if let Some(dst_stack_frame) = stack_frame { 
                                dst_stack_frame.registers[dst_index_usize] = Some(create_constant_double(&self.accumulator));
                            }
                        },
                        (RegisterLocation::Local, RegisterLocation::Global) => {
                            let stack_frame = self.call_stack.last_mut();
                            if let Some(dst_stack_frame) = stack_frame {
                                dst_stack_frame.registers[dst_index_usize] = self.global[src_index_usize].take();
                            }
                        },
                        (RegisterLocation::Local, RegisterLocation::Local) => {
                            let stack_frame = self.call_stack.last_mut();
                            if let Some(dst_stack_frame) = stack_frame {
                                dst_stack_frame.registers[dst_index_usize] = dst_stack_frame.registers[src_index_usize].take();
                            }                            
                        },

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