use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Read};
use std::io::{Error, ErrorKind};
use std::result::Result;
use std::slice;

use crate::objects::codeholder::CodeHolder;
use crate::objects::instruction::Instruction;
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

/// Creates a register instance from 5 bytes
fn make_register(cur: &mut Cursor<&Vec<u8>>) -> Result<Register, Error> {
    let reg = cur.read_u32::<BigEndian>()?;
    let mut locval: u8 = 0;
    cur.read(slice::from_mut(&mut locval))?;

    let regloc = match locval {
        0x01 => RegisterLocation::ConstantPool,
        0x02 => RegisterLocation::Accumulator,
        0x03 => RegisterLocation::Global,
        0x04 => RegisterLocation::Local,
        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Invalid RegisterLocation value {} at position {}",
                    locval,
                    cur.position() - 1
                ),
            ));
        }
    };

    Ok(Register(reg, regloc))
}

/// Creates a register reference
fn make_rref(cur: &mut Cursor<&Vec<u8>>) -> Result<RegisterReference, Error> {
    let mut v: u8 = 0;
    cur.read(slice::from_mut(&mut v))?;

    let rref = match v {
        0x01 => RegisterReference::AsIs,
        0x02 => RegisterReference::Dereference,
        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Invalid RegisterReference value {} at position {}",
                    v,
                    cur.position() - 1
                ),
            ));
        }
    };

    Ok(rref)
}

/// Opens and reads bytecode from a file and parses it into a usable
/// CodeHolder.
pub fn read_bytecode_file(path: &str) -> Result<CodeHolder, Error> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new(); // has subtype u8
    file.read_to_end(&mut buf)?;

    let holder = read_bytecode(&buf)?;

    Ok(holder)
}

/// Parses bytecode contained in a Vec<u8> and returns a usable CodeHolder.
pub fn read_bytecode(buf: &Vec<u8>) -> Result<CodeHolder, Error> {
    let len = buf.len();

    let mut cur = Cursor::new(buf);
    let mut holder = CodeHolder {
        instructions: Vec::new(),
        constant_pool: Vec::new(),
    };

    // check if this is a rvm bytecode file
    // 52564D88
    if !(cur.read_u32::<BigEndian>()? == 0x52564D88) {
        return Err(Error::new(
            ErrorKind::Other,
            "Invalid bytecode (Header missing)",
        ));
    }

    // check if bytecode version is supported
    let ver = cur.read_u16::<BigEndian>()?;
    if ver != 1 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Unsupported bytecode version {}", ver),
        ));
    }

    // read bytecode into vector
    loop {
        if cur.position() == (len as u64) {
            break;
        }

        let mut op: u8 = 0; // opcode
        cur.read(slice::from_mut(&mut op))?;

        match op {
            0x00 => {
                // NOOP
                continue;
            }
            0x01 => {
                // Alloc
                let size = cur.read_u32::<BigEndian>()?;
                holder.instructions.push(Instruction::Alloc(size));
            }
            0x02 => {
                // Free
                let size = cur.read_u32::<BigEndian>()?;
                holder.instructions.push(Instruction::Free(size));
            }
            0x03 => {
                // Jump
                let addr = cur.read_i64::<BigEndian>()?;
                holder.instructions.push(Instruction::Jump(addr));
            }
            0x04 => {
                // Call
                let addr = cur.read_u64::<BigEndian>()?;
                holder.instructions.push(Instruction::Call(addr));
            }
            0x05 => {
                // ExtCall
                let id = cur.read_u64::<BigEndian>()?;
                holder.instructions.push(Instruction::ExtCall(id));
            }
            0x06 => {
                // Mov
                let ra = make_register(&mut cur)?;
                let aref = make_rref(&mut cur)?;
                let rb = make_register(&mut cur)?;
                let bref = make_rref(&mut cur)?;
                holder
                    .instructions
                    .push(Instruction::Mov(ra, aref, rb, bref));
            }
            0x07 => {
                // Cpy
                let ra = make_register(&mut cur)?;
                let aref = make_rref(&mut cur)?;
                let rb = make_register(&mut cur)?;
                let bref = make_rref(&mut cur)?;
                holder
                    .instructions
                    .push(Instruction::Cpy(ra, aref, rb, bref));
            }
            0x08 => {
                // Ref
                let ra = make_register(&mut cur)?;
                let aref = make_rref(&mut cur)?;
                let rb = make_register(&mut cur)?;
                let bref = make_rref(&mut cur)?;
                holder
                    .instructions
                    .push(Instruction::Ref(ra, aref, rb, bref));
            }
            0x09 => {
                // StackPush
                let reg = make_register(&mut cur)?;
                let rref = make_rref(&mut cur)?;
                holder.instructions.push(Instruction::StackPush(reg, rref));
            }
            0x0A => {
                // StackPop
                holder.instructions.push(Instruction::StackPop);
            }
            0x0B => {
                // Add
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                let rc = make_register(&mut cur)?;
                holder.instructions.push(Instruction::Add(ra, rb, rc));
            }
            0x0C => {
                // Sub
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                let rc = make_register(&mut cur)?;
                holder.instructions.push(Instruction::Sub(ra, rb, rc));
            }
            0x0D => {
                // Mul
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                let rc = make_register(&mut cur)?;
                holder.instructions.push(Instruction::Mul(ra, rb, rc));
            }
            0x0E => {
                // Div
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                let rc = make_register(&mut cur)?;
                holder.instructions.push(Instruction::Div(ra, rb, rc));
            }
            0x0F => {
                // Equal
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                holder.instructions.push(Instruction::Equal(ra, rb));
            }
            0x10 => {
                // NotEqual
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                holder.instructions.push(Instruction::NotEqual(ra, rb));
            }
            0x11 => {
                // Greater
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                holder.instructions.push(Instruction::Greater(ra, rb));
            }
            0x12 => {
                // Less
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                holder.instructions.push(Instruction::Less(ra, rb));
            }
            0x13 => {
                // GreaterEqual
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                holder.instructions.push(Instruction::GreaterEqual(ra, rb));
            }
            0x14 => {
                // LessEqual
                let ra = make_register(&mut cur)?;
                let rb = make_register(&mut cur)?;
                holder.instructions.push(Instruction::LessEqual(ra, rb));
            }
            _ => {
                // catch-all for invalid instructions
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "Unrecognized instruction {} at position {}",
                        op,
                        cur.position() - 1
                    ),
                ));
            }
        }
    }

    Ok(holder)
}
