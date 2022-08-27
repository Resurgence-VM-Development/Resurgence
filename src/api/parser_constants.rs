/*!
# Parser Constants
This module describes constant values used in bytecode reading and writing.
*/

/// Magic filetype header
pub(crate) const MAGIC_NUMBER: u32 = 0x52564D88;

/// Format version number
pub(crate) const VERSION: u16 = 3;

/*
 * Constant types
 */

pub(crate) const CONST_INT: u8 = 0x01;
pub(crate) const CONST_DOUBLE: u8 = 0x02;
pub(crate) const CONST_STRING: u8 = 0x03;
pub(crate) const CONST_BOOLEAN: u8 = 0x04;
pub(crate) const CONST_ADDRESS: u8 = 0x05;

/*
 * Register locations
 */
pub(crate) const LOC_CONSTANT: u8 = 0x01;
pub(crate) const LOC_ACCUMULATOR: u8 = 0x02;
pub(crate) const LOC_GLOBAL: u8 = 0x03;
pub(crate) const LOC_LOCAL: u8 = 0x04;

/*
 * Register reference types
 */
pub(crate) const REF_AS_IS: u8 = 0x01;
pub(crate) const REF_DEREFERENCE: u8 = 0x02;

/*
 * Instruction numbers
 */

/// No-op - not an actual instruction, mostly useful for debugging
pub(crate) const INST_NOOP: u8 = 0x00;

/// Alloc
pub(crate) const INST_ALLOC: u8 = 0x01;

/// Free
pub(crate) const INST_FREE: u8 = 0x02;

/// Jump
pub(crate) const INST_JUMP: u8 = 0x03;

/// Call
pub(crate) const INST_CALL: u8 = 0x04;

/// Extcall
pub(crate) const INST_EXTCALL: u8 = 0x05;

/// Mov
pub(crate) const INST_MOV: u8 = 0x06;

/// Copy
pub(crate) const INST_CPY: u8 = 0x07;

/// Ref
pub(crate) const INST_REF: u8 = 0x08;

/// Stack Push
pub(crate) const INST_STACK_PUSH: u8 = 0x09;

/// Stack Pop
pub(crate) const INST_STACK_POP: u8 = 0x0A;

/// ADd
pub(crate) const INST_ADD: u8 = 0x0B;

/// Subtract
pub(crate) const INST_SUB: u8 = 0x0C;

/// Multiply
pub(crate) const INST_MUL: u8 = 0x0D;

/// Divide
pub(crate) const INST_DIV: u8 = 0x0E;

/// Equal
pub(crate) const INST_EQUAL: u8 = 0x0F;

/// NotEqual
pub(crate) const INST_NOT_EQUAL: u8 = 0x10;

/// Greater
pub(crate) const INST_GREATER: u8 = 0x11;

/// Less
pub(crate) const INST_LESS: u8 = 0x12;

/// GreaterEqual
pub(crate) const INST_GREATER_EQUAL: u8 = 0x13;

/// LessEqual
pub(crate) const INST_LESS_EQUAL: u8 = 0x14;

// FrameAlloc
pub(crate) const INST_FRAME_ALLOC: u8 = 0x015;

// FrameFree
pub(crate) const INST_FRAME_FREE: u8 = 0x016;

// StackMov
pub(crate) const INST_STACK_MOV: u8 = 0x017;

// Mod
pub(crate) const INST_MOD: u8 = 0x018;
