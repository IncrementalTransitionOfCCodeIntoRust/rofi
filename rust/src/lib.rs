#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

//include!(concat!("bindings.rs"));

mod bindings;
mod pub_functions;

mod box_;
mod widget_internal;
mod widget_;

pub use bindings::{
    widget_draw
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
