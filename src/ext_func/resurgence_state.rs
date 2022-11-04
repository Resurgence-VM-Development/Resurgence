use crate::objects::constant::Constant;
use std::io::{Error, ErrorKind};
pub struct ResurgenceState<'a> {
    args: &'a mut Vec<Constant>,
}

impl ResurgenceState<'_> {
    pub(crate) fn new(args: &mut Vec<Constant>) -> ResurgenceState {
        ResurgenceState { 
            args,
        }
    }
}

impl ResurgenceState<'_> {

    /// Returns an `Result<i64>` from the top of the stack
    ///
    /// ```
    /// let int_val = state.get_i64();
    /// ```
    pub fn get_i64(&mut self) -> Result<i64, Error> { 
        if let Constant::Int(res) = self.args.pop().unwrap() {
            return Result::Ok(res);
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected i64")))
    }
    
    /// Returns an `Result<f64>` from the top of the stack
    ///
    /// ```
    /// let f64_val = state.get_f64();
    /// ```
    pub fn get_f64(&mut self) -> Result<f64, Error> {
        if let Constant::Double(res) = self.args.pop().unwrap() {
            return Result::Ok(res);
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected f64")))
    }

    /// Returns an `Result<String>` from the top of the stack
    ///
    /// ```
    /// let string_val = state.get_string();
    /// ```
    pub fn get_string(&mut self) -> Result<String, Error> {
        if let Constant::String(res) = self.args.pop().unwrap() {
            return Result::Ok((*res).to_string());
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected String")))
    }

    /// Returns an `Result<bool>` from the top of the stack
    ///
    /// ```
    /// let bool_val = state.get_bool();
    /// ```
    pub fn get_bool(&mut self) -> Result<bool, Error> {
        if let Constant::Boolean(res) = self.args.pop().unwrap() {
            return Result::Ok(res);
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected bool")))
    }

    /// Returns the topmost constant as an `Result<String>`
    ///
    /// ```
    /// let val = state.get_value_as_string();
    /// ```
    pub fn get_value_as_string(&mut self) -> Result<String, Error> {
        let constant = self.args.pop().unwrap();
        match constant {
            Constant::Int(ref val) => Result::Ok(val.to_string()),
            Constant::Double(ref val) => Result::Ok(val.to_string()),
            Constant::String(ref val) => Result::Ok(val.clone()),
            Constant::Boolean(ref val) => Result::Ok(val.to_string()),
            _ => Err(Error::new(ErrorKind::Other, String::from("Invalid type"))),
        }
    }

    /// Pushes an `i64` on the stack
    ///
    ///
    /// val (`i64`): The value to be pushed on the stack
    ///
    /// ``` 
    /// state.push_i64(69);
    /// ```
    pub fn push_i64(&mut self, val: i64) {
        self.args.push(Constant::Int(val)); 
    }

    /// Pushes a `f64` on the stack
    ///
    ///
    /// val (`f64`): The value to be pushed on the stack
    ///
    /// ```
    /// state.push_f64(420.5);
    /// ```
    pub fn push_f64(&mut self, val: f64) {
        self.args.push(Constant::Double(val));
    }

    /// Pushes a `String` on the stack
    ///
    ///
    /// val (`String`): The value to be pushed on the stack
    ///
    /// ```
    /// state.push_string("Hello World".as_string());
    /// ```
    pub fn push_string(&mut self, val: String) {
        self.args.push(Constant::String(val));
    }

    /// Pushes a bool on the stack
    ///
    ///
    /// val (`bool`): The value to be pushed on the stack
    ///
    /// ```
    /// state.push_bool(true);
    /// ```
    pub fn push_bool(&mut self, val: bool) {
        self.args.push(Constant::Boolean(val));
    }
}
