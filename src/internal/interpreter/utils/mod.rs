use crate::{
    objects::{
        constant::{create_constant_double, Constant},
        register::{Register, RegisterLocation},
        stackframe::StackFrame,
    },
    Interpreter,
};

impl Interpreter {
    /// Moves the value of a global register
    ///
    /// `index` (`usize`): index of register
    pub fn mov_global(&mut self, index: usize) -> Constant {
        self.global[index].take().expect("Non-existant register!")
    }

    /// Returns a copy of a global register
    ///
    /// `index` (`usize`): index of register
    pub fn cpy_global(&mut self, index: usize) -> Constant {
        self.global[index].clone().expect("Non-existant register!")
    }

    /// Returns a reference to a global register
    ///
    /// `index` (`usize`): index of register
    pub fn ref_global(&self, index: usize) -> &Constant {
        self.global[index].as_ref().expect("Non-existant register!")
    }

    /// Moves a local register
    ///
    /// `index` (`usize`): index of register
    pub fn mov_local(&mut self, index: usize) -> Constant {
        let stack_frame = self.ref_stack_frame();
        stack_frame.mov_register(index)
    }

    /// Copies a local register
    ///
    /// `index` (`usize`): index of register
    pub fn cpy_local(&mut self, index: usize) -> Constant {
        let stack_frame = self.ref_stack_frame();
        stack_frame.cpy_register(index)
    }

    /// References a local register
    ///
    /// `index` (`usize`): index of register
    pub fn ref_local(&mut self, index: usize) -> &Constant {
        let stack_frame = self.ref_stack_frame();
        stack_frame.ref_register(index)
    }

    /// Copies a constant from the constant pool
    ///
    /// `index` (`usize`): index of the constant
    pub fn cpy_constant(&self, index: usize) -> Constant {
        self.code_holder.constant_pool[index].clone()
    }

    /// Returns a reference to the last stackframe
    pub fn ref_stack_frame(&mut self) -> &mut StackFrame {
        self.call_stack.last_mut().unwrap()
    }

    pub fn dereference_register(&mut self, index: usize, reg_loc: &RegisterLocation) -> Register {
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
    pub fn get_constants(&mut self, reg_1: &Register, reg_2: &Register) -> (Constant, Constant) {
        let Register(index_1, loc_1) = reg_1;
        let Register(index_2, loc_2) = reg_2;
        let index_1_usize = *index_1 as usize;
        let index_2_usize = *index_2 as usize;

        match (loc_1, loc_2) {
            (RegisterLocation::ConstantPool, RegisterLocation::ConstantPool) => (
                self.cpy_constant(index_1_usize),
                self.cpy_constant(index_2_usize),
            ),
            (RegisterLocation::ConstantPool, RegisterLocation::Accumulator) => (
                self.cpy_constant(index_1_usize),
                create_constant_double(&self.accumulator),
            ),
            (RegisterLocation::ConstantPool, RegisterLocation::Global) => (
                self.cpy_constant(index_1_usize),
                self.cpy_global(index_2_usize),
            ),
            (RegisterLocation::ConstantPool, RegisterLocation::Local) => (
                self.cpy_constant(index_1_usize),
                self.cpy_local(index_2_usize),
            ),
            (RegisterLocation::Accumulator, RegisterLocation::ConstantPool) => (
                create_constant_double(&self.accumulator),
                self.cpy_constant(index_2_usize),
            ),
            (RegisterLocation::Accumulator, RegisterLocation::Global) => (
                create_constant_double(&self.accumulator),
                self.cpy_global(index_2_usize),
            ),
            (RegisterLocation::Accumulator, RegisterLocation::Local) => (
                create_constant_double(&self.accumulator),
                self.cpy_local(index_2_usize),
            ),
            (RegisterLocation::Global, RegisterLocation::ConstantPool) => (
                self.cpy_global(index_1_usize),
                self.cpy_constant(index_2_usize),
            ),
            (RegisterLocation::Global, RegisterLocation::Accumulator) => (
                self.cpy_global(index_1_usize),
                create_constant_double(&self.accumulator),
            ),
            (RegisterLocation::Global, RegisterLocation::Global) => (
                self.cpy_global(index_1_usize),
                self.cpy_global(index_2_usize),
            ),
            (RegisterLocation::Global, RegisterLocation::Local) => (
                self.cpy_global(index_1_usize),
                self.cpy_local(index_2_usize),
            ),
            (RegisterLocation::Local, RegisterLocation::ConstantPool) => (
                self.cpy_local(index_1_usize),
                self.cpy_constant(index_2_usize),
            ),
            (RegisterLocation::Local, RegisterLocation::Accumulator) => (
                self.cpy_local(index_1_usize),
                create_constant_double(&self.accumulator),
            ),
            (RegisterLocation::Local, RegisterLocation::Global) => (
                self.cpy_local(index_1_usize),
                self.cpy_global(index_2_usize),
            ),
            (RegisterLocation::Local, RegisterLocation::Local) => {
                (self.cpy_local(index_1_usize), self.cpy_local(index_2_usize))
            }
            _ => panic!("Segmentation fault!"),
        }
    }
}
