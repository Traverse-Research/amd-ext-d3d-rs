#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
pub struct IAmdExtD3DDevice(::windows::core::IUnknown);
impl IAmdExtD3DDevice {
    pub unsafe fn CreateGraphicsPipelineState<T>(
        &self,
        pamdextcreateinfo: *const AmdExtD3DCreateInfo,
        pdesc: *const ::windows::Win32::Graphics::Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    ) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateGraphicsPipelineState)(
            ::windows::core::Interface::as_raw(self),
            pamdextcreateinfo,
            pdesc,
            &<T as ::windows::core::ComInterface>::IID,
            &mut result__,
        )
        .from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAmdExtD3DDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAmdExtD3DDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAmdExtD3DDevice {}
impl ::core::fmt::Debug for IAmdExtD3DDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAmdExtD3DDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAmdExtD3DDevice {
    type Vtable = IAmdExtD3DDevice_Vtbl;
}
impl ::core::clone::Clone for IAmdExtD3DDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAmdExtD3DDevice {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8104c0fc_7413_410f_8e83_aa617e908648);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAmdExtD3DDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateGraphicsPipelineState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pamdextcreateinfo: *const AmdExtD3DCreateInfo,
        pdesc: *const ::windows::Win32::Graphics::Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
        riid: *const ::windows::core::GUID,
        pppipelinestate: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IAmdExtD3DDevice1(::windows::core::IUnknown);
impl IAmdExtD3DDevice1 {
    pub unsafe fn CreateGraphicsPipelineState<T>(
        &self,
        pamdextcreateinfo: *const AmdExtD3DCreateInfo,
        pdesc: *const ::windows::Win32::Graphics::Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    ) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self)
            .base__
            .CreateGraphicsPipelineState)(
            ::windows::core::Interface::as_raw(self),
            pamdextcreateinfo,
            pdesc,
            &<T as ::windows::core::ComInterface>::IID,
            &mut result__,
        )
        .from_abi(result__)
    }
    pub unsafe fn PushMarker<P0, P1>(&self, pgfxcmdlist: P0, pmarker: P1)
    where
        P0: ::windows::core::IntoParam<
            ::windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
        >,
        P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).PushMarker)(
            ::windows::core::Interface::as_raw(self),
            pgfxcmdlist.into_param().abi(),
            pmarker.into_param().abi(),
        )
    }
    pub unsafe fn PopMarker<P0>(&self, pgfxcmdlist: P0)
    where
        P0: ::windows::core::IntoParam<
            ::windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
        >,
    {
        (::windows::core::Interface::vtable(self).PopMarker)(
            ::windows::core::Interface::as_raw(self),
            pgfxcmdlist.into_param().abi(),
        )
    }
    pub unsafe fn SetMarker<P0, P1>(&self, pgfxcmdlist: P0, pmarker: P1)
    where
        P0: ::windows::core::IntoParam<
            ::windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
        >,
        P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).SetMarker)(
            ::windows::core::Interface::as_raw(self),
            pgfxcmdlist.into_param().abi(),
            pmarker.into_param().abi(),
        )
    }
}
::windows::imp::interface_hierarchy!(
    IAmdExtD3DDevice1,
    ::windows::core::IUnknown,
    IAmdExtD3DDevice
);
impl ::core::cmp::PartialEq for IAmdExtD3DDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAmdExtD3DDevice1 {}
impl ::core::fmt::Debug for IAmdExtD3DDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAmdExtD3DDevice1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAmdExtD3DDevice1 {
    type Vtable = IAmdExtD3DDevice1_Vtbl;
}
impl ::core::clone::Clone for IAmdExtD3DDevice1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAmdExtD3DDevice1 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4bbcaf68_eaf7_4fa4_b653_cb458c334a4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAmdExtD3DDevice1_Vtbl {
    pub base__: IAmdExtD3DDevice_Vtbl,
    pub PushMarker: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pgfxcmdlist: *mut ::core::ffi::c_void,
        pmarker: ::windows::core::PCSTR,
    ),
    pub PopMarker: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pgfxcmdlist: *mut ::core::ffi::c_void,
    ),
    pub SetMarker: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pgfxcmdlist: *mut ::core::ffi::c_void,
        pmarker: ::windows::core::PCSTR,
    ),
}
#[repr(transparent)]
pub struct IAmdExtD3DFactory(::windows::core::IUnknown);
impl IAmdExtD3DFactory {
    pub unsafe fn CreateInterface<P0, T>(&self, pouter: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateInterface)(
            ::windows::core::Interface::as_raw(self),
            pouter.into_param().abi(),
            &<T as ::windows::core::ComInterface>::IID,
            &mut result__,
        )
        .from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAmdExtD3DFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAmdExtD3DFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAmdExtD3DFactory {}
impl ::core::fmt::Debug for IAmdExtD3DFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAmdExtD3DFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAmdExtD3DFactory {
    type Vtable = IAmdExtD3DFactory_Vtbl;
}
impl ::core::clone::Clone for IAmdExtD3DFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAmdExtD3DFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x014937ec_9288_446f_a9ac_d75a8e3a984f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAmdExtD3DFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateInterface: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pouter: *mut ::core::ffi::c_void,
        riid: *const ::windows::core::GUID,
        ppvobject: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AmdExtD3DStructType(pub u32);
pub const AmdExtD3DStructUnknown: AmdExtD3DStructType = AmdExtD3DStructType(0u32);
pub const AmdExtD3DStructPipelineState: AmdExtD3DStructType = AmdExtD3DStructType(1u32);
impl ::core::marker::Copy for AmdExtD3DStructType {}
impl ::core::clone::Clone for AmdExtD3DStructType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AmdExtD3DStructType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AmdExtD3DStructType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AmdExtD3DStructType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AmdExtD3DStructType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct AmdExtD3DCreateInfo {
    pub r#type: AmdExtD3DStructType,
    pub pNext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for AmdExtD3DCreateInfo {}
impl ::core::clone::Clone for AmdExtD3DCreateInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AmdExtD3DCreateInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AmdExtD3DCreateInfo")
            .field("type", &self.r#type)
            .field("pNext", &self.pNext)
            .finish()
    }
}
impl ::windows::core::TypeKind for AmdExtD3DCreateInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AmdExtD3DCreateInfo {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.pNext == other.pNext
    }
}
impl ::core::cmp::Eq for AmdExtD3DCreateInfo {}
impl ::core::default::Default for AmdExtD3DCreateInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AmdExtD3DPipelineCreateInfo {
    pub Base: AmdExtD3DCreateInfo,
    pub flags: AmdExtD3DPipelineFlags,
}
impl ::core::marker::Copy for AmdExtD3DPipelineCreateInfo {}
impl ::core::clone::Clone for AmdExtD3DPipelineCreateInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AmdExtD3DPipelineCreateInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AmdExtD3DPipelineCreateInfo")
            .field("Base", &self.Base)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::windows::core::TypeKind for AmdExtD3DPipelineCreateInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AmdExtD3DPipelineCreateInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for AmdExtD3DPipelineCreateInfo {}
impl ::core::default::Default for AmdExtD3DPipelineCreateInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AmdExtD3DPipelineFlags {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for AmdExtD3DPipelineFlags {}
impl ::core::clone::Clone for AmdExtD3DPipelineFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AmdExtD3DPipelineFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AmdExtD3DPipelineFlags")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::windows::core::TypeKind for AmdExtD3DPipelineFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AmdExtD3DPipelineFlags {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for AmdExtD3DPipelineFlags {}
impl ::core::default::Default for AmdExtD3DPipelineFlags {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type PFNAmdExtD3DCreateInterface = ::core::option::Option<
    unsafe extern "system" fn(
        pouter: ::core::option::Option<::windows::core::IUnknown>,
        riid: *const ::windows::core::GUID,
        ppvobject: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
>;
pub trait IAmdExtD3DDevice_Impl: Sized {
    fn CreateGraphicsPipelineState(
        &self,
        pamdextcreateinfo: *const AmdExtD3DCreateInfo,
        pdesc: *const ::windows::Win32::Graphics::Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
        riid: *const ::windows::core::GUID,
        pppipelinestate: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAmdExtD3DDevice {}
impl IAmdExtD3DDevice_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAmdExtD3DDevice_Impl,
        const OFFSET: isize,
    >() -> IAmdExtD3DDevice_Vtbl {
        unsafe extern "system" fn CreateGraphicsPipelineState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAmdExtD3DDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            pamdextcreateinfo: *const AmdExtD3DCreateInfo,
            pdesc : *const ::windows::Win32::Graphics::Direct3D12:: D3D12_GRAPHICS_PIPELINE_STATE_DESC,
            riid: *const ::windows::core::GUID,
            pppipelinestate: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateGraphicsPipelineState(
                ::core::mem::transmute_copy(&pamdextcreateinfo),
                ::core::mem::transmute_copy(&pdesc),
                ::core::mem::transmute_copy(&riid),
                ::core::mem::transmute_copy(&pppipelinestate),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateGraphicsPipelineState: CreateGraphicsPipelineState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAmdExtD3DDevice as ::windows::core::ComInterface>::IID
    }
}
pub trait IAmdExtD3DDevice1_Impl: Sized + IAmdExtD3DDevice_Impl {
    fn PushMarker(
        &self,
        pgfxcmdlist: ::core::option::Option<
            &::windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
        >,
        pmarker: &::windows::core::PCSTR,
    );
    fn PopMarker(
        &self,
        pgfxcmdlist: ::core::option::Option<
            &::windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
        >,
    );
    fn SetMarker(
        &self,
        pgfxcmdlist: ::core::option::Option<
            &::windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList,
        >,
        pmarker: &::windows::core::PCSTR,
    );
}
impl ::windows::core::RuntimeName for IAmdExtD3DDevice1 {}
impl IAmdExtD3DDevice1_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAmdExtD3DDevice1_Impl,
        const OFFSET: isize,
    >() -> IAmdExtD3DDevice1_Vtbl {
        unsafe extern "system" fn PushMarker<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAmdExtD3DDevice1_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            pgfxcmdlist: *mut ::core::ffi::c_void,
            pmarker: ::windows::core::PCSTR,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushMarker(
                ::windows::core::from_raw_borrowed(&pgfxcmdlist),
                ::core::mem::transmute(&pmarker),
            )
        }
        unsafe extern "system" fn PopMarker<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAmdExtD3DDevice1_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            pgfxcmdlist: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopMarker(::windows::core::from_raw_borrowed(&pgfxcmdlist))
        }
        unsafe extern "system" fn SetMarker<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAmdExtD3DDevice1_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            pgfxcmdlist: *mut ::core::ffi::c_void,
            pmarker: ::windows::core::PCSTR,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMarker(
                ::windows::core::from_raw_borrowed(&pgfxcmdlist),
                ::core::mem::transmute(&pmarker),
            )
        }
        Self {
            base__: IAmdExtD3DDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            PushMarker: PushMarker::<Identity, Impl, OFFSET>,
            PopMarker: PopMarker::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAmdExtD3DDevice1 as ::windows::core::ComInterface>::IID
            || iid == &<IAmdExtD3DDevice as ::windows::core::ComInterface>::IID
    }
}
pub trait IAmdExtD3DFactory_Impl: Sized {
    fn CreateInterface(
        &self,
        pouter: ::core::option::Option<&::windows::core::IUnknown>,
        riid: *const ::windows::core::GUID,
        ppvobject: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAmdExtD3DFactory {}
impl IAmdExtD3DFactory_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAmdExtD3DFactory_Impl,
        const OFFSET: isize,
    >() -> IAmdExtD3DFactory_Vtbl {
        unsafe extern "system" fn CreateInterface<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAmdExtD3DFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            pouter: *mut ::core::ffi::c_void,
            riid: *const ::windows::core::GUID,
            ppvobject: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInterface(
                ::windows::core::from_raw_borrowed(&pouter),
                ::core::mem::transmute_copy(&riid),
                ::core::mem::transmute_copy(&ppvobject),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInterface: CreateInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAmdExtD3DFactory as ::windows::core::ComInterface>::IID
    }
}
