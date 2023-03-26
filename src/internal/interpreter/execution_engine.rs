use super::super::{execution_engine::ExecutionEngine, interpreter::Interpreter};
use crate::{objects::{
    instruction::Instruction, stackframe::StackFrame, resurgence_error::{ResurgenceError, ResurgenceErrorKind, ResurgenceContext}
}, create_new_trace};

/// Creates a `ResurgenceContext` object
/// 
/// ```
/// let context = create_context(instance, instruction, instruction_pointer, recursion_depth);
/// ```
///
/// This assumes whoever is using this knows what to input as the parameters
macro_rules! create_context {
    ($self:expr, $ins:expr, $ip:expr) =>
    {
        ResurgenceContext {
            call_stack: $self.call_stack.clone(), 
            constant_stack: $self.stack.clone(),
            rust_and_native_fns: $self.rust_functions.clone(),
            instruction: $ins,
            instruction_pointer: $ip,
            recursion_depth: $self.current_recursion_depth,
        }
    }
}

impl ExecutionEngine for Interpreter {
    /// Execute Resurgence Instructions
    fn execute_instruction(&mut self, start_index: usize) -> Result<(), ResurgenceError> {
        // Resolve imports if the programmer already hasn't done so 
        if !self.code_holder.resolved_imports {
            let res = self.resolve_imports();
            if let Err(mut err) = res {
                // This will always occur in the first call
                err.context = Some(create_context!(self, vec![Instruction::Ret], vec![0]));
                err.add_trace(&format!("{}: line {}", file!(), line!()));
                return Err(err);
            }
        }
        let mut index = start_index;
        let max_length = self.code_holder.instructions.len();
        while index < max_length {
            // Move operation out of vector
            let operation = self.code_holder.instructions[index].take().unwrap();
            let ins_index = index;

            // To encourage the compiler to optimze extra bounds checks
            assert!(ins_index < max_length);
            assert!(ins_index == index);

            // Instruction evaluation
            match operation {
                Instruction::Alloc(ref register_amount) => {
                    self.call_stack.push(StackFrame::from(*register_amount))
                }
                Instruction::FrameAlloc(ref register_amount, ref location) => {
                    match *location {
                        crate::objects::register::RegisterLocation::Global => {
                            for _ in 0..*register_amount {
                                self.global.push(Option::None);
                            }
                        },
                        crate::objects::register::RegisterLocation::Local => {
                            let stackframe = self.call_stack.last_mut().unwrap();
                            for _ in 0..*register_amount {
                                stackframe.registers.push(Option::None);
                            }
                        }
                        _ => {
                            let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Attempted to add more memory to an invalid location!");
                            let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                            context.instruction.push(operation);
                            context.instruction_pointer.push(index);
                            create_new_trace!(err);
                            return Err(err);
                        }
                    }
                }
                Instruction::Free(ref block_amount) => {
                    for _ in 0..*block_amount {
                        self.call_stack.pop();
                    }
                }
                Instruction::FrameFree(ref register_amount, ref location) => {
                    match *location {
                        crate::objects::register::RegisterLocation::Global => {
                            for _ in 0..*register_amount {
                                self.global.pop();
                            }
                        },
                        crate::objects::register::RegisterLocation::Local => {
                            let stackframe = self.call_stack.last_mut().unwrap();
                            for _ in 0..*register_amount {
                                stackframe.registers.pop();
                            }
                        },
                        _ => {
                            let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not allocate more memory outside of local and global memory.");
                            let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                            context.instruction.push(operation);
                            context.instruction_pointer.push(index);
                            create_new_trace!(err);
                            return Err(err);
                        }
                    }
                }
                Instruction::Jump(ref jmp_amount) => {
                    index = (index as i64 + jmp_amount) as usize;
                    self.code_holder.instructions[ins_index] = Some(operation);
                    continue;
                }
                Instruction::Call(ref func_index) => {
                    let res = self.execute_instruction(*func_index as usize);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                },
                Instruction::ExtCall(ref func_reg) => {
                    let res = self.ext_call(*func_reg);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err); 
                        return Err(err);
                    }
                },
                Instruction::Ret => {
                    self.code_holder.instructions[ins_index] = Some(operation);
                    return Result::Ok(());
                },

                Instruction::Mov(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => {
                    let res = self.mov_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::Cpy(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => {
                    let res = self.cpy_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::Ref(ref dst_reg, ref dst_reg_ref, ref src_reg, ref src_reg_ref) => {
                    let res = self.ref_registers(dst_reg, dst_reg_ref, src_reg, src_reg_ref);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }

                Instruction::StackPush(ref register, ref reference) => {
                    let res = self.push_on_stack(register, reference);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::StackMov(ref register, ref reference) => {
                    let res = self.stack_mov(register, reference);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::StackPop => {
                    self.stack.pop();
                } 

                Instruction::Add(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.add(dst_reg, reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::Sub(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.sub(dst_reg, reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::Mul(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.mul(dst_reg, reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::Div(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.div(dst_reg, reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }
                Instruction::Mod(ref dst_reg, ref reg_1, ref reg_2) => {
                    let res = self.modlo(dst_reg, reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                }

                Instruction::Equal(ref reg_1, ref reg_2) => {
                    let res = self.equal(reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                    if res.unwrap() {
                        index += 1;
                    }
                }
                Instruction::NotEqual(ref reg_1, ref reg_2) => {
                    let res = self.not_equal(reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                    if res.unwrap() {
                        index += 1;
                    }
                }
                Instruction::Greater(ref reg_1, ref reg_2) => {
                    let res = self.greater_than(reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                    if res.unwrap() {
                        index += 1;
                    }
                }
                Instruction::Less(ref reg_1, ref reg_2) => {
                    let res = self.less_than(reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                    if res.unwrap() {
                        index += 1;
                    }
                }
                Instruction::GreaterEqual(ref reg_1, ref reg_2) => {
                    let res = self.greater_or_equal(reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                    if res.unwrap() {
                        index += 1;
                    }
                }
                Instruction::LessEqual(ref reg_1, ref reg_2) => {
                    let res = self.less_or_equal(reg_1, reg_2);
                    if let Err(mut err) = res {
                        let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                        context.instruction.push(operation);
                        context.instruction_pointer.push(index);
                        create_new_trace!(err);
                        return Err(err);
                    }
                    if res.unwrap() {
                        index += 1;
                    }
                }
                _ => {
                    let mut err = ResurgenceError::from(ResurgenceErrorKind::I_GOOFED_UP, "Either this bytecode operation is from a future version of RVM or God himself because I don't know what to do with it");
                    let context = err.context.get_or_insert_with(|| create_context!(self, vec![], vec![]));
                    context.instruction.push(operation);
                    context.instruction_pointer.push(index);
                    create_new_trace!(err);
                    return Err(err);
                }
            }

            // Store instruction back into memory and increment index
            self.code_holder.instructions[ins_index] = Some(operation);
            index += 1;
        }
        Result::Ok(())
    }

    // Execute an exported function.
    fn execute_function(&mut self, func_name: &str) -> Result<(), ResurgenceError> {
        match self.code_holder.exports.get(func_name) {
            // This call is the first call of the instace
            Some(inst) => self.execute_instruction(*inst as usize),
            None => {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::FUNCTION_DOES_NOT_EXIST, 
                &format!("Function {} does not exist!", func_name));
                create_new_trace!(err);
                Err(err)
            }
        }
    }
}
