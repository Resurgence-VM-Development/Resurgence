use crate::{ResurgenceError, create_new_trace};

use super::{register::{Register, RegisterLocation}, resurgence_error::ResurgenceErrorKind};

/// `Constant`: Represents a constant in the backend
#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    /// 64 bit integer
    Int(i64),
    /// 64 bit float
    Double(f64),
    /// Rust String type
    String(String),
    /// bool
    Boolean(bool),
    /// Represents a register in memory
    Address(Register),
    /// Represents a vector
    Vec(Vec<Constant>)
}

impl Constant {
    fn check_overflow(&self, value: Option<i64>) -> Result<i64, ResurgenceError> {
        if value.is_none() {
            let mut err = ResurgenceError::from(ResurgenceErrorKind::OVERFLOW, "Overflow error!");
            err.add_trace(&format!("{}: line {}", file!(), line!()));
            return Err(err);
        }

        // We know there was no error, so we don't need unwrap to check for us again
        unsafe {
            Ok(value.unwrap_unchecked())
        }
    }
    /// Adds 2 numerical Constants together
    /// 
    /// `constant` (`&Constant::Int` or `$Constant::Double`): Constant you want to add to self
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
    pub fn add(&self, constant: &Self) -> Result<Self, ResurgenceError> {
        match (self.clone(), (*constant).clone()) {
            (Self::Int(val_1), Self::Int(val_2)) => {
                let res = self.check_overflow(val_1.checked_add(val_2));
                if let Err(mut err) = res {
                    err.add_trace(&format!("{}: line {}", file!(), line!()));
                    Err(err)
                } else {
                    // If no error was returned, then we don't need to use the checked version of unwrap
                    unsafe { Ok(Self::Int(res.unwrap_unchecked())) }
                }
            },
            (Self::Double(val_1), Self::Double(val_2)) => {
                Ok(Self::Double(val_1 + val_2))
            },
            (Self::Int(val_1), Self::Double(val_2)) | (Self::Double(val_2), Self::Int(val_1)) => {
                Ok(Self::Double(val_1 as f64 + val_2))
            },
            (Self::Address(val_1), Self::Int(val_2)) | (Self::Int(val_2), Self::Address(val_1)) => {
                Ok(Self::Address(Register(val_1.0 + val_2 as u32, val_1.1)))
            },
            _ => {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not add non-numerical types!");
                err.add_trace(&format!("{}: line {}", file!(), line!()));
                Err(err)
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
    pub fn sub(&self, constant: &Self) -> Result<Self, ResurgenceError> {
        match (self.clone(), (*constant).clone()) {
            (Self::Int(val_1), Self::Int(val_2)) => {
                let res = self.check_overflow(val_1.checked_sub(val_2));
                if let Err(mut err) = res {
                    create_new_trace!(err);
                    Err(err)
                } else {
                    // We know there was no error, then we don't need to use the checked version of unwrap
                    unsafe { Ok(Self::Int(res.unwrap_unchecked())) }
                }
            },
            (Self::Double(val_1), Self::Double(val_2)) => {
                Ok(Self::Double(val_1 - val_2))
            },
            (Self::Int(val_1), Self::Double(val_2)) | (Self::Double(val_2), Self::Int(val_1)) => {
                Ok(Self::Double(val_1 as f64 - val_2))
            },
            (Self::Address(val_1), Self::Int(val_2)) | (Self::Int(val_2), Self::Address(val_1)) => {
                Ok(Self::Address(Register(val_1.0 - val_2 as u32, val_1.1)))
            },
            _ => {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not subtract non-numerical types!");
                create_new_trace!(err);
                Err(err)
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
    pub fn mul(&self, constant: &Self) -> Result<Self, ResurgenceError> {
        match (self.clone(), (*constant).clone()) {
            (Self::Int(val_1), Self::Int(val_2)) => {
                let res = self.check_overflow(val_1.checked_mul(val_2));
                if let Err(mut err) = res {
                    create_new_trace!(err);
                    Err(err)
                } else {
                    // We know there was no error, so we don't need to use the checked version of unwrap
                    unsafe { Ok(Self::Int(res.unwrap_unchecked())) }
                }
            },
            (Self::Double(val_1), Self::Double(val_2)) => {
                Ok(Self::Double(val_1 * val_2))
            },
            (Self::Int(val_1), Self::Double(val_2)) | (Self::Double(val_2), Self::Int(val_1)) => {
                Ok(Self::Double(val_1 as f64 * val_2))
            },
            _ => {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can only multiply non-numerical types!");
                create_new_trace!(err);
                Err(err)
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
    pub fn div(&self, constant: &Self) -> Result<Self, ResurgenceError> {
        match (self.clone(), (*constant).clone()) {
            (Self::Int(val_1), Self::Int(val_2)) => {
                if val_2 == 0 {
                    let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not divide by 0!");
                    create_new_trace!(err);
                    return Err(err);
                }
                let res = self.check_overflow(val_1.checked_div(val_2));
                if let Err(mut err) = res {
                    create_new_trace!(err);
                    Err(err)
                } else {
                    // We know there was no error, so we don't need to use the checked version of unwrap
                    unsafe { Ok(Self::Int(res.unwrap_unchecked())) }
                }
            },
            (Self::Double(val_1), Self::Double(val_2)) => {
                if val_2 == 0.0 {
                    let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not divide by 0!");
                    create_new_trace!(err);
                    return Err(err);
                }
                Ok(Self::Double(val_1 / val_2))
            },
            (Self::Int(val_1), Self::Double(val_2)) | (Self::Double(val_2), Self::Int(val_1)) => {
                if val_2 == 0.0 {
                    let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not divide by 0!");
                    create_new_trace!(err);
                    return Err(err);
                }
                Ok(Self::Double(val_1 as f64 / val_2))
            },
            _ => {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can't divide non-numerical types!");
                create_new_trace!(err);
                Err(err)
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
    pub fn modlo(&self, constant: &Self) -> Result<Self, ResurgenceError> {
        match (self.clone(), (*constant).clone()) {
            (Self::Int(val_1), Self::Int(val_2)) => {
                if val_2 == 0 {
                    let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not divide by 0!");
                    create_new_trace!(err);
                    return Err(err);
                }
                Ok(Self::Int(val_1 % val_2))
            },
            (Self::Double(val_1), Self::Double(val_2)) => {
                if val_2 == 0.0 {
                    let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not divide by 0!");
                    create_new_trace!(err);
                    return Err(err);
                }
                Ok(Self::Double(val_1 % val_2))
            },
            (Self::Int(val_1), Self::Double(val_2)) | (Self::Double(val_2), Self::Int(val_1)) => {
                if val_2 == 0.0 {
                    let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not divide by 0!");
                    create_new_trace!(err);
                    return Err(err);
                }
                Ok(Self::Double(val_1 as f64 % val_2))
            },
            _ => {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can not perform a modlo operation on non-numerical types!");
                create_new_trace!(err);
                Err(err)
            }
        }
    }
    
    /// Returns the type as `String` for error handling reasons
    #[inline]
    pub fn type_as_string(&self) -> String {
        match &*self {
            Self::Int(ref int_val) => format!("i64 Constant: {}", *int_val),
            Self::Double(ref double_val) => format!("f64 Constant: {}", *double_val),
            Self::String(ref string_val) => format!("String Constant: {}", *string_val),
            Self::Boolean(ref bool_val) => format!("bool Constant: {}", if *bool_val {"true"} else {"false"}),
            Self::Address(ref address_value) => format!("Register constant: index({}) location({})", address_value.0, match address_value.1 {
                RegisterLocation::ConstantPool => "Constant Pool",
                RegisterLocation::Accumulator => "Accumulator",
                RegisterLocation::Global => "Global",
                RegisterLocation::Local => "Local"
            }),
            Self::Vec(ref vec_val) => {
                let mut final_string = String::from("");
                for obj in vec_val {
                    final_string += &obj.type_as_string();
                }
                final_string
            }
        }
    } 
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
