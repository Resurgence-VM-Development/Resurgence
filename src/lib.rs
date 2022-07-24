mod objects;
pub use crate::objects::constant::{
        create_constant_int, 
        create_constant_double, 
        create_constant_string, 
        create_constant_bool, 
    };

#[cfg(test)]
mod tests {
    use crate::{
    create_constant_int, 
    create_constant_double, 
    create_constant_string, 
    create_constant_bool, 
    objects::constant::Constant
    };
    use smartstring::alias::String;

    #[test]
    fn const_int()
    {
        let test_int = 99;
        let constant = create_constant_int(&test_int);
        assert_eq!(constant, Constant::Int(test_int));
    }
    #[test]
    fn const_double()
    {
        let test_double = 3.14;
        let constant = create_constant_double(&test_double);
        assert_eq!(constant, Constant::Double(test_double));
    }
    #[test]
    fn const_string() 
    {
        let test_string = String::from("Hello World");
        let constant = create_constant_string(&test_string);
        assert_eq!(constant, Constant::String(test_string))
    }
    #[test]
    fn const_bool()
    {
        let test_bool = true;
        let constant = create_constant_bool(&test_bool);
        assert_eq!(constant, Constant::Boolean(test_bool));
    }
}
