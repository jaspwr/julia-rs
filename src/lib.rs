
#![feature(try_from)]
#![feature(unique)]
#![feature(concat_idents)]

#![allow(unknown_lints)]
#![allow(not_unsafe_ptr_arg_deref)]

extern crate libc;
extern crate smallvec;
extern crate julia_sys;

pub mod sys;
pub mod error;
pub mod exception;
pub mod string;

#[macro_use]
pub mod julia;
#[macro_use]
pub mod value;
pub mod function;
pub mod sym;
pub mod module;
pub mod datatype;

pub use julia::Julia;
pub use value::Value;
pub use function::Function;
pub use sym::Symbol;
pub use module::Module;
pub use datatype::Datatype;

#[cfg(test)]
mod tests {
    use super::Julia;

    #[test]
    fn sanity() {
        let _jl = Julia::new();
    }
}
