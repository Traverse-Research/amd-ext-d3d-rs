// Bindings generated by `windows-bindgen` 0.61.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Amd {
    pub mod Ext {
        pub mod D3D {
            #[repr(C)]
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct AmdExtD3DCreateInfo {
                pub r#type: AmdExtD3DStructType,
                pub pNext: *mut core::ffi::c_void,
            }
            impl Default for AmdExtD3DCreateInfo {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct AmdExtD3DPipelineCreateInfo {
                pub Base: AmdExtD3DCreateInfo,
                pub flags: AmdExtD3DPipelineFlags,
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct AmdExtD3DPipelineFlags {
                pub _bitfield: u32,
            }
            pub const AmdExtD3DStructPipelineState: AmdExtD3DStructType = AmdExtD3DStructType(1u32);
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct AmdExtD3DStructType(pub u32);
            pub const AmdExtD3DStructUnknown: AmdExtD3DStructType = AmdExtD3DStructType(0u32);
            windows_core::imp::define_interface!(
                IAmdExtD3DDevice,
                IAmdExtD3DDevice_Vtbl,
                0x8104c0fc_7413_410f_8e83_aa617e908648
            );
            windows_core::imp::interface_hierarchy!(IAmdExtD3DDevice, windows_core::IUnknown);
            impl IAmdExtD3DDevice {
                pub unsafe fn CreateGraphicsPipelineState<T>(
                    &self,
                    pamdextcreateinfo: *const AmdExtD3DCreateInfo,
                    pdesc : *const windows::Win32::Graphics::Direct3D12:: D3D12_GRAPHICS_PIPELINE_STATE_DESC,
                ) -> windows_core::Result<T>
                where
                    T: windows_core::Interface,
                {
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        (windows_core::Interface::vtable(self).CreateGraphicsPipelineState)(
                            windows_core::Interface::as_raw(self),
                            pamdextcreateinfo,
                            core::mem::transmute(pdesc),
                            &T::IID,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAmdExtD3DDevice_Vtbl {
                pub base__: windows_core::IUnknown_Vtbl,
                pub CreateGraphicsPipelineState: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *const AmdExtD3DCreateInfo,
                    *const windows::Win32::Graphics::Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
                    *const windows_core::GUID,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            pub trait IAmdExtD3DDevice_Impl: windows_core::IUnknownImpl {
                fn CreateGraphicsPipelineState(
                    &self,
                    pamdextcreateinfo: *const AmdExtD3DCreateInfo,
                    pdesc : *const windows::Win32::Graphics::Direct3D12:: D3D12_GRAPHICS_PIPELINE_STATE_DESC,
                    riid: *const windows_core::GUID,
                    pppipelinestate: *mut *mut core::ffi::c_void,
                ) -> windows_core::Result<()>;
            }
            impl IAmdExtD3DDevice_Vtbl {
                pub const fn new<Identity: IAmdExtD3DDevice_Impl, const OFFSET: isize>() -> Self {
                    unsafe extern "system" fn CreateGraphicsPipelineState<
                        Identity: IAmdExtD3DDevice_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        pamdextcreateinfo: *const AmdExtD3DCreateInfo,
                        pdesc : *const windows::Win32::Graphics::Direct3D12:: D3D12_GRAPHICS_PIPELINE_STATE_DESC,
                        riid: *const windows_core::GUID,
                        pppipelinestate: *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            IAmdExtD3DDevice_Impl::CreateGraphicsPipelineState(
                                this,
                                core::mem::transmute_copy(&pamdextcreateinfo),
                                core::mem::transmute_copy(&pdesc),
                                core::mem::transmute_copy(&riid),
                                core::mem::transmute_copy(&pppipelinestate),
                            )
                            .into()
                        }
                    }
                    Self {
                        base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
                        CreateGraphicsPipelineState: CreateGraphicsPipelineState::<Identity, OFFSET>,
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IAmdExtD3DDevice as windows_core::Interface>::IID
                }
            }
            impl windows_core::RuntimeName for IAmdExtD3DDevice {}
            windows_core::imp::define_interface!(
                IAmdExtD3DDevice1,
                IAmdExtD3DDevice1_Vtbl,
                0x4bbcaf68_eaf7_4fa4_b653_cb458c334a4e
            );
            impl core::ops::Deref for IAmdExtD3DDevice1 {
                type Target = IAmdExtD3DDevice;
                fn deref(&self) -> &Self::Target {
                    unsafe { core::mem::transmute(self) }
                }
            }
            windows_core::imp::interface_hierarchy!(
                IAmdExtD3DDevice1,
                windows_core::IUnknown,
                IAmdExtD3DDevice
            );
            impl IAmdExtD3DDevice1 {
                pub unsafe fn PushMarker<P0, P1>(&self, pgfxcmdlist: P0, pmarker: P1)
                where
                    P0: windows_core::Param<
                        windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
                    >,
                    P1: windows_core::Param<windows_core::PCSTR>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).PushMarker)(
                            windows_core::Interface::as_raw(self),
                            pgfxcmdlist.param().abi(),
                            pmarker.param().abi(),
                        )
                    }
                }
                pub unsafe fn PopMarker<P0>(&self, pgfxcmdlist: P0)
                where
                    P0: windows_core::Param<
                        windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
                    >,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).PopMarker)(
                            windows_core::Interface::as_raw(self),
                            pgfxcmdlist.param().abi(),
                        )
                    }
                }
                pub unsafe fn SetMarker<P0, P1>(&self, pgfxcmdlist: P0, pmarker: P1)
                where
                    P0: windows_core::Param<
                        windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
                    >,
                    P1: windows_core::Param<windows_core::PCSTR>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).SetMarker)(
                            windows_core::Interface::as_raw(self),
                            pgfxcmdlist.param().abi(),
                            pmarker.param().abi(),
                        )
                    }
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAmdExtD3DDevice1_Vtbl {
                pub base__: IAmdExtD3DDevice_Vtbl,
                pub PushMarker: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    windows_core::PCSTR,
                ),
                pub PopMarker:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
                pub SetMarker: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    windows_core::PCSTR,
                ),
            }
            pub trait IAmdExtD3DDevice1_Impl: IAmdExtD3DDevice_Impl {
                fn PushMarker(
                    &self,
                    pgfxcmdlist: windows_core::Ref<
                        '_,
                        windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
                    >,
                    pmarker: &windows_core::PCSTR,
                );
                fn PopMarker(
                    &self,
                    pgfxcmdlist: windows_core::Ref<
                        '_,
                        windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
                    >,
                );
                fn SetMarker(
                    &self,
                    pgfxcmdlist: windows_core::Ref<
                        '_,
                        windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
                    >,
                    pmarker: &windows_core::PCSTR,
                );
            }
            impl IAmdExtD3DDevice1_Vtbl {
                pub const fn new<Identity: IAmdExtD3DDevice1_Impl, const OFFSET: isize>() -> Self {
                    unsafe extern "system" fn PushMarker<
                        Identity: IAmdExtD3DDevice1_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        pgfxcmdlist: *mut core::ffi::c_void,
                        pmarker: windows_core::PCSTR,
                    ) {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            IAmdExtD3DDevice1_Impl::PushMarker(
                                this,
                                core::mem::transmute_copy(&pgfxcmdlist),
                                core::mem::transmute(&pmarker),
                            )
                        }
                    }
                    unsafe extern "system" fn PopMarker<
                        Identity: IAmdExtD3DDevice1_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        pgfxcmdlist: *mut core::ffi::c_void,
                    ) {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            IAmdExtD3DDevice1_Impl::PopMarker(
                                this,
                                core::mem::transmute_copy(&pgfxcmdlist),
                            )
                        }
                    }
                    unsafe extern "system" fn SetMarker<
                        Identity: IAmdExtD3DDevice1_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        pgfxcmdlist: *mut core::ffi::c_void,
                        pmarker: windows_core::PCSTR,
                    ) {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            IAmdExtD3DDevice1_Impl::SetMarker(
                                this,
                                core::mem::transmute_copy(&pgfxcmdlist),
                                core::mem::transmute(&pmarker),
                            )
                        }
                    }
                    Self {
                        base__: IAmdExtD3DDevice_Vtbl::new::<Identity, OFFSET>(),
                        PushMarker: PushMarker::<Identity, OFFSET>,
                        PopMarker: PopMarker::<Identity, OFFSET>,
                        SetMarker: SetMarker::<Identity, OFFSET>,
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IAmdExtD3DDevice1 as windows_core::Interface>::IID
                        || iid == &<IAmdExtD3DDevice as windows_core::Interface>::IID
                }
            }
            impl windows_core::RuntimeName for IAmdExtD3DDevice1 {}
            windows_core::imp::define_interface!(
                IAmdExtD3DFactory,
                IAmdExtD3DFactory_Vtbl,
                0x014937ec_9288_446f_a9ac_d75a8e3a984f
            );
            windows_core::imp::interface_hierarchy!(IAmdExtD3DFactory, windows_core::IUnknown);
            impl IAmdExtD3DFactory {
                pub unsafe fn CreateInterface<P0, T>(&self, pouter: P0) -> windows_core::Result<T>
                where
                    P0: windows_core::Param<windows_core::IUnknown>,
                    T: windows_core::Interface,
                {
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        (windows_core::Interface::vtable(self).CreateInterface)(
                            windows_core::Interface::as_raw(self),
                            pouter.param().abi(),
                            &T::IID,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAmdExtD3DFactory_Vtbl {
                pub base__: windows_core::IUnknown_Vtbl,
                pub CreateInterface: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *const windows_core::GUID,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            pub trait IAmdExtD3DFactory_Impl: windows_core::IUnknownImpl {
                fn CreateInterface(
                    &self,
                    pouter: windows_core::Ref<'_, windows_core::IUnknown>,
                    riid: *const windows_core::GUID,
                    ppvobject: *mut *mut core::ffi::c_void,
                ) -> windows_core::Result<()>;
            }
            impl IAmdExtD3DFactory_Vtbl {
                pub const fn new<Identity: IAmdExtD3DFactory_Impl, const OFFSET: isize>() -> Self {
                    unsafe extern "system" fn CreateInterface<
                        Identity: IAmdExtD3DFactory_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        pouter: *mut core::ffi::c_void,
                        riid: *const windows_core::GUID,
                        ppvobject: *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            IAmdExtD3DFactory_Impl::CreateInterface(
                                this,
                                core::mem::transmute_copy(&pouter),
                                core::mem::transmute_copy(&riid),
                                core::mem::transmute_copy(&ppvobject),
                            )
                            .into()
                        }
                    }
                    Self {
                        base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
                        CreateInterface: CreateInterface::<Identity, OFFSET>,
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IAmdExtD3DFactory as windows_core::Interface>::IID
                }
            }
            impl windows_core::RuntimeName for IAmdExtD3DFactory {}
            pub type PFNAmdExtD3DCreateInterface = Option<
                unsafe extern "system" fn(
                    pouter: windows_core::Ref<'_, windows_core::IUnknown>,
                    riid: *const windows_core::GUID,
                    ppvobject: *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            >;
        }
    }
}
