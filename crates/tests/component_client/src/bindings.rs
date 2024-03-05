#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Nested {
    windows_core::imp::com_interface!(IThing, IThing_Vtbl, 0x5448be22_9873_5ae6_9106_f6e8455d2fdd);
    windows_core::imp::interface_hierarchy!(
        IThing,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IThing {
        pub fn Method(&self) -> windows_core::Result<()> {
            let this = self;
            unsafe {
                (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(
                    this,
                ))
                .ok()
            }
        }
    }
    impl windows_core::RuntimeType for IThing {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    #[repr(C)]
    pub struct IThing_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Method: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    }
}
windows_core::imp::com_interface!(IClass, IClass_Vtbl, 0x97540591_1323_59c0_9ae0_f510cae62e54);
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProperty:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Flags:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Flags) -> windows_core::HRESULT,
    pub Int32Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const i32,
        u32,
        *mut i32,
        *mut u32,
        *mut *mut i32,
        *mut u32,
        *mut *mut i32,
    ) -> windows_core::HRESULT,
    pub StringArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const std::mem::MaybeUninit<windows_core::HSTRING>,
        u32,
        *mut std::mem::MaybeUninit<windows_core::HSTRING>,
        *mut u32,
        *mut *mut std::mem::MaybeUninit<windows_core::HSTRING>,
        *mut u32,
        *mut *mut std::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Input: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetProperty)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Flags(&self) -> windows_core::Result<Flags> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Flags)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Int32Array(
        &self,
        a: &[i32],
        b: &mut [i32],
        c: &mut windows_core::Array<i32>,
    ) -> windows_core::Result<windows_core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Int32Array)(
                windows_core::Interface::as_raw(this),
                a.len().try_into().unwrap(),
                a.as_ptr(),
                b.len().try_into().unwrap(),
                b.as_mut_ptr(),
                c.set_abi_len(),
                c as *mut _ as _,
                windows_core::Array::<i32>::set_abi_len(std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn StringArray(
        &self,
        a: &[windows_core::HSTRING],
        b: &mut [windows_core::HSTRING],
        c: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).StringArray)(
                windows_core::Interface::as_raw(this),
                a.len().try_into().unwrap(),
                core::mem::transmute(a.as_ptr()),
                b.len().try_into().unwrap(),
                core::mem::transmute_copy(&b),
                c.set_abi_len(),
                c as *mut _ as _,
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn Input<P0, P1, P2, P3>(&self, a: P0, b: P1, c: P2, d: P3) -> windows_core::Result<()>
    where
        P0: windows_core::IntoParam<windows_core::IInspectable>,
        P1: windows_core::IntoParam<Class>,
        P2: windows_core::IntoParam<windows::Foundation::IStringable>,
        P3: windows_core::IntoParam<Callback>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Input)(
                windows_core::Interface::as_raw(this),
                a.into_param().abi(),
                b.into_param().abi(),
                c.into_param().abi(),
                d.into_param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = IClass_Vtbl;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "test_component.Class";
}
unsafe impl Send for Class {}
unsafe impl Sync for Class {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Flags(pub u32);
impl Flags {
    pub const Ok: Self = Self(0u32);
}
impl windows_core::TypeKind for Flags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Flags").field(&self.0).finish()
    }
}
impl Flags {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for Flags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for Flags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for Flags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for Flags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for Flags {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(test_component.Flags;u4)");
}
windows_core::imp::com_interface!(
    Callback,
    Callback_Vtbl,
    0xe39afc7e_93f1_5a1d_92ef_bd5f71c62cb8
);
impl Callback {
    pub fn new<F: FnMut(i32) -> windows_core::Result<i32> + Send + 'static>(invoke: F) -> Self {
        let com = CallbackBox::<F> {
            vtable: &CallbackBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
struct CallbackBox<F: FnMut(i32) -> windows_core::Result<i32> + Send + 'static> {
    vtable: *const Callback_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(i32) -> windows_core::Result<i32> + Send + 'static> CallbackBox<F> {
    const VTABLE: Callback_Vtbl = Callback_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <Callback as windows_core::Interface>::IID
            || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
            || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            core::ptr::null_mut()
        };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        a: i32,
        result__: *mut i32,
    ) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        match ((*this).invoke)(a) {
            Ok(ok__) => {
                core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                windows_core::HRESULT(0)
            }
            Err(err) => err.into(),
        }
    }
}
impl windows_core::RuntimeType for Callback {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct Callback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
