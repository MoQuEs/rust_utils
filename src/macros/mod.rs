#[macro_use]
#[cfg(feature = "macros-enum_struct")]
pub mod enum_struct;

#[macro_use]
#[cfg(feature = "macros-str_function")]
pub mod str_function;

#[macro_use]
#[cfg(feature = "macros-transparent_struct")]
pub mod transparent_struct;

#[macro_use]
#[cfg(feature = "macros-diff")]
pub mod diff;

#[macro_use]
#[cfg(feature = "macros-log")]
pub mod log;
