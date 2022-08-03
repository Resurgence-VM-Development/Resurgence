use crate::{Interpreter, objects::{constant::Constant, stackframe::StackFrame, register::{Register, RegisterLocation}}};

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

    /// Returns a reference to the last stackframe
    pub fn ref_stack_frame(&mut self) -> &mut StackFrame {
        self.call_stack.last_mut().unwrap()
    }

    pub fn dereference_register(&mut self, index: usize, reg_loc: &RegisterLocation) -> Register {
        match reg_loc {
            RegisterLocation::Global => {
                let register = self.ref_global(index); // get the register that stores the address
                if let Constant::Address(dref_reg) = register {
                    dref_reg.clone()
                } else {
                    panic!("Dereferencing requires address")
                }
            },
            RegisterLocation::Local => {
                let stack_frame = self.ref_stack_frame(); // reference the last stackframe
                let src_reg = stack_frame.ref_register(index); // get the register that stores the address
                if let Constant::Address(dref_reg) = src_reg {
                    dref_reg.clone()
                } else {
                    panic!("Must dereference a address!")
                }
            },
            _ => panic!("Invalid dereference operation!")
        }
    }
}
