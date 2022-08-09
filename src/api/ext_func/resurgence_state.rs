use crate::objects::constant::Constant;

pub struct ResurgenceState {
    args: Vec<Constant>,
}

impl From<Vec<Constant>> for ResurgenceState {
    fn from(args: Vec<Constant>) -> ResurgenceState {
        ResurgenceState { 
            args 
        }
    }
}