use byteorder::{BigEndian, WriteBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::result::Result;

use crate::objects::codeholder::CodeHolder;
use crate::objects::instruction::Instruction;
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

fn write_register(buf: &mut Vec<u8>, r: &Register) -> Result<(), Error> {
    buf.write_u32::<BigEndian>(r.0)?;
    buf.push(match r.1 {
        RegisterLocation::ConstantPool => 0x01,
        RegisterLocation::Accumulator => 0x02,
        RegisterLocation::Global => 0x03,
        RegisterLocation::Local => 0x04,
    });
    return Ok(());
}

fn write_rref(buf: &mut Vec<u8>, rref: &RegisterReference) {
    buf.push(match rref {
        RegisterReference::AsIs => 0x01,
        RegisterReference::Dereference => 0x02,
    });
}

/// Takes a CodeHolder and generates and writes bytecode to a file.
pub fn write_bytecode_file(code: &CodeHolder, path: &str) -> Result<(), Error> {
    let mut f = File::create(path)?;

    let data = write_bytecode(&code)?;
    f.write_all(&data)?;

    return Ok(());
}

/// Takes a CodeHolder and outputs a vec containing bytecode in binary form.
pub fn write_bytecode(code: &CodeHolder) -> Result<Vec<u8>, Error> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend([0x52, 0x56, 0x4D, 0x88, 0x00, 0x01]); // write header (4 bytes) + version (2 bytes)

    for i in &(code.instructions) {
        match i {
            Instruction::Alloc(size) => {
                buf.push(0x01);
                buf.write_u32::<BigEndian>(*size)?;
            }
            Instruction::Free(size) => {
                buf.push(0x02);
                buf.write_u32::<BigEndian>(*size)?;
            }
            Instruction::Jump(addr) => {
                buf.push(0x03);
                buf.write_i64::<BigEndian>(*addr)?;
            }
            Instruction::Call(addr) => {
                buf.push(0x04);
                buf.write_u64::<BigEndian>(*addr)?;
            }
            Instruction::ExtCall(id) => {
                buf.push(0x05);
                buf.write_u64::<BigEndian>(*id)?;
            }
            Instruction::Mov(ra, aref, rb, bref) => {
                buf.push(0x06);
                write_register(&mut buf, ra)?;
                write_rref(&mut buf, aref);
                write_register(&mut buf, rb)?;
                write_rref(&mut buf, bref);
            }
            Instruction::Cpy(ra, aref, rb, bref) => {
                buf.push(0x07);
                write_register(&mut buf, ra)?;
                write_rref(&mut buf, aref);
                write_register(&mut buf, rb)?;
                write_rref(&mut buf, bref);
            }
            Instruction::Ref(ra, aref, rb, bref) => {
                buf.push(0x08);
                write_register(&mut buf, ra)?;
                write_rref(&mut buf, aref);
                write_register(&mut buf, rb)?;
                write_rref(&mut buf, bref);
            }
            Instruction::StackPush(reg, rref) => {
                buf.push(0x09);
                write_register(&mut buf, reg)?;
                write_rref(&mut buf, rref);
            }
            Instruction::StackPop => {
                buf.push(0x0A);
            }
            Instruction::Add(ra, rb, rc) => {
                buf.push(0x0B);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Sub(ra, rb, rc) => {
                buf.push(0x0C);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Mul(ra, rb, rc) => {
                buf.push(0x0D);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Div(ra, rb, rc) => {
                buf.push(0x0E);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
                write_register(&mut buf, rc)?;
            }
            Instruction::Equal(ra, rb) => {
                buf.push(0x0F);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::NotEqual(ra, rb) => {
                buf.push(0x10);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::Greater(ra, rb) => {
                buf.push(0x11);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::Less(ra, rb) => {
                buf.push(0x12);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::GreaterEqual(ra, rb) => {
                buf.push(0x13);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
            Instruction::LessEqual(ra, rb) => {
                buf.push(0x14);
                write_register(&mut buf, ra)?;
                write_register(&mut buf, rb)?;
            }
        }
    }

    return Ok(buf);
}
