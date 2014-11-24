use std::collections::enum_set::CLike;
use std::mem;

#[repr(uint)]
pub enum Settings {
    CBREAK,
    ECHO,
}

impl CLike for Settings {
    fn to_uint(&self) -> uint {
        *self as uint
    }

    fn from_uint(v: uint) -> Settings {
        unsafe { mem::transmute(v) }
    }
}