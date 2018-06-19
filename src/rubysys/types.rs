use rubysys::libc::{intptr_t, uintptr_t};

pub use rubysys::libc::{c_char, c_double, c_int, c_long, c_void, size_t};

#[cfg(unix)]
pub use std::os::unix::io::RawFd;

pub use rubysys::typed_data::{RbDataType, RbDataTypeFunction};
pub use rubysys::value::{Value, ValueType};

pub type Id = uintptr_t;
pub type InternalValue = uintptr_t;
pub type SignedValue = intptr_t;

pub type Argc = c_int;
pub type CallbackPtr = *const c_void;
pub type CallbackMutPtr = *mut c_void;

#[repr(C)]
pub struct RBasic {
    pub flags: InternalValue,
    pub klass: InternalValue,
}
