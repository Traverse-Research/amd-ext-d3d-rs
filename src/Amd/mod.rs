#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Ext")]
pub mod Ext;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
