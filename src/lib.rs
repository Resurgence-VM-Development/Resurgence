/*!
# Resurgence

A VM backend library that makes developing interpreters easy. Can be used either as an entire backend, or to create a backend
*/

pub(crate) mod objects;
pub(crate) use objects::constant;
pub(crate) mod internal;
pub use internal::interpreter::Interpreter;