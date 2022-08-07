use crate::{Interpreter, objects::register::{Register, RegisterLocation}, objects::constant::{Constant, create_constant_double}};


impl Interpreter {
    /// Takes 2 `Register` objects, and returns 2 `Constant` objects
    fn get_constants(&mut self, reg_1: &Register, reg_2: &Register) -> (Constant, Constant) {
        let Register(index_1, loc_1) = reg_1; let Register(index_2, loc_2) = reg_2;
        let index_1_usize = *index_1 as usize; let index_2_usize = *index_2 as usize;
    
        match (loc_1, loc_2) {
            (RegisterLocation::ConstantPool, RegisterLocation::ConstantPool) => (self.cpy_constant(index_1_usize), self.cpy_constant(index_2_usize)),
            (RegisterLocation::ConstantPool, RegisterLocation::Accumulator) => (self.cpy_constant(index_1_usize), create_constant_double(&self.accumulator)),
            (RegisterLocation::ConstantPool, RegisterLocation::Global) => (self.cpy_constant(index_1_usize), self.cpy_global(index_2_usize)),
            (RegisterLocation::ConstantPool, RegisterLocation::Local) => (self.cpy_constant(index_1_usize), self.cpy_local(index_2_usize)),
            (RegisterLocation::Accumulator, RegisterLocation::ConstantPool) => (create_constant_double(&self.accumulator), self.cpy_constant(index_2_usize)),
            (RegisterLocation::Accumulator, RegisterLocation::Global) => (create_constant_double(&self.accumulator), self.cpy_global(index_2_usize)),
            (RegisterLocation::Accumulator, RegisterLocation::Local) => (create_constant_double(&self.accumulator), self.cpy_local(index_2_usize)),
            (RegisterLocation::Global, RegisterLocation::ConstantPool) => (self.cpy_global(index_1_usize), self.cpy_constant(index_2_usize)),
            (RegisterLocation::Global, RegisterLocation::Accumulator) => (self.cpy_global(index_1_usize), create_constant_double(&self.accumulator)),
            (RegisterLocation::Global, RegisterLocation::Global) => (self.cpy_global(index_1_usize), self.cpy_global(index_2_usize)),
            (RegisterLocation::Global, RegisterLocation::Local) => (self.cpy_global(index_1_usize), self.cpy_local(index_2_usize)),
            (RegisterLocation::Local, RegisterLocation::ConstantPool) => (self.cpy_local(index_1_usize), self.cpy_constant(index_2_usize)),
            (RegisterLocation::Local, RegisterLocation::Accumulator) => (self.cpy_local(index_1_usize), create_constant_double(&self.accumulator)),
            (RegisterLocation::Local, RegisterLocation::Global) => (self.cpy_local(index_1_usize), self.cpy_global(index_2_usize)),
            (RegisterLocation::Local, RegisterLocation::Local) => (self.cpy_local(index_1_usize), self.cpy_local(index_2_usize)),
            _ => panic!("Segmentation fault!")
        }
    }
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

    pub fn add(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let dst_value = constant_1.add(&constant_2);
        self.mov_dst(dst, dst_value);
    }

    pub fn sub(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let dst_value = constant_1.sub(&constant_2);
        self.mov_dst(dst, dst_value)
    }

    pub fn mul(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let dst_value = constant_1.sub(&constant_2);
        self.mov_dst(dst, dst_value);
    }

    pub fn div(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) {
        let (constant_1, constant_2) = self.get_constants(reg_1, reg_2);
        let dst_value = constant_1.div(&constant_2);
        self.mov_dst(dst, dst_value);
    }
}