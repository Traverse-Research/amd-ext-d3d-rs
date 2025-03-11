#![doc = include_str!("../README.md")]

use std::{ffi::CStr, sync::Arc};

use anyhow::{Context, Result};
use windows::{
    core::{IUnknown, Interface, Param, Type, PCSTR},
    Win32::Graphics::Direct3D12,
};

mod bindings;

pub use bindings::Amd::Ext::D3D::AmdExtD3DCreateInfo;
use bindings::Amd::Ext::D3D::{
    IAmdExtD3DCommandListMarker, IAmdExtD3DDevice1, IAmdExtD3DFactory, IAmdExtGpaInterface,
    IAmdExtGpaSession, PFNAmdExtD3DCreateInterface,
};

#[derive(Clone, Debug)]
#[doc(alias = "IAmdExtD3DFactory")]
#[doc(alias = "IAmdExtD3DDevice1")]
pub struct AmdExtD3DDevice {
    factory: IAmdExtD3DFactory,
    device_object: IAmdExtD3DDevice1,
    // Keep the library opened, even though it already/always seems to be oped from the driver already.
    _lib: Arc<libloading::Library>,
}

impl AmdExtD3DDevice {
    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "AmdExtD3DCreateInterface")]
    #[doc(alias = "CreateInterface")]
    pub unsafe fn new(device: &IUnknown) -> Result<Self> {
        // TODO: Load and cache the library?
        let lib =
            libloading::Library::new("amdxc64.dll").context("Could not open `amdxc64.dll`")?;

        let amd_create_interface = lib
            .get::<PFNAmdExtD3DCreateInterface>(c"AmdExtD3DCreateInterface".to_bytes())
            .context("Could not find `AmdExtD3DCreateInterface` symbol in `amdxc64.dll`")?
            .context("Symbol value (pointer) is NULL")?;

        // TODO: Try convincing the Windows maintainers again to have a helper (with all the proper annotations, Param generics, and checks):
        // https://github.com/microsoft/windows-rs/issues/1835
        // I keep an updated revision on my branch:
        // https://github.com/MarijnS95/windows-rs/commit/13033a0b6e09a66a72d6e02bc050046730af157c
        let mut result__ = ::std::ptr::null_mut();
        let factory: IAmdExtD3DFactory = (amd_create_interface)(
            // Windows internally uses a transmute_copy to construct a Ref<T>, which is the pointer
            // value (Type::Abi) contained inside the COM object.
            std::mem::transmute_copy(device),
            &IAmdExtD3DFactory::IID,
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
        .context("While creating `IAmdExtD3DFactory`")?;

        // Immediately create a device wrapper for the given device/factory. Note that this "per
        // device" factory can be used to create other objects like AmdExtD3DCommandList.
        let device_object = factory.CreateInterface(device).context(
            "While calling `IAmdExtD3DFactory::CreateInterface()` for `IAmdExtD3DDevice1`",
        )?;

        Ok(Self {
            factory,
            device_object,
            _lib: Arc::new(lib),
        })
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "CreateInterface")]
    #[doc(alias = "IAmdExtD3DCommandListMarker")]
    pub unsafe fn create_cmd_list_marker(
        &self,
        cmd_list: &IUnknown,
    ) -> Result<AmdExtD3DCommandListMarker> {
        let cmd_list_marker_object = self.factory.CreateInterface(cmd_list).context(
            "While calling `IAmdExtD3DFactory::CreateInterface()` for `IAmdExtD3DCommandListMarker`",
        )?;

        Ok(AmdExtD3DCommandListMarker {
            cmd_list_marker_object,
            _lib: self._lib.clone(),
        })
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "CreateInterface")]
    #[doc(alias = "IAmdExtGpaInterface")]
    pub unsafe fn create_gpa_interface(&self) -> Result<AmdExtGpaInterface> {
        let gpa_interface_object = self
            .factory
            // TODO: Store and use actual device
            .CreateInterface(&self.device_object)
            .context(
                "While calling `IAmdExtD3DFactory::CreateInterface()` for `IAmdExtGpaInterface`",
            )?;

        Ok(AmdExtGpaInterface {
            gpa_interface_object,
            _lib: self._lib.clone(),
        })
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "CreateGraphicsPipelineState")]
    pub unsafe fn create_graphics_pipeline_state<T: Interface>(
        &self,
        amd_ext_create_info: &AmdExtD3DCreateInfo,
        graphics_pipeline_state_desc: &Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    ) -> Result<T> {
        self.device_object
            .CreateGraphicsPipelineState(amd_ext_create_info, graphics_pipeline_state_desc)
            .context("While calling `CreateGraphicsPipelineState`")
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "PushMarker")]
    pub unsafe fn push_marker(
        &self,
        gfx_cmd_list: impl Param<Direct3D12::ID3D12GraphicsCommandList>,
        marker: &CStr,
    ) {
        self.device_object
            .PushMarker(gfx_cmd_list, PCSTR::from_raw(marker.as_ptr().cast()))
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "PopMarker")]
    pub unsafe fn pop_marker(
        &self,
        gfx_cmd_list: impl Param<Direct3D12::ID3D12GraphicsCommandList>,
    ) {
        self.device_object.PopMarker(gfx_cmd_list)
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "SetMarker")]
    pub unsafe fn set_marker(
        &self,
        gfx_cmd_list: impl Param<Direct3D12::ID3D12GraphicsCommandList>,
        marker: &CStr,
    ) {
        self.device_object
            .SetMarker(gfx_cmd_list, PCSTR::from_raw(marker.as_ptr().cast()))
    }
}

/// Provides AMD-specific functions to emit markers on Direct3D12 Command Lists.
///
/// Created on an existing Device and Command List via [`AmdExtD3DDevice::create_cmd_list_marker()`].
#[derive(Clone, Debug)]
#[doc(alias = "IAmdExtD3DCommandListMarker")]
pub struct AmdExtD3DCommandListMarker {
    cmd_list_marker_object: IAmdExtD3DCommandListMarker,
    _lib: Arc<libloading::Library>,
}

impl AmdExtD3DCommandListMarker {
    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "PushMarker")]
    pub unsafe fn push_marker(&self, marker: &CStr) {
        self.cmd_list_marker_object
            .PushMarker(PCSTR::from_raw(marker.as_ptr().cast()))
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "PopMarker")]
    pub unsafe fn pop_marker(&self) {
        self.cmd_list_marker_object.PopMarker()
    }

    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "SetMarker")]
    pub unsafe fn set_marker(&self, marker: &CStr) {
        self.cmd_list_marker_object
            .SetMarker(PCSTR::from_raw(marker.as_ptr().cast()))
    }
}

#[derive(Clone, Debug)]
#[doc(alias = "IAmdExtGpaInterface")]
pub struct AmdExtGpaInterface {
    gpa_interface_object: IAmdExtGpaInterface,
    // Keep the library opened, even though it already/always seems to be oped from the driver already.
    _lib: Arc<libloading::Library>,
}

impl AmdExtGpaInterface {
    /// # Safety
    /// Calls an unsafe function on the Windows API.
    #[doc(alias = "CreateGpaSession")]
    pub unsafe fn create_gpa_session(&self) -> Option<AmdExtGpaSession> {
        let gpa_session_object = self.gpa_interface_object.CreateGpaSession()?;

        Some(AmdExtGpaSession {
            gpa_session_object,
            _lib: self._lib.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct AmdExtGpaSession {
    gpa_session_object: IAmdExtGpaSession,
    // Keep the library opened, even though it already/always seems to be oped from the driver already.
    _lib: Arc<libloading::Library>,
}

impl AmdExtGpaSession {
    #[doc(alias = "Begin")]
    pub unsafe fn begin(&self) {}
    #[doc(alias = "End")]
    pub unsafe fn end(&self) {}
    #[doc(alias = "BeginSample")]
    pub unsafe fn begin_sample(&self) {}
    #[doc(alias = "EndSample")]
    pub unsafe fn end_sample(&self) {}
    #[doc(alias = "IsReady")]
    pub unsafe fn is_ready(&self) -> bool {
        self.gpa_session_object.IsReady()
    }
    #[doc(alias = "GetResults")]
    pub unsafe fn get_results(&self) {}
    #[doc(alias = "Reset")]
    pub unsafe fn reset(&self) {}
    #[doc(alias = "CopyResults")]
    pub unsafe fn copy_results(&self) {}
}
