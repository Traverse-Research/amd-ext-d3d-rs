#![doc = include_str!("../README.md")]

use std::ffi::CStr;

use anyhow::{Context, Result};
use d3d12_agility::{ID3D12GraphicsCommandList, D3D12_GRAPHICS_PIPELINE_STATE_DESC};
use windows_core::{IUnknown, Interface, Param};

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

pub use bindings::AmdExtD3DCreateInfo;
use bindings::{IAmdExtD3DDevice1, IAmdExtD3DFactory, PFNAmdExtD3DCreateInterface};

#[derive(Clone, Debug)]
pub struct AmdExtD3DDevice {
    amd_device_object: IAmdExtD3DDevice1,
}

// SAFETY: The AMD extension objects are free-threaded, just like the `ID3D12Device` they extend.
// The previous metadata marked all `IAmdExtD3D*` interfaces `[Agile]`, which made the generated
// interfaces `Send + Sync`; RDL metadata on top of `d3d12-agility` has no such annotation.
unsafe impl Send for AmdExtD3DDevice {}
unsafe impl Sync for AmdExtD3DDevice {}

impl AmdExtD3DDevice {
    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn new(device: &IUnknown) -> Result<Self> {
        // TODO: Load and cache the library?
        let lib =
            libloading::Library::new("amdxc64.dll").context("Could not open `amdxc64.dll`")?;

        let amd_create_interface = lib
            .get::<PFNAmdExtD3DCreateInterface>(b"AmdExtD3DCreateInterface\0")
            .context("Could not find `AmdExtD3DCreateInterface` symbol in `amdxc64.dll`")?
            .context("Symbol value (pointer) is NULL")?;

        // TODO: Try convincing the Windows maintainers again to have a helper (with all the proper annotations, Param generics, and checks):
        // https://github.com/microsoft/windows-rs/issues/1835
        // I keep an updated revision on my branch:
        // https://github.com/MarijnS95/windows-rs/commit/13033a0b6e09a66a72d6e02bc050046730af157c
        let mut result__ = std::ptr::null_mut();
        (amd_create_interface)(device.into(), &IAmdExtD3DFactory::IID, &mut result__)
            .ok()
            .context("While creating `IAmdExtD3DFactory`")?;
        // SAFETY: On success, `result__` holds an owned (AddRef'd) `IAmdExtD3DFactory` pointer.
        let amd_factory = IAmdExtD3DFactory::from_raw(result__);

        let amd_device_object = amd_factory
            .CreateInterface(device)
            .context("While creating `IAmdExtD3DDevice1`")?;

        Ok(Self { amd_device_object })
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn create_graphics_pipeline_state<T: Interface>(
        &self,
        amd_ext_create_info: &AmdExtD3DCreateInfo,
        graphics_pipeline_state_desc: &D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    ) -> Result<T> {
        self.amd_device_object
            .CreateGraphicsPipelineState(amd_ext_create_info, graphics_pipeline_state_desc)
            .context("While calling `CreateGraphicsPipelineState`")
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn push_marker(
        &self,
        gfx_cmd_list: impl Param<ID3D12GraphicsCommandList>,
        marker: &CStr,
    ) {
        self.amd_device_object
            .PushMarker(gfx_cmd_list, marker.as_ptr().cast())
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn pop_marker(&self, gfx_cmd_list: impl Param<ID3D12GraphicsCommandList>) {
        self.amd_device_object.PopMarker(gfx_cmd_list)
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn set_marker(
        &self,
        gfx_cmd_list: impl Param<ID3D12GraphicsCommandList>,
        marker: &CStr,
    ) {
        self.amd_device_object
            .SetMarker(gfx_cmd_list, marker.as_ptr().cast())
    }
}
