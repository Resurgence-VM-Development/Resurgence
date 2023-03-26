use crate::{objects::{register::{Register, RegisterReference, RegisterLocation}, constant::create_constant_double}, Interpreter, create_new_trace, ResurgenceError};


impl Interpreter {
    pub(crate) fn push_on_stack(&mut self, register: &Register, reference: &RegisterReference) -> Result<(), ResurgenceError> {
        let Register(mut reg_index, mut reg_loc) = register; let mut reg_index_usize = reg_index as usize;
        if *reference == RegisterReference::Dereference {
            Register(reg_index, reg_loc) = self.dereference_register(reg_index_usize, &reg_loc); reg_index_usize = reg_index as usize;
        }
        
        match reg_loc {
            RegisterLocation::ConstantPool => self.stack.push(self.cpy_constant(reg_index_usize)),
            RegisterLocation::Accumulator => self.stack.push(create_constant_double(&self.accumulator)),
            RegisterLocation::Global => {
                let val = self.mov_global(reg_index_usize);
                if let Err(mut err) = val {
                    create_new_trace!(err);
                    return Err(err);
                }
                self.stack.push(val.unwrap());
            },
            RegisterLocation::Local => {
                let val = self.mov_local(reg_index_usize);
                self.stack.push(val);
            },
        }
        Ok(())
    }
}
