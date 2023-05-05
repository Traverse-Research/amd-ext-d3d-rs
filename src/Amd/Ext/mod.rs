#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Ext_D3D")]
pub mod D3D;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
