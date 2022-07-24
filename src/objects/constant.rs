use smartstring::alias::String;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Constant
{
    Int(i64),
    Double(f64),
    String(String),
    Boolean(bool),
}

pub fn create_constant_int(init_val: &i64) -> Constant
{
    return Constant::Int(init_val.clone());
}
pub fn create_constant_double(init_val: &f64) -> Constant
{
    return Constant::Double(init_val.clone());
}
pub fn create_constant_string(init_val: &String) -> Constant
{
    return Constant::String(init_val.clone());
}
pub fn create_constant_bool(init_val: &bool) -> Constant
{
    return Constant::Boolean(init_val.clone());
}