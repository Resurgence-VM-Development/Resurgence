use crate::{Interpreter, objects::{register::Register, constant::Constant}};

impl Interpreter {
    /// Checks if 2 registers are equal
    /// 
    /// `reg_1` (`&Register`): first register
    /// `reg_2` (`&Register`): second register
    pub fn equal(&mut self, reg_1: &Register, reg_2: &Register) -> bool {
        let (const_1, const_2) = self.get_constants(reg_1, reg_2);
        match (const_1, const_2) {
            (Constant::Int(val_1), Constant::Int(val_2)) => *val_1 == *val_2,
            (Constant::Int(val_1), Constant::Double(val_2)) => (*val_1) as f64 == *val_2,
            (Constant::Double(val_1), Constant::Int(val_2)) => *val_1 == (*val_2) as f64,
            (Constant::Double(val_1), Constant::Double(val_2)) => *val_1 == *val_2,
            (Constant::String(val_1), Constant::String(val_2)) => *val_1 == *val_2,
            (Constant::Boolean(val_1), Constant::Boolean(val_2)) => *val_1 == *val_2,
            (Constant::Address(val_1), Constant::Address(val_2)) => *val_1 == *val_2,
            _ => panic!("Invalid comparison!"),
        }
    }

    /// Checks if 2 registers are not equal
    /// 
    /// `reg_1` (`&Register`): first register
    /// `reg_2` (`&Register`): second register
    pub fn not_equal(&mut self, reg_1: &Register, reg_2: &Register) -> bool {
        let (const_1, const_2) = self.get_constants(reg_1, reg_2);
        match (const_1, const_2) {
            (Constant::Int(val_1), Constant::Int(val_2)) => *val_1 != *val_2,
            (Constant::Int(val_1), Constant::Double(val_2)) => (*val_1) as f64 != *val_2,
            (Constant::Double(val_1), Constant::Int(val_2)) => *val_1 != (*val_2) as f64,
            (Constant::Double(val_1), Constant::Double(val_2)) => *val_1 != *val_2,
            (Constant::String(val_1), Constant::String(val_2)) => *val_1 != *val_2,
            (Constant::Boolean(val_1), Constant::Boolean(val_2)) => *val_1 != *val_2,
            (Constant::Address(val_1), Constant::Address(val_2)) => *val_1 != *val_2,
            _ => panic!("Invalid comparison!"),
        }
    }

    /// Checks if a register is greater then another 
    /// 
    /// `reg_1` (`&Register`): first register
    /// `reg_2` (`&Register`): second register
    pub fn greater_than(&mut self, reg_1: &Register, reg_2: &Register) -> bool {
        let (const_1, const_2) = self.get_constants(reg_1, reg_2);
        match (const_1, const_2) {
            (Constant::Int(val_1), Constant::Int(val_2)) => *val_1 > *val_2,
            (Constant::Int(val_1), Constant::Double(val_2)) => (*val_1) as f64 > *val_2,
            (Constant::Double(val_1), Constant::Int(val_2)) => *val_1 > (*val_2) as f64,
            (Constant::Double(val_1), Constant::Double(val_2)) => *val_1 > *val_2,
            _ => panic!("Invalid comparison!"),
        }
    }

    /// Checks if a register is less than another register
    /// 
    /// `reg_1` (`&Register`): first register
    /// `reg_2` (`&Register`): second register
    pub fn less_than(&mut self, reg_1: &Register, reg_2: &Register) -> bool {
        let (const_1, const_2) = self.get_constants(reg_1, reg_2);
        match (const_1, const_2) {
            (Constant::Int(val_1), Constant::Int(val_2)) => *val_1 < *val_2,
            (Constant::Int(val_1), Constant::Double(val_2)) => ((*val_1) as f64) < *val_2,
            (Constant::Double(val_1), Constant::Int(val_2)) => *val_1 < (*val_2) as f64,
            (Constant::Double(val_1), Constant::Double(val_2)) => *val_1 < *val_2,
            _ => panic!("Invalid comparison!")
        }
    }


    /// Checks if a register is greater than or equal to another
    /// 
    /// `reg_1` (`&Register`): first register
    /// `reg_2` (`&Register`): second register
    pub fn greater_or_equal(&mut self, reg_1: &Register, reg_2: &Register) -> bool {
        let (const_1, const_2) = self.get_constants(reg_1, reg_2);
        match (const_1, const_2) {
            (Constant::Int(val_1), Constant::Int(val_2)) => *val_1 >= *val_2,
            (Constant::Int(val_1), Constant::Double(val_2)) => (*val_1) as f64 >= *val_2,
            (Constant::Double(val_1), Constant::Int(val_2)) => *val_1 >= (*val_2) as f64,
            (Constant::Double(val_1), Constant::Double(val_2)) => *val_1 >= *val_2,
            _ => panic!("Invalid comparison!")
        }
    }

    /// Checks if a register is less then or equal to another
    /// 
    /// `reg_1` (`&Register`): first register
    /// `reg_2` (`&Register`): second register
    pub fn less_or_equal(&mut self, reg_1: &Register, reg_2: &Register) -> bool {
        let (const_1, const_2) = self.get_constants(reg_1, reg_2);
        match (const_1, const_2) {
            (Constant::Int(val_1), Constant::Int(val_2)) => *val_1 <= *val_2,
            (Constant::Int(val_1), Constant::Double(val_2)) => (*val_1) as f64 <= *val_2,
            (Constant::Double(val_1), Constant::Int(val_2)) => *val_1 <= (*val_2) as f64,
            (Constant::Double(val_1), Constant::Double(val_2)) => *val_1 <= *val_2,
            _ => panic!("Invalid comparison!")
        }
    }
}