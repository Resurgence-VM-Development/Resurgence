use crate::constant::Constant;
/// `StackFrame`: Represents a stack frame.
pub struct StackFrame {
    /// Represents the registers of the stack frame
    pub registers: Vec<Option<Constant>>,
}

impl StackFrame {
    /// Moves a value out of a register
    /// 
    /// `index` (`usize`): index of register
    pub fn mov_register(&mut self, index: usize) -> Constant {
        let ret = self.registers[index].take().expect("Non-existant register!");
        self.registers[index] = Some(Constant::Boolean(false));
        ret
    }

    /// Copys a value out of a register
    /// 
    /// `index` (`usize`): index of register
    pub fn cpy_register(&mut self, index: usize) -> Constant {
        self.registers[index].clone().expect("Non-existant register!")
    }

    /// References a value out of a register
    /// 
    /// `index` (`usize`): index of register
    pub fn ref_register(&self, index: usize) -> &Constant {
        self.registers[index].as_ref().expect("Non-existant register!")
    }
}

impl From<u32> for StackFrame
{
    /// Constructs a `StackFrame` object from a `u32`.
    /// 
    /// `size` (`u32`): Amount of elements to preallocate
    fn from(size: u32) -> Self {
        let mut new_frame = StackFrame { 
            registers: Vec::new() 
        };
        new_frame.registers.resize(size as usize, Option::Some(Constant::Int(0)));
        new_frame
    }
}

