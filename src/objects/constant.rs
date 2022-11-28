use std::io::{Error, ErrorKind};

use super::register::{Register, RegisterLocation};

/// `Constant`: Represents a constant in the backend
/// 
/// Possible Values:
/// * `Int(i64)`
/// * `Double(f64)`
/// * `String(String)`
/// * `Boolean(bool)`
#[derive(Clone, Debug, PartialEq)]

pub enum Constant {
    Int(i64),
    Double(f64),
    String(String),
    Boolean(bool),
    Address(Register)
}

impl Constant {
    fn check_overflow(&self, value: Option<i64>) -> Result<i64, Error> {
        if value.is_none() {
            return Err(Error::new(ErrorKind::OutOfMemory, "Overflowed integer!".to_string()));
        }

        // We know there was no error, so we don't need unwrap to check for us again
        unsafe {
            Ok(value.unwrap_unchecked())
        }
    }
    /// Adds 2 numerical Constants together
    /// 
    /// `constant` (`&Constant::Int` or `&Constant::Double`): Constant you want to add to self
    /// 
    /// # Examples
    /// ```no_run
    /// let int_const = create_constant_int(&5);
    /// let res = int_const.add(&create_constant_int(&5));
    /// if let Err(err) = res {
    ///     panic!("{}", err);
    /// }
    ///
    /// assert_eq!(res.unwrap(), Constant::Int(10));
    /// ```
    pub fn add(&self, constant: &Constant) -> Result<Constant, Error> {
        match (self.clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                let res = self.check_overflow(val_1.checked_add(val_2));
                if let Err(err) = res {
                    Err(err)
                } else {
                    // If no error was returned, then we don't need to use the checked version of unwrap
                    unsafe { Ok(Constant::Int(res.unwrap_unchecked())) }
                }
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Ok(Constant::Double(val_1 + val_2))
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Ok(Constant::Double(val_1 as f64 + val_2))
            },
            _ => {
                Err(Error::new(ErrorKind::InvalidData, "Can not add non-numerical types!".to_string()))
            }
        }
    }

    /// Subtracts 2 numerical Constants together
    /// 
    /// `constant` (`&Constant::Int` or `&Constant::Double`): Constant you want to subtract from self
    /// # Examples
    /// ```no_run
    /// let int_const = create_constant_int(&5);
    /// let res = int_const.sub(&create_constant_int(&5));
    /// if let Err(err) = res {
    ///     panic!("{}", err);
    /// }
    /// 
    /// assert_eq!(res.unwrap(), Constant::Int(0));
    /// ```
    pub fn sub(&self, constant: &Constant) -> Result<Constant, Error> {
        match (self.clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                let res = self.check_overflow(val_1.checked_sub(val_2));
                if let Err(err) = res {
                    Err(err)
                } else {
                    // We know there was no error, then we don't need to use the checked version of unwrap
                    unsafe { Ok(Constant::Int(res.unwrap_unchecked())) }
                }
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Ok(Constant::Double(val_1 - val_2))
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Ok(Constant::Double(val_1 as f64 - val_2))
            },
            _ => {
                Err(Error::new(ErrorKind::InvalidData, "Can not subtract non-numerical types!"))
            }
        }
    }

    /// Multiplies 2 numerical Constants together
    /// 
    /// `constant` (`&Constant::Int` or `&Constant::Double`): Constant you want to multiply self by
    /// 
    /// # Examples
    /// ```no_run
    /// let int_const = create_constant_int(&5);
    /// let res = int_const.mul(&create_constant_int(&5));
    /// if let Err(err) = res {
    ///     panic!("{}", err);
    /// }
    /// 
    /// assert_eq!(res.unwrap(), Constant::Int(25));
    /// ```
    pub fn mul(&self, constant: &Constant) -> Result<Constant, Error> {
        match (self.clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                let res = self.check_overflow(val_1.checked_mul(val_2));
                if let Err(err) = res {
                    Err(err)
                } else {
                    // We know there was no error, so we don't need to use the checked version of unwrap
                    unsafe { Ok(Constant::Int(res.unwrap_unchecked())) }
                }
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Ok(Constant::Double(val_1 * val_2))
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Ok(Constant::Double(val_1 as f64 * val_2))
            },
            _ => {
                Err(Error::new(ErrorKind::InvalidData, "Can not multiply non-numerical types!"))
            }
        }
    }

    /// Divides 2 numerical Constants together
    /// 
    /// `constant` (`&Constant::Int` or `&Constant::Double`): Constant you want to divide self by
    /// 
    /// # Examples
    /// ```no_run
    /// let int_const = create_constant_int(&5);
    /// let res = int_const.div(&create_constant_int(&5));
    /// if let Err(err) = res {
    ///     panic!("{}", err);
    /// }
    /// 
    /// assert_eq!(res.unwrap(), Constant::Int(1));
    /// ```
    pub fn div(&self, constant: &Constant) -> Result<Constant, Error> {
        match (self.clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                let res = self.check_overflow(val_1.checked_div(val_2));
                if let Err(err) = res {
                    Err(err)
                } else {
                    // We know there was no error, so we don't need to use the checked version of unwrap
                    unsafe { Ok(Constant::Int(res.unwrap_unchecked())) }
                }
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Ok(Constant::Double(val_1 / val_2))
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Ok(Constant::Double(val_1 as f64 / val_2))
            },
            _ => {
                Err(Error::new(ErrorKind::InvalidData, "Can't divide non-numerical types!"))
            }
        }
    }

    /// Divides 2 numerical Constants together and returns the remainder
    /// 
    /// `constant` (`&Constant::Int` or `&Constant::Double`): Constant you want to divide self by
    /// 
    /// # Examples
    /// ```no_run
    /// let int_const = create_constant_int(&5);
    /// let res = int_const.modlo(&create_constant_int(&5));
    /// if let Err(err) = res {
    ///     panic!("{}", err);
    /// }
    /// 
    /// assert_eq!(res.unwrap(), Constant::Int(1));
    /// ```
    pub fn modlo(&self, constant: &Constant) -> Result<Constant, Error> {
        match (self.clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                Ok(Constant::Int(val_1 % val_2))
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Ok(Constant::Double(val_1 % val_2))
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Ok(Constant::Double(val_1 as f64 % val_2))
            },
            _ => {
                Err(Error::new(ErrorKind::InvalidData, "Can not perform division and return the remainder of non-numerical types!"))
            }
        }
    }
    
    /// Returns the type as `String` for error handling reasons
    #[inline]
    pub fn type_as_string(&self) -> String {
        match &*self {
            Constant::Int(ref int_val) => format!("i64 Constant: {}", *int_val),
            Constant::Double(ref double_val) => format!("f64 Constant: {}", *double_val),
            Constant::String(ref string_val) => format!("String Constant: {}", *string_val),
            Constant::Boolean(ref bool_val) => format!("bool Constant: {}", if *bool_val {"true"} else {"false"}),
            Constant::Address(ref address_value) => format!("Register constant: index({}) location({})", address_value.0, match address_value.1 {
                RegisterLocation::ConstantPool => "Constant Pool",
                RegisterLocation::Accumulator => "Accumulator",
                RegisterLocation::Global => "Global",
                RegisterLocation::Local => "Local"
            }),
        }
    } 
}

/// Creates a `Constant::Int`
/// 
/// `init_val` (`&i64`): The value you want to create a Constant with
/// 
/// # Examples
/// ```no_run
/// let constant = create_constant_int(&10);
/// ```
pub fn create_constant_int(init_val: &i64) -> Constant {
    Constant::Int(*init_val)
}

/// Creates a `Constant::Double`
/// 
/// `init_val` (`&f64`): The value you want to create a Constant with
/// 
/// # Examples
/// ```no_run
/// let constant = create_constant_double(&3.2);
/// ```
pub fn create_constant_double(init_val: &f64) -> Constant {
    Constant::Double(*init_val)
}

/// Creates a `Constant::String`
/// 
/// `init_val` (`&str`): The value you want to create a Constant with
///
/// # Examples
/// ```no_run
/// use smartstring::alias::String;
/// let constant = create_constant_string("Hello World!");
/// ```
pub fn create_constant_string(init_val: &str) -> Constant {
    Constant::String(String::from(init_val))
}

/// Creates a `Constant::Bool`
/// 
/// `init_val` (`&bool`): The value you want to create a Constant with
///
/// # Examples
/// ```no_run
/// let constant = create_constant_bool(&true);
/// ```
pub fn create_constant_bool(init_val: &bool) -> Constant {
    Constant::Boolean(*init_val)
}

#[cfg(test)]
mod constant_init_tests {
    use super::*;

    #[test]
    fn const_int() {
        let test_int = 99;
        let constant = create_constant_int(&test_int);
        assert_eq!(constant, Constant::Int(test_int));
    }
    
    #[test]
    fn const_double() {
        let test_double = std::f64::consts::PI;
        let constant = create_constant_double(&test_double);
        assert_eq!(constant, Constant::Double(test_double));
    }

    #[test]
    fn const_string() {
        let test_string = "Hello World";
        let constant = create_constant_string(test_string);
        assert_eq!(constant, Constant::String(String::from(test_string)))
    }

    #[test]
    fn const_bool() {
        let test_bool = true;
        let constant = create_constant_bool(&test_bool);
        assert_eq!(constant, Constant::Boolean(test_bool));
    }
}

#[cfg(test)]
mod const_impl_tests {
    use super::*;

    #[test]
    fn math_test() {
        let int_const = create_constant_int(&99);
        let double_const = create_constant_double(&1.5);

        let add_constant = int_const.add(&double_const);
        let sub_constant = int_const.sub(&double_const);
        let mul_constant = int_const.mul(&double_const);
        let div_constant = int_const.div(&double_const);

        if let Constant::Double(res) = add_constant.unwrap() {
            assert_eq!(res, 99.0 + 1.5);
        }
        if let Constant::Double(res) = sub_constant.unwrap() {
            assert_eq!(res, 99.0 - 1.5);
        }
        if let Constant::Double(res) = mul_constant.unwrap() {
            assert_eq!(res, 99.0 * 1.5);
        }
        if let Constant::Double(res) = div_constant.unwrap() {
            assert_eq!(res, 99.0 / 1.5);
        }
    }

    #[test]
    fn concat_test() {
        let (hello, world) = (create_constant_string("Hello "), create_constant_string("World!"));
        let hello_world = hello.concat(&world);
        if let Err(err) = hello_world {
            panic!("{}", err);
        }
        assert_eq!(hello_world.unwrap(), create_constant_string("Hello World!"));
    }
}
