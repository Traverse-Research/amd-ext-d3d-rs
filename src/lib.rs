#![doc = include_str!("../README.md")]

use std::ffi::CStr;

use anyhow::{Context, Result};
use windows::{
    core::{ComInterface, IUnknown, IntoParam, PCSTR},
    Win32::Graphics::Direct3D12,
};

mod Amd;
pub use crate::Amd::Ext::D3D::AmdExtD3DCreateInfo;
use crate::Amd::Ext::D3D::{IAmdExtD3DDevice1, IAmdExtD3DFactory, PFNAmdExtD3DCreateInterface};

#[derive(Clone, Debug)]
pub struct AmdExtD3DDevice {
    amd_device_object: IAmdExtD3DDevice1,
}

impl AmdExtD3DDevice {
    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn new(device: IUnknown) -> Result<Self> {
        // TODO: Load and cache the library?
        let lib =
            libloading::Library::new("amdxc64.dll").context("Could not open `amdxc64.dll`")?;

        let amd_create_interface = lib
            .get::<PFNAmdExtD3DCreateInterface>(b"AmdExtD3DCreateInterface\0")
            .context("Could not find `AmdExtD3DCreateInterface` symbol in `amdxc64.dll`")?
            .context("Symbol value (pointer) is NULL")?;

        // TODO: Try convincing the Windows maintainers again to have a helper (with all the proper annotations, IntoParam generics, and checks):
        // https://github.com/microsoft/windows-rs/issues/1835
        // I keep an updated revision on my branch:
        // https://github.com/MarijnS95/windows-rs/commit/13033a0b6e09a66a72d6e02bc050046730af157c
        let mut result__ = ::std::ptr::null_mut();
        let amd_factory: IAmdExtD3DFactory =
            (amd_create_interface)(Some(device.clone()), &IAmdExtD3DFactory::IID, &mut result__)
                .from_abi(result__)
                .context("While creating `IAmdExtD3DFactory`")?;

        let amd_device_object = amd_factory
            .CreateInterface(&device)
            .context("While creating `IAmdExtD3DDevice1`")?;

        Ok(Self { amd_device_object })
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn create_graphics_pipeline_state<T: ComInterface>(
        &self,
        amd_ext_create_info: &AmdExtD3DCreateInfo,
        graphics_pipeline_state_desc: &Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    ) -> windows::core::Result<T> {
        self.amd_device_object
            .CreateGraphicsPipelineState(amd_ext_create_info, graphics_pipeline_state_desc)
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn push_marker(
        &self,
        gfx_cmd_list: impl IntoParam<Direct3D12::ID3D12GraphicsCommandList>,
        marker: &CStr,
    ) {
        self.amd_device_object
            .PushMarker(gfx_cmd_list, PCSTR::from_raw(marker.as_ptr().cast()))
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn pop_marker(
        &self,
        gfx_cmd_list: impl IntoParam<Direct3D12::ID3D12GraphicsCommandList>,
    ) {
        self.amd_device_object.PopMarker(gfx_cmd_list)
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    pub unsafe fn set_marker(
        &self,
        gfx_cmd_list: impl IntoParam<Direct3D12::ID3D12GraphicsCommandList>,
        marker: &CStr,
    ) {
        self.amd_device_object
            .SetMarker(gfx_cmd_list, PCSTR::from_raw(marker.as_ptr().cast()))
    }
}
