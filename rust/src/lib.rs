#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub use gtypes::primitive::gboolean;
pub use rofi_types::*;
pub use widget::*;
pub use widget_internal::*;

mod rofi_types;
mod widget;
mod widget_internal;
mod _box;
mod theme;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
