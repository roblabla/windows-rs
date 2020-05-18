use crate::{hstring, ErrorCode, Guid, RawPtr};

#[link(name = "winrt_kernel32")]
extern "system" {
    pub fn GetProcessHeap() -> RawPtr;
    pub fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    pub fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;

    pub fn CreateEventW(security: RawPtr, manual: i32, state: i32, name: RawPtr) -> RawPtr;
    pub fn SetEvent(handle: RawPtr) -> i32;
    pub fn WaitForSingleObject(handle: RawPtr, milliseconds: u32) -> u32;
    pub fn CloseHandle(handle: RawPtr) -> i32;
}

#[link(name = "winrt_combase")]
extern "system" {
    pub fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;
    pub fn RoGetActivationFactory(
        hstring: *mut hstring::Header,
        interface: &Guid,
        result: *mut RawPtr,
    ) -> ErrorCode;
}
