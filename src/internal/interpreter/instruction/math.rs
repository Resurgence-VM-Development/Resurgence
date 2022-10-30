use std::io::Error;

use crate::{Interpreter, objects::register::{Register, RegisterLocation}, objects::constant::{Constant}};


impl Interpreter {
    /*
        Private utility functions used by this module
    */

    /// Moves a value to the destination register
    /// 
    /// `dst` (`&Register`): Destination register
    /// `value` (`&Constant`): Constant being moved
    fn mov_dst(&mut self, dst: &Register, value: Constant) {
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

    /*
        All of the actual math functions used in the execution engine
    */
    pub fn add(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) -> Result<(), Error> {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let res = constant_1.add(constant_2);
        if let Err(err) = res {
            return Err(err);
        }
        self.mov_dst(dst, res.unwrap());
        Ok(())
    }

    pub fn sub(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) -> Result<(), Error> {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let res = constant_1.sub(constant_2);
        if let Err(err) = res {
            return Err(err);
        }
        self.mov_dst(dst, res.unwrap());
        Ok(())
    }

    pub fn mul(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) -> Result<(), Error> {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let res = constant_1.mul(constant_2);
        if let Err(err) = res {
            return Err(err);
        }
        self.mov_dst(dst, res.unwrap());
        Ok(())
    }

    pub fn div(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register)-> Result<(), Error> {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let res = constant_1.div(constant_2);
        if let Err(err) = res {
            return Err(err);
        }
        self.mov_dst(dst, res.unwrap());
        Ok(())
    }

    pub fn modlo(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) -> Result<(), Error> {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let res = constant_1.modlo(constant_2);
        if let Err(err) = res {
            return Err(err);
        }
        self.mov_dst(dst, res.unwrap());
        Ok(())
    }
}