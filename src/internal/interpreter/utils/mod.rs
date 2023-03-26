use crate::{
    objects::{
        constant::Constant,
        register::{Register, RegisterLocation},
        stackframe::StackFrame, resurgence_error::ResurgenceErrorKind,
    },
    Interpreter, ResurgenceError, create_new_trace,
};

impl Interpreter {
    /// Moves a value from a global register and returns that value, or an error if:
    /// - Register is beyond bounds
    /// - Register contains a `Option::None` instead of `Option::Some`
    pub(crate) fn mov_global(&mut self, index: usize) -> Result<Constant, ResurgenceError> {
        if index >= self.global.len() {
            let mut err = ResurgenceError::from(ResurgenceErrorKind::REGISTER_OUT_OF_BOUNDS, "Register beyond bounds!");
            create_new_trace!(err);
            return Err(err);
        }
        let reg = self.global[index].take();
        if let None = reg {
            let mut err = ResurgenceError::from(ResurgenceErrorKind::MEMORY_ADDRESS_NONE, "Global register None!");
            create_new_trace!(err);
            return Err(err);
        }
        Ok(reg.unwrap())
    }

    /// Copies a value from a global register and returns that value, or an error if:
    /// - Register is beyond bounds
    /// - Register contains a `Option::None` instead of `Option::Some`
    pub(crate) fn cpy_global(&mut self, index: usize) -> Result<Constant, ResurgenceError> {
        if index >= self.global.len() {
            let mut err = ResurgenceError::from(ResurgenceErrorKind::REGISTER_OUT_OF_BOUNDS, "Register beyond bounds!");
            create_new_trace!(err);
            return Err(err);
        }
        let reg = self.global[index].clone();
        if let None = reg {
            let mut err = ResurgenceError::from(ResurgenceErrorKind::MEMORY_ADDRESS_NONE, "Global register None!");
            create_new_trace!(err);
            return Err(err);
        }
        Ok(reg.unwrap())
    }

    /// Returns a reference to a global register
    ///
    /// `index` (`usize`): index of register
    pub(crate) fn ref_global(&self, index: usize) -> &Constant {
        self.global[index].as_ref().expect("Non-existant register!")
    }

    /// Moves a local register
    ///
    /// `index` (`usize`): index of register
    pub(crate) fn mov_local(&mut self, index: usize) -> Constant {
        let stack_frame = self.ref_stack_frame();
        stack_frame.mov_register(index)
    }

    /// Copies a local register
    ///
    /// `index` (`usize`): index of register
    pub(crate) fn cpy_local(&mut self, index: usize) -> Constant {
        let stack_frame = self.ref_stack_frame();
        stack_frame.cpy_register(index)
    }

    /// References a local register
    ///
    /// `index` (`usize`): index of register
    pub(crate) fn ref_local(&self, index: usize) -> &Constant {
        let stack_frame = self.ref_stack_frame_imut();
        stack_frame.ref_register(index)
    }

    /// Copies a constant from the constant pool
    ///
    /// `index` (`usize`): index of the constant
    pub(crate) fn cpy_constant(&self, index: usize) -> Constant {
        unsafe {
            self.code_holder.constant_pool.get_unchecked(index).clone()
        }
    }

    /// References a constant from the constant pool
    /// 
    /// `index` (`usize`): index of the constant
    pub(crate) fn ref_constant(&self, index: usize) -> &Constant {
        /*
            Since the constant pool is a fixed size, the Resurgence code generator should never allow any non-existant constants
        */
        unsafe {
            self.code_holder.constant_pool.get_unchecked(index)
        }
    }

    /// Returns a reference to the last stackframe
    pub(crate) fn ref_stack_frame(&mut self) -> &mut StackFrame {
        self.call_stack.last_mut().unwrap()
    }

    pub(crate) fn ref_stack_frame_imut(&self) -> &StackFrame {
        self.call_stack.last().unwrap()
    }

    pub(crate) fn accu_const(&mut self) {
        let val = match self.accumulator_as_const {
                    Constant::Double(ref mut val) => val,
                    _ => unreachable!(),
                };
        *val = self.accumulator
    }

    pub(crate) fn dereference_register(&mut self, index: usize, reg_loc: &RegisterLocation) -> Register {
        match reg_loc {
            RegisterLocation::Global => {
                let register = self.ref_global(index); // get the register that stores the address
                if let Constant::Address(dref_reg) = register {
                    *dref_reg
                } else {
                    panic!("Dereferencing requires address")
                }
            }
            RegisterLocation::Local => {
                let stack_frame = self.ref_stack_frame(); // reference the last stackframe
                let src_reg = stack_frame.ref_register(index); // get the register that stores the address
                if let Constant::Address(dref_reg) = src_reg {
                    *dref_reg
                } else {
                    panic!("Must dereference a address!")
                }
            }
            _ => panic!("Invalid dereference operation!"),
        }
    }

    /// Takes 2 `Register` objects, and returns 2 `Constant` objects
    pub(crate) fn get_constants(&mut self, reg_1: &Register, reg_2: &Register) -> (&Constant, &Constant) {
        let Register(index_1, loc_1) = reg_1;
        let Register(index_2, loc_2) = reg_2;
        let index_1_usize = *index_1 as usize;
        let index_2_usize = *index_2 as usize;

        let const_1: &Constant;
        let const_2: &Constant;
        match (loc_1, loc_2) {
            (RegisterLocation::ConstantPool, RegisterLocation::ConstantPool) => {
                const_1 = self.ref_constant(index_1_usize);
                const_2 = self.ref_constant(index_2_usize);
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Accumulator) => {
                self.accu_const();
                const_1 = self.ref_constant(index_1_usize);
                const_2 = &self.accumulator_as_const;
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Global) => {
                const_1 = self.ref_constant(index_1_usize);
                const_2 = self.ref_global(index_2_usize);
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Local) => {
                const_1 = self.ref_constant(index_1_usize);
                const_2 = self.ref_local(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::ConstantPool) => {
                self.accu_const();
                const_1 = &self.accumulator_as_const;
                const_2 = self.ref_constant(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::Accumulator) => {
                self.accu_const();
                const_1 = &self.accumulator_as_const;
                const_2 = &self.accumulator_as_const;
            },
            (RegisterLocation::Accumulator, RegisterLocation::Global) => {
                self.accu_const();
                const_1 = &self.accumulator_as_const;
                const_2 = self.ref_global(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::Local) => {
                self.accu_const();
                const_1 = &self.accumulator_as_const;
                const_2 = self.ref_local(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::ConstantPool) => {
                const_1 = self.ref_global(index_1_usize);
                const_2 = self.ref_constant(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::Accumulator) => {
                self.accu_const();
                const_1 = self.ref_global(index_1_usize);
                const_2 = &self.accumulator_as_const;
            },
            (RegisterLocation::Global, RegisterLocation::Global) => {
                const_1 = self.ref_global(index_1_usize);
                const_2 = self.ref_global(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::Local) => {
                const_1 = self.ref_global(index_1_usize);
                const_2 = self.ref_local(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::ConstantPool) => {
                const_1 = self.ref_local(index_1_usize);
                const_2 = self.ref_constant(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::Accumulator) => {
                self.accu_const();
                const_1 = self.ref_local(index_1_usize);
                const_2 = &self.accumulator_as_const;
            },
            (RegisterLocation::Local, RegisterLocation::Global) => {
                const_1 = self.ref_local(index_1_usize);
                const_2 = self.ref_global(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::Local) => {
                const_1 = self.ref_local(index_1_usize);
                const_2 = self.ref_local(index_2_usize);
            }
        }
        (const_1, const_2)
    }
}
