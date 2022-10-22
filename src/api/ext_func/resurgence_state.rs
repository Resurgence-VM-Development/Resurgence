use crate::objects::constant::Constant;
use std::io::{Error, ErrorKind};
pub struct ResurgenceState {
    args: Vec<Constant>,
    current_arg: usize
}

impl From<Vec<Constant>> for ResurgenceState {
    fn from(args: Vec<Constant>) -> ResurgenceState {
        ResurgenceState { 
            args,
            current_arg: 0 
        }
    }
}

impl ResurgenceState {
    pub fn get_i64(&mut self) -> Result<i64, Error> { 
        if let Constant::Int(res) = self.args[self.current_arg] {
            self.current_arg += 1;
            return Result::Ok(res);
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected i64")))
    }
    pub fn get_f64(&mut self) -> Result<f64, Error> {
        if let Constant::Double(res) = self.args[self.current_arg] {
            self.current_arg += 1;
            return Result::Ok(res);
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected f64")))
    }
    pub fn get_string(&mut self) -> Result<String, Error> {
        if let Constant::String(res) = &self.args[self.current_arg] {
            self.current_arg += 1;
            return Result::Ok((*res).to_string());
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected String")))
    }
    pub fn get_bool(&mut self) -> Result<bool, Error> {
        if let Constant::Boolean(res) = self.args[self.current_arg] {
            self.current_arg += 1;
            return Result::Ok(res);
        }
        Err(Error::new(ErrorKind::Other, String::from("Invalid type, expected bool")))
    }
    pub fn get_value_as_string(&mut self) -> Result<String, Error> {
        let constant = &self.args[self.current_arg];
        self.current_arg += 1;
        match constant {
            Constant::Int(ref val) => Result::Ok(val.to_string()),
            Constant::Double(ref val) => Result::Ok(val.to_string()),
            Constant::String(ref val) => Result::Ok(val.to_string()),
            Constant::Boolean(ref val) => Result::Ok(val.to_string()),
            _ => Err(Error::new(ErrorKind::Other, String::from("Invalid type"))),
        }
    }
}