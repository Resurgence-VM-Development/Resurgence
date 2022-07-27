use crate::constant::Constant;
/// `StackFrame`: Represents a stack frame.
/// 
/// `registers` (`Vec<Constant>`): Represents the registers of the stack frame
struct StackFrame {
    pub registers: Vec<Constant>,
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
        new_frame.registers.resize(size as usize, Constant::Int(0));
        new_frame
    }
}

