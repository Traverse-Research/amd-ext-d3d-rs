#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Direct3D12",
    feature = "Win32_Graphics_Dxgi_Common"
))]
pub trait IAmdExtD3DDevice_Impl: Sized {
    fn CreateGraphicsPipelineState(
        &self,
        pamdextcreateinfo: *const AmdExtD3DCreateInfo,
        pdesc: *const ::windows::Win32::Graphics::Direct3D12::D3D12_GRAPHICS_PIPELINE_STATE_DESC,
        riid: *const ::windows::core::GUID,
        pppipelinestate: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()>;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Direct3D12",
    feature = "Win32_Graphics_Dxgi_Common"
))]
impl ::windows::core::RuntimeName for IAmdExtD3DDevice {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Direct3D12",
    feature = "Win32_Graphics_Dxgi_Common"
))]
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
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Direct3D12",
    feature = "Win32_Graphics_Dxgi_Common"
))]
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
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Direct3D12",
    feature = "Win32_Graphics_Dxgi_Common"
))]
impl ::windows::core::RuntimeName for IAmdExtD3DDevice1 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Direct3D12",
    feature = "Win32_Graphics_Dxgi_Common"
))]
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
