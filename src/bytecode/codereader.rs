/*!
# Bytecode Reader API
This module provides functions for reading raw bytecode data into a [`crate::CodeHolder`]
instance.
*/

use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Read};
use std::io::{Error, ErrorKind};
use std::result::Result;

use super::parser_constants as pc;
use crate::objects::codeholder::CodeHolder;
use crate::objects::constant::Constant;
use crate::objects::instruction::Instruction;
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

/// Reads a string from a cursor
fn read_string(cur: &mut Cursor<&Vec<u8>>) -> Result<String, Error> {
    let length = cur.read_u64::<BigEndian>()? as usize;
    let mut data = vec![0u8; length];
    cur.read_exact(&mut data)?;
    match String::from_utf8(data) {
        Ok(d) => Ok(d),
        Err(error) => Err(Error::new(
            ErrorKind::Other,
            format!(
                "Bad UTF-8 string at position {}: {}",
                cur.position() - 1,
                error
            ),
        )),
    }
}

/// Creates a register instance from 5 bytes
fn read_register(cur: &mut Cursor<&Vec<u8>>) -> Result<Register, Error> {
    let reg = cur.read_u32::<BigEndian>()?;

    let regloc = read_reg_loc(cur)?;

    Ok(Register(reg, regloc))
}

// Creates a register location value from a single byte
fn read_reg_loc(cur: &mut Cursor<&Vec<u8>>) -> Result<RegisterLocation, Error> {
    let locval = cur.read_u8()?;

    let regloc = match locval {
        pc::LOC_CONSTANT => RegisterLocation::ConstantPool,
        pc::LOC_ACCUMULATOR => RegisterLocation::Accumulator,
        pc::LOC_GLOBAL => RegisterLocation::Global,
        pc::LOC_LOCAL => RegisterLocation::Local,
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
    Ok(regloc)
}

/// Creates a register reference
fn read_reg_ref(cur: &mut Cursor<&Vec<u8>>) -> Result<RegisterReference, Error> {
    let v = cur.read_u8()?;

    let rref = match v {
        pc::REF_AS_IS => RegisterReference::AsIs,
        pc::REF_DEREFERENCE => RegisterReference::Dereference,
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

fn read_constant(cur: &mut Cursor<&Vec<u8>>) -> Result<Constant, Error> {
    let ctype = cur.read_u8()?;
    match ctype {
        pc::CONST_INT => {
            // integer
            let val = cur.read_i64::<BigEndian>()?;
            return Ok(Constant::Int(val));
        }
        pc::CONST_DOUBLE => {
            // float / double
            let val = cur.read_f64::<BigEndian>()?;
            return Ok(Constant::Double(val));
        }
        pc::CONST_STRING => {
            // string
            let str = read_string(cur)?;
            return Ok(Constant::String(str));
        }
        pc::CONST_BOOLEAN => {
            // boolean
            let val = !matches!(cur.read_u8()?, 0x00);
            return Ok(Constant::Boolean(val));
        }
        pc::CONST_ADDRESS => {
            // address / register
            let val = read_register(cur)?;
            return Ok(Constant::Address(val));
        }
        pc::CONST_VEC => {
            let size = cur.read_u8()?;
            let mut vi = Vec::new();
            for _ in 0..size {
                vi.push(read_constant(cur)?);
            }
            return Ok(Constant::Vec(vi));
        }
        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Unrecognized constant type {} at position {}",
                    ctype,
                    cur.position() - 1
                ),
            ));
        }
    }
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
    let mut holder = CodeHolder::new();

    // check if this is a rvm bytecode file
    // 52564D88
    if cur.read_u32::<BigEndian>()? != pc::MAGIC_NUMBER {
        return Err(Error::new(
            ErrorKind::Other,
            "Invalid bytecode (Missing header)",
        ));
    }

    // check if bytecode version is supported
    let vmajor = cur.read_u16::<BigEndian>()?;
    let vminor = match vmajor {
        1 | 2 | 3 | 4 | 5 | 6 => 0, // match old formats that don't contain a minor version
        _ => cur.read_u16::<BigEndian>()?,
    };

    if vmajor != pc::VER_MAJOR || vminor > pc::VER_MINOR {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Unsupported bytecode version {}.{} ({}.{} supported)",
                vmajor,
                vminor,
                pc::VER_MAJOR,
                pc::VER_MINOR
            ),
        ));
    }

    // constants table
    let clen = cur.read_u32::<BigEndian>()?;
    for _ in 0..clen {
        holder.constant_pool.push(read_constant(&mut cur)?);
    }

    // read imports table
    let ilen = cur.read_u64::<BigEndian>()?;
    for _ in 0..ilen {
        let import_func = read_string(&mut cur)?;
        holder.imports.push(import_func);
    }

    // read exports table
    let elen = cur.read_u64::<BigEndian>()?;
    for _ in 0..elen {
        let export_name = read_string(&mut cur)?;
        let export_pos = cur.read_u64::<BigEndian>()?;
        holder.exports.insert(export_name, export_pos);
    }

    // read bytecode into vector
    loop {
        if cur.position() == (len as u64) {
            break;
        }

        let op = cur.read_u8()?; // opcode

        match op {
            pc::INST_NOOP => {
                // NOOP
                continue;
            }
            pc::INST_ALLOC => {
                // Alloc
                let size = cur.read_u32::<BigEndian>()?;
                holder.instructions.push(Some(Instruction::Alloc(size)));
            }
            pc::INST_FRAME_ALLOC => {
                // FrameAlloc
                let size = cur.read_u32::<BigEndian>()?;
                holder
                    .instructions
                    .push(Some(Instruction::FrameAlloc(size, read_reg_loc(&mut cur)?)));
            }
            pc::INST_FREE => {
                // Free
                let size = cur.read_u32::<BigEndian>()?;
                holder.instructions.push(Some(Instruction::Free(size)));
            }
            pc::INST_FRAME_FREE => {
                // FrameFree
                let size = cur.read_u32::<BigEndian>()?;
                holder
                    .instructions
                    .push(Some(Instruction::FrameFree(size, read_reg_loc(&mut cur)?)));
            }
            pc::INST_JUMP => {
                // Jump
                let addr = cur.read_i64::<BigEndian>()?;
                holder.instructions.push(Some(Instruction::Jump(addr)));
            }
            pc::INST_CALL => {
                // Call
                let addr = cur.read_u64::<BigEndian>()?;
                holder.instructions.push(Some(Instruction::Call(addr)));
            }
            pc::INST_EXTCALL => {
                // ExtCall
                let id = cur.read_u64::<BigEndian>()?;
                holder.instructions.push(Some(Instruction::ExtCall(id)));
            }
            pc::INST_RET => {
                // Ret
                holder.instructions.push(Some(Instruction::Ret));
            }
            pc::INST_MOV => {
                // Mov
                let ra = read_register(&mut cur)?;
                let aref = read_reg_ref(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let bref = read_reg_ref(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::Mov(ra, aref, rb, bref)));
            }
            pc::INST_CPY => {
                // Cpy
                let ra = read_register(&mut cur)?;
                let aref = read_reg_ref(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let bref = read_reg_ref(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::Cpy(ra, aref, rb, bref)));
            }
            pc::INST_REF => {
                // Ref
                let ra = read_register(&mut cur)?;
                let aref = read_reg_ref(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let bref = read_reg_ref(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::Ref(ra, aref, rb, bref)));
            }
            pc::INST_STACK_PUSH => {
                // StackPush
                let reg = read_register(&mut cur)?;
                let rref = read_reg_ref(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::StackPush(reg, rref)));
            }
            pc::INST_STACK_MOV => {
                // StackMov
                let reg = read_register(&mut cur)?;
                let rref = read_reg_ref(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::StackMov(reg, rref)));
            }
            pc::INST_STACK_POP => {
                // StackPop
                holder.instructions.push(Some(Instruction::StackPop));
            }
            pc::INST_ADD => {
                // Add
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let rc = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Add(ra, rb, rc)));
            }
            pc::INST_SUB => {
                // Sub
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let rc = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Sub(ra, rb, rc)));
            }
            pc::INST_MUL => {
                // Mul
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let rc = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Mul(ra, rb, rc)));
            }
            pc::INST_DIV => {
                // Div
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let rc = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Div(ra, rb, rc)));
            }
            pc::INST_MOD => {
                // Mod
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                let rc = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Mod(ra, rb, rc)));
            }
            pc::INST_EQUAL => {
                // Equal
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Equal(ra, rb)));
            }
            pc::INST_NOT_EQUAL => {
                // NotEqual
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::NotEqual(ra, rb)));
            }
            pc::INST_GREATER => {
                // Greater
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Greater(ra, rb)));
            }
            pc::INST_LESS => {
                // Less
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                holder.instructions.push(Some(Instruction::Less(ra, rb)));
            }
            pc::INST_GREATER_EQUAL => {
                // GreaterEqual
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::GreaterEqual(ra, rb)));
            }
            pc::INST_LESS_EQUAL => {
                // LessEqual
                let ra = read_register(&mut cur)?;
                let rb = read_register(&mut cur)?;
                holder
                    .instructions
                    .push(Some(Instruction::LessEqual(ra, rb)));
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
