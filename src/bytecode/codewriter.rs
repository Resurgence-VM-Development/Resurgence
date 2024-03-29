/*!
# Bytecode Writer API
This module provides functions for writing a [`crate::CodeHolder`] into bytecode. This is not
particularly useful for a pure virtual machine, but is useful for compilers and interpreters.
*/

use byteorder::{BigEndian, WriteBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::result::Result;

use super::parser_constants as pc;
use crate::objects::codeholder::CodeHolder;
use crate::objects::constant::Constant;
use crate::objects::instruction::Instruction;
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

fn write_string(buf: &mut Vec<u8>, val: &str) -> Result<(), Error> {
    let bytes = val.to_owned().into_bytes();
    buf.write_u64::<BigEndian>(bytes.len() as u64)?;
    buf.write_all(&bytes)?;
    Ok(())
}

fn write_register(buf: &mut Vec<u8>, r: &Register) -> Result<(), Error> {
    buf.write_u32::<BigEndian>(r.0)?;
    write_reg_loc(buf, &r.1)?;
    Ok(())
}

fn write_reg_loc(buf: &mut Vec<u8>, rl: &RegisterLocation) -> Result<(), Error> {
    buf.push(match *rl {
        RegisterLocation::ConstantPool => pc::LOC_CONSTANT,
        RegisterLocation::Accumulator => pc::LOC_ACCUMULATOR,
        RegisterLocation::Global => pc::LOC_GLOBAL,
        RegisterLocation::Local => pc::LOC_LOCAL,
    });
    Ok(())
}

fn write_reg_ref(buf: &mut Vec<u8>, rref: &RegisterReference) {
    buf.push(match rref {
        RegisterReference::AsIs => pc::REF_AS_IS,
        RegisterReference::Dereference => pc::REF_DEREFERENCE,
    });
}

fn write_constant(buf: &mut Vec<u8>, constant: &Constant) -> Result<(), Error> {
    match constant {
        Constant::Int(val) => {
            buf.write_u8(pc::CONST_INT)?;
            buf.write_i64::<BigEndian>(*val)?;
        }
        Constant::Double(val) => {
            buf.write_u8(pc::CONST_DOUBLE)?;
            buf.write_f64::<BigEndian>(*val)?;
        }
        Constant::String(val) => {
            buf.write_u8(pc::CONST_STRING)?;
            write_string(buf, val)?;
        }
        Constant::Boolean(val) => {
            buf.write_u8(pc::CONST_BOOLEAN)?;
            buf.write_u8(match val {
                false => 0x00,
                true => 0x01,
            })?;
        }
        Constant::Address(val) => {
            buf.write_u8(pc::CONST_ADDRESS)?;
            write_register(buf, val)?;
        }
        Constant::Vec(val) => {
            buf.write_u8(pc::CONST_VEC)?;
            for obj in val {
                write_constant(buf, obj)?;
            }
        }
    }
    Ok(())
}
/// Takes a CodeHolder and generates and writes bytecode to a file.
pub fn write_bytecode_file(code: &CodeHolder, path: &str) -> Result<(), Error> {
    let mut f = File::create(path)?;

    let data = write_bytecode(code)?;
    f.write_all(&data)?;

    Ok(())
}

/// Takes a CodeHolder and outputs a vec containing bytecode in binary form.
pub fn write_bytecode(code: &CodeHolder) -> Result<Vec<u8>, Error> {
    let mut buf: Vec<u8> = Vec::new();

    // write magic number
    buf.write_u32::<BigEndian>(pc::MAGIC_NUMBER)?;
    // write version number
    buf.write_u16::<BigEndian>(pc::VER_MAJOR)?;
    buf.write_u16::<BigEndian>(pc::VER_MINOR)?;

    // constants pool
    buf.write_u32::<BigEndian>(code.constant_pool.len() as u32)?;
    for i in &(code.constant_pool) {
        write_constant(&mut buf, i)?;
    }

    // imports table
    buf.write_u64::<BigEndian>(code.imports.len() as u64)?;
    for i in &(code.imports) {
        write_string(&mut buf, i)?;
    }

    // exports table
    buf.write_u64::<BigEndian>(code.exports.len() as u64)?;
    for (export_name, export_pos) in &(code.exports) {
        write_string(&mut buf, export_name)?;
        buf.write_u64::<BigEndian>(*export_pos)?;
    }

    // instructions
    for i in &(code.instructions) {
        let unwrapped_i = i.as_ref();
        match unwrapped_i.unwrap() {
            Instruction::Alloc(size) => {
                buf.push(pc::INST_ALLOC);
                buf.write_u32::<BigEndian>(*size)?;
            }
            Instruction::FrameAlloc(size, location) => {
                buf.push(pc::INST_FRAME_ALLOC);
                buf.write_u32::<BigEndian>(*size)?;
                write_reg_loc(&mut buf, location)?;
            }
            Instruction::Free(size) => {
                buf.push(pc::INST_FREE);
                buf.write_u32::<BigEndian>(*size)?;
            }
            Instruction::FrameFree(size, location) => {
                buf.push(pc::INST_FRAME_FREE);
                buf.write_u32::<BigEndian>(*size)?;
                write_reg_loc(&mut buf, location)?;
            }
            Instruction::Jump(addr) => {
                buf.push(pc::INST_JUMP);
                buf.write_i64::<BigEndian>(*addr)?;
            }
            Instruction::Call(addr) => {
                buf.push(pc::INST_CALL);
                buf.write_u64::<BigEndian>(*addr)?;
            }
            Instruction::ExtCall(id) => {
                buf.push(pc::INST_EXTCALL);
                buf.write_u64::<BigEndian>(*id)?;
            }
            Instruction::Ret => {
                buf.push(pc::INST_RET);
            }
            Instruction::Mov(ra, aref, rb, bref) => {
                buf.push(pc::INST_MOV);
                write_register(&mut buf, ra)?;
                write_reg_ref(&mut buf, aref);
                write_register(&mut buf, rb)?;
                write_reg_ref(&mut buf, bref);
            }
            Instruction::Cpy(ra, aref, rb, bref) => {
                buf.push(pc::INST_CPY);
                write_register(&mut buf, ra)?;
                write_reg_ref(&mut buf, aref);
                write_register(&mut buf, rb)?;
                write_reg_ref(&mut buf, bref);
            }
            Instruction::Ref(ra, aref, rb, bref) => {
                buf.push(pc::INST_REF);
                write_register(&mut buf, ra)?;
                write_reg_ref(&mut buf, aref);
                write_register(&mut buf, rb)?;
                write_reg_ref(&mut buf, bref);
            }
            Instruction::StackPush(reg, rref) => {
                buf.push(pc::INST_STACK_PUSH);
                write_register(&mut buf, reg)?;
                write_reg_ref(&mut buf, rref);
            }
            Instruction::StackMov(ra, aref) => {
                buf.push(pc::INST_STACK_MOV);
                write_register(&mut buf, ra)?;
                write_reg_ref(&mut buf, aref);
            }
            Instruction::StackPop => {
                buf.push(pc::INST_STACK_POP);
            }
            Instruction::Add(ra, rb, rc) => {
                buf.push(pc::INST_ADD);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Sub(ra, rb, rc) => {
                buf.push(pc::INST_SUB);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Mul(ra, rb, rc) => {
                buf.push(pc::INST_MUL);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Div(ra, rb, rc) => {
                buf.push(pc::INST_DIV);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Mod(ra, rb, rc) => {
                buf.push(pc::INST_MOD);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Equal(ra, rb) => {
                buf.push(pc::INST_EQUAL);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::NotEqual(ra, rb) => {
                buf.push(pc::INST_NOT_EQUAL);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::Greater(ra, rb) => {
                buf.push(pc::INST_GREATER);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::Less(ra, rb) => {
                buf.push(pc::INST_LESS);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::GreaterEqual(ra, rb) => {
                buf.push(pc::INST_GREATER_EQUAL);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::LessEqual(ra, rb) => {
                buf.push(pc::INST_LESS_EQUAL);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            _ => {
                panic!(".__. I don't reconize this instruction");
            }
        }
    }

    Ok(buf)
}
