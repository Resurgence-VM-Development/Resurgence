/// `RegisterLocation`: Defines the location of a register
/// 
/// Possible Values:
/// * `ConstantPool`: The value being referenced is a constant
/// * `SpecialRegister`: Register referenced is a special register
/// * `LastStackFrame`: Register is in the last stack frame
/// * `Global`: Global scope
/// * `Local`: Local scope
#[derive(Clone, Debug, PartialEq)]
pub enum RegisterLocation {
    ConstantPool,
    SpecialRegister,
    LastStackFrame,
    Global,
    Local,
}

/// `Register`: Abstraction for a virtual register.
/// 
/// `u32`: Location of the virtual register in an array
/// `RegisterLocation`: The scope of a register
#[derive(Clone, Debug, PartialEq)]
pub struct Register(u32, RegisterLocation);

/// `RegisterReference`: Defines how we refer to a register in bytecode
/// 
/// Possible Values:
/// * `AsIs`: Register in memory location holds a value
/// * `Dereference`: Register in memory location holds an address to another location
pub enum RegisterReference {
    AsIs,
    Dereference
}
