use crate::{Interpreter, objects::register::{Register, RegisterLocation}, objects::constant::{Constant, create_constant_double}};

impl Interpreter {
    pub fn add(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) {
        let Register(index_1, loc_1) = reg_1; let Register(index_2, loc_2) = reg_2;
        let index_1_usize = *index_1 as usize; let index_2_usize = *index_2 as usize;
        let constant_1: Constant; let constant_2: Constant;
        
        match (loc_1, loc_2) {
            (RegisterLocation::ConstantPool, RegisterLocation::ConstantPool) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Accumulator) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = create_constant_double(&self.accumulator);
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Global) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Local) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = self.cpy_local(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::ConstantPool) => {
                constant_1 = create_constant_double(&self.accumulator); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::Global) => {
                constant_1 = create_constant_double(&self.accumulator); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::Local) => {
                constant_1 = create_constant_double(&self.accumulator); constant_2 = self.cpy_local(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::ConstantPool) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::Accumulator) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = create_constant_double(&self.accumulator);
            },
            (RegisterLocation::Global, RegisterLocation::Global) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::Local) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = self.cpy_local(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::ConstantPool) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::Accumulator) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = create_constant_double(&self.accumulator);
            },
            (RegisterLocation::Local, RegisterLocation::Global) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::Local) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = self.cpy_local(index_2_usize);
            },
            _ => panic!("Segmentation Fault")
        }

        // Actual value to be stored in the destination register
        let dst_value = constant_1.add(&constant_2);
        self.mov_dst_math(dst, dst_value);
    }

    pub fn sub(&mut self, dst: &Register, reg_1: &Register, reg_2: &Register) {
        let Register(index_1, loc_1) = reg_1; let Register(index_2, loc_2) = reg_2;
        let index_1_usize = *index_1 as usize; let index_2_usize = *index_2 as usize;
        let constant_1: Constant; let constant_2: Constant;

        match (loc_1, loc_2) {
            (RegisterLocation::ConstantPool, RegisterLocation::ConstantPool) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Accumulator) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = create_constant_double(&self.accumulator)
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Global) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::ConstantPool, RegisterLocation::Local) => {
                constant_1 = self.cpy_constant(index_1_usize); constant_2 = self.cpy_local(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::ConstantPool) => {
                constant_1 = create_constant_double(&self.accumulator); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::Global) => {
                constant_1 = create_constant_double(&self.accumulator); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::Accumulator, RegisterLocation::Local) => {
                constant_1 = create_constant_double(&self.accumulator); constant_2 = self.cpy_local(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::ConstantPool) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::Accumulator) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = create_constant_double(&self.accumulator);
            },
            (RegisterLocation::Global, RegisterLocation::Global) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::Global, RegisterLocation::Local) => {
                constant_1 = self.cpy_global(index_1_usize); constant_2 = self.cpy_local(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::ConstantPool) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = self.cpy_constant(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::Accumulator) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = create_constant_double(&self.accumulator);
            },
            (RegisterLocation::Local, RegisterLocation::Global) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = self.cpy_global(index_2_usize);
            },
            (RegisterLocation::Local, RegisterLocation::Local) => {
                constant_1 = self.cpy_local(index_1_usize); constant_2 = self.cpy_local(index_2_usize);
            },
            _ => panic!("Segmentation fault!")
        }

        let dst_value = constant_1.sub(&constant_2);
        self.mov_dst_math(dst, dst_value)
    }
}