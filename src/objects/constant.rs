use smartstring::alias::String;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Constant {
    Int(i64),
    Double(f64),
    String(String),
    Boolean(bool),
}

impl Constant {
    pub fn add(&self, constant: &Constant) -> Constant {
        match ((*self).clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                Constant::Int(val_1 + val_2)
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Constant::Double(val_1 + val_2)
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Constant::Double(val_1 as f64 + val_2)
            },
            _ => {
                panic!("Can't add non-numerical types");
            }
        }
    }
    pub fn sub(&self, constant: &Constant) -> Constant {
        match ((*self).clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                Constant::Int(val_1 - val_2)
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Constant::Double(val_1 - val_2)
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Constant::Double(val_1 as f64 - val_2)
            },
            _ => {
                panic!("Can't subtract non-numerical types");
            }
        }
    }
    pub fn mul(&self, constant: &Constant) -> Constant {
        match ((*self).clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                Constant::Int(val_1 * val_2)
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Constant::Double(val_1 * val_2)
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Constant::Double(val_1 as f64 * val_2)
            },
            _ => {
                panic!("Can't multiply non-numerical types");
            }
        }
    }
    pub fn div(&self, constant: &Constant) -> Constant {
        match ((*self).clone(), (*constant).clone()) {
            (Constant::Int(val_1), Constant::Int(val_2)) => {
                Constant::Int(val_1 / val_2)
            },
            (Constant::Double(val_1), Constant::Double(val_2)) => {
                Constant::Double(val_1 / val_2)
            },
            (Constant::Int(val_1), Constant::Double(val_2)) | (Constant::Double(val_2), Constant::Int(val_1)) => {
                Constant::Double(val_1 as f64 / val_2)
            },
            _ => {
                panic!("Can't subtract non-numerical types");
            }
        }
    }
    pub fn concat(&self, constant: &Constant) -> Constant {
        if let (Constant::String(str_1), Constant::String(str_2)) = ((*self).clone(), (*constant).clone()) {
            Constant::String(str_1 + str_2)
        } 
        else {
            panic!("Concat only works for strings!");
        }
    }
}

pub fn create_constant_int(init_val: &i64) -> Constant {
    Constant::Int(*init_val)
}
pub fn create_constant_double(init_val: &f64) -> Constant {
    Constant::Double(*init_val)
}
pub fn create_constant_string(init_val: &str) -> Constant {
    Constant::String(String::from(init_val))
}
pub fn create_constant_bool(init_val: &bool) -> Constant {
    Constant::Boolean(*init_val)
}

#[cfg(test)]
mod constant_init_tests {
    use super::*;
    use smartstring::alias::String;

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
    use smartstring::alias::String;

    #[test]
    fn math_test() {
        let int_const = create_constant_int(&99);
        let double_const = create_constant_double(&1.5);

        let add_constant = int_const.add(&double_const);
        let sub_constant = int_const.sub(&double_const);
        let mul_constant = int_const.mul(&double_const);
        let div_constant = int_const.div(&double_const);

        if let Constant::Double(res) = add_constant {
            assert_eq!(res, 99.0 + 1.5);
        }
        if let Constant::Double(res) = sub_constant {
            assert_eq!(res, 99.0 - 1.5);
        }
        if let Constant::Double(res) = mul_constant {
            assert_eq!(res, 99.0 * 1.5);
        }
        if let Constant::Double(res) = div_constant {
            assert_eq!(res, 99.0 / 1.5);
        }
    }

    #[test]
    fn concat_test() {
        let (hello, world) = (create_constant_string("Hello "), create_constant_string("World!"));
        let hello_world = hello.concat(&world);
        assert_eq!(hello_world, Constant::String(String::from("Hello World!")));
    }
}