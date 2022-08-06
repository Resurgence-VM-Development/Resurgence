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

    /// Copies a constant from the constant pool
    /// 
    /// `index` (`usize`): index of the constant
    pub fn cpy_constant(&self, index: usize) -> Constant {
        self.constant_pool[index].clone()
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

    pub fn mov_dst_math(&mut self, dst: &Register, value: Constant) {
        // Destination register itself
        let Register(dst_index, dst_loc) = dst; let dst_index_usize = *dst_index as usize;

        // Get the location of the destination register
        match *dst_loc {
            RegisterLocation::ConstantPool => panic!("Segmentation fault! Can not assign to a constant!"),
            RegisterLocation::Accumulator => {
                match value {
                    Constant::Int(int_value) => self.accumulator = int_value as f64,
                    Constant::Double(double_value) => self.accumulator = double_value,
                    _ => panic!("Invalid type!")
                }
            } 
            RegisterLocation::Global => self.global[dst_index_usize] = Some(value),
            RegisterLocation::Local => {
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = Some(value);
            }
        }
    }
}
