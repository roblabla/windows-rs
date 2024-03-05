#[inline]
pub unsafe fn CloseHandle<P0>(hobject: P0) -> windows_core::Result<()>
where
    P0: windows_core::IntoParam<HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
    CloseHandle(hobject.into_param().abi()).ok()
}
#[inline]
pub unsafe fn CompareObjectHandles<P0, P1>(hfirstobjecthandle: P0, hsecondobjecthandle: P1) -> BOOL
where
    P0: windows_core::IntoParam<HANDLE>,
    P1: windows_core::IntoParam<HANDLE>,
{
    windows_targets::link!("api-ms-win-core-handle-l1-1-0.dll" "system" fn CompareObjectHandles(hfirstobjecthandle : HANDLE, hsecondobjecthandle : HANDLE) -> BOOL);
    CompareObjectHandles(hfirstobjecthandle.into_param().abi(), hsecondobjecthandle.into_param().abi())
}
#[inline]
pub unsafe fn DuplicateHandle<P0, P1, P2, P3>(hsourceprocesshandle: P0, hsourcehandle: P1, htargetprocesshandle: P2, lptargethandle: *mut HANDLE, dwdesiredaccess: u32, binherithandle: P3, dwoptions: DUPLICATE_HANDLE_OPTIONS) -> windows_core::Result<()>
where
    P0: windows_core::IntoParam<HANDLE>,
    P1: windows_core::IntoParam<HANDLE>,
    P2: windows_core::IntoParam<HANDLE>,
    P3: windows_core::IntoParam<BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : HANDLE, hsourcehandle : HANDLE, htargetprocesshandle : HANDLE, lptargethandle : *mut HANDLE, dwdesiredaccess : u32, binherithandle : BOOL, dwoptions : DUPLICATE_HANDLE_OPTIONS) -> BOOL);
    DuplicateHandle(hsourceprocesshandle.into_param().abi(), hsourcehandle.into_param().abi(), htargetprocesshandle.into_param().abi(), lptargethandle, dwdesiredaccess, binherithandle.into_param().abi(), dwoptions).ok()
}
#[inline]
pub unsafe fn FreeLibrary<P0>(hlibmodule: P0) -> windows_core::Result<()>
where
    P0: windows_core::IntoParam<HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> BOOL);
    FreeLibrary(hlibmodule.into_param().abi()).ok()
}
#[inline]
pub unsafe fn GetHandleInformation<P0>(hobject: P0, lpdwflags: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::IntoParam<HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetHandleInformation(hobject : HANDLE, lpdwflags : *mut u32) -> BOOL);
    GetHandleInformation(hobject.into_param().abi(), lpdwflags).ok()
}
#[inline]
pub unsafe fn GetLastError() -> WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
    GetLastError()
}
#[inline]
pub unsafe fn GlobalFree<P0>(hmem: P0) -> windows_core::Result<HGLOBAL>
where
    P0: windows_core::IntoParam<HGLOBAL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GlobalFree(hmem : HGLOBAL) -> HGLOBAL);
    let result__ = GlobalFree(hmem.into_param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LocalFree<P0>(hmem: P0) -> HLOCAL
where
    P0: windows_core::IntoParam<HLOCAL>,
{
    windows_targets::link!("kernel32.dll" "system" fn LocalFree(hmem : HLOCAL) -> HLOCAL);
    LocalFree(hmem.into_param().abi())
}
#[inline]
pub unsafe fn RtlNtStatusToDosError<P0>(status: P0) -> u32
where
    P0: windows_core::IntoParam<NTSTATUS>,
{
    windows_targets::link!("ntdll.dll" "system" fn RtlNtStatusToDosError(status : NTSTATUS) -> u32);
    RtlNtStatusToDosError(status.into_param().abi())
}
#[inline]
pub unsafe fn SetHandleInformation<P0>(hobject: P0, dwmask: u32, dwflags: HANDLE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::IntoParam<HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetHandleInformation(hobject : HANDLE, dwmask : u32, dwflags : HANDLE_FLAGS) -> BOOL);
    SetHandleInformation(hobject.into_param().abi(), dwmask, dwflags).ok()
}
#[inline]
pub unsafe fn SetLastError(dwerrcode: WIN32_ERROR) {
    windows_targets::link!("kernel32.dll" "system" fn SetLastError(dwerrcode : WIN32_ERROR));
    SetLastError(dwerrcode)
}
#[inline]
pub unsafe fn SetLastErrorEx(dwerrcode: WIN32_ERROR, dwtype: u32) {
    windows_targets::link!("user32.dll" "system" fn SetLastErrorEx(dwerrcode : WIN32_ERROR, dwtype : u32));
    SetLastErrorEx(dwerrcode, dwtype)
}
#[inline]
pub unsafe fn SysAddRefString<P0>(bstrstring: P0) -> windows_core::Result<()>
where
    P0: windows_core::IntoParam<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysAddRefString(bstrstring : std::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    SysAddRefString(bstrstring.into_param().abi()).ok()
}
#[inline]
pub unsafe fn SysAllocString<P0>(psz: P0) -> windows_core::BSTR
where
    P0: windows_core::IntoParam<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysAllocString(psz : windows_core::PCWSTR) -> windows_core::BSTR);
    SysAllocString(psz.into_param().abi())
}
#[inline]
pub unsafe fn SysAllocStringByteLen(psz: Option<&[u8]>) -> windows_core::BSTR {
    windows_targets::link!("oleaut32.dll" "system" fn SysAllocStringByteLen(psz : windows_core::PCSTR, len : u32) -> windows_core::BSTR);
    SysAllocStringByteLen(core::mem::transmute(psz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SysAllocStringLen(strin: Option<&[u16]>) -> windows_core::BSTR {
    windows_targets::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : windows_core::PCWSTR, ui : u32) -> windows_core::BSTR);
    SysAllocStringLen(core::mem::transmute(strin.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), strin.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SysFreeString<P0>(bstrstring: P0)
where
    P0: windows_core::IntoParam<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : std::mem::MaybeUninit < windows_core::BSTR >));
    SysFreeString(bstrstring.into_param().abi())
}
#[inline]
pub unsafe fn SysReAllocString<P0>(pbstr: *mut windows_core::BSTR, psz: P0) -> i32
where
    P0: windows_core::IntoParam<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysReAllocString(pbstr : *mut std::mem::MaybeUninit < windows_core::BSTR >, psz : windows_core::PCWSTR) -> i32);
    SysReAllocString(core::mem::transmute(pbstr), psz.into_param().abi())
}
#[inline]
pub unsafe fn SysReAllocStringLen<P0>(pbstr: *mut windows_core::BSTR, psz: P0, len: u32) -> i32
where
    P0: windows_core::IntoParam<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysReAllocStringLen(pbstr : *mut std::mem::MaybeUninit < windows_core::BSTR >, psz : windows_core::PCWSTR, len : u32) -> i32);
    SysReAllocStringLen(core::mem::transmute(pbstr), psz.into_param().abi(), len)
}
#[inline]
pub unsafe fn SysReleaseString<P0>(bstrstring: P0)
where
    P0: windows_core::IntoParam<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysReleaseString(bstrstring : std::mem::MaybeUninit < windows_core::BSTR >));
    SysReleaseString(bstrstring.into_param().abi())
}
#[inline]
pub unsafe fn SysStringByteLen<P0>(bstr: P0) -> u32
where
    P0: windows_core::IntoParam<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysStringByteLen(bstr : std::mem::MaybeUninit < windows_core::BSTR >) -> u32);
    SysStringByteLen(bstr.into_param().abi())
}
#[inline]
pub unsafe fn SysStringLen<P0>(pbstr: P0) -> u32
where
    P0: windows_core::IntoParam<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : std::mem::MaybeUninit < windows_core::BSTR >) -> u32);
    SysStringLen(pbstr.into_param().abi())
}
pub const APPMODEL_ERROR_DYNAMIC_PROPERTY_INVALID: WIN32_ERROR = WIN32_ERROR(15705u32);
pub const APPMODEL_ERROR_DYNAMIC_PROPERTY_READ_FAILED: WIN32_ERROR = WIN32_ERROR(15704u32);
pub const APPMODEL_ERROR_NO_APPLICATION: WIN32_ERROR = WIN32_ERROR(15703u32);
pub const APPMODEL_ERROR_NO_MUTABLE_DIRECTORY: WIN32_ERROR = WIN32_ERROR(15707u32);
pub const APPMODEL_ERROR_NO_PACKAGE: WIN32_ERROR = WIN32_ERROR(15700u32);
pub const APPMODEL_ERROR_PACKAGE_IDENTITY_CORRUPT: WIN32_ERROR = WIN32_ERROR(15702u32);
pub const APPMODEL_ERROR_PACKAGE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(15706u32);
pub const APPMODEL_ERROR_PACKAGE_RUNTIME_CORRUPT: WIN32_ERROR = WIN32_ERROR(15701u32);
pub const APPX_E_BLOCK_HASH_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80080207_u32 as _);
pub const APPX_E_CORRUPT_CONTENT: windows_core::HRESULT = windows_core::HRESULT(0x80080206_u32 as _);
pub const APPX_E_DELTA_APPENDED_PACKAGE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80080210_u32 as _);
pub const APPX_E_DELTA_BASELINE_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8008020D_u32 as _);
pub const APPX_E_DELTA_PACKAGE_MISSING_FILE: windows_core::HRESULT = windows_core::HRESULT(0x8008020E_u32 as _);
pub const APPX_E_DIGEST_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80080219_u32 as _);
pub const APPX_E_FILE_COMPRESSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80080214_u32 as _);
pub const APPX_E_INTERLEAVING_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80080201_u32 as _);
pub const APPX_E_INVALID_APPINSTALLER: windows_core::HRESULT = windows_core::HRESULT(0x8008020C_u32 as _);
pub const APPX_E_INVALID_BLOCKMAP: windows_core::HRESULT = windows_core::HRESULT(0x80080205_u32 as _);
pub const APPX_E_INVALID_CONTENTGROUPMAP: windows_core::HRESULT = windows_core::HRESULT(0x8008020B_u32 as _);
pub const APPX_E_INVALID_DELTA_PACKAGE: windows_core::HRESULT = windows_core::HRESULT(0x8008020F_u32 as _);
pub const APPX_E_INVALID_ENCRYPTION_EXCLUSION_FILE_LIST: windows_core::HRESULT = windows_core::HRESULT(0x80080216_u32 as _);
pub const APPX_E_INVALID_KEY_INFO: windows_core::HRESULT = windows_core::HRESULT(0x8008020A_u32 as _);
pub const APPX_E_INVALID_MANIFEST: windows_core::HRESULT = windows_core::HRESULT(0x80080204_u32 as _);
pub const APPX_E_INVALID_PACKAGESIGNCONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80080212_u32 as _);
pub const APPX_E_INVALID_PACKAGE_FOLDER_ACLS: windows_core::HRESULT = windows_core::HRESULT(0x80080217_u32 as _);
pub const APPX_E_INVALID_PACKAGING_LAYOUT: windows_core::HRESULT = windows_core::HRESULT(0x80080211_u32 as _);
pub const APPX_E_INVALID_PAYLOAD_PACKAGE_EXTENSION: windows_core::HRESULT = windows_core::HRESULT(0x80080215_u32 as _);
pub const APPX_E_INVALID_PUBLISHER_BRIDGING: windows_core::HRESULT = windows_core::HRESULT(0x80080218_u32 as _);
pub const APPX_E_INVALID_SIP_CLIENT_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80080209_u32 as _);
pub const APPX_E_MISSING_REQUIRED_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80080203_u32 as _);
pub const APPX_E_PACKAGING_INTERNAL: windows_core::HRESULT = windows_core::HRESULT(0x80080200_u32 as _);
pub const APPX_E_RELATIONSHIPS_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80080202_u32 as _);
pub const APPX_E_REQUESTED_RANGE_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80080208_u32 as _);
pub const APPX_E_RESOURCESPRI_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80080213_u32 as _);
pub const APP_LOCAL_DEVICE_ID_SIZE: u32 = 32u32;
pub const BT_E_SPURIOUS_ACTIVATION: windows_core::HRESULT = windows_core::HRESULT(0x80080300_u32 as _);
pub const CACHE_E_FIRST: i32 = -2147221136i32;
pub const CACHE_E_LAST: i32 = -2147221121i32;
pub const CACHE_E_NOCACHE_UPDATED: windows_core::HRESULT = windows_core::HRESULT(0x80040170_u32 as _);
pub const CACHE_S_FIRST: i32 = 262512i32;
pub const CACHE_S_FORMATETC_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x40170_u32 as _);
pub const CACHE_S_LAST: i32 = 262527i32;
pub const CACHE_S_SAMECACHE: windows_core::HRESULT = windows_core::HRESULT(0x40171_u32 as _);
pub const CACHE_S_SOMECACHES_NOTUPDATED: windows_core::HRESULT = windows_core::HRESULT(0x40172_u32 as _);
pub const CAT_E_CATIDNOEXIST: windows_core::HRESULT = windows_core::HRESULT(0x80040160_u32 as _);
pub const CAT_E_FIRST: i32 = -2147221152i32;
pub const CAT_E_LAST: i32 = -2147221151i32;
pub const CAT_E_NODESCRIPTION: windows_core::HRESULT = windows_core::HRESULT(0x80040161_u32 as _);
pub const CERTSRV_E_ADMIN_DENIED_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80094014_u32 as _);
pub const CERTSRV_E_ALIGNMENT_FAULT: windows_core::HRESULT = windows_core::HRESULT(0x80094010_u32 as _);
pub const CERTSRV_E_ARCHIVED_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094804_u32 as _);
pub const CERTSRV_E_ARCHIVED_KEY_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80094810_u32 as _);
pub const CERTSRV_E_BAD_RENEWAL_CERT_ATTRIBUTE: windows_core::HRESULT = windows_core::HRESULT(0x8009400E_u32 as _);
pub const CERTSRV_E_BAD_RENEWAL_SUBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80094806_u32 as _);
pub const CERTSRV_E_BAD_REQUESTSTATUS: windows_core::HRESULT = windows_core::HRESULT(0x80094003_u32 as _);
pub const CERTSRV_E_BAD_REQUESTSUBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80094001_u32 as _);
pub const CERTSRV_E_BAD_REQUEST_KEY_ARCHIVAL: windows_core::HRESULT = windows_core::HRESULT(0x8009400C_u32 as _);
pub const CERTSRV_E_BAD_TEMPLATE_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80094807_u32 as _);
pub const CERTSRV_E_CERT_TYPE_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0x80094814_u32 as _);
pub const CERTSRV_E_CORRUPT_KEY_ATTESTATION: windows_core::HRESULT = windows_core::HRESULT(0x8009481B_u32 as _);
pub const CERTSRV_E_DOWNLEVEL_DC_SSL_OR_UPGRADE: windows_core::HRESULT = windows_core::HRESULT(0x80094013_u32 as _);
pub const CERTSRV_E_ENCODING_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x80094007_u32 as _);
pub const CERTSRV_E_ENCRYPTION_CERT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094018_u32 as _);
pub const CERTSRV_E_ENROLL_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80094011_u32 as _);
pub const CERTSRV_E_EXPIRED_CHALLENGE: windows_core::HRESULT = windows_core::HRESULT(0x8009481C_u32 as _);
pub const CERTSRV_E_INVALID_ATTESTATION: windows_core::HRESULT = windows_core::HRESULT(0x80094819_u32 as _);
pub const CERTSRV_E_INVALID_CA_CERTIFICATE: windows_core::HRESULT = windows_core::HRESULT(0x80094005_u32 as _);
pub const CERTSRV_E_INVALID_EK: windows_core::HRESULT = windows_core::HRESULT(0x80094817_u32 as _);
pub const CERTSRV_E_INVALID_IDBINDING: windows_core::HRESULT = windows_core::HRESULT(0x80094818_u32 as _);
pub const CERTSRV_E_INVALID_REQUESTID: windows_core::HRESULT = windows_core::HRESULT(0x8009481E_u32 as _);
pub const CERTSRV_E_INVALID_RESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x8009481D_u32 as _);
pub const CERTSRV_E_ISSUANCE_POLICY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8009480C_u32 as _);
pub const CERTSRV_E_KEY_ARCHIVAL_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x8009400A_u32 as _);
pub const CERTSRV_E_KEY_ATTESTATION: windows_core::HRESULT = windows_core::HRESULT(0x8009481A_u32 as _);
pub const CERTSRV_E_KEY_ATTESTATION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80094017_u32 as _);
pub const CERTSRV_E_KEY_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x80094811_u32 as _);
pub const CERTSRV_E_NO_CAADMIN_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x8009400D_u32 as _);
pub const CERTSRV_E_NO_CERT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80094801_u32 as _);
pub const CERTSRV_E_NO_DB_SESSIONS: windows_core::HRESULT = windows_core::HRESULT(0x8009400F_u32 as _);
pub const CERTSRV_E_NO_POLICY_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80094015_u32 as _);
pub const CERTSRV_E_NO_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80094002_u32 as _);
pub const CERTSRV_E_NO_VALID_KRA: windows_core::HRESULT = windows_core::HRESULT(0x8009400B_u32 as _);
pub const CERTSRV_E_PENDING_CLIENT_RESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x80094820_u32 as _);
pub const CERTSRV_E_PROPERTY_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80094004_u32 as _);
pub const CERTSRV_E_RENEWAL_BAD_PUBLIC_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80094816_u32 as _);
pub const CERTSRV_E_REQUEST_PRECERTIFICATE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8009481F_u32 as _);
pub const CERTSRV_E_RESTRICTEDOFFICER: windows_core::HRESULT = windows_core::HRESULT(0x80094009_u32 as _);
pub const CERTSRV_E_ROLECONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x80094008_u32 as _);
pub const CERTSRV_E_SEC_EXT_DIRECTORY_SID_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094821_u32 as _);
pub const CERTSRV_E_SERVER_SUSPENDED: windows_core::HRESULT = windows_core::HRESULT(0x80094006_u32 as _);
pub const CERTSRV_E_SIGNATURE_COUNT: windows_core::HRESULT = windows_core::HRESULT(0x8009480A_u32 as _);
pub const CERTSRV_E_SIGNATURE_POLICY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094809_u32 as _);
pub const CERTSRV_E_SIGNATURE_REJECTED: windows_core::HRESULT = windows_core::HRESULT(0x8009480B_u32 as _);
pub const CERTSRV_E_SMIME_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094805_u32 as _);
pub const CERTSRV_E_SUBJECT_ALT_NAME_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094803_u32 as _);
pub const CERTSRV_E_SUBJECT_DIRECTORY_GUID_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8009480E_u32 as _);
pub const CERTSRV_E_SUBJECT_DNS_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8009480F_u32 as _);
pub const CERTSRV_E_SUBJECT_EMAIL_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094812_u32 as _);
pub const CERTSRV_E_SUBJECT_UPN_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8009480D_u32 as _);
pub const CERTSRV_E_TEMPLATE_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x80094802_u32 as _);
pub const CERTSRV_E_TEMPLATE_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80094012_u32 as _);
pub const CERTSRV_E_TEMPLATE_POLICY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80094808_u32 as _);
pub const CERTSRV_E_TOO_MANY_SIGNATURES: windows_core::HRESULT = windows_core::HRESULT(0x80094815_u32 as _);
pub const CERTSRV_E_UNKNOWN_CERT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80094813_u32 as _);
pub const CERTSRV_E_UNSUPPORTED_CERT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80094800_u32 as _);
pub const CERTSRV_E_WEAK_SIGNATURE_OR_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80094016_u32 as _);
pub const CERT_E_CHAINING: windows_core::HRESULT = windows_core::HRESULT(0x800B010A_u32 as _);
pub const CERT_E_CN_NO_MATCH: windows_core::HRESULT = windows_core::HRESULT(0x800B010F_u32 as _);
pub const CERT_E_CRITICAL: windows_core::HRESULT = windows_core::HRESULT(0x800B0105_u32 as _);
pub const CERT_E_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x800B0101_u32 as _);
pub const CERT_E_INVALID_NAME: windows_core::HRESULT = windows_core::HRESULT(0x800B0114_u32 as _);
pub const CERT_E_INVALID_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x800B0113_u32 as _);
pub const CERT_E_ISSUERCHAINING: windows_core::HRESULT = windows_core::HRESULT(0x800B0107_u32 as _);
pub const CERT_E_MALFORMED: windows_core::HRESULT = windows_core::HRESULT(0x800B0108_u32 as _);
pub const CERT_E_PATHLENCONST: windows_core::HRESULT = windows_core::HRESULT(0x800B0104_u32 as _);
pub const CERT_E_PURPOSE: windows_core::HRESULT = windows_core::HRESULT(0x800B0106_u32 as _);
pub const CERT_E_REVOCATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x800B010E_u32 as _);
pub const CERT_E_REVOKED: windows_core::HRESULT = windows_core::HRESULT(0x800B010C_u32 as _);
pub const CERT_E_ROLE: windows_core::HRESULT = windows_core::HRESULT(0x800B0103_u32 as _);
pub const CERT_E_UNTRUSTEDCA: windows_core::HRESULT = windows_core::HRESULT(0x800B0112_u32 as _);
pub const CERT_E_UNTRUSTEDROOT: windows_core::HRESULT = windows_core::HRESULT(0x800B0109_u32 as _);
pub const CERT_E_UNTRUSTEDTESTROOT: windows_core::HRESULT = windows_core::HRESULT(0x800B010D_u32 as _);
pub const CERT_E_VALIDITYPERIODNESTING: windows_core::HRESULT = windows_core::HRESULT(0x800B0102_u32 as _);
pub const CERT_E_WRONG_USAGE: windows_core::HRESULT = windows_core::HRESULT(0x800B0110_u32 as _);
pub const CI_CORRUPT_CATALOG: windows_core::HRESULT = windows_core::HRESULT(0xC0041801_u32 as _);
pub const CI_CORRUPT_DATABASE: windows_core::HRESULT = windows_core::HRESULT(0xC0041800_u32 as _);
pub const CI_CORRUPT_FILTER_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0xC0041807_u32 as _);
pub const CI_E_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8004180A_u32 as _);
pub const CI_E_BUFFERTOOSMALL: windows_core::HRESULT = windows_core::HRESULT(0x8004180C_u32 as _);
pub const CI_E_CARDINALITY_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80041827_u32 as _);
pub const CI_E_CLIENT_FILTER_ABORT: windows_core::HRESULT = windows_core::HRESULT(0xC0041824_u32 as _);
pub const CI_E_CONFIG_DISK_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80041828_u32 as _);
pub const CI_E_DISK_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80041811_u32 as _);
pub const CI_E_DISTRIBUTED_GROUPBY_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80041829_u32 as _);
pub const CI_E_DUPLICATE_NOTIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x80041817_u32 as _);
pub const CI_E_ENUMERATION_STARTED: windows_core::HRESULT = windows_core::HRESULT(0xC0041822_u32 as _);
pub const CI_E_FILTERING_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80041810_u32 as _);
pub const CI_E_INVALID_FLAGS_COMBINATION: windows_core::HRESULT = windows_core::HRESULT(0x80041819_u32 as _);
pub const CI_E_INVALID_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8004180F_u32 as _);
pub const CI_E_LOGON_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8004181C_u32 as _);
pub const CI_E_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80041815_u32 as _);
pub const CI_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8004180B_u32 as _);
pub const CI_E_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80041820_u32 as _);
pub const CI_E_NO_CATALOG: windows_core::HRESULT = windows_core::HRESULT(0x8004181D_u32 as _);
pub const CI_E_OUTOFSEQ_INCREMENT_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8004181A_u32 as _);
pub const CI_E_PROPERTY_NOT_CACHED: windows_core::HRESULT = windows_core::HRESULT(0x8004180D_u32 as _);
pub const CI_E_PROPERTY_TOOLARGE: windows_core::HRESULT = windows_core::HRESULT(0xC0041823_u32 as _);
pub const CI_E_SHARING_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x8004181B_u32 as _);
pub const CI_E_SHUTDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80041812_u32 as _);
pub const CI_E_STRANGE_PAGEORSECTOR_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x8004181E_u32 as _);
pub const CI_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004181F_u32 as _);
pub const CI_E_UPDATES_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80041818_u32 as _);
pub const CI_E_USE_DEFAULT_PID: windows_core::HRESULT = windows_core::HRESULT(0x80041816_u32 as _);
pub const CI_E_WORKID_NOTVALID: windows_core::HRESULT = windows_core::HRESULT(0x80041813_u32 as _);
pub const CI_INCORRECT_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC0041821_u32 as _);
pub const CI_INVALID_INDEX: windows_core::HRESULT = windows_core::HRESULT(0xC0041808_u32 as _);
pub const CI_INVALID_PARTITION: windows_core::HRESULT = windows_core::HRESULT(0xC0041802_u32 as _);
pub const CI_INVALID_PRIORITY: windows_core::HRESULT = windows_core::HRESULT(0xC0041803_u32 as _);
pub const CI_NO_CATALOG: windows_core::HRESULT = windows_core::HRESULT(0xC0041806_u32 as _);
pub const CI_NO_STARTING_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC0041804_u32 as _);
pub const CI_OUT_OF_INDEX_IDS: windows_core::HRESULT = windows_core::HRESULT(0xC0041805_u32 as _);
pub const CI_PROPSTORE_INCONSISTENCY: windows_core::HRESULT = windows_core::HRESULT(0xC0041809_u32 as _);
pub const CI_S_CAT_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0x41826_u32 as _);
pub const CI_S_END_OF_ENUMERATION: windows_core::HRESULT = windows_core::HRESULT(0x41814_u32 as _);
pub const CI_S_NO_DOCSTORE: windows_core::HRESULT = windows_core::HRESULT(0x41825_u32 as _);
pub const CI_S_WORKID_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x4180E_u32 as _);
pub const CLASSFACTORY_E_FIRST: i32 = -2147221232i32;
pub const CLASSFACTORY_E_LAST: i32 = -2147221217i32;
pub const CLASSFACTORY_S_FIRST: i32 = 262416i32;
pub const CLASSFACTORY_S_LAST: i32 = 262431i32;
pub const CLASS_E_CLASSNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80040111_u32 as _);
pub const CLASS_E_NOAGGREGATION: windows_core::HRESULT = windows_core::HRESULT(0x80040110_u32 as _);
pub const CLASS_E_NOTLICENSED: windows_core::HRESULT = windows_core::HRESULT(0x80040112_u32 as _);
pub const CLIENTSITE_E_FIRST: i32 = -2147221104i32;
pub const CLIENTSITE_E_LAST: i32 = -2147221089i32;
pub const CLIENTSITE_S_FIRST: i32 = 262544i32;
pub const CLIENTSITE_S_LAST: i32 = 262559i32;
pub const CLIPBRD_E_BAD_DATA: windows_core::HRESULT = windows_core::HRESULT(0x800401D3_u32 as _);
pub const CLIPBRD_E_CANT_CLOSE: windows_core::HRESULT = windows_core::HRESULT(0x800401D4_u32 as _);
pub const CLIPBRD_E_CANT_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x800401D1_u32 as _);
pub const CLIPBRD_E_CANT_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x800401D0_u32 as _);
pub const CLIPBRD_E_CANT_SET: windows_core::HRESULT = windows_core::HRESULT(0x800401D2_u32 as _);
pub const CLIPBRD_E_FIRST: i32 = -2147221040i32;
pub const CLIPBRD_E_LAST: i32 = -2147221025i32;
pub const CLIPBRD_S_FIRST: i32 = 262608i32;
pub const CLIPBRD_S_LAST: i32 = 262623i32;
pub const COMADMIN_E_ALREADYINSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80110404_u32 as _);
pub const COMADMIN_E_AMBIGUOUS_APPLICATION_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8011045C_u32 as _);
pub const COMADMIN_E_AMBIGUOUS_PARTITION_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8011045D_u32 as _);
pub const COMADMIN_E_APPDIRNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8011041F_u32 as _);
pub const COMADMIN_E_APPLICATIONEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8011040B_u32 as _);
pub const COMADMIN_E_APPLID_MATCHES_CLSID: windows_core::HRESULT = windows_core::HRESULT(0x80110446_u32 as _);
pub const COMADMIN_E_APP_FILE_READFAIL: windows_core::HRESULT = windows_core::HRESULT(0x80110408_u32 as _);
pub const COMADMIN_E_APP_FILE_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80110409_u32 as _);
pub const COMADMIN_E_APP_FILE_WRITEFAIL: windows_core::HRESULT = windows_core::HRESULT(0x80110407_u32 as _);
pub const COMADMIN_E_APP_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8011080A_u32 as _);
pub const COMADMIN_E_AUTHENTICATIONLEVEL: windows_core::HRESULT = windows_core::HRESULT(0x80110413_u32 as _);
pub const COMADMIN_E_BADPATH: windows_core::HRESULT = windows_core::HRESULT(0x8011040A_u32 as _);
pub const COMADMIN_E_BADREGISTRYLIBID: windows_core::HRESULT = windows_core::HRESULT(0x8011041E_u32 as _);
pub const COMADMIN_E_BADREGISTRYPROGID: windows_core::HRESULT = windows_core::HRESULT(0x80110412_u32 as _);
pub const COMADMIN_E_BASEPARTITION_REQUIRED_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0x8011081F_u32 as _);
pub const COMADMIN_E_BASE_PARTITION_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80110450_u32 as _);
pub const COMADMIN_E_CANNOT_ALIAS_EVENTCLASS: windows_core::HRESULT = windows_core::HRESULT(0x80110820_u32 as _);
pub const COMADMIN_E_CANTCOPYFILE: windows_core::HRESULT = windows_core::HRESULT(0x8011040D_u32 as _);
pub const COMADMIN_E_CANTMAKEINPROCSERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80110814_u32 as _);
pub const COMADMIN_E_CANTRECYCLELIBRARYAPPS: windows_core::HRESULT = windows_core::HRESULT(0x8011080F_u32 as _);
pub const COMADMIN_E_CANTRECYCLESERVICEAPPS: windows_core::HRESULT = windows_core::HRESULT(0x80110811_u32 as _);
pub const COMADMIN_E_CANT_SUBSCRIBE_TO_COMPONENT: windows_core::HRESULT = windows_core::HRESULT(0x8011044D_u32 as _);
pub const COMADMIN_E_CAN_NOT_EXPORT_APP_PROXY: windows_core::HRESULT = windows_core::HRESULT(0x8011044A_u32 as _);
pub const COMADMIN_E_CAN_NOT_EXPORT_SYS_APP: windows_core::HRESULT = windows_core::HRESULT(0x8011044C_u32 as _);
pub const COMADMIN_E_CAN_NOT_START_APP: windows_core::HRESULT = windows_core::HRESULT(0x8011044B_u32 as _);
pub const COMADMIN_E_CAT_BITNESSMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80110482_u32 as _);
pub const COMADMIN_E_CAT_DUPLICATE_PARTITION_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80110457_u32 as _);
pub const COMADMIN_E_CAT_IMPORTED_COMPONENTS_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8011045B_u32 as _);
pub const COMADMIN_E_CAT_INVALID_PARTITION_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80110458_u32 as _);
pub const COMADMIN_E_CAT_PARTITION_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80110459_u32 as _);
pub const COMADMIN_E_CAT_PAUSE_RESUME_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80110485_u32 as _);
pub const COMADMIN_E_CAT_SERVERFAULT: windows_core::HRESULT = windows_core::HRESULT(0x80110486_u32 as _);
pub const COMADMIN_E_CAT_UNACCEPTABLEBITNESS: windows_core::HRESULT = windows_core::HRESULT(0x80110483_u32 as _);
pub const COMADMIN_E_CAT_WRONGAPPBITNESS: windows_core::HRESULT = windows_core::HRESULT(0x80110484_u32 as _);
pub const COMADMIN_E_CLSIDORIIDMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80110418_u32 as _);
pub const COMADMIN_E_COMPFILE_BADTLB: windows_core::HRESULT = windows_core::HRESULT(0x80110428_u32 as _);
pub const COMADMIN_E_COMPFILE_CLASSNOTAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x80110427_u32 as _);
pub const COMADMIN_E_COMPFILE_DOESNOTEXIST: windows_core::HRESULT = windows_core::HRESULT(0x80110424_u32 as _);
pub const COMADMIN_E_COMPFILE_GETCLASSOBJ: windows_core::HRESULT = windows_core::HRESULT(0x80110426_u32 as _);
pub const COMADMIN_E_COMPFILE_LOADDLLFAIL: windows_core::HRESULT = windows_core::HRESULT(0x80110425_u32 as _);
pub const COMADMIN_E_COMPFILE_NOREGISTRAR: windows_core::HRESULT = windows_core::HRESULT(0x80110434_u32 as _);
pub const COMADMIN_E_COMPFILE_NOTINSTALLABLE: windows_core::HRESULT = windows_core::HRESULT(0x80110429_u32 as _);
pub const COMADMIN_E_COMPONENTEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80110439_u32 as _);
pub const COMADMIN_E_COMP_MOVE_BAD_DEST: windows_core::HRESULT = windows_core::HRESULT(0x8011042E_u32 as _);
pub const COMADMIN_E_COMP_MOVE_DEST: windows_core::HRESULT = windows_core::HRESULT(0x8011081D_u32 as _);
pub const COMADMIN_E_COMP_MOVE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8011042D_u32 as _);
pub const COMADMIN_E_COMP_MOVE_PRIVATE: windows_core::HRESULT = windows_core::HRESULT(0x8011081E_u32 as _);
pub const COMADMIN_E_COMP_MOVE_SOURCE: windows_core::HRESULT = windows_core::HRESULT(0x8011081C_u32 as _);
pub const COMADMIN_E_COREQCOMPINSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80110435_u32 as _);
pub const COMADMIN_E_DEFAULT_PARTITION_NOT_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0x80110816_u32 as _);
pub const COMADMIN_E_DLLLOADFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8011041D_u32 as _);
pub const COMADMIN_E_DLLREGISTERSERVER: windows_core::HRESULT = windows_core::HRESULT(0x8011041A_u32 as _);
pub const COMADMIN_E_EVENTCLASS_CANT_BE_SUBSCRIBER: windows_core::HRESULT = windows_core::HRESULT(0x8011044E_u32 as _);
pub const COMADMIN_E_FILE_PARTITION_DUPLICATE_FILES: windows_core::HRESULT = windows_core::HRESULT(0x8011045A_u32 as _);
pub const COMADMIN_E_INVALIDUSERIDS: windows_core::HRESULT = windows_core::HRESULT(0x80110410_u32 as _);
pub const COMADMIN_E_INVALID_PARTITION: windows_core::HRESULT = windows_core::HRESULT(0x8011080B_u32 as _);
pub const COMADMIN_E_KEYMISSING: windows_core::HRESULT = windows_core::HRESULT(0x80110403_u32 as _);
pub const COMADMIN_E_LEGACYCOMPS_NOT_ALLOWED_IN_1_0_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8011081A_u32 as _);
pub const COMADMIN_E_LEGACYCOMPS_NOT_ALLOWED_IN_NONBASE_PARTITIONS: windows_core::HRESULT = windows_core::HRESULT(0x8011081B_u32 as _);
pub const COMADMIN_E_LIB_APP_PROXY_INCOMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0x8011044F_u32 as _);
pub const COMADMIN_E_MIG_SCHEMANOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80110481_u32 as _);
pub const COMADMIN_E_MIG_VERSIONNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80110480_u32 as _);
pub const COMADMIN_E_NOREGISTRYCLSID: windows_core::HRESULT = windows_core::HRESULT(0x80110411_u32 as _);
pub const COMADMIN_E_NOSERVERSHARE: windows_core::HRESULT = windows_core::HRESULT(0x8011041B_u32 as _);
pub const COMADMIN_E_NOTCHANGEABLE: windows_core::HRESULT = windows_core::HRESULT(0x8011042A_u32 as _);
pub const COMADMIN_E_NOTDELETEABLE: windows_core::HRESULT = windows_core::HRESULT(0x8011042B_u32 as _);
pub const COMADMIN_E_NOTINREGISTRY: windows_core::HRESULT = windows_core::HRESULT(0x8011043E_u32 as _);
pub const COMADMIN_E_NOUSER: windows_core::HRESULT = windows_core::HRESULT(0x8011040F_u32 as _);
pub const COMADMIN_E_OBJECTERRORS: windows_core::HRESULT = windows_core::HRESULT(0x80110401_u32 as _);
pub const COMADMIN_E_OBJECTEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80110438_u32 as _);
pub const COMADMIN_E_OBJECTINVALID: windows_core::HRESULT = windows_core::HRESULT(0x80110402_u32 as _);
pub const COMADMIN_E_OBJECTNOTPOOLABLE: windows_core::HRESULT = windows_core::HRESULT(0x8011043F_u32 as _);
pub const COMADMIN_E_OBJECT_DOES_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80110809_u32 as _);
pub const COMADMIN_E_OBJECT_PARENT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80110808_u32 as _);
pub const COMADMIN_E_PARTITIONS_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80110824_u32 as _);
pub const COMADMIN_E_PARTITION_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80110818_u32 as _);
pub const COMADMIN_E_PARTITION_MSI_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80110819_u32 as _);
pub const COMADMIN_E_PAUSEDPROCESSMAYNOTBERECYCLED: windows_core::HRESULT = windows_core::HRESULT(0x80110813_u32 as _);
pub const COMADMIN_E_PRIVATE_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80110821_u32 as _);
pub const COMADMIN_E_PROCESSALREADYRECYCLED: windows_core::HRESULT = windows_core::HRESULT(0x80110812_u32 as _);
pub const COMADMIN_E_PROGIDINUSEBYCLSID: windows_core::HRESULT = windows_core::HRESULT(0x80110815_u32 as _);
pub const COMADMIN_E_PROPERTYSAVEFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80110437_u32 as _);
pub const COMADMIN_E_PROPERTY_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x8011043C_u32 as _);
pub const COMADMIN_E_RECYCLEDPROCESSMAYNOTBEPAUSED: windows_core::HRESULT = windows_core::HRESULT(0x80110817_u32 as _);
pub const COMADMIN_E_REGDB_ALREADYRUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80110475_u32 as _);
pub const COMADMIN_E_REGDB_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80110472_u32 as _);
pub const COMADMIN_E_REGDB_NOTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80110473_u32 as _);
pub const COMADMIN_E_REGDB_SYSTEMERR: windows_core::HRESULT = windows_core::HRESULT(0x80110474_u32 as _);
pub const COMADMIN_E_REGFILE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x8011043B_u32 as _);
pub const COMADMIN_E_REGISTERTLB: windows_core::HRESULT = windows_core::HRESULT(0x80110430_u32 as _);
pub const COMADMIN_E_REGISTRARFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80110423_u32 as _);
pub const COMADMIN_E_REGISTRY_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80110823_u32 as _);
pub const COMADMIN_E_REMOTEINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80110419_u32 as _);
pub const COMADMIN_E_REQUIRES_DIFFERENT_PLATFORM: windows_core::HRESULT = windows_core::HRESULT(0x80110449_u32 as _);
pub const COMADMIN_E_ROLEEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8011040C_u32 as _);
pub const COMADMIN_E_ROLE_DOES_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80110447_u32 as _);
pub const COMADMIN_E_SAFERINVALID: windows_core::HRESULT = windows_core::HRESULT(0x80110822_u32 as _);
pub const COMADMIN_E_SERVICENOTINSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80110436_u32 as _);
pub const COMADMIN_E_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x8011042C_u32 as _);
pub const COMADMIN_E_START_APP_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80110451_u32 as _);
pub const COMADMIN_E_START_APP_NEEDS_COMPONENTS: windows_core::HRESULT = windows_core::HRESULT(0x80110448_u32 as _);
pub const COMADMIN_E_SVCAPP_NOT_POOLABLE_OR_RECYCLABLE: windows_core::HRESULT = windows_core::HRESULT(0x8011080D_u32 as _);
pub const COMADMIN_E_SYSTEMAPP: windows_core::HRESULT = windows_core::HRESULT(0x80110433_u32 as _);
pub const COMADMIN_E_USERPASSWDNOTVALID: windows_core::HRESULT = windows_core::HRESULT(0x80110414_u32 as _);
pub const COMADMIN_E_USER_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0x8011080E_u32 as _);
pub const COMQC_E_APPLICATION_NOT_QUEUED: windows_core::HRESULT = windows_core::HRESULT(0x80110600_u32 as _);
pub const COMQC_E_BAD_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x80110604_u32 as _);
pub const COMQC_E_NO_IPERSISTSTREAM: windows_core::HRESULT = windows_core::HRESULT(0x80110603_u32 as _);
pub const COMQC_E_NO_QUEUEABLE_INTERFACES: windows_core::HRESULT = windows_core::HRESULT(0x80110601_u32 as _);
pub const COMQC_E_QUEUING_SERVICE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80110602_u32 as _);
pub const COMQC_E_UNAUTHENTICATED: windows_core::HRESULT = windows_core::HRESULT(0x80110605_u32 as _);
pub const COMQC_E_UNTRUSTED_ENQUEUER: windows_core::HRESULT = windows_core::HRESULT(0x80110606_u32 as _);
pub const CONTEXT_E_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004E002_u32 as _);
pub const CONTEXT_E_ABORTING: windows_core::HRESULT = windows_core::HRESULT(0x8004E003_u32 as _);
pub const CONTEXT_E_FIRST: i32 = -2147164160i32;
pub const CONTEXT_E_LAST: i32 = -2147164113i32;
pub const CONTEXT_E_NOCONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x8004E004_u32 as _);
pub const CONTEXT_E_NOJIT: windows_core::HRESULT = windows_core::HRESULT(0x8004E026_u32 as _);
pub const CONTEXT_E_NOTRANSACTION: windows_core::HRESULT = windows_core::HRESULT(0x8004E027_u32 as _);
pub const CONTEXT_E_OLDREF: windows_core::HRESULT = windows_core::HRESULT(0x8004E007_u32 as _);
pub const CONTEXT_E_ROLENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8004E00C_u32 as _);
pub const CONTEXT_E_SYNCH_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004E006_u32 as _);
pub const CONTEXT_E_TMNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8004E00F_u32 as _);
pub const CONTEXT_E_WOULD_DEADLOCK: windows_core::HRESULT = windows_core::HRESULT(0x8004E005_u32 as _);
pub const CONTEXT_S_FIRST: i32 = 319488i32;
pub const CONTEXT_S_LAST: i32 = 319535i32;
pub const CONTROL_C_EXIT: NTSTATUS = NTSTATUS(0xC000013A_u32 as _);
pub const CONVERT10_E_FIRST: i32 = -2147221056i32;
pub const CONVERT10_E_LAST: i32 = -2147221041i32;
pub const CONVERT10_E_OLELINK_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x800401C7_u32 as _);
pub const CONVERT10_E_OLESTREAM_BITMAP_TO_DIB: windows_core::HRESULT = windows_core::HRESULT(0x800401C3_u32 as _);
pub const CONVERT10_E_OLESTREAM_FMT: windows_core::HRESULT = windows_core::HRESULT(0x800401C2_u32 as _);
pub const CONVERT10_E_OLESTREAM_GET: windows_core::HRESULT = windows_core::HRESULT(0x800401C0_u32 as _);
pub const CONVERT10_E_OLESTREAM_PUT: windows_core::HRESULT = windows_core::HRESULT(0x800401C1_u32 as _);
pub const CONVERT10_E_STG_DIB_TO_BITMAP: windows_core::HRESULT = windows_core::HRESULT(0x800401C6_u32 as _);
pub const CONVERT10_E_STG_FMT: windows_core::HRESULT = windows_core::HRESULT(0x800401C4_u32 as _);
pub const CONVERT10_E_STG_NO_STD_STREAM: windows_core::HRESULT = windows_core::HRESULT(0x800401C5_u32 as _);
pub const CONVERT10_S_FIRST: i32 = 262592i32;
pub const CONVERT10_S_LAST: i32 = 262607i32;
pub const CONVERT10_S_NO_PRESENTATION: windows_core::HRESULT = windows_core::HRESULT(0x401C0_u32 as _);
pub const CO_E_ACCESSCHECKFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8001012A_u32 as _);
pub const CO_E_ACESINWRONGORDER: windows_core::HRESULT = windows_core::HRESULT(0x8001013A_u32 as _);
pub const CO_E_ACNOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8001013F_u32 as _);
pub const CO_E_ACTIVATIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004E021_u32 as _);
pub const CO_E_ACTIVATIONFAILED_CATALOGERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004E023_u32 as _);
pub const CO_E_ACTIVATIONFAILED_EVENTLOGGED: windows_core::HRESULT = windows_core::HRESULT(0x8004E022_u32 as _);
pub const CO_E_ACTIVATIONFAILED_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004E024_u32 as _);
pub const CO_E_ALREADYINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x800401F1_u32 as _);
pub const CO_E_APPDIDNTREG: windows_core::HRESULT = windows_core::HRESULT(0x800401FE_u32 as _);
pub const CO_E_APPNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x800401F5_u32 as _);
pub const CO_E_APPSINGLEUSE: windows_core::HRESULT = windows_core::HRESULT(0x800401F6_u32 as _);
pub const CO_E_ASYNC_WORK_REJECTED: windows_core::HRESULT = windows_core::HRESULT(0x80004029_u32 as _);
pub const CO_E_ATTEMPT_TO_CREATE_OUTSIDE_CLIENT_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80004024_u32 as _);
pub const CO_E_BAD_PATH: windows_core::HRESULT = windows_core::HRESULT(0x80080004_u32 as _);
pub const CO_E_BAD_SERVER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80004014_u32 as _);
pub const CO_E_CALL_OUT_OF_TX_SCOPE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8004E030_u32 as _);
pub const CO_E_CANCEL_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80010140_u32 as _);
pub const CO_E_CANTDETERMINECLASS: windows_core::HRESULT = windows_core::HRESULT(0x800401F2_u32 as _);
pub const CO_E_CANT_REMOTE: windows_core::HRESULT = windows_core::HRESULT(0x80004013_u32 as _);
pub const CO_E_CLASSSTRING: windows_core::HRESULT = windows_core::HRESULT(0x800401F3_u32 as _);
pub const CO_E_CLASS_CREATE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80080001_u32 as _);
pub const CO_E_CLASS_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80004027_u32 as _);
pub const CO_E_CLRNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80004028_u32 as _);
pub const CO_E_CLSREG_INCONSISTENT: windows_core::HRESULT = windows_core::HRESULT(0x8000401F_u32 as _);
pub const CO_E_CONVERSIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8001012E_u32 as _);
pub const CO_E_CREATEPROCESS_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80004018_u32 as _);
pub const CO_E_DBERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004E02B_u32 as _);
pub const CO_E_DECODEFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8001013D_u32 as _);
pub const CO_E_DLLNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x800401F8_u32 as _);
pub const CO_E_ELEVATION_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80080017_u32 as _);
pub const CO_E_ERRORINAPP: windows_core::HRESULT = windows_core::HRESULT(0x800401F7_u32 as _);
pub const CO_E_ERRORINDLL: windows_core::HRESULT = windows_core::HRESULT(0x800401F9_u32 as _);
pub const CO_E_EXCEEDSYSACLLIMIT: windows_core::HRESULT = windows_core::HRESULT(0x80010139_u32 as _);
pub const CO_E_EXIT_TRANSACTION_SCOPE_NOT_CALLED: windows_core::HRESULT = windows_core::HRESULT(0x8004E031_u32 as _);
pub const CO_E_FAILEDTOCLOSEHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80010138_u32 as _);
pub const CO_E_FAILEDTOCREATEFILE: windows_core::HRESULT = windows_core::HRESULT(0x80010137_u32 as _);
pub const CO_E_FAILEDTOGENUUID: windows_core::HRESULT = windows_core::HRESULT(0x80010136_u32 as _);
pub const CO_E_FAILEDTOGETSECCTX: windows_core::HRESULT = windows_core::HRESULT(0x80010124_u32 as _);
pub const CO_E_FAILEDTOGETTOKENINFO: windows_core::HRESULT = windows_core::HRESULT(0x80010126_u32 as _);
pub const CO_E_FAILEDTOGETWINDIR: windows_core::HRESULT = windows_core::HRESULT(0x80010134_u32 as _);
pub const CO_E_FAILEDTOIMPERSONATE: windows_core::HRESULT = windows_core::HRESULT(0x80010123_u32 as _);
pub const CO_E_FAILEDTOOPENPROCESSTOKEN: windows_core::HRESULT = windows_core::HRESULT(0x8001013C_u32 as _);
pub const CO_E_FAILEDTOOPENTHREADTOKEN: windows_core::HRESULT = windows_core::HRESULT(0x80010125_u32 as _);
pub const CO_E_FAILEDTOQUERYCLIENTBLANKET: windows_core::HRESULT = windows_core::HRESULT(0x80010128_u32 as _);
pub const CO_E_FAILEDTOSETDACL: windows_core::HRESULT = windows_core::HRESULT(0x80010129_u32 as _);
pub const CO_E_FIRST: i32 = -2147221008i32;
pub const CO_E_IIDREG_INCONSISTENT: windows_core::HRESULT = windows_core::HRESULT(0x80004020_u32 as _);
pub const CO_E_IIDSTRING: windows_core::HRESULT = windows_core::HRESULT(0x800401F4_u32 as _);
pub const CO_E_INCOMPATIBLESTREAMVERSION: windows_core::HRESULT = windows_core::HRESULT(0x8001013B_u32 as _);
pub const CO_E_INITIALIZATIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004E025_u32 as _);
pub const CO_E_INIT_CLASS_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x80004009_u32 as _);
pub const CO_E_INIT_MEMORY_ALLOCATOR: windows_core::HRESULT = windows_core::HRESULT(0x80004008_u32 as _);
pub const CO_E_INIT_ONLY_SINGLE_THREADED: windows_core::HRESULT = windows_core::HRESULT(0x80004012_u32 as _);
pub const CO_E_INIT_RPC_CHANNEL: windows_core::HRESULT = windows_core::HRESULT(0x8000400A_u32 as _);
pub const CO_E_INIT_SCM_EXEC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80004011_u32 as _);
pub const CO_E_INIT_SCM_FILE_MAPPING_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8000400F_u32 as _);
pub const CO_E_INIT_SCM_MAP_VIEW_OF_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80004010_u32 as _);
pub const CO_E_INIT_SCM_MUTEX_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8000400E_u32 as _);
pub const CO_E_INIT_SHARED_ALLOCATOR: windows_core::HRESULT = windows_core::HRESULT(0x80004007_u32 as _);
pub const CO_E_INIT_TLS: windows_core::HRESULT = windows_core::HRESULT(0x80004006_u32 as _);
pub const CO_E_INIT_TLS_CHANNEL_CONTROL: windows_core::HRESULT = windows_core::HRESULT(0x8000400C_u32 as _);
pub const CO_E_INIT_TLS_SET_CHANNEL_CONTROL: windows_core::HRESULT = windows_core::HRESULT(0x8000400B_u32 as _);
pub const CO_E_INIT_UNACCEPTED_USER_ALLOCATOR: windows_core::HRESULT = windows_core::HRESULT(0x8000400D_u32 as _);
pub const CO_E_INVALIDSID: windows_core::HRESULT = windows_core::HRESULT(0x8001012D_u32 as _);
pub const CO_E_ISOLEVELMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8004E02F_u32 as _);
pub const CO_E_LAST: i32 = -2147220993i32;
pub const CO_E_LAUNCH_PERMSSION_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x8000401B_u32 as _);
pub const CO_E_LOOKUPACCNAMEFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80010132_u32 as _);
pub const CO_E_LOOKUPACCSIDFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80010130_u32 as _);
pub const CO_E_MALFORMED_SPN: windows_core::HRESULT = windows_core::HRESULT(0x80004033_u32 as _);
pub const CO_E_MISSING_DISPLAYNAME: windows_core::HRESULT = windows_core::HRESULT(0x80080015_u32 as _);
pub const CO_E_MSI_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80004023_u32 as _);
pub const CO_E_NETACCESSAPIFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8001012B_u32 as _);
pub const CO_E_NOCOOKIES: windows_core::HRESULT = windows_core::HRESULT(0x8004E02A_u32 as _);
pub const CO_E_NOIISINTRINSICS: windows_core::HRESULT = windows_core::HRESULT(0x8004E029_u32 as _);
pub const CO_E_NOMATCHINGNAMEFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80010131_u32 as _);
pub const CO_E_NOMATCHINGSIDFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8001012F_u32 as _);
pub const CO_E_NOSYNCHRONIZATION: windows_core::HRESULT = windows_core::HRESULT(0x8004E02E_u32 as _);
pub const CO_E_NOTCONSTRUCTED: windows_core::HRESULT = windows_core::HRESULT(0x8004E02D_u32 as _);
pub const CO_E_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x800401F0_u32 as _);
pub const CO_E_NOTPOOLED: windows_core::HRESULT = windows_core::HRESULT(0x8004E02C_u32 as _);
pub const CO_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80004021_u32 as _);
pub const CO_E_NO_SECCTX_IN_ACTIVATE: windows_core::HRESULT = windows_core::HRESULT(0x8000402B_u32 as _);
pub const CO_E_OBJISREG: windows_core::HRESULT = windows_core::HRESULT(0x800401FC_u32 as _);
pub const CO_E_OBJNOTCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x800401FD_u32 as _);
pub const CO_E_OBJNOTREG: windows_core::HRESULT = windows_core::HRESULT(0x800401FB_u32 as _);
pub const CO_E_OBJSRV_RPC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80080006_u32 as _);
pub const CO_E_OLE1DDE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80004016_u32 as _);
pub const CO_E_PATHTOOLONG: windows_core::HRESULT = windows_core::HRESULT(0x80010135_u32 as _);
pub const CO_E_PREMATURE_STUB_RUNDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80004035_u32 as _);
pub const CO_E_RELEASED: windows_core::HRESULT = windows_core::HRESULT(0x800401FF_u32 as _);
pub const CO_E_RELOAD_DLL: windows_core::HRESULT = windows_core::HRESULT(0x80004022_u32 as _);
pub const CO_E_REMOTE_COMMUNICATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8000401D_u32 as _);
pub const CO_E_RUNAS_CREATEPROCESS_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80004019_u32 as _);
pub const CO_E_RUNAS_LOGON_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8000401A_u32 as _);
pub const CO_E_RUNAS_SYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x80004017_u32 as _);
pub const CO_E_RUNAS_VALUE_MUST_BE_AAA: windows_core::HRESULT = windows_core::HRESULT(0x80080016_u32 as _);
pub const CO_E_SCM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80080002_u32 as _);
pub const CO_E_SCM_RPC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80080003_u32 as _);
pub const CO_E_SERVER_EXEC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80080005_u32 as _);
pub const CO_E_SERVER_INIT_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8000402A_u32 as _);
pub const CO_E_SERVER_NOT_PAUSED: windows_core::HRESULT = windows_core::HRESULT(0x80004026_u32 as _);
pub const CO_E_SERVER_PAUSED: windows_core::HRESULT = windows_core::HRESULT(0x80004025_u32 as _);
pub const CO_E_SERVER_START_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8000401E_u32 as _);
pub const CO_E_SERVER_STOPPING: windows_core::HRESULT = windows_core::HRESULT(0x80080008_u32 as _);
pub const CO_E_SETSERLHNDLFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80010133_u32 as _);
pub const CO_E_START_SERVICE_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8000401C_u32 as _);
pub const CO_E_SXS_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80004032_u32 as _);
pub const CO_E_THREADINGMODEL_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x8004E028_u32 as _);
pub const CO_E_THREADPOOL_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80004031_u32 as _);
pub const CO_E_TRACKER_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80004030_u32 as _);
pub const CO_E_TRUSTEEDOESNTMATCHCLIENT: windows_core::HRESULT = windows_core::HRESULT(0x80010127_u32 as _);
pub const CO_E_UNREVOKED_REGISTRATION_ON_APARTMENT_SHUTDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80004034_u32 as _);
pub const CO_E_WRONGOSFORAPP: windows_core::HRESULT = windows_core::HRESULT(0x800401FA_u32 as _);
pub const CO_E_WRONGTRUSTEENAMESYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x8001012C_u32 as _);
pub const CO_E_WRONG_SERVER_IDENTITY: windows_core::HRESULT = windows_core::HRESULT(0x80004015_u32 as _);
pub const CO_S_FIRST: i32 = 262640i32;
pub const CO_S_LAST: i32 = 262655i32;
pub const CO_S_MACHINENAMENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80013_u32 as _);
pub const CO_S_NOTALLINTERFACES: windows_core::HRESULT = windows_core::HRESULT(0x80012_u32 as _);
pub const CRYPT_E_ALREADY_DECRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x80091009_u32 as _);
pub const CRYPT_E_ASN1_BADARGS: windows_core::HRESULT = windows_core::HRESULT(0x80093109_u32 as _);
pub const CRYPT_E_ASN1_BADPDU: windows_core::HRESULT = windows_core::HRESULT(0x80093108_u32 as _);
pub const CRYPT_E_ASN1_BADREAL: windows_core::HRESULT = windows_core::HRESULT(0x8009310A_u32 as _);
pub const CRYPT_E_ASN1_BADTAG: windows_core::HRESULT = windows_core::HRESULT(0x8009310B_u32 as _);
pub const CRYPT_E_ASN1_CHOICE: windows_core::HRESULT = windows_core::HRESULT(0x8009310C_u32 as _);
pub const CRYPT_E_ASN1_CONSTRAINT: windows_core::HRESULT = windows_core::HRESULT(0x80093105_u32 as _);
pub const CRYPT_E_ASN1_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80093103_u32 as _);
pub const CRYPT_E_ASN1_EOD: windows_core::HRESULT = windows_core::HRESULT(0x80093102_u32 as _);
pub const CRYPT_E_ASN1_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80093100_u32 as _);
pub const CRYPT_E_ASN1_EXTENDED: windows_core::HRESULT = windows_core::HRESULT(0x80093201_u32 as _);
pub const CRYPT_E_ASN1_INTERNAL: windows_core::HRESULT = windows_core::HRESULT(0x80093101_u32 as _);
pub const CRYPT_E_ASN1_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80093104_u32 as _);
pub const CRYPT_E_ASN1_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80093106_u32 as _);
pub const CRYPT_E_ASN1_NOEOD: windows_core::HRESULT = windows_core::HRESULT(0x80093202_u32 as _);
pub const CRYPT_E_ASN1_NYI: windows_core::HRESULT = windows_core::HRESULT(0x80093134_u32 as _);
pub const CRYPT_E_ASN1_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x80093107_u32 as _);
pub const CRYPT_E_ASN1_PDU_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80093133_u32 as _);
pub const CRYPT_E_ASN1_RULE: windows_core::HRESULT = windows_core::HRESULT(0x8009310D_u32 as _);
pub const CRYPT_E_ASN1_UTF8: windows_core::HRESULT = windows_core::HRESULT(0x8009310E_u32 as _);
pub const CRYPT_E_ATTRIBUTES_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x8009100F_u32 as _);
pub const CRYPT_E_AUTH_ATTR_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80091006_u32 as _);
pub const CRYPT_E_BAD_ENCODE: windows_core::HRESULT = windows_core::HRESULT(0x80092002_u32 as _);
pub const CRYPT_E_BAD_LEN: windows_core::HRESULT = windows_core::HRESULT(0x80092001_u32 as _);
pub const CRYPT_E_BAD_MSG: windows_core::HRESULT = windows_core::HRESULT(0x8009200D_u32 as _);
pub const CRYPT_E_CONTROL_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8009100C_u32 as _);
pub const CRYPT_E_DELETED_PREV: windows_core::HRESULT = windows_core::HRESULT(0x80092008_u32 as _);
pub const CRYPT_E_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80092005_u32 as _);
pub const CRYPT_E_FILERESIZED: windows_core::HRESULT = windows_core::HRESULT(0x80092025_u32 as _);
pub const CRYPT_E_FILE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80092003_u32 as _);
pub const CRYPT_E_HASH_VALUE: windows_core::HRESULT = windows_core::HRESULT(0x80091007_u32 as _);
pub const CRYPT_E_INVALID_IA5_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80092022_u32 as _);
pub const CRYPT_E_INVALID_INDEX: windows_core::HRESULT = windows_core::HRESULT(0x80091008_u32 as _);
pub const CRYPT_E_INVALID_MSG_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80091004_u32 as _);
pub const CRYPT_E_INVALID_NUMERIC_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80092020_u32 as _);
pub const CRYPT_E_INVALID_PRINTABLE_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80092021_u32 as _);
pub const CRYPT_E_INVALID_X500_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80092023_u32 as _);
pub const CRYPT_E_ISSUER_SERIALNUMBER: windows_core::HRESULT = windows_core::HRESULT(0x8009100D_u32 as _);
pub const CRYPT_E_MISSING_PUBKEY_PARA: windows_core::HRESULT = windows_core::HRESULT(0x8009202C_u32 as _);
pub const CRYPT_E_MSG_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80091001_u32 as _);
pub const CRYPT_E_NOT_CHAR_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80092024_u32 as _);
pub const CRYPT_E_NOT_DECRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x8009100A_u32 as _);
pub const CRYPT_E_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80092004_u32 as _);
pub const CRYPT_E_NOT_IN_CTL: windows_core::HRESULT = windows_core::HRESULT(0x8009202A_u32 as _);
pub const CRYPT_E_NOT_IN_REVOCATION_DATABASE: windows_core::HRESULT = windows_core::HRESULT(0x80092014_u32 as _);
pub const CRYPT_E_NO_DECRYPT_CERT: windows_core::HRESULT = windows_core::HRESULT(0x8009200C_u32 as _);
pub const CRYPT_E_NO_KEY_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8009200B_u32 as _);
pub const CRYPT_E_NO_MATCH: windows_core::HRESULT = windows_core::HRESULT(0x80092009_u32 as _);
pub const CRYPT_E_NO_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80092006_u32 as _);
pub const CRYPT_E_NO_REVOCATION_CHECK: windows_core::HRESULT = windows_core::HRESULT(0x80092012_u32 as _);
pub const CRYPT_E_NO_REVOCATION_DLL: windows_core::HRESULT = windows_core::HRESULT(0x80092011_u32 as _);
pub const CRYPT_E_NO_SIGNER: windows_core::HRESULT = windows_core::HRESULT(0x8009200E_u32 as _);
pub const CRYPT_E_NO_TRUSTED_SIGNER: windows_core::HRESULT = windows_core::HRESULT(0x8009202B_u32 as _);
pub const CRYPT_E_NO_VERIFY_USAGE_CHECK: windows_core::HRESULT = windows_core::HRESULT(0x80092028_u32 as _);
pub const CRYPT_E_NO_VERIFY_USAGE_DLL: windows_core::HRESULT = windows_core::HRESULT(0x80092027_u32 as _);
pub const CRYPT_E_OBJECT_LOCATOR_OBJECT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8009202D_u32 as _);
pub const CRYPT_E_OID_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80091003_u32 as _);
pub const CRYPT_E_OSS_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80093000_u32 as _);
pub const CRYPT_E_PENDING_CLOSE: windows_core::HRESULT = windows_core::HRESULT(0x8009200F_u32 as _);
pub const CRYPT_E_RECIPIENT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8009100B_u32 as _);
pub const CRYPT_E_REVOCATION_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80092013_u32 as _);
pub const CRYPT_E_REVOKED: windows_core::HRESULT = windows_core::HRESULT(0x80092010_u32 as _);
pub const CRYPT_E_SECURITY_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x80092026_u32 as _);
pub const CRYPT_E_SELF_SIGNED: windows_core::HRESULT = windows_core::HRESULT(0x80092007_u32 as _);
pub const CRYPT_E_SIGNER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8009100E_u32 as _);
pub const CRYPT_E_STREAM_INSUFFICIENT_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80091011_u32 as _);
pub const CRYPT_E_STREAM_MSG_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80091010_u32 as _);
pub const CRYPT_E_UNEXPECTED_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x80091005_u32 as _);
pub const CRYPT_E_UNEXPECTED_MSG_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8009200A_u32 as _);
pub const CRYPT_E_UNKNOWN_ALGO: windows_core::HRESULT = windows_core::HRESULT(0x80091002_u32 as _);
pub const CRYPT_E_VERIFY_USAGE_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80092029_u32 as _);
pub const CRYPT_I_NEW_PROTECTION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x91012_u32 as _);
pub const CS_E_ADMIN_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8004016D_u32 as _);
pub const CS_E_CLASS_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040166_u32 as _);
pub const CS_E_FIRST: i32 = -2147221148i32;
pub const CS_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004016F_u32 as _);
pub const CS_E_INVALID_PATH: windows_core::HRESULT = windows_core::HRESULT(0x8004016B_u32 as _);
pub const CS_E_INVALID_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80040167_u32 as _);
pub const CS_E_LAST: i32 = -2147221137i32;
pub const CS_E_NETWORK_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004016C_u32 as _);
pub const CS_E_NOT_DELETABLE: windows_core::HRESULT = windows_core::HRESULT(0x80040165_u32 as _);
pub const CS_E_NO_CLASSSTORE: windows_core::HRESULT = windows_core::HRESULT(0x80040168_u32 as _);
pub const CS_E_OBJECT_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8004016A_u32 as _);
pub const CS_E_OBJECT_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040169_u32 as _);
pub const CS_E_PACKAGE_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040164_u32 as _);
pub const CS_E_SCHEMA_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8004016E_u32 as _);
pub const D2DERR_BAD_NUMBER: windows_core::HRESULT = windows_core::HRESULT(0x88990011_u32 as _);
pub const D2DERR_BITMAP_BOUND_AS_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x88990025_u32 as _);
pub const D2DERR_BITMAP_CANNOT_DRAW: windows_core::HRESULT = windows_core::HRESULT(0x88990021_u32 as _);
pub const D2DERR_CYCLIC_GRAPH: windows_core::HRESULT = windows_core::HRESULT(0x88990020_u32 as _);
pub const D2DERR_DISPLAY_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88990009_u32 as _);
pub const D2DERR_DISPLAY_STATE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x88990006_u32 as _);
pub const D2DERR_EFFECT_IS_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x88990028_u32 as _);
pub const D2DERR_EXCEEDS_MAX_BITMAP_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x8899001D_u32 as _);
pub const D2DERR_INCOMPATIBLE_BRUSH_TYPES: windows_core::HRESULT = windows_core::HRESULT(0x88990018_u32 as _);
pub const D2DERR_INSUFFICIENT_DEVICE_CAPABILITIES: windows_core::HRESULT = windows_core::HRESULT(0x88990026_u32 as _);
pub const D2DERR_INTERMEDIATE_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x88990027_u32 as _);
pub const D2DERR_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88990008_u32 as _);
pub const D2DERR_INVALID_CALL: windows_core::HRESULT = windows_core::HRESULT(0x8899000A_u32 as _);
pub const D2DERR_INVALID_GLYPH_IMAGE: windows_core::HRESULT = windows_core::HRESULT(0x8899002E_u32 as _);
pub const D2DERR_INVALID_GRAPH_CONFIGURATION: windows_core::HRESULT = windows_core::HRESULT(0x8899001E_u32 as _);
pub const D2DERR_INVALID_INTERNAL_GRAPH_CONFIGURATION: windows_core::HRESULT = windows_core::HRESULT(0x8899001F_u32 as _);
pub const D2DERR_INVALID_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x88990029_u32 as _);
pub const D2DERR_INVALID_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x88990024_u32 as _);
pub const D2DERR_LAYER_ALREADY_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x88990013_u32 as _);
pub const D2DERR_MAX_TEXTURE_SIZE_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8899000F_u32 as _);
pub const D2DERR_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x88990002_u32 as _);
pub const D2DERR_NO_HARDWARE_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x8899000B_u32 as _);
pub const D2DERR_NO_SUBPROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0x8899002A_u32 as _);
pub const D2DERR_ORIGINAL_TARGET_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x88990023_u32 as _);
pub const D2DERR_OUTSTANDING_BITMAP_REFERENCES: windows_core::HRESULT = windows_core::HRESULT(0x88990022_u32 as _);
pub const D2DERR_POP_CALL_DID_NOT_MATCH_PUSH: windows_core::HRESULT = windows_core::HRESULT(0x88990014_u32 as _);
pub const D2DERR_PRINT_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8899002C_u32 as _);
pub const D2DERR_PRINT_JOB_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0x8899002B_u32 as _);
pub const D2DERR_PUSH_POP_UNBALANCED: windows_core::HRESULT = windows_core::HRESULT(0x88990016_u32 as _);
pub const D2DERR_RECREATE_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x8899000C_u32 as _);
pub const D2DERR_RENDER_TARGET_HAS_LAYER_OR_CLIPRECT: windows_core::HRESULT = windows_core::HRESULT(0x88990017_u32 as _);
pub const D2DERR_SCANNER_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88990004_u32 as _);
pub const D2DERR_SCREEN_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x88990005_u32 as _);
pub const D2DERR_SHADER_COMPILE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8899000E_u32 as _);
pub const D2DERR_TARGET_NOT_GDI_COMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0x8899001A_u32 as _);
pub const D2DERR_TEXT_EFFECT_IS_WRONG_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8899001B_u32 as _);
pub const D2DERR_TEXT_RENDERER_NOT_RELEASED: windows_core::HRESULT = windows_core::HRESULT(0x8899001C_u32 as _);
pub const D2DERR_TOO_MANY_SHADER_ELEMENTS: windows_core::HRESULT = windows_core::HRESULT(0x8899000D_u32 as _);
pub const D2DERR_TOO_MANY_TRANSFORM_INPUTS: windows_core::HRESULT = windows_core::HRESULT(0x8899002D_u32 as _);
pub const D2DERR_UNSUPPORTED_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x88990003_u32 as _);
pub const D2DERR_UNSUPPORTED_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x88990010_u32 as _);
pub const D2DERR_WIN32_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88990019_u32 as _);
pub const D2DERR_WRONG_FACTORY: windows_core::HRESULT = windows_core::HRESULT(0x88990012_u32 as _);
pub const D2DERR_WRONG_RESOURCE_DOMAIN: windows_core::HRESULT = windows_core::HRESULT(0x88990015_u32 as _);
pub const D2DERR_WRONG_STATE: windows_core::HRESULT = windows_core::HRESULT(0x88990001_u32 as _);
pub const D2DERR_ZERO_VECTOR: windows_core::HRESULT = windows_core::HRESULT(0x88990007_u32 as _);
pub const D3D10_ERROR_FILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x88790002_u32 as _);
pub const D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0x88790001_u32 as _);
pub const D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD: windows_core::HRESULT = windows_core::HRESULT(0x887C0004_u32 as _);
pub const D3D11_ERROR_FILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x887C0002_u32 as _);
pub const D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0x887C0001_u32 as _);
pub const D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0x887C0003_u32 as _);
pub const D3D12_ERROR_ADAPTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x887E0001_u32 as _);
pub const D3D12_ERROR_DRIVER_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x887E0002_u32 as _);
pub const D3D12_ERROR_INVALID_REDIST: windows_core::HRESULT = windows_core::HRESULT(0x887E0003_u32 as _);
pub const DATA_E_FIRST: i32 = -2147221200i32;
pub const DATA_E_LAST: i32 = -2147221185i32;
pub const DATA_S_FIRST: i32 = 262448i32;
pub const DATA_S_LAST: i32 = 262463i32;
pub const DATA_S_SAMEFORMATETC: windows_core::HRESULT = windows_core::HRESULT(0x40130_u32 as _);
pub const DBG_APP_NOT_IDLE: NTSTATUS = NTSTATUS(0xC0010002_u32 as _);
pub const DBG_COMMAND_EXCEPTION: NTSTATUS = NTSTATUS(0x40010009_u32 as _);
pub const DBG_CONTINUE: NTSTATUS = NTSTATUS(0x10002_u32 as _);
pub const DBG_CONTROL_BREAK: NTSTATUS = NTSTATUS(0x40010008_u32 as _);
pub const DBG_CONTROL_C: NTSTATUS = NTSTATUS(0x40010005_u32 as _);
pub const DBG_EXCEPTION_HANDLED: NTSTATUS = NTSTATUS(0x10001_u32 as _);
pub const DBG_EXCEPTION_NOT_HANDLED: NTSTATUS = NTSTATUS(0x80010001_u32 as _);
pub const DBG_NO_STATE_CHANGE: NTSTATUS = NTSTATUS(0xC0010001_u32 as _);
pub const DBG_PRINTEXCEPTION_C: NTSTATUS = NTSTATUS(0x40010006_u32 as _);
pub const DBG_PRINTEXCEPTION_WIDE_C: NTSTATUS = NTSTATUS(0x4001000A_u32 as _);
pub const DBG_REPLY_LATER: NTSTATUS = NTSTATUS(0x40010001_u32 as _);
pub const DBG_RIPEXCEPTION: NTSTATUS = NTSTATUS(0x40010007_u32 as _);
pub const DBG_TERMINATE_PROCESS: NTSTATUS = NTSTATUS(0x40010004_u32 as _);
pub const DBG_TERMINATE_THREAD: NTSTATUS = NTSTATUS(0x40010003_u32 as _);
pub const DBG_UNABLE_TO_PROVIDE_HANDLE: NTSTATUS = NTSTATUS(0x40010002_u32 as _);
pub const DCOMPOSITION_ERROR_SURFACE_BEING_RENDERED: windows_core::HRESULT = windows_core::HRESULT(0x88980801_u32 as _);
pub const DCOMPOSITION_ERROR_SURFACE_NOT_BEING_RENDERED: windows_core::HRESULT = windows_core::HRESULT(0x88980802_u32 as _);
pub const DCOMPOSITION_ERROR_WINDOW_ALREADY_COMPOSED: windows_core::HRESULT = windows_core::HRESULT(0x88980800_u32 as _);
pub const DIGSIG_E_CRYPTO: windows_core::HRESULT = windows_core::HRESULT(0x800B0008_u32 as _);
pub const DIGSIG_E_DECODE: windows_core::HRESULT = windows_core::HRESULT(0x800B0006_u32 as _);
pub const DIGSIG_E_ENCODE: windows_core::HRESULT = windows_core::HRESULT(0x800B0005_u32 as _);
pub const DIGSIG_E_EXTENSIBILITY: windows_core::HRESULT = windows_core::HRESULT(0x800B0007_u32 as _);
pub const DISP_E_ARRAYISLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8002000D_u32 as _);
pub const DISP_E_BADCALLEE: windows_core::HRESULT = windows_core::HRESULT(0x80020010_u32 as _);
pub const DISP_E_BADINDEX: windows_core::HRESULT = windows_core::HRESULT(0x8002000B_u32 as _);
pub const DISP_E_BADPARAMCOUNT: windows_core::HRESULT = windows_core::HRESULT(0x8002000E_u32 as _);
pub const DISP_E_BADVARTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80020008_u32 as _);
pub const DISP_E_BUFFERTOOSMALL: windows_core::HRESULT = windows_core::HRESULT(0x80020013_u32 as _);
pub const DISP_E_DIVBYZERO: windows_core::HRESULT = windows_core::HRESULT(0x80020012_u32 as _);
pub const DISP_E_EXCEPTION: windows_core::HRESULT = windows_core::HRESULT(0x80020009_u32 as _);
pub const DISP_E_MEMBERNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80020003_u32 as _);
pub const DISP_E_NONAMEDARGS: windows_core::HRESULT = windows_core::HRESULT(0x80020007_u32 as _);
pub const DISP_E_NOTACOLLECTION: windows_core::HRESULT = windows_core::HRESULT(0x80020011_u32 as _);
pub const DISP_E_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x8002000A_u32 as _);
pub const DISP_E_PARAMNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80020004_u32 as _);
pub const DISP_E_PARAMNOTOPTIONAL: windows_core::HRESULT = windows_core::HRESULT(0x8002000F_u32 as _);
pub const DISP_E_TYPEMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80020005_u32 as _);
pub const DISP_E_UNKNOWNINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80020001_u32 as _);
pub const DISP_E_UNKNOWNLCID: windows_core::HRESULT = windows_core::HRESULT(0x8002000C_u32 as _);
pub const DISP_E_UNKNOWNNAME: windows_core::HRESULT = windows_core::HRESULT(0x80020006_u32 as _);
pub const DNS_ERROR_ADDRESS_REQUIRED: WIN32_ERROR = WIN32_ERROR(9573u32);
pub const DNS_ERROR_ALIAS_LOOP: WIN32_ERROR = WIN32_ERROR(9722u32);
pub const DNS_ERROR_AUTOZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9610u32);
pub const DNS_ERROR_AXFR: WIN32_ERROR = WIN32_ERROR(9752u32);
pub const DNS_ERROR_BACKGROUND_LOADING: WIN32_ERROR = WIN32_ERROR(9568u32);
pub const DNS_ERROR_BAD_KEYMASTER: WIN32_ERROR = WIN32_ERROR(9122u32);
pub const DNS_ERROR_BAD_PACKET: WIN32_ERROR = WIN32_ERROR(9502u32);
pub const DNS_ERROR_CANNOT_FIND_ROOT_HINTS: WIN32_ERROR = WIN32_ERROR(9564u32);
pub const DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9977u32);
pub const DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9976u32);
pub const DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED: WIN32_ERROR = WIN32_ERROR(9975u32);
pub const DNS_ERROR_CNAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9709u32);
pub const DNS_ERROR_CNAME_LOOP: WIN32_ERROR = WIN32_ERROR(9707u32);
pub const DNS_ERROR_DATABASE_BASE: WIN32_ERROR = WIN32_ERROR(9700u32);
pub const DNS_ERROR_DATAFILE_BASE: WIN32_ERROR = WIN32_ERROR(9650u32);
pub const DNS_ERROR_DATAFILE_OPEN_FAILURE: WIN32_ERROR = WIN32_ERROR(9653u32);
pub const DNS_ERROR_DATAFILE_PARSING: WIN32_ERROR = WIN32_ERROR(9655u32);
pub const DNS_ERROR_DEFAULT_SCOPE: WIN32_ERROR = WIN32_ERROR(9960u32);
pub const DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE: WIN32_ERROR = WIN32_ERROR(9925u32);
pub const DNS_ERROR_DEFAULT_ZONESCOPE: WIN32_ERROR = WIN32_ERROR(9953u32);
pub const DNS_ERROR_DELEGATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(9571u32);
pub const DNS_ERROR_DNAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9721u32);
pub const DNS_ERROR_DNSSEC_BASE: WIN32_ERROR = WIN32_ERROR(9100u32);
pub const DNS_ERROR_DNSSEC_IS_DISABLED: WIN32_ERROR = WIN32_ERROR(9125u32);
pub const DNS_ERROR_DP_ALREADY_ENLISTED: WIN32_ERROR = WIN32_ERROR(9904u32);
pub const DNS_ERROR_DP_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9902u32);
pub const DNS_ERROR_DP_BASE: WIN32_ERROR = WIN32_ERROR(9900u32);
pub const DNS_ERROR_DP_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9901u32);
pub const DNS_ERROR_DP_FSMO_ERROR: WIN32_ERROR = WIN32_ERROR(9906u32);
pub const DNS_ERROR_DP_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(9905u32);
pub const DNS_ERROR_DP_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(9903u32);
pub const DNS_ERROR_DS_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(9717u32);
pub const DNS_ERROR_DS_ZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9718u32);
pub const DNS_ERROR_DWORD_VALUE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(9567u32);
pub const DNS_ERROR_DWORD_VALUE_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(9566u32);
pub const DNS_ERROR_FILE_WRITEBACK_FAILED: WIN32_ERROR = WIN32_ERROR(9654u32);
pub const DNS_ERROR_FORWARDER_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9619u32);
pub const DNS_ERROR_GENERAL_API_BASE: WIN32_ERROR = WIN32_ERROR(9550u32);
pub const DNS_ERROR_INCONSISTENT_ROOT_HINTS: WIN32_ERROR = WIN32_ERROR(9565u32);
pub const DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME: WIN32_ERROR = WIN32_ERROR(9924u32);
pub const DNS_ERROR_INVALID_CLIENT_SUBNET_NAME: WIN32_ERROR = WIN32_ERROR(9984u32);
pub const DNS_ERROR_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(13u32);
pub const DNS_ERROR_INVALID_DATAFILE_NAME: WIN32_ERROR = WIN32_ERROR(9652u32);
pub const DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET: WIN32_ERROR = WIN32_ERROR(9115u32);
pub const DNS_ERROR_INVALID_IP_ADDRESS: WIN32_ERROR = WIN32_ERROR(9552u32);
pub const DNS_ERROR_INVALID_KEY_SIZE: WIN32_ERROR = WIN32_ERROR(9106u32);
pub const DNS_ERROR_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(123u32);
pub const DNS_ERROR_INVALID_NAME_CHAR: WIN32_ERROR = WIN32_ERROR(9560u32);
pub const DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT: WIN32_ERROR = WIN32_ERROR(9124u32);
pub const DNS_ERROR_INVALID_POLICY_TABLE: WIN32_ERROR = WIN32_ERROR(9572u32);
pub const DNS_ERROR_INVALID_PROPERTY: WIN32_ERROR = WIN32_ERROR(9553u32);
pub const DNS_ERROR_INVALID_ROLLOVER_PERIOD: WIN32_ERROR = WIN32_ERROR(9114u32);
pub const DNS_ERROR_INVALID_SCOPE_NAME: WIN32_ERROR = WIN32_ERROR(9958u32);
pub const DNS_ERROR_INVALID_SCOPE_OPERATION: WIN32_ERROR = WIN32_ERROR(9961u32);
pub const DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD: WIN32_ERROR = WIN32_ERROR(9123u32);
pub const DNS_ERROR_INVALID_TYPE: WIN32_ERROR = WIN32_ERROR(9551u32);
pub const DNS_ERROR_INVALID_XML: WIN32_ERROR = WIN32_ERROR(9126u32);
pub const DNS_ERROR_INVALID_ZONESCOPE_NAME: WIN32_ERROR = WIN32_ERROR(9954u32);
pub const DNS_ERROR_INVALID_ZONE_OPERATION: WIN32_ERROR = WIN32_ERROR(9603u32);
pub const DNS_ERROR_INVALID_ZONE_TYPE: WIN32_ERROR = WIN32_ERROR(9611u32);
pub const DNS_ERROR_KEYMASTER_REQUIRED: WIN32_ERROR = WIN32_ERROR(9101u32);
pub const DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION: WIN32_ERROR = WIN32_ERROR(9108u32);
pub const DNS_ERROR_KSP_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(9112u32);
pub const DNS_ERROR_LOAD_ZONESCOPE_FAILED: WIN32_ERROR = WIN32_ERROR(9956u32);
pub const DNS_ERROR_MASK: WIN32_ERROR = WIN32_ERROR(9000u32);
pub const DNS_ERROR_NAME_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9714u32);
pub const DNS_ERROR_NAME_NOT_IN_ZONE: WIN32_ERROR = WIN32_ERROR(9706u32);
pub const DNS_ERROR_NBSTAT_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(9617u32);
pub const DNS_ERROR_NEED_SECONDARY_ADDRESSES: WIN32_ERROR = WIN32_ERROR(9614u32);
pub const DNS_ERROR_NEED_WINS_SERVERS: WIN32_ERROR = WIN32_ERROR(9616u32);
pub const DNS_ERROR_NODE_CREATION_FAILED: WIN32_ERROR = WIN32_ERROR(9703u32);
pub const DNS_ERROR_NODE_IS_CNAME: WIN32_ERROR = WIN32_ERROR(9708u32);
pub const DNS_ERROR_NODE_IS_DNAME: WIN32_ERROR = WIN32_ERROR(9720u32);
pub const DNS_ERROR_NON_RFC_NAME: WIN32_ERROR = WIN32_ERROR(9556u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD: WIN32_ERROR = WIN32_ERROR(9119u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_RODC: WIN32_ERROR = WIN32_ERROR(9569u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER: WIN32_ERROR = WIN32_ERROR(9562u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE: WIN32_ERROR = WIN32_ERROR(9102u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE: WIN32_ERROR = WIN32_ERROR(9121u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_ZSK: WIN32_ERROR = WIN32_ERROR(9118u32);
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION: WIN32_ERROR = WIN32_ERROR(9563u32);
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DNAME: WIN32_ERROR = WIN32_ERROR(9570u32);
pub const DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES: WIN32_ERROR = WIN32_ERROR(9955u32);
pub const DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS: WIN32_ERROR = WIN32_ERROR(9104u32);
pub const DNS_ERROR_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(9555u32);
pub const DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE: WIN32_ERROR = WIN32_ERROR(9719u32);
pub const DNS_ERROR_NO_CREATE_CACHE_DATA: WIN32_ERROR = WIN32_ERROR(9713u32);
pub const DNS_ERROR_NO_DNS_SERVERS: WIN32_ERROR = WIN32_ERROR(9852u32);
pub const DNS_ERROR_NO_MEMORY: WIN32_ERROR = WIN32_ERROR(14u32);
pub const DNS_ERROR_NO_PACKET: WIN32_ERROR = WIN32_ERROR(9503u32);
pub const DNS_ERROR_NO_TCPIP: WIN32_ERROR = WIN32_ERROR(9851u32);
pub const DNS_ERROR_NO_VALID_TRUST_ANCHORS: WIN32_ERROR = WIN32_ERROR(9127u32);
pub const DNS_ERROR_NO_ZONE_INFO: WIN32_ERROR = WIN32_ERROR(9602u32);
pub const DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1: WIN32_ERROR = WIN32_ERROR(9103u32);
pub const DNS_ERROR_NSEC3_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9129u32);
pub const DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1: WIN32_ERROR = WIN32_ERROR(9130u32);
pub const DNS_ERROR_NUMERIC_NAME: WIN32_ERROR = WIN32_ERROR(9561u32);
pub const DNS_ERROR_OPERATION_BASE: WIN32_ERROR = WIN32_ERROR(9750u32);
pub const DNS_ERROR_PACKET_FMT_BASE: WIN32_ERROR = WIN32_ERROR(9500u32);
pub const DNS_ERROR_POLICY_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9971u32);
pub const DNS_ERROR_POLICY_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9972u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA: WIN32_ERROR = WIN32_ERROR(9973u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET: WIN32_ERROR = WIN32_ERROR(9990u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN: WIN32_ERROR = WIN32_ERROR(9994u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE: WIN32_ERROR = WIN32_ERROR(9993u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL: WIN32_ERROR = WIN32_ERROR(9992u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE: WIN32_ERROR = WIN32_ERROR(9995u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY: WIN32_ERROR = WIN32_ERROR(9996u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL: WIN32_ERROR = WIN32_ERROR(9991u32);
pub const DNS_ERROR_POLICY_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(9982u32);
pub const DNS_ERROR_POLICY_INVALID_SETTINGS: WIN32_ERROR = WIN32_ERROR(9974u32);
pub const DNS_ERROR_POLICY_INVALID_WEIGHT: WIN32_ERROR = WIN32_ERROR(9981u32);
pub const DNS_ERROR_POLICY_LOCKED: WIN32_ERROR = WIN32_ERROR(9980u32);
pub const DNS_ERROR_POLICY_MISSING_CRITERIA: WIN32_ERROR = WIN32_ERROR(9983u32);
pub const DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID: WIN32_ERROR = WIN32_ERROR(9985u32);
pub const DNS_ERROR_POLICY_SCOPE_MISSING: WIN32_ERROR = WIN32_ERROR(9986u32);
pub const DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(9987u32);
pub const DNS_ERROR_PRIMARY_REQUIRES_DATAFILE: WIN32_ERROR = WIN32_ERROR(9651u32);
pub const DNS_ERROR_RCODE: WIN32_ERROR = WIN32_ERROR(9504u32);
pub const DNS_ERROR_RCODE_BADKEY: WIN32_ERROR = WIN32_ERROR(9017u32);
pub const DNS_ERROR_RCODE_BADSIG: WIN32_ERROR = WIN32_ERROR(9016u32);
pub const DNS_ERROR_RCODE_BADTIME: WIN32_ERROR = WIN32_ERROR(9018u32);
pub const DNS_ERROR_RCODE_FORMAT_ERROR: WIN32_ERROR = WIN32_ERROR(9001u32);
pub const DNS_ERROR_RCODE_LAST: WIN32_ERROR = WIN32_ERROR(9018u32);
pub const DNS_ERROR_RCODE_NAME_ERROR: WIN32_ERROR = WIN32_ERROR(9003u32);
pub const DNS_ERROR_RCODE_NOTAUTH: WIN32_ERROR = WIN32_ERROR(9009u32);
pub const DNS_ERROR_RCODE_NOTZONE: WIN32_ERROR = WIN32_ERROR(9010u32);
pub const DNS_ERROR_RCODE_NOT_IMPLEMENTED: WIN32_ERROR = WIN32_ERROR(9004u32);
pub const DNS_ERROR_RCODE_NO_ERROR: WIN32_ERROR = WIN32_ERROR(0u32);
pub const DNS_ERROR_RCODE_NXRRSET: WIN32_ERROR = WIN32_ERROR(9008u32);
pub const DNS_ERROR_RCODE_REFUSED: WIN32_ERROR = WIN32_ERROR(9005u32);
pub const DNS_ERROR_RCODE_SERVER_FAILURE: WIN32_ERROR = WIN32_ERROR(9002u32);
pub const DNS_ERROR_RCODE_YXDOMAIN: WIN32_ERROR = WIN32_ERROR(9006u32);
pub const DNS_ERROR_RCODE_YXRRSET: WIN32_ERROR = WIN32_ERROR(9007u32);
pub const DNS_ERROR_RECORD_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9711u32);
pub const DNS_ERROR_RECORD_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9701u32);
pub const DNS_ERROR_RECORD_FORMAT: WIN32_ERROR = WIN32_ERROR(9702u32);
pub const DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT: WIN32_ERROR = WIN32_ERROR(9710u32);
pub const DNS_ERROR_RECORD_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(9705u32);
pub const DNS_ERROR_RESPONSE_CODES_BASE: WIN32_ERROR = WIN32_ERROR(9000u32);
pub const DNS_ERROR_ROLLOVER_ALREADY_QUEUED: WIN32_ERROR = WIN32_ERROR(9120u32);
pub const DNS_ERROR_ROLLOVER_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(9116u32);
pub const DNS_ERROR_ROLLOVER_NOT_POKEABLE: WIN32_ERROR = WIN32_ERROR(9128u32);
pub const DNS_ERROR_RRL_INVALID_IPV4_PREFIX: WIN32_ERROR = WIN32_ERROR(9913u32);
pub const DNS_ERROR_RRL_INVALID_IPV6_PREFIX: WIN32_ERROR = WIN32_ERROR(9914u32);
pub const DNS_ERROR_RRL_INVALID_LEAK_RATE: WIN32_ERROR = WIN32_ERROR(9916u32);
pub const DNS_ERROR_RRL_INVALID_TC_RATE: WIN32_ERROR = WIN32_ERROR(9915u32);
pub const DNS_ERROR_RRL_INVALID_WINDOW_SIZE: WIN32_ERROR = WIN32_ERROR(9912u32);
pub const DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE: WIN32_ERROR = WIN32_ERROR(9917u32);
pub const DNS_ERROR_RRL_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(9911u32);
pub const DNS_ERROR_SCOPE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9963u32);
pub const DNS_ERROR_SCOPE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9959u32);
pub const DNS_ERROR_SCOPE_LOCKED: WIN32_ERROR = WIN32_ERROR(9962u32);
pub const DNS_ERROR_SECONDARY_DATA: WIN32_ERROR = WIN32_ERROR(9712u32);
pub const DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP: WIN32_ERROR = WIN32_ERROR(9612u32);
pub const DNS_ERROR_SECURE_BASE: WIN32_ERROR = WIN32_ERROR(9800u32);
pub const DNS_ERROR_SERVERSCOPE_IS_REFERENCED: WIN32_ERROR = WIN32_ERROR(9988u32);
pub const DNS_ERROR_SETUP_BASE: WIN32_ERROR = WIN32_ERROR(9850u32);
pub const DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(9107u32);
pub const DNS_ERROR_SOA_DELETE_INVALID: WIN32_ERROR = WIN32_ERROR(9618u32);
pub const DNS_ERROR_STANDBY_KEY_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(9117u32);
pub const DNS_ERROR_SUBNET_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9979u32);
pub const DNS_ERROR_SUBNET_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9978u32);
pub const DNS_ERROR_TOO_MANY_SKDS: WIN32_ERROR = WIN32_ERROR(9113u32);
pub const DNS_ERROR_TRY_AGAIN_LATER: WIN32_ERROR = WIN32_ERROR(9554u32);
pub const DNS_ERROR_UNEXPECTED_CNG_ERROR: WIN32_ERROR = WIN32_ERROR(9110u32);
pub const DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR: WIN32_ERROR = WIN32_ERROR(9109u32);
pub const DNS_ERROR_UNKNOWN_RECORD_TYPE: WIN32_ERROR = WIN32_ERROR(9704u32);
pub const DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION: WIN32_ERROR = WIN32_ERROR(9111u32);
pub const DNS_ERROR_UNSECURE_PACKET: WIN32_ERROR = WIN32_ERROR(9505u32);
pub const DNS_ERROR_UNSUPPORTED_ALGORITHM: WIN32_ERROR = WIN32_ERROR(9105u32);
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9921u32);
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9922u32);
pub const DNS_ERROR_VIRTUALIZATION_TREE_LOCKED: WIN32_ERROR = WIN32_ERROR(9923u32);
pub const DNS_ERROR_WINS_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(9615u32);
pub const DNS_ERROR_ZONESCOPE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9951u32);
pub const DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9952u32);
pub const DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED: WIN32_ERROR = WIN32_ERROR(9957u32);
pub const DNS_ERROR_ZONESCOPE_IS_REFERENCED: WIN32_ERROR = WIN32_ERROR(9989u32);
pub const DNS_ERROR_ZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9609u32);
pub const DNS_ERROR_ZONE_BASE: WIN32_ERROR = WIN32_ERROR(9600u32);
pub const DNS_ERROR_ZONE_CONFIGURATION_ERROR: WIN32_ERROR = WIN32_ERROR(9604u32);
pub const DNS_ERROR_ZONE_CREATION_FAILED: WIN32_ERROR = WIN32_ERROR(9608u32);
pub const DNS_ERROR_ZONE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9601u32);
pub const DNS_ERROR_ZONE_HAS_NO_NS_RECORDS: WIN32_ERROR = WIN32_ERROR(9606u32);
pub const DNS_ERROR_ZONE_HAS_NO_SOA_RECORD: WIN32_ERROR = WIN32_ERROR(9605u32);
pub const DNS_ERROR_ZONE_IS_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(9621u32);
pub const DNS_ERROR_ZONE_LOCKED: WIN32_ERROR = WIN32_ERROR(9607u32);
pub const DNS_ERROR_ZONE_LOCKED_FOR_SIGNING: WIN32_ERROR = WIN32_ERROR(9622u32);
pub const DNS_ERROR_ZONE_NOT_SECONDARY: WIN32_ERROR = WIN32_ERROR(9613u32);
pub const DNS_ERROR_ZONE_REQUIRES_MASTER_IP: WIN32_ERROR = WIN32_ERROR(9620u32);
pub const DNS_INFO_ADDED_LOCAL_WINS: i32 = 9753i32;
pub const DNS_INFO_AXFR_COMPLETE: i32 = 9751i32;
pub const DNS_INFO_NO_RECORDS: i32 = 9501i32;
pub const DNS_REQUEST_PENDING: i32 = 9506i32;
pub const DNS_STATUS_CONTINUE_NEEDED: i32 = 9801i32;
pub const DNS_STATUS_DOTTED_NAME: i32 = 9558i32;
pub const DNS_STATUS_FQDN: i32 = 9557i32;
pub const DNS_STATUS_SINGLE_PART_NAME: i32 = 9559i32;
pub const DNS_WARNING_DOMAIN_UNDELETED: i32 = 9716i32;
pub const DNS_WARNING_PTR_CREATE_FAILED: i32 = 9715i32;
pub const DRAGDROP_E_ALREADYREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80040101_u32 as _);
pub const DRAGDROP_E_CONCURRENT_DRAG_ATTEMPTED: windows_core::HRESULT = windows_core::HRESULT(0x80040103_u32 as _);
pub const DRAGDROP_E_FIRST: i32 = -2147221248i32;
pub const DRAGDROP_E_INVALIDHWND: windows_core::HRESULT = windows_core::HRESULT(0x80040102_u32 as _);
pub const DRAGDROP_E_LAST: i32 = -2147221233i32;
pub const DRAGDROP_E_NOTREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80040100_u32 as _);
pub const DRAGDROP_S_CANCEL: windows_core::HRESULT = windows_core::HRESULT(0x40101_u32 as _);
pub const DRAGDROP_S_DROP: windows_core::HRESULT = windows_core::HRESULT(0x40100_u32 as _);
pub const DRAGDROP_S_FIRST: i32 = 262400i32;
pub const DRAGDROP_S_LAST: i32 = 262415i32;
pub const DRAGDROP_S_USEDEFAULTCURSORS: windows_core::HRESULT = windows_core::HRESULT(0x40102_u32 as _);
pub const DUPLICATE_CLOSE_SOURCE: DUPLICATE_HANDLE_OPTIONS = DUPLICATE_HANDLE_OPTIONS(1u32);
pub const DUPLICATE_SAME_ACCESS: DUPLICATE_HANDLE_OPTIONS = DUPLICATE_HANDLE_OPTIONS(2u32);
pub const DV_E_CLIPFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004006A_u32 as _);
pub const DV_E_DVASPECT: windows_core::HRESULT = windows_core::HRESULT(0x8004006B_u32 as _);
pub const DV_E_DVTARGETDEVICE: windows_core::HRESULT = windows_core::HRESULT(0x80040065_u32 as _);
pub const DV_E_DVTARGETDEVICE_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x8004006C_u32 as _);
pub const DV_E_FORMATETC: windows_core::HRESULT = windows_core::HRESULT(0x80040064_u32 as _);
pub const DV_E_LINDEX: windows_core::HRESULT = windows_core::HRESULT(0x80040068_u32 as _);
pub const DV_E_NOIVIEWOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x8004006D_u32 as _);
pub const DV_E_STATDATA: windows_core::HRESULT = windows_core::HRESULT(0x80040067_u32 as _);
pub const DV_E_STGMEDIUM: windows_core::HRESULT = windows_core::HRESULT(0x80040066_u32 as _);
pub const DV_E_TYMED: windows_core::HRESULT = windows_core::HRESULT(0x80040069_u32 as _);
pub const DWMERR_CATASTROPHIC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x88980702_u32 as _);
pub const DWMERR_STATE_TRANSITION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980700_u32 as _);
pub const DWMERR_THEME_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980701_u32 as _);
pub const DWM_E_ADAPTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80263005_u32 as _);
pub const DWM_E_COMPOSITIONDISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80263001_u32 as _);
pub const DWM_E_NOT_QUEUING_PRESENTS: windows_core::HRESULT = windows_core::HRESULT(0x80263004_u32 as _);
pub const DWM_E_NO_REDIRECTION_SURFACE_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80263003_u32 as _);
pub const DWM_E_REMOTING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80263002_u32 as _);
pub const DWM_E_TEXTURE_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80263007_u32 as _);
pub const DWM_S_GDI_REDIRECTION_SURFACE: windows_core::HRESULT = windows_core::HRESULT(0x263005_u32 as _);
pub const DWM_S_GDI_REDIRECTION_SURFACE_BLT_VIA_GDI: windows_core::HRESULT = windows_core::HRESULT(0x263008_u32 as _);
pub const DWRITE_E_ALREADYREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x88985006_u32 as _);
pub const DWRITE_E_CACHEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x88985007_u32 as _);
pub const DWRITE_E_CACHEVERSION: windows_core::HRESULT = windows_core::HRESULT(0x88985008_u32 as _);
pub const DWRITE_E_FILEACCESS: windows_core::HRESULT = windows_core::HRESULT(0x88985004_u32 as _);
pub const DWRITE_E_FILEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x88985000_u32 as _);
pub const DWRITE_E_FILENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x88985003_u32 as _);
pub const DWRITE_E_FLOWDIRECTIONCONFLICTS: windows_core::HRESULT = windows_core::HRESULT(0x8898500B_u32 as _);
pub const DWRITE_E_FONTCOLLECTIONOBSOLETE: windows_core::HRESULT = windows_core::HRESULT(0x88985005_u32 as _);
pub const DWRITE_E_NOCOLOR: windows_core::HRESULT = windows_core::HRESULT(0x8898500C_u32 as _);
pub const DWRITE_E_NOFONT: windows_core::HRESULT = windows_core::HRESULT(0x88985002_u32 as _);
pub const DWRITE_E_TEXTRENDERERINCOMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0x8898500A_u32 as _);
pub const DWRITE_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x88985001_u32 as _);
pub const DWRITE_E_UNSUPPORTEDOPERATION: windows_core::HRESULT = windows_core::HRESULT(0x88985009_u32 as _);
pub const DXCORE_ERROR_EVENT_NOT_UNREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x88800001_u32 as _);
pub const DXGI_DDI_ERR_NONEXCLUSIVE: windows_core::HRESULT = windows_core::HRESULT(0x887B0003_u32 as _);
pub const DXGI_DDI_ERR_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x887B0002_u32 as _);
pub const DXGI_DDI_ERR_WASSTILLDRAWING: windows_core::HRESULT = windows_core::HRESULT(0x887B0001_u32 as _);
pub const DXGI_STATUS_CLIPPED: windows_core::HRESULT = windows_core::HRESULT(0x87A0002_u32 as _);
pub const DXGI_STATUS_DDA_WAS_STILL_DRAWING: windows_core::HRESULT = windows_core::HRESULT(0x87A000A_u32 as _);
pub const DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x87A0006_u32 as _);
pub const DXGI_STATUS_MODE_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x87A0007_u32 as _);
pub const DXGI_STATUS_MODE_CHANGE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x87A0008_u32 as _);
pub const DXGI_STATUS_NO_DESKTOP_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x87A0005_u32 as _);
pub const DXGI_STATUS_NO_REDIRECTION: windows_core::HRESULT = windows_core::HRESULT(0x87A0004_u32 as _);
pub const DXGI_STATUS_OCCLUDED: windows_core::HRESULT = windows_core::HRESULT(0x87A0001_u32 as _);
pub const DXGI_STATUS_PRESENT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x87A002F_u32 as _);
pub const DXGI_STATUS_UNOCCLUDED: windows_core::HRESULT = windows_core::HRESULT(0x87A0009_u32 as _);
pub const EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80550008_u32 as _);
pub const EAS_E_ADMINS_HAVE_BLANK_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80550007_u32 as _);
pub const EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x8055000B_u32 as _);
pub const EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x8055000D_u32 as _);
pub const EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80550004_u32 as _);
pub const EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80550009_u32 as _);
pub const EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS: windows_core::HRESULT = windows_core::HRESULT(0x8055000A_u32 as _);
pub const EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER: windows_core::HRESULT = windows_core::HRESULT(0x8055000C_u32 as _);
pub const EAS_E_POLICY_COMPLIANT_WITH_ACTIONS: windows_core::HRESULT = windows_core::HRESULT(0x80550002_u32 as _);
pub const EAS_E_POLICY_NOT_MANAGED_BY_OS: windows_core::HRESULT = windows_core::HRESULT(0x80550001_u32 as _);
pub const EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE: windows_core::HRESULT = windows_core::HRESULT(0x80550003_u32 as _);
pub const EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0x80550005_u32 as _);
pub const EAS_E_USER_CANNOT_CHANGE_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80550006_u32 as _);
pub const ENUM_E_FIRST: i32 = -2147221072i32;
pub const ENUM_E_LAST: i32 = -2147221057i32;
pub const ENUM_S_FIRST: i32 = 262576i32;
pub const ENUM_S_LAST: i32 = 262591i32;
pub const EPT_NT_CANT_CREATE: NTSTATUS = NTSTATUS(0xC002004C_u32 as _);
pub const EPT_NT_CANT_PERFORM_OP: NTSTATUS = NTSTATUS(0xC0020035_u32 as _);
pub const EPT_NT_INVALID_ENTRY: NTSTATUS = NTSTATUS(0xC0020034_u32 as _);
pub const EPT_NT_NOT_REGISTERED: NTSTATUS = NTSTATUS(0xC0020036_u32 as _);
pub const ERROR_ABANDONED_WAIT_0: WIN32_ERROR = WIN32_ERROR(735u32);
pub const ERROR_ABANDONED_WAIT_63: WIN32_ERROR = WIN32_ERROR(736u32);
pub const ERROR_ABANDON_HIBERFILE: WIN32_ERROR = WIN32_ERROR(787u32);
pub const ERROR_ABIOS_ERROR: WIN32_ERROR = WIN32_ERROR(538u32);
pub const ERROR_ACCESS_AUDIT_BY_POLICY: WIN32_ERROR = WIN32_ERROR(785u32);
pub const ERROR_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(5u32);
pub const ERROR_ACCESS_DENIED_APPDATA: WIN32_ERROR = WIN32_ERROR(502u32);
pub const ERROR_ACCESS_DISABLED_BY_POLICY: WIN32_ERROR = WIN32_ERROR(1260u32);
pub const ERROR_ACCESS_DISABLED_NO_SAFER_UI_BY_POLICY: WIN32_ERROR = WIN32_ERROR(786u32);
pub const ERROR_ACCESS_DISABLED_WEBBLADE: WIN32_ERROR = WIN32_ERROR(1277u32);
pub const ERROR_ACCESS_DISABLED_WEBBLADE_TAMPER: WIN32_ERROR = WIN32_ERROR(1278u32);
pub const ERROR_ACCOUNT_DISABLED: WIN32_ERROR = WIN32_ERROR(1331u32);
pub const ERROR_ACCOUNT_EXPIRED: WIN32_ERROR = WIN32_ERROR(1793u32);
pub const ERROR_ACCOUNT_LOCKED_OUT: WIN32_ERROR = WIN32_ERROR(1909u32);
pub const ERROR_ACCOUNT_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1327u32);
pub const ERROR_ACPI_ERROR: WIN32_ERROR = WIN32_ERROR(669u32);
pub const ERROR_ACTIVATION_COUNT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(7059u32);
pub const ERROR_ACTIVE_CONNECTIONS: WIN32_ERROR = WIN32_ERROR(2402u32);
pub const ERROR_ADAP_HDW_ERR: WIN32_ERROR = WIN32_ERROR(57u32);
pub const ERROR_ADDRESS_ALREADY_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(1227u32);
pub const ERROR_ADDRESS_NOT_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(1228u32);
pub const ERROR_ADVANCED_INSTALLER_FAILED: WIN32_ERROR = WIN32_ERROR(14099u32);
pub const ERROR_ALERTED: WIN32_ERROR = WIN32_ERROR(739u32);
pub const ERROR_ALIAS_EXISTS: WIN32_ERROR = WIN32_ERROR(1379u32);
pub const ERROR_ALLOCATE_BUCKET: WIN32_ERROR = WIN32_ERROR(602u32);
pub const ERROR_ALLOTTED_SPACE_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1344u32);
pub const ERROR_ALLOWED_PORT_TYPE_RESTRICTION: u32 = 941u32;
pub const ERROR_ALL_NODES_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5037u32);
pub const ERROR_ALL_SIDS_FILTERED: windows_core::HRESULT = windows_core::HRESULT(0xC0090002_u32 as _);
pub const ERROR_ALL_USER_TRUST_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1933u32);
pub const ERROR_ALREADY_ASSIGNED: WIN32_ERROR = WIN32_ERROR(85u32);
pub const ERROR_ALREADY_CONNECTED: u32 = 901u32;
pub const ERROR_ALREADY_CONNECTING: u32 = 910u32;
pub const ERROR_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(183u32);
pub const ERROR_ALREADY_FIBER: WIN32_ERROR = WIN32_ERROR(1280u32);
pub const ERROR_ALREADY_HAS_STREAM_ID: WIN32_ERROR = WIN32_ERROR(4444u32);
pub const ERROR_ALREADY_INITIALIZED: WIN32_ERROR = WIN32_ERROR(1247u32);
pub const ERROR_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(1242u32);
pub const ERROR_ALREADY_RUNNING_LKG: WIN32_ERROR = WIN32_ERROR(1074u32);
pub const ERROR_ALREADY_THREAD: WIN32_ERROR = WIN32_ERROR(1281u32);
pub const ERROR_ALREADY_WAITING: WIN32_ERROR = WIN32_ERROR(1904u32);
pub const ERROR_ALREADY_WIN32: WIN32_ERROR = WIN32_ERROR(719u32);
pub const ERROR_AMBIGUOUS_SYSTEM_DEVICE: WIN32_ERROR = WIN32_ERROR(15250u32);
pub const ERROR_API_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(15841u32);
pub const ERROR_APPCONTAINER_REQUIRED: WIN32_ERROR = WIN32_ERROR(4251u32);
pub const ERROR_APPEXEC_APP_COMPAT_BLOCK: WIN32_ERROR = WIN32_ERROR(3068u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT: WIN32_ERROR = WIN32_ERROR(3069u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_LICENSING: WIN32_ERROR = WIN32_ERROR(3071u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_RESOURCES: WIN32_ERROR = WIN32_ERROR(3072u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_TERMINATION: WIN32_ERROR = WIN32_ERROR(3070u32);
pub const ERROR_APPEXEC_CONDITION_NOT_SATISFIED: WIN32_ERROR = WIN32_ERROR(3060u32);
pub const ERROR_APPEXEC_HANDLE_INVALIDATED: WIN32_ERROR = WIN32_ERROR(3061u32);
pub const ERROR_APPEXEC_HOST_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(3066u32);
pub const ERROR_APPEXEC_INVALID_HOST_GENERATION: WIN32_ERROR = WIN32_ERROR(3062u32);
pub const ERROR_APPEXEC_INVALID_HOST_STATE: WIN32_ERROR = WIN32_ERROR(3064u32);
pub const ERROR_APPEXEC_NO_DONOR: WIN32_ERROR = WIN32_ERROR(3065u32);
pub const ERROR_APPEXEC_UNEXPECTED_PROCESS_REGISTRATION: WIN32_ERROR = WIN32_ERROR(3063u32);
pub const ERROR_APPEXEC_UNKNOWN_USER: WIN32_ERROR = WIN32_ERROR(3067u32);
pub const ERROR_APPHELP_BLOCK: WIN32_ERROR = WIN32_ERROR(1259u32);
pub const ERROR_APPINSTALLER_ACTIVATION_BLOCKED: WIN32_ERROR = WIN32_ERROR(15646u32);
pub const ERROR_APPINSTALLER_IS_MANAGED_BY_SYSTEM: WIN32_ERROR = WIN32_ERROR(15672u32);
pub const ERROR_APPINSTALLER_URI_IN_USE: WIN32_ERROR = WIN32_ERROR(15671u32);
pub const ERROR_APPX_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(409u32);
pub const ERROR_APPX_INTEGRITY_FAILURE_CLR_NGEN: WIN32_ERROR = WIN32_ERROR(15624u32);
pub const ERROR_APPX_RAW_DATA_WRITE_FAILED: WIN32_ERROR = WIN32_ERROR(15648u32);
pub const ERROR_APP_DATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(4402u32);
pub const ERROR_APP_DATA_EXPIRED: WIN32_ERROR = WIN32_ERROR(4401u32);
pub const ERROR_APP_DATA_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(4403u32);
pub const ERROR_APP_DATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4400u32);
pub const ERROR_APP_DATA_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(4404u32);
pub const ERROR_APP_HANG: WIN32_ERROR = WIN32_ERROR(1298u32);
pub const ERROR_APP_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(575u32);
pub const ERROR_APP_WRONG_OS: WIN32_ERROR = WIN32_ERROR(1151u32);
pub const ERROR_ARBITRATION_UNHANDLED: WIN32_ERROR = WIN32_ERROR(723u32);
pub const ERROR_ARENA_TRASHED: WIN32_ERROR = WIN32_ERROR(7u32);
pub const ERROR_ARITHMETIC_OVERFLOW: WIN32_ERROR = WIN32_ERROR(534u32);
pub const ERROR_ASSERTION_FAILURE: WIN32_ERROR = WIN32_ERROR(668u32);
pub const ERROR_ATOMIC_LOCKS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(174u32);
pub const ERROR_ATTRIBUTE_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x8083000A_u32 as _);
pub const ERROR_AUDITING_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0xC0090001_u32 as _);
pub const ERROR_AUDIT_FAILED: WIN32_ERROR = WIN32_ERROR(606u32);
pub const ERROR_AUTHENTICATION_FIREWALL_FAILED: WIN32_ERROR = WIN32_ERROR(1935u32);
pub const ERROR_AUTHENTICATOR_MISMATCH: u32 = 955u32;
pub const ERROR_AUTHENTICODE_DISALLOWED: WIN32_ERROR = WIN32_ERROR(3758096960u32);
pub const ERROR_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: WIN32_ERROR = WIN32_ERROR(3758096963u32);
pub const ERROR_AUTHENTICODE_TRUSTED_PUBLISHER: WIN32_ERROR = WIN32_ERROR(3758096961u32);
pub const ERROR_AUTHENTICODE_TRUST_NOT_ESTABLISHED: WIN32_ERROR = WIN32_ERROR(3758096962u32);
pub const ERROR_AUTHIP_FAILURE: WIN32_ERROR = WIN32_ERROR(1469u32);
pub const ERROR_AUTH_PROTOCOL_REJECTED: u32 = 917u32;
pub const ERROR_AUTH_PROTOCOL_RESTRICTION: u32 = 942u32;
pub const ERROR_AUTH_SERVER_TIMEOUT: u32 = 930u32;
pub const ERROR_AUTODATASEG_EXCEEDS_64k: WIN32_ERROR = WIN32_ERROR(199u32);
pub const ERROR_BACKUP_CONTROLLER: WIN32_ERROR = WIN32_ERROR(586u32);
pub const ERROR_BADDB: WIN32_ERROR = WIN32_ERROR(1009u32);
pub const ERROR_BADKEY: WIN32_ERROR = WIN32_ERROR(1010u32);
pub const ERROR_BADSTARTPOSITION: WIN32_ERROR = WIN32_ERROR(778u32);
pub const ERROR_BAD_ACCESSOR_FLAGS: WIN32_ERROR = WIN32_ERROR(773u32);
pub const ERROR_BAD_ARGUMENTS: WIN32_ERROR = WIN32_ERROR(160u32);
pub const ERROR_BAD_CLUSTERS: WIN32_ERROR = WIN32_ERROR(6849u32);
pub const ERROR_BAD_COMMAND: WIN32_ERROR = WIN32_ERROR(22u32);
pub const ERROR_BAD_COMPRESSION_BUFFER: WIN32_ERROR = WIN32_ERROR(605u32);
pub const ERROR_BAD_CONFIGURATION: WIN32_ERROR = WIN32_ERROR(1610u32);
pub const ERROR_BAD_CURRENT_DIRECTORY: WIN32_ERROR = WIN32_ERROR(703u32);
pub const ERROR_BAD_DESCRIPTOR_FORMAT: WIN32_ERROR = WIN32_ERROR(1361u32);
pub const ERROR_BAD_DEVICE: WIN32_ERROR = WIN32_ERROR(1200u32);
pub const ERROR_BAD_DEVICE_PATH: WIN32_ERROR = WIN32_ERROR(330u32);
pub const ERROR_BAD_DEV_TYPE: WIN32_ERROR = WIN32_ERROR(66u32);
pub const ERROR_BAD_DLL_ENTRYPOINT: WIN32_ERROR = WIN32_ERROR(609u32);
pub const ERROR_BAD_DRIVER: WIN32_ERROR = WIN32_ERROR(2001u32);
pub const ERROR_BAD_DRIVER_LEVEL: WIN32_ERROR = WIN32_ERROR(119u32);
pub const ERROR_BAD_ENVIRONMENT: WIN32_ERROR = WIN32_ERROR(10u32);
pub const ERROR_BAD_EXE_FORMAT: WIN32_ERROR = WIN32_ERROR(193u32);
pub const ERROR_BAD_FILE_TYPE: WIN32_ERROR = WIN32_ERROR(222u32);
pub const ERROR_BAD_FORMAT: WIN32_ERROR = WIN32_ERROR(11u32);
pub const ERROR_BAD_FUNCTION_TABLE: WIN32_ERROR = WIN32_ERROR(559u32);
pub const ERROR_BAD_IMPERSONATION_LEVEL: WIN32_ERROR = WIN32_ERROR(1346u32);
pub const ERROR_BAD_INHERITANCE_ACL: WIN32_ERROR = WIN32_ERROR(1340u32);
pub const ERROR_BAD_INTERFACE_INSTALLSECT: WIN32_ERROR = WIN32_ERROR(3758096925u32);
pub const ERROR_BAD_LENGTH: WIN32_ERROR = WIN32_ERROR(24u32);
pub const ERROR_BAD_LOGON_SESSION_STATE: WIN32_ERROR = WIN32_ERROR(1365u32);
pub const ERROR_BAD_MCFG_TABLE: WIN32_ERROR = WIN32_ERROR(791u32);
pub const ERROR_BAD_NETPATH: WIN32_ERROR = WIN32_ERROR(53u32);
pub const ERROR_BAD_NET_NAME: WIN32_ERROR = WIN32_ERROR(67u32);
pub const ERROR_BAD_NET_RESP: WIN32_ERROR = WIN32_ERROR(58u32);
pub const ERROR_BAD_PATHNAME: WIN32_ERROR = WIN32_ERROR(161u32);
pub const ERROR_BAD_PIPE: WIN32_ERROR = WIN32_ERROR(230u32);
pub const ERROR_BAD_PROFILE: WIN32_ERROR = WIN32_ERROR(1206u32);
pub const ERROR_BAD_PROVIDER: WIN32_ERROR = WIN32_ERROR(1204u32);
pub const ERROR_BAD_QUERY_SYNTAX: WIN32_ERROR = WIN32_ERROR(1615u32);
pub const ERROR_BAD_RECOVERY_POLICY: WIN32_ERROR = WIN32_ERROR(6012u32);
pub const ERROR_BAD_REM_ADAP: WIN32_ERROR = WIN32_ERROR(60u32);
pub const ERROR_BAD_SECTION_NAME_LINE: WIN32_ERROR = WIN32_ERROR(3758096385u32);
pub const ERROR_BAD_SERVICE_ENTRYPOINT: WIN32_ERROR = WIN32_ERROR(610u32);
pub const ERROR_BAD_SERVICE_INSTALLSECT: WIN32_ERROR = WIN32_ERROR(3758096919u32);
pub const ERROR_BAD_STACK: WIN32_ERROR = WIN32_ERROR(543u32);
pub const ERROR_BAD_THREADID_ADDR: WIN32_ERROR = WIN32_ERROR(159u32);
pub const ERROR_BAD_TOKEN_TYPE: WIN32_ERROR = WIN32_ERROR(1349u32);
pub const ERROR_BAD_UNIT: WIN32_ERROR = WIN32_ERROR(20u32);
pub const ERROR_BAD_USERNAME: WIN32_ERROR = WIN32_ERROR(2202u32);
pub const ERROR_BAD_USER_PROFILE: WIN32_ERROR = WIN32_ERROR(1253u32);
pub const ERROR_BAD_VALIDATION_CLASS: WIN32_ERROR = WIN32_ERROR(1348u32);
pub const ERROR_BAP_DISCONNECTED: u32 = 936u32;
pub const ERROR_BAP_REQUIRED: u32 = 943u32;
pub const ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED: WIN32_ERROR = WIN32_ERROR(2151219201u32);
pub const ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED: WIN32_ERROR = WIN32_ERROR(2151219203u32);
pub const ERROR_BCD_TOO_MANY_ELEMENTS: WIN32_ERROR = WIN32_ERROR(3224961026u32);
pub const ERROR_BEGINNING_OF_MEDIA: WIN32_ERROR = WIN32_ERROR(1102u32);
pub const ERROR_BEYOND_VDL: WIN32_ERROR = WIN32_ERROR(1289u32);
pub const ERROR_BIOS_FAILED_TO_CONNECT_INTERRUPT: WIN32_ERROR = WIN32_ERROR(585u32);
pub const ERROR_BIZRULES_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0xC0090003_u32 as _);
pub const ERROR_BLOCKED_BY_PARENTAL_CONTROLS: WIN32_ERROR = WIN32_ERROR(346u32);
pub const ERROR_BLOCK_SHARED: WIN32_ERROR = WIN32_ERROR(514u32);
pub const ERROR_BLOCK_SOURCE_WEAK_REFERENCE_INVALID: WIN32_ERROR = WIN32_ERROR(512u32);
pub const ERROR_BLOCK_TARGET_WEAK_REFERENCE_INVALID: WIN32_ERROR = WIN32_ERROR(513u32);
pub const ERROR_BLOCK_TOO_MANY_REFERENCES: WIN32_ERROR = WIN32_ERROR(347u32);
pub const ERROR_BLOCK_WEAK_REFERENCE_INVALID: WIN32_ERROR = WIN32_ERROR(511u32);
pub const ERROR_BOOT_ALREADY_ACCEPTED: WIN32_ERROR = WIN32_ERROR(1076u32);
pub const ERROR_BROKEN_PIPE: WIN32_ERROR = WIN32_ERROR(109u32);
pub const ERROR_BUFFER_ALL_ZEROS: WIN32_ERROR = WIN32_ERROR(754u32);
pub const ERROR_BUFFER_OVERFLOW: WIN32_ERROR = WIN32_ERROR(111u32);
pub const ERROR_BUSY: WIN32_ERROR = WIN32_ERROR(170u32);
pub const ERROR_BUSY_DRIVE: WIN32_ERROR = WIN32_ERROR(142u32);
pub const ERROR_BUS_RESET: WIN32_ERROR = WIN32_ERROR(1111u32);
pub const ERROR_BYPASSIO_FLT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(506u32);
pub const ERROR_CACHE_PAGE_LOCKED: WIN32_ERROR = WIN32_ERROR(752u32);
pub const ERROR_CALLBACK_INVOKE_INLINE: WIN32_ERROR = WIN32_ERROR(812u32);
pub const ERROR_CALLBACK_POP_STACK: WIN32_ERROR = WIN32_ERROR(768u32);
pub const ERROR_CALLBACK_SUPPLIED_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(1273u32);
pub const ERROR_CALL_NOT_IMPLEMENTED: WIN32_ERROR = WIN32_ERROR(120u32);
pub const ERROR_CANCELLED: WIN32_ERROR = WIN32_ERROR(1223u32);
pub const ERROR_CANCEL_VIOLATION: WIN32_ERROR = WIN32_ERROR(173u32);
pub const ERROR_CANNOT_ABORT_TRANSACTIONS: WIN32_ERROR = WIN32_ERROR(6848u32);
pub const ERROR_CANNOT_ACCEPT_TRANSACTED_WORK: WIN32_ERROR = WIN32_ERROR(6847u32);
pub const ERROR_CANNOT_BREAK_OPLOCK: WIN32_ERROR = WIN32_ERROR(802u32);
pub const ERROR_CANNOT_COPY: WIN32_ERROR = WIN32_ERROR(266u32);
pub const ERROR_CANNOT_DETECT_DRIVER_FAILURE: WIN32_ERROR = WIN32_ERROR(1080u32);
pub const ERROR_CANNOT_DETECT_PROCESS_ABORT: WIN32_ERROR = WIN32_ERROR(1081u32);
pub const ERROR_CANNOT_EXECUTE_FILE_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6838u32);
pub const ERROR_CANNOT_FIND_WND_CLASS: WIN32_ERROR = WIN32_ERROR(1407u32);
pub const ERROR_CANNOT_GRANT_REQUESTED_OPLOCK: WIN32_ERROR = WIN32_ERROR(801u32);
pub const ERROR_CANNOT_IMPERSONATE: WIN32_ERROR = WIN32_ERROR(1368u32);
pub const ERROR_CANNOT_LOAD_REGISTRY_FILE: WIN32_ERROR = WIN32_ERROR(589u32);
pub const ERROR_CANNOT_MAKE: WIN32_ERROR = WIN32_ERROR(82u32);
pub const ERROR_CANNOT_OPEN_PROFILE: WIN32_ERROR = WIN32_ERROR(1205u32);
pub const ERROR_CANNOT_SWITCH_RUNLEVEL: WIN32_ERROR = WIN32_ERROR(15400u32);
pub const ERROR_CANTFETCHBACKWARDS: WIN32_ERROR = WIN32_ERROR(770u32);
pub const ERROR_CANTOPEN: WIN32_ERROR = WIN32_ERROR(1011u32);
pub const ERROR_CANTREAD: WIN32_ERROR = WIN32_ERROR(1012u32);
pub const ERROR_CANTSCROLLBACKWARDS: WIN32_ERROR = WIN32_ERROR(771u32);
pub const ERROR_CANTWRITE: WIN32_ERROR = WIN32_ERROR(1013u32);
pub const ERROR_CANT_ACCESS_DOMAIN_INFO: WIN32_ERROR = WIN32_ERROR(1351u32);
pub const ERROR_CANT_ACCESS_FILE: WIN32_ERROR = WIN32_ERROR(1920u32);
pub const ERROR_CANT_ATTACH_TO_DEV_VOLUME: WIN32_ERROR = WIN32_ERROR(478u32);
pub const ERROR_CANT_BREAK_TRANSACTIONAL_DEPENDENCY: WIN32_ERROR = WIN32_ERROR(6824u32);
pub const ERROR_CANT_CLEAR_ENCRYPTION_FLAG: WIN32_ERROR = WIN32_ERROR(432u32);
pub const ERROR_CANT_CREATE_MORE_STREAM_MINIVERSIONS: WIN32_ERROR = WIN32_ERROR(6812u32);
pub const ERROR_CANT_CROSS_RM_BOUNDARY: WIN32_ERROR = WIN32_ERROR(6825u32);
pub const ERROR_CANT_DELETE_LAST_ITEM: WIN32_ERROR = WIN32_ERROR(4335u32);
pub const ERROR_CANT_DISABLE_MANDATORY: WIN32_ERROR = WIN32_ERROR(1310u32);
pub const ERROR_CANT_ENABLE_DENY_ONLY: WIN32_ERROR = WIN32_ERROR(629u32);
pub const ERROR_CANT_EVICT_ACTIVE_NODE: WIN32_ERROR = WIN32_ERROR(5009u32);
pub const ERROR_CANT_LOAD_CLASS_ICON: WIN32_ERROR = WIN32_ERROR(3758096908u32);
pub const ERROR_CANT_OPEN_ANONYMOUS: WIN32_ERROR = WIN32_ERROR(1347u32);
pub const ERROR_CANT_OPEN_MINIVERSION_WITH_MODIFY_INTENT: WIN32_ERROR = WIN32_ERROR(6811u32);
pub const ERROR_CANT_RECOVER_WITH_HANDLE_OPEN: WIN32_ERROR = WIN32_ERROR(6818u32);
pub const ERROR_CANT_REMOVE_DEVINST: WIN32_ERROR = WIN32_ERROR(3758096946u32);
pub const ERROR_CANT_RESOLVE_FILENAME: WIN32_ERROR = WIN32_ERROR(1921u32);
pub const ERROR_CANT_TERMINATE_SELF: WIN32_ERROR = WIN32_ERROR(555u32);
pub const ERROR_CANT_WAIT: WIN32_ERROR = WIN32_ERROR(554u32);
pub const ERROR_CAN_NOT_COMPLETE: WIN32_ERROR = WIN32_ERROR(1003u32);
pub const ERROR_CAN_NOT_DEL_LOCAL_WINS: WIN32_ERROR = WIN32_ERROR(4001u32);
pub const ERROR_CAPAUTHZ_CHANGE_TYPE: WIN32_ERROR = WIN32_ERROR(451u32);
pub const ERROR_CAPAUTHZ_DB_CORRUPTED: WIN32_ERROR = WIN32_ERROR(455u32);
pub const ERROR_CAPAUTHZ_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(453u32);
pub const ERROR_CAPAUTHZ_NOT_DEVUNLOCKED: WIN32_ERROR = WIN32_ERROR(450u32);
pub const ERROR_CAPAUTHZ_NOT_PROVISIONED: WIN32_ERROR = WIN32_ERROR(452u32);
pub const ERROR_CAPAUTHZ_NO_POLICY: WIN32_ERROR = WIN32_ERROR(454u32);
pub const ERROR_CAPAUTHZ_SCCD_DEV_MODE_REQUIRED: WIN32_ERROR = WIN32_ERROR(459u32);
pub const ERROR_CAPAUTHZ_SCCD_INVALID_CATALOG: WIN32_ERROR = WIN32_ERROR(456u32);
pub const ERROR_CAPAUTHZ_SCCD_NO_AUTH_ENTITY: WIN32_ERROR = WIN32_ERROR(457u32);
pub const ERROR_CAPAUTHZ_SCCD_NO_CAPABILITY_MATCH: WIN32_ERROR = WIN32_ERROR(460u32);
pub const ERROR_CAPAUTHZ_SCCD_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(458u32);
pub const ERROR_CARDBUS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(724u32);
pub const ERROR_CASE_DIFFERING_NAMES_IN_DIR: WIN32_ERROR = WIN32_ERROR(424u32);
pub const ERROR_CASE_SENSITIVE_PATH: WIN32_ERROR = WIN32_ERROR(442u32);
pub const ERROR_CERTIFICATE_VALIDATION_PREFERENCE_CONFLICT: WIN32_ERROR = WIN32_ERROR(817u32);
pub const ERROR_CHECKING_FILE_SYSTEM: WIN32_ERROR = WIN32_ERROR(712u32);
pub const ERROR_CHECKOUT_REQUIRED: WIN32_ERROR = WIN32_ERROR(221u32);
pub const ERROR_CHILD_MUST_BE_VOLATILE: WIN32_ERROR = WIN32_ERROR(1021u32);
pub const ERROR_CHILD_NOT_COMPLETE: WIN32_ERROR = WIN32_ERROR(129u32);
pub const ERROR_CHILD_PROCESS_BLOCKED: WIN32_ERROR = WIN32_ERROR(367u32);
pub const ERROR_CHILD_WINDOW_MENU: WIN32_ERROR = WIN32_ERROR(1436u32);
pub const ERROR_CIMFS_IMAGE_CORRUPT: WIN32_ERROR = WIN32_ERROR(470u32);
pub const ERROR_CIMFS_IMAGE_VERSION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(471u32);
pub const ERROR_CIRCULAR_DEPENDENCY: WIN32_ERROR = WIN32_ERROR(1059u32);
pub const ERROR_CLASSIC_COMPAT_MODE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15667u32);
pub const ERROR_CLASS_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(1410u32);
pub const ERROR_CLASS_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1411u32);
pub const ERROR_CLASS_HAS_WINDOWS: WIN32_ERROR = WIN32_ERROR(1412u32);
pub const ERROR_CLASS_MISMATCH: WIN32_ERROR = WIN32_ERROR(3758096897u32);
pub const ERROR_CLEANER_CARTRIDGE_INSTALLED: WIN32_ERROR = WIN32_ERROR(4340u32);
pub const ERROR_CLEANER_CARTRIDGE_SPENT: WIN32_ERROR = WIN32_ERROR(4333u32);
pub const ERROR_CLEANER_SLOT_NOT_SET: WIN32_ERROR = WIN32_ERROR(4332u32);
pub const ERROR_CLEANER_SLOT_SET: WIN32_ERROR = WIN32_ERROR(4331u32);
pub const ERROR_CLIENT_INTERFACE_ALREADY_EXISTS: u32 = 915u32;
pub const ERROR_CLIENT_SERVER_PARAMETERS_INVALID: WIN32_ERROR = WIN32_ERROR(597u32);
pub const ERROR_CLIPBOARD_NOT_OPEN: WIN32_ERROR = WIN32_ERROR(1418u32);
pub const ERROR_CLIPPING_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2005u32);
pub const ERROR_CLIP_DEVICE_LICENSE_MISSING: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0003_u32 as _);
pub const ERROR_CLIP_KEYHOLDER_LICENSE_MISSING_OR_INVALID: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0005_u32 as _);
pub const ERROR_CLIP_LICENSE_DEVICE_ID_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC0EA000A_u32 as _);
pub const ERROR_CLIP_LICENSE_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0006_u32 as _);
pub const ERROR_CLIP_LICENSE_HARDWARE_ID_OUT_OF_TOLERANCE: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0009_u32 as _);
pub const ERROR_CLIP_LICENSE_INVALID_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0004_u32 as _);
pub const ERROR_CLIP_LICENSE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0002_u32 as _);
pub const ERROR_CLIP_LICENSE_NOT_SIGNED: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0008_u32 as _);
pub const ERROR_CLIP_LICENSE_SIGNED_BY_UNKNOWN_SOURCE: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0007_u32 as _);
pub const ERROR_CLOUD_FILE_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(395u32);
pub const ERROR_CLOUD_FILE_ALREADY_CONNECTED: WIN32_ERROR = WIN32_ERROR(378u32);
pub const ERROR_CLOUD_FILE_AUTHENTICATION_FAILED: WIN32_ERROR = WIN32_ERROR(386u32);
pub const ERROR_CLOUD_FILE_CONNECTED_PROVIDER_ONLY: WIN32_ERROR = WIN32_ERROR(382u32);
pub const ERROR_CLOUD_FILE_DEHYDRATION_DISALLOWED: WIN32_ERROR = WIN32_ERROR(434u32);
pub const ERROR_CLOUD_FILE_INCOMPATIBLE_HARDLINKS: WIN32_ERROR = WIN32_ERROR(396u32);
pub const ERROR_CLOUD_FILE_INSUFFICIENT_RESOURCES: WIN32_ERROR = WIN32_ERROR(387u32);
pub const ERROR_CLOUD_FILE_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(380u32);
pub const ERROR_CLOUD_FILE_IN_USE: WIN32_ERROR = WIN32_ERROR(391u32);
pub const ERROR_CLOUD_FILE_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(363u32);
pub const ERROR_CLOUD_FILE_METADATA_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(364u32);
pub const ERROR_CLOUD_FILE_NETWORK_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(388u32);
pub const ERROR_CLOUD_FILE_NOT_IN_SYNC: WIN32_ERROR = WIN32_ERROR(377u32);
pub const ERROR_CLOUD_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(379u32);
pub const ERROR_CLOUD_FILE_NOT_UNDER_SYNC_ROOT: WIN32_ERROR = WIN32_ERROR(390u32);
pub const ERROR_CLOUD_FILE_PINNED: WIN32_ERROR = WIN32_ERROR(392u32);
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH: WIN32_ERROR = WIN32_ERROR(366u32);
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(365u32);
pub const ERROR_CLOUD_FILE_PROPERTY_CORRUPT: WIN32_ERROR = WIN32_ERROR(394u32);
pub const ERROR_CLOUD_FILE_PROPERTY_LOCK_CONFLICT: WIN32_ERROR = WIN32_ERROR(397u32);
pub const ERROR_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(375u32);
pub const ERROR_CLOUD_FILE_PROVIDER_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(362u32);
pub const ERROR_CLOUD_FILE_PROVIDER_TERMINATED: WIN32_ERROR = WIN32_ERROR(404u32);
pub const ERROR_CLOUD_FILE_READ_ONLY_VOLUME: WIN32_ERROR = WIN32_ERROR(381u32);
pub const ERROR_CLOUD_FILE_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(393u32);
pub const ERROR_CLOUD_FILE_REQUEST_CANCELED: WIN32_ERROR = WIN32_ERROR(398u32);
pub const ERROR_CLOUD_FILE_REQUEST_TIMEOUT: WIN32_ERROR = WIN32_ERROR(426u32);
pub const ERROR_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(358u32);
pub const ERROR_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS: WIN32_ERROR = WIN32_ERROR(374u32);
pub const ERROR_CLOUD_FILE_UNSUCCESSFUL: WIN32_ERROR = WIN32_ERROR(389u32);
pub const ERROR_CLOUD_FILE_US_MESSAGE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(475u32);
pub const ERROR_CLOUD_FILE_VALIDATION_FAILED: WIN32_ERROR = WIN32_ERROR(383u32);
pub const ERROR_CLUSCFG_ALREADY_COMMITTED: WIN32_ERROR = WIN32_ERROR(5901u32);
pub const ERROR_CLUSCFG_ROLLBACK_FAILED: WIN32_ERROR = WIN32_ERROR(5902u32);
pub const ERROR_CLUSCFG_SYSTEM_DISK_DRIVE_LETTER_CONFLICT: WIN32_ERROR = WIN32_ERROR(5903u32);
pub const ERROR_CLUSTERLOG_CHKPOINT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5032u32);
pub const ERROR_CLUSTERLOG_CORRUPT: WIN32_ERROR = WIN32_ERROR(5029u32);
pub const ERROR_CLUSTERLOG_EXCEEDS_MAXSIZE: WIN32_ERROR = WIN32_ERROR(5031u32);
pub const ERROR_CLUSTERLOG_NOT_ENOUGH_SPACE: WIN32_ERROR = WIN32_ERROR(5033u32);
pub const ERROR_CLUSTERLOG_RECORD_EXCEEDS_MAXSIZE: WIN32_ERROR = WIN32_ERROR(5030u32);
pub const ERROR_CLUSTERSET_MANAGEMENT_CLUSTER_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(5999u32);
pub const ERROR_CLUSTER_AFFINITY_CONFLICT: WIN32_ERROR = WIN32_ERROR(5971u32);
pub const ERROR_CLUSTER_BACKUP_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5949u32);
pub const ERROR_CLUSTER_CANNOT_RETURN_PROPERTIES: WIN32_ERROR = WIN32_ERROR(5968u32);
pub const ERROR_CLUSTER_CANT_CREATE_DUP_CLUSTER_NAME: WIN32_ERROR = WIN32_ERROR(5900u32);
pub const ERROR_CLUSTER_CANT_DESERIALIZE_DATA: WIN32_ERROR = WIN32_ERROR(5923u32);
pub const ERROR_CLUSTER_CSV_INVALID_HANDLE: WIN32_ERROR = WIN32_ERROR(5989u32);
pub const ERROR_CLUSTER_CSV_IO_PAUSE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(5979u32);
pub const ERROR_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR: WIN32_ERROR = WIN32_ERROR(5990u32);
pub const ERROR_CLUSTER_DATABASE_SEQMISMATCH: WIN32_ERROR = WIN32_ERROR(5083u32);
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5918u32);
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_NOT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5919u32);
pub const ERROR_CLUSTER_DATABASE_UPDATE_CONDITION_FAILED: WIN32_ERROR = WIN32_ERROR(5986u32);
pub const ERROR_CLUSTER_DISK_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(5963u32);
pub const ERROR_CLUSTER_EVICT_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(5939u32);
pub const ERROR_CLUSTER_EVICT_WITHOUT_CLEANUP: WIN32_ERROR = WIN32_ERROR(5896u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_FAILED_S2D_VALIDATION: WIN32_ERROR = WIN32_ERROR(5996u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_INVALID_HIERARCHY: WIN32_ERROR = WIN32_ERROR(5995u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_PARENT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5994u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_S2D_CONNECTIVITY_LOSS: WIN32_ERROR = WIN32_ERROR(5997u32);
pub const ERROR_CLUSTER_GROUP_BUSY: WIN32_ERROR = WIN32_ERROR(5944u32);
pub const ERROR_CLUSTER_GROUP_MOVING: WIN32_ERROR = WIN32_ERROR(5908u32);
pub const ERROR_CLUSTER_GROUP_QUEUED: WIN32_ERROR = WIN32_ERROR(5959u32);
pub const ERROR_CLUSTER_GROUP_SINGLETON_RESOURCE: WIN32_ERROR = WIN32_ERROR(5941u32);
pub const ERROR_CLUSTER_GUM_NOT_LOCKER: WIN32_ERROR = WIN32_ERROR(5085u32);
pub const ERROR_CLUSTER_INCOMPATIBLE_VERSIONS: WIN32_ERROR = WIN32_ERROR(5075u32);
pub const ERROR_CLUSTER_INSTANCE_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(5893u32);
pub const ERROR_CLUSTER_INTERNAL_INVALID_FUNCTION: WIN32_ERROR = WIN32_ERROR(5912u32);
pub const ERROR_CLUSTER_INVALID_INFRASTRUCTURE_FILESERVER_NAME: WIN32_ERROR = WIN32_ERROR(5998u32);
pub const ERROR_CLUSTER_INVALID_IPV6_NETWORK: WIN32_ERROR = WIN32_ERROR(5926u32);
pub const ERROR_CLUSTER_INVALID_IPV6_TUNNEL_NETWORK: WIN32_ERROR = WIN32_ERROR(5927u32);
pub const ERROR_CLUSTER_INVALID_NETWORK: WIN32_ERROR = WIN32_ERROR(5054u32);
pub const ERROR_CLUSTER_INVALID_NETWORK_PROVIDER: WIN32_ERROR = WIN32_ERROR(5049u32);
pub const ERROR_CLUSTER_INVALID_NODE: WIN32_ERROR = WIN32_ERROR(5039u32);
pub const ERROR_CLUSTER_INVALID_NODE_WEIGHT: WIN32_ERROR = WIN32_ERROR(5954u32);
pub const ERROR_CLUSTER_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(5048u32);
pub const ERROR_CLUSTER_INVALID_SECURITY_DESCRIPTOR: WIN32_ERROR = WIN32_ERROR(5946u32);
pub const ERROR_CLUSTER_INVALID_STRING_FORMAT: WIN32_ERROR = WIN32_ERROR(5917u32);
pub const ERROR_CLUSTER_INVALID_STRING_TERMINATION: WIN32_ERROR = WIN32_ERROR(5916u32);
pub const ERROR_CLUSTER_IPADDR_IN_USE: WIN32_ERROR = WIN32_ERROR(5057u32);
pub const ERROR_CLUSTER_JOIN_ABORTED: WIN32_ERROR = WIN32_ERROR(5074u32);
pub const ERROR_CLUSTER_JOIN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5041u32);
pub const ERROR_CLUSTER_JOIN_NOT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5053u32);
pub const ERROR_CLUSTER_LAST_INTERNAL_NETWORK: WIN32_ERROR = WIN32_ERROR(5066u32);
pub const ERROR_CLUSTER_LOCAL_NODE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5043u32);
pub const ERROR_CLUSTER_MAXNUM_OF_RESOURCES_EXCEEDED: WIN32_ERROR = WIN32_ERROR(5076u32);
pub const ERROR_CLUSTER_MAX_NODES_IN_CLUSTER: WIN32_ERROR = WIN32_ERROR(5934u32);
pub const ERROR_CLUSTER_MEMBERSHIP_HALT: WIN32_ERROR = WIN32_ERROR(5892u32);
pub const ERROR_CLUSTER_MEMBERSHIP_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5890u32);
pub const ERROR_CLUSTER_MISMATCHED_COMPUTER_ACCT_NAME: WIN32_ERROR = WIN32_ERROR(5905u32);
pub const ERROR_CLUSTER_NETINTERFACE_EXISTS: WIN32_ERROR = WIN32_ERROR(5046u32);
pub const ERROR_CLUSTER_NETINTERFACE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5047u32);
pub const ERROR_CLUSTER_NETWORK_ALREADY_OFFLINE: WIN32_ERROR = WIN32_ERROR(5064u32);
pub const ERROR_CLUSTER_NETWORK_ALREADY_ONLINE: WIN32_ERROR = WIN32_ERROR(5063u32);
pub const ERROR_CLUSTER_NETWORK_EXISTS: WIN32_ERROR = WIN32_ERROR(5044u32);
pub const ERROR_CLUSTER_NETWORK_HAS_DEPENDENTS: WIN32_ERROR = WIN32_ERROR(5067u32);
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5045u32);
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND_FOR_IP: WIN32_ERROR = WIN32_ERROR(5894u32);
pub const ERROR_CLUSTER_NETWORK_NOT_INTERNAL: WIN32_ERROR = WIN32_ERROR(5060u32);
pub const ERROR_CLUSTER_NODE_ALREADY_DOWN: WIN32_ERROR = WIN32_ERROR(5062u32);
pub const ERROR_CLUSTER_NODE_ALREADY_HAS_DFS_ROOT: WIN32_ERROR = WIN32_ERROR(5088u32);
pub const ERROR_CLUSTER_NODE_ALREADY_MEMBER: WIN32_ERROR = WIN32_ERROR(5065u32);
pub const ERROR_CLUSTER_NODE_ALREADY_UP: WIN32_ERROR = WIN32_ERROR(5061u32);
pub const ERROR_CLUSTER_NODE_DOWN: WIN32_ERROR = WIN32_ERROR(5050u32);
pub const ERROR_CLUSTER_NODE_DRAIN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5962u32);
pub const ERROR_CLUSTER_NODE_EXISTS: WIN32_ERROR = WIN32_ERROR(5040u32);
pub const ERROR_CLUSTER_NODE_IN_GRACE_PERIOD: WIN32_ERROR = WIN32_ERROR(5978u32);
pub const ERROR_CLUSTER_NODE_ISOLATED: WIN32_ERROR = WIN32_ERROR(5984u32);
pub const ERROR_CLUSTER_NODE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5042u32);
pub const ERROR_CLUSTER_NODE_NOT_MEMBER: WIN32_ERROR = WIN32_ERROR(5052u32);
pub const ERROR_CLUSTER_NODE_NOT_PAUSED: WIN32_ERROR = WIN32_ERROR(5058u32);
pub const ERROR_CLUSTER_NODE_NOT_READY: WIN32_ERROR = WIN32_ERROR(5072u32);
pub const ERROR_CLUSTER_NODE_PAUSED: WIN32_ERROR = WIN32_ERROR(5070u32);
pub const ERROR_CLUSTER_NODE_QUARANTINED: WIN32_ERROR = WIN32_ERROR(5985u32);
pub const ERROR_CLUSTER_NODE_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(5073u32);
pub const ERROR_CLUSTER_NODE_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(5051u32);
pub const ERROR_CLUSTER_NODE_UP: WIN32_ERROR = WIN32_ERROR(5056u32);
pub const ERROR_CLUSTER_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(5932u32);
pub const ERROR_CLUSTER_NOT_SHARED_VOLUME: WIN32_ERROR = WIN32_ERROR(5945u32);
pub const ERROR_CLUSTER_NO_NET_ADAPTERS: WIN32_ERROR = WIN32_ERROR(5906u32);
pub const ERROR_CLUSTER_NO_QUORUM: WIN32_ERROR = WIN32_ERROR(5925u32);
pub const ERROR_CLUSTER_NO_RPC_PACKAGES_REGISTERED: WIN32_ERROR = WIN32_ERROR(5081u32);
pub const ERROR_CLUSTER_NO_SECURITY_CONTEXT: WIN32_ERROR = WIN32_ERROR(5059u32);
pub const ERROR_CLUSTER_NULL_DATA: WIN32_ERROR = WIN32_ERROR(5920u32);
pub const ERROR_CLUSTER_OBJECT_ALREADY_USED: WIN32_ERROR = WIN32_ERROR(5936u32);
pub const ERROR_CLUSTER_OBJECT_IS_CLUSTER_SET_VM: WIN32_ERROR = WIN32_ERROR(6250u32);
pub const ERROR_CLUSTER_OLD_VERSION: WIN32_ERROR = WIN32_ERROR(5904u32);
pub const ERROR_CLUSTER_OWNER_NOT_IN_PREFLIST: WIN32_ERROR = WIN32_ERROR(5082u32);
pub const ERROR_CLUSTER_PARAMETER_MISMATCH: WIN32_ERROR = WIN32_ERROR(5897u32);
pub const ERROR_CLUSTER_PARAMETER_OUT_OF_BOUNDS: WIN32_ERROR = WIN32_ERROR(5913u32);
pub const ERROR_CLUSTER_PARTIAL_READ: WIN32_ERROR = WIN32_ERROR(5921u32);
pub const ERROR_CLUSTER_PARTIAL_SEND: WIN32_ERROR = WIN32_ERROR(5914u32);
pub const ERROR_CLUSTER_PARTIAL_WRITE: WIN32_ERROR = WIN32_ERROR(5922u32);
pub const ERROR_CLUSTER_POISONED: WIN32_ERROR = WIN32_ERROR(5907u32);
pub const ERROR_CLUSTER_PROPERTY_DATA_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(5895u32);
pub const ERROR_CLUSTER_QUORUMLOG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5891u32);
pub const ERROR_CLUSTER_REGISTRY_INVALID_FUNCTION: WIN32_ERROR = WIN32_ERROR(5915u32);
pub const ERROR_CLUSTER_RESNAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5080u32);
pub const ERROR_CLUSTER_RESOURCES_MUST_BE_ONLINE_ON_THE_SAME_NODE: WIN32_ERROR = WIN32_ERROR(5933u32);
pub const ERROR_CLUSTER_RESOURCE_CONFIGURATION_ERROR: WIN32_ERROR = WIN32_ERROR(5943u32);
pub const ERROR_CLUSTER_RESOURCE_CONTAINS_UNSUPPORTED_DIFF_AREA_FOR_SHARED_VOLUMES: WIN32_ERROR = WIN32_ERROR(5969u32);
pub const ERROR_CLUSTER_RESOURCE_DOES_NOT_SUPPORT_UNMONITORED: WIN32_ERROR = WIN32_ERROR(5982u32);
pub const ERROR_CLUSTER_RESOURCE_IS_IN_MAINTENANCE_MODE: WIN32_ERROR = WIN32_ERROR(5970u32);
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICATED: WIN32_ERROR = WIN32_ERROR(5983u32);
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICA_VIRTUAL_MACHINE: WIN32_ERROR = WIN32_ERROR(5972u32);
pub const ERROR_CLUSTER_RESOURCE_LOCKED_STATUS: WIN32_ERROR = WIN32_ERROR(5960u32);
pub const ERROR_CLUSTER_RESOURCE_NOT_MONITORED: WIN32_ERROR = WIN32_ERROR(5981u32);
pub const ERROR_CLUSTER_RESOURCE_PROVIDER_FAILED: WIN32_ERROR = WIN32_ERROR(5942u32);
pub const ERROR_CLUSTER_RESOURCE_TYPE_BUSY: WIN32_ERROR = WIN32_ERROR(5909u32);
pub const ERROR_CLUSTER_RESOURCE_TYPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5078u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_CALL: WIN32_ERROR = WIN32_ERROR(5955u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_INCOMPATIBLE_NODES: WIN32_ERROR = WIN32_ERROR(5953u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_DESTINATION: WIN32_ERROR = WIN32_ERROR(5957u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_SOURCE: WIN32_ERROR = WIN32_ERROR(5958u32);
pub const ERROR_CLUSTER_RESTYPE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(5079u32);
pub const ERROR_CLUSTER_RHS_FAILED_INITIALIZATION: WIN32_ERROR = WIN32_ERROR(5931u32);
pub const ERROR_CLUSTER_SHARED_VOLUMES_IN_USE: WIN32_ERROR = WIN32_ERROR(5947u32);
pub const ERROR_CLUSTER_SHARED_VOLUME_FAILOVER_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(5961u32);
pub const ERROR_CLUSTER_SHARED_VOLUME_NOT_REDIRECTED: WIN32_ERROR = WIN32_ERROR(5967u32);
pub const ERROR_CLUSTER_SHARED_VOLUME_REDIRECTED: WIN32_ERROR = WIN32_ERROR(5966u32);
pub const ERROR_CLUSTER_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(5022u32);
pub const ERROR_CLUSTER_SINGLETON_RESOURCE: WIN32_ERROR = WIN32_ERROR(5940u32);
pub const ERROR_CLUSTER_SPACE_DEGRADED: WIN32_ERROR = WIN32_ERROR(5987u32);
pub const ERROR_CLUSTER_SYSTEM_CONFIG_CHANGED: WIN32_ERROR = WIN32_ERROR(5077u32);
pub const ERROR_CLUSTER_TOKEN_DELEGATION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(5988u32);
pub const ERROR_CLUSTER_TOO_MANY_NODES: WIN32_ERROR = WIN32_ERROR(5935u32);
pub const ERROR_CLUSTER_UPGRADE_FIX_QUORUM_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(5974u32);
pub const ERROR_CLUSTER_UPGRADE_INCOMPATIBLE_VERSIONS: WIN32_ERROR = WIN32_ERROR(5973u32);
pub const ERROR_CLUSTER_UPGRADE_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(5977u32);
pub const ERROR_CLUSTER_UPGRADE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5976u32);
pub const ERROR_CLUSTER_UPGRADE_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(5975u32);
pub const ERROR_CLUSTER_USE_SHARED_VOLUMES_API: WIN32_ERROR = WIN32_ERROR(5948u32);
pub const ERROR_CLUSTER_WATCHDOG_TERMINATING: WIN32_ERROR = WIN32_ERROR(5952u32);
pub const ERROR_CLUSTER_WRONG_OS_VERSION: WIN32_ERROR = WIN32_ERROR(5899u32);
pub const ERROR_COLORSPACE_MISMATCH: WIN32_ERROR = WIN32_ERROR(2021u32);
pub const ERROR_COMMITMENT_LIMIT: WIN32_ERROR = WIN32_ERROR(1455u32);
pub const ERROR_COMMITMENT_MINIMUM: WIN32_ERROR = WIN32_ERROR(635u32);
pub const ERROR_COMPRESSED_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(335u32);
pub const ERROR_COMPRESSION_DISABLED: WIN32_ERROR = WIN32_ERROR(769u32);
pub const ERROR_COMPRESSION_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6850u32);
pub const ERROR_COMPRESSION_NOT_BENEFICIAL: WIN32_ERROR = WIN32_ERROR(344u32);
pub const ERROR_COM_TASK_STOP_PENDING: WIN32_ERROR = WIN32_ERROR(15501u32);
pub const ERROR_CONNECTED_OTHER_PASSWORD: WIN32_ERROR = WIN32_ERROR(2108u32);
pub const ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT: WIN32_ERROR = WIN32_ERROR(2109u32);
pub const ERROR_CONNECTION_ABORTED: WIN32_ERROR = WIN32_ERROR(1236u32);
pub const ERROR_CONNECTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(1230u32);
pub const ERROR_CONNECTION_COUNT_LIMIT: WIN32_ERROR = WIN32_ERROR(1238u32);
pub const ERROR_CONNECTION_INVALID: WIN32_ERROR = WIN32_ERROR(1229u32);
pub const ERROR_CONNECTION_REFUSED: WIN32_ERROR = WIN32_ERROR(1225u32);
pub const ERROR_CONNECTION_UNAVAIL: WIN32_ERROR = WIN32_ERROR(1201u32);
pub const ERROR_CONTAINER_ASSIGNED: WIN32_ERROR = WIN32_ERROR(1504u32);
pub const ERROR_CONTENT_BLOCKED: WIN32_ERROR = WIN32_ERROR(1296u32);
pub const ERROR_CONTEXT_EXPIRED: WIN32_ERROR = WIN32_ERROR(1931u32);
pub const ERROR_CONTINUE: WIN32_ERROR = WIN32_ERROR(1246u32);
pub const ERROR_CONTROLLING_IEPORT: WIN32_ERROR = WIN32_ERROR(4329u32);
pub const ERROR_CONTROL_C_EXIT: WIN32_ERROR = WIN32_ERROR(572u32);
pub const ERROR_CONTROL_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1421u32);
pub const ERROR_CONVERT_TO_LARGE: WIN32_ERROR = WIN32_ERROR(600u32);
pub const ERROR_CORE_DRIVER_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3016u32);
pub const ERROR_CORE_RESOURCE: WIN32_ERROR = WIN32_ERROR(5026u32);
pub const ERROR_CORRUPT_LOG_CLEARED: WIN32_ERROR = WIN32_ERROR(798u32);
pub const ERROR_CORRUPT_LOG_CORRUPTED: WIN32_ERROR = WIN32_ERROR(795u32);
pub const ERROR_CORRUPT_LOG_DELETED_FULL: WIN32_ERROR = WIN32_ERROR(797u32);
pub const ERROR_CORRUPT_LOG_OVERFULL: WIN32_ERROR = WIN32_ERROR(794u32);
pub const ERROR_CORRUPT_LOG_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(796u32);
pub const ERROR_CORRUPT_SYSTEM_FILE: WIN32_ERROR = WIN32_ERROR(634u32);
pub const ERROR_COULD_NOT_INTERPRET: WIN32_ERROR = WIN32_ERROR(552u32);
pub const ERROR_COULD_NOT_RESIZE_LOG: WIN32_ERROR = WIN32_ERROR(6629u32);
pub const ERROR_COUNTER_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1121u32);
pub const ERROR_CPU_SET_INVALID: WIN32_ERROR = WIN32_ERROR(813u32);
pub const ERROR_CRASH_DUMP: WIN32_ERROR = WIN32_ERROR(753u32);
pub const ERROR_CRC: WIN32_ERROR = WIN32_ERROR(23u32);
pub const ERROR_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(1631u32);
pub const ERROR_CRED_REQUIRES_CONFIRMATION: windows_core::HRESULT = windows_core::HRESULT(0x80097019_u32 as _);
pub const ERROR_CRM_PROTOCOL_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(6710u32);
pub const ERROR_CRM_PROTOCOL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6712u32);
pub const ERROR_CROSS_PARTITION_VIOLATION: WIN32_ERROR = WIN32_ERROR(1661u32);
pub const ERROR_CSCSHARE_OFFLINE: WIN32_ERROR = WIN32_ERROR(1262u32);
pub const ERROR_CSV_VOLUME_NOT_LOCAL: WIN32_ERROR = WIN32_ERROR(5951u32);
pub const ERROR_CS_ENCRYPTION_EXISTING_ENCRYPTED_FILE: WIN32_ERROR = WIN32_ERROR(6019u32);
pub const ERROR_CS_ENCRYPTION_FILE_NOT_CSE: WIN32_ERROR = WIN32_ERROR(6021u32);
pub const ERROR_CS_ENCRYPTION_INVALID_SERVER_RESPONSE: WIN32_ERROR = WIN32_ERROR(6017u32);
pub const ERROR_CS_ENCRYPTION_NEW_ENCRYPTED_FILE: WIN32_ERROR = WIN32_ERROR(6020u32);
pub const ERROR_CS_ENCRYPTION_UNSUPPORTED_SERVER: WIN32_ERROR = WIN32_ERROR(6018u32);
pub const ERROR_CTLOG_INCONSISTENT_TRACKING_FILE: WIN32_ERROR = WIN32_ERROR(3225026596u32);
pub const ERROR_CTLOG_INVALID_TRACKING_STATE: WIN32_ERROR = WIN32_ERROR(3225026595u32);
pub const ERROR_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE: WIN32_ERROR = WIN32_ERROR(3225026593u32);
pub const ERROR_CTLOG_TRACKING_NOT_INITIALIZED: WIN32_ERROR = WIN32_ERROR(3225026592u32);
pub const ERROR_CTLOG_VHD_CHANGED_OFFLINE: WIN32_ERROR = WIN32_ERROR(3225026594u32);
pub const ERROR_CTX_ACCOUNT_RESTRICTION: WIN32_ERROR = WIN32_ERROR(7064u32);
pub const ERROR_CTX_BAD_VIDEO_MODE: WIN32_ERROR = WIN32_ERROR(7025u32);
pub const ERROR_CTX_CANNOT_MAKE_EVENTLOG_ENTRY: WIN32_ERROR = WIN32_ERROR(7005u32);
pub const ERROR_CTX_CDM_CONNECT: WIN32_ERROR = WIN32_ERROR(7066u32);
pub const ERROR_CTX_CDM_DISCONNECT: WIN32_ERROR = WIN32_ERROR(7067u32);
pub const ERROR_CTX_CLIENT_LICENSE_IN_USE: WIN32_ERROR = WIN32_ERROR(7052u32);
pub const ERROR_CTX_CLIENT_LICENSE_NOT_SET: WIN32_ERROR = WIN32_ERROR(7053u32);
pub const ERROR_CTX_CLIENT_QUERY_TIMEOUT: WIN32_ERROR = WIN32_ERROR(7040u32);
pub const ERROR_CTX_CLOSE_PENDING: WIN32_ERROR = WIN32_ERROR(7007u32);
pub const ERROR_CTX_CONSOLE_CONNECT: WIN32_ERROR = WIN32_ERROR(7042u32);
pub const ERROR_CTX_CONSOLE_DISCONNECT: WIN32_ERROR = WIN32_ERROR(7041u32);
pub const ERROR_CTX_ENCRYPTION_LEVEL_REQUIRED: WIN32_ERROR = WIN32_ERROR(7061u32);
pub const ERROR_CTX_GRAPHICS_INVALID: WIN32_ERROR = WIN32_ERROR(7035u32);
pub const ERROR_CTX_INVALID_MODEMNAME: WIN32_ERROR = WIN32_ERROR(7010u32);
pub const ERROR_CTX_INVALID_PD: WIN32_ERROR = WIN32_ERROR(7002u32);
pub const ERROR_CTX_INVALID_WD: WIN32_ERROR = WIN32_ERROR(7049u32);
pub const ERROR_CTX_LICENSE_CLIENT_INVALID: WIN32_ERROR = WIN32_ERROR(7055u32);
pub const ERROR_CTX_LICENSE_EXPIRED: WIN32_ERROR = WIN32_ERROR(7056u32);
pub const ERROR_CTX_LICENSE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(7054u32);
pub const ERROR_CTX_LOGON_DISABLED: WIN32_ERROR = WIN32_ERROR(7037u32);
pub const ERROR_CTX_MODEM_INF_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7009u32);
pub const ERROR_CTX_MODEM_RESPONSE_BUSY: WIN32_ERROR = WIN32_ERROR(7015u32);
pub const ERROR_CTX_MODEM_RESPONSE_ERROR: WIN32_ERROR = WIN32_ERROR(7011u32);
pub const ERROR_CTX_MODEM_RESPONSE_NO_CARRIER: WIN32_ERROR = WIN32_ERROR(7013u32);
pub const ERROR_CTX_MODEM_RESPONSE_NO_DIALTONE: WIN32_ERROR = WIN32_ERROR(7014u32);
pub const ERROR_CTX_MODEM_RESPONSE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(7012u32);
pub const ERROR_CTX_MODEM_RESPONSE_VOICE: WIN32_ERROR = WIN32_ERROR(7016u32);
pub const ERROR_CTX_NOT_CONSOLE: WIN32_ERROR = WIN32_ERROR(7038u32);
pub const ERROR_CTX_NO_FORCE_LOGOFF: WIN32_ERROR = WIN32_ERROR(7063u32);
pub const ERROR_CTX_NO_OUTBUF: WIN32_ERROR = WIN32_ERROR(7008u32);
pub const ERROR_CTX_PD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7003u32);
pub const ERROR_CTX_SECURITY_LAYER_ERROR: WIN32_ERROR = WIN32_ERROR(7068u32);
pub const ERROR_CTX_SERVICE_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(7006u32);
pub const ERROR_CTX_SESSION_IN_USE: WIN32_ERROR = WIN32_ERROR(7062u32);
pub const ERROR_CTX_SHADOW_DENIED: WIN32_ERROR = WIN32_ERROR(7044u32);
pub const ERROR_CTX_SHADOW_DISABLED: WIN32_ERROR = WIN32_ERROR(7051u32);
pub const ERROR_CTX_SHADOW_ENDED_BY_MODE_CHANGE: WIN32_ERROR = WIN32_ERROR(7058u32);
pub const ERROR_CTX_SHADOW_INVALID: WIN32_ERROR = WIN32_ERROR(7050u32);
pub const ERROR_CTX_SHADOW_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(7057u32);
pub const ERROR_CTX_TD_ERROR: WIN32_ERROR = WIN32_ERROR(7017u32);
pub const ERROR_CTX_WD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7004u32);
pub const ERROR_CTX_WINSTATIONS_DISABLED: WIN32_ERROR = WIN32_ERROR(7060u32);
pub const ERROR_CTX_WINSTATION_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(7045u32);
pub const ERROR_CTX_WINSTATION_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(7023u32);
pub const ERROR_CTX_WINSTATION_BUSY: WIN32_ERROR = WIN32_ERROR(7024u32);
pub const ERROR_CTX_WINSTATION_NAME_INVALID: WIN32_ERROR = WIN32_ERROR(7001u32);
pub const ERROR_CTX_WINSTATION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7022u32);
pub const ERROR_CURRENT_DIRECTORY: WIN32_ERROR = WIN32_ERROR(16u32);
pub const ERROR_CURRENT_DOMAIN_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(1399u32);
pub const ERROR_CURRENT_TRANSACTION_NOT_VALID: WIN32_ERROR = WIN32_ERROR(6714u32);
pub const ERROR_DATABASE_BACKUP_CORRUPT: WIN32_ERROR = WIN32_ERROR(5087u32);
pub const ERROR_DATABASE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1065u32);
pub const ERROR_DATABASE_FAILURE: WIN32_ERROR = WIN32_ERROR(4313u32);
pub const ERROR_DATABASE_FULL: WIN32_ERROR = WIN32_ERROR(4314u32);
pub const ERROR_DATATYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(1629u32);
pub const ERROR_DATA_CHECKSUM_ERROR: WIN32_ERROR = WIN32_ERROR(323u32);
pub const ERROR_DATA_LOST_REPAIR: WIN32_ERROR = WIN32_ERROR(6843u32);
pub const ERROR_DATA_NOT_ACCEPTED: WIN32_ERROR = WIN32_ERROR(592u32);
pub const ERROR_DAX_MAPPING_EXISTS: WIN32_ERROR = WIN32_ERROR(361u32);
pub const ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80B00002_u32 as _);
pub const ERROR_DBG_COMMAND_EXCEPTION: WIN32_ERROR = WIN32_ERROR(697u32);
pub const ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80B00003_u32 as _);
pub const ERROR_DBG_CONTINUE: WIN32_ERROR = WIN32_ERROR(767u32);
pub const ERROR_DBG_CONTROL_BREAK: WIN32_ERROR = WIN32_ERROR(696u32);
pub const ERROR_DBG_CONTROL_C: WIN32_ERROR = WIN32_ERROR(693u32);
pub const ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80B00001_u32 as _);
pub const ERROR_DBG_EXCEPTION_HANDLED: WIN32_ERROR = WIN32_ERROR(766u32);
pub const ERROR_DBG_EXCEPTION_NOT_HANDLED: WIN32_ERROR = WIN32_ERROR(688u32);
pub const ERROR_DBG_PRINTEXCEPTION_C: WIN32_ERROR = WIN32_ERROR(694u32);
pub const ERROR_DBG_REPLY_LATER: WIN32_ERROR = WIN32_ERROR(689u32);
pub const ERROR_DBG_RIPEXCEPTION: WIN32_ERROR = WIN32_ERROR(695u32);
pub const ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80B00004_u32 as _);
pub const ERROR_DBG_TERMINATE_PROCESS: WIN32_ERROR = WIN32_ERROR(692u32);
pub const ERROR_DBG_TERMINATE_THREAD: WIN32_ERROR = WIN32_ERROR(691u32);
pub const ERROR_DBG_UNABLE_TO_PROVIDE_HANDLE: WIN32_ERROR = WIN32_ERROR(690u32);
pub const ERROR_DC_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1425u32);
pub const ERROR_DDE_FAIL: WIN32_ERROR = WIN32_ERROR(1156u32);
pub const ERROR_DDM_NOT_RUNNING: u32 = 903u32;
pub const ERROR_DEBUGGER_INACTIVE: WIN32_ERROR = WIN32_ERROR(1284u32);
pub const ERROR_DEBUG_ATTACH_FAILED: WIN32_ERROR = WIN32_ERROR(590u32);
pub const ERROR_DECRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6001u32);
pub const ERROR_DELAY_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(1285u32);
pub const ERROR_DELETE_PENDING: WIN32_ERROR = WIN32_ERROR(303u32);
pub const ERROR_DELETING_EXISTING_APPLICATIONDATA_STORE_FAILED: WIN32_ERROR = WIN32_ERROR(15621u32);
pub const ERROR_DELETING_ICM_XFORM: WIN32_ERROR = WIN32_ERROR(2019u32);
pub const ERROR_DEPENDENCY_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(5003u32);
pub const ERROR_DEPENDENCY_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(5069u32);
pub const ERROR_DEPENDENCY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5002u32);
pub const ERROR_DEPENDENCY_TREE_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(5929u32);
pub const ERROR_DEPENDENT_RESOURCE_EXISTS: WIN32_ERROR = WIN32_ERROR(5001u32);
pub const ERROR_DEPENDENT_RESOURCE_PROPERTY_CONFLICT: WIN32_ERROR = WIN32_ERROR(5924u32);
pub const ERROR_DEPENDENT_SERVICES_RUNNING: WIN32_ERROR = WIN32_ERROR(1051u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_POLICY: WIN32_ERROR = WIN32_ERROR(15617u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_PROFILE_POLICY: WIN32_ERROR = WIN32_ERROR(15651u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_USER_LOG_OFF: WIN32_ERROR = WIN32_ERROR(15641u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_MACHINE: WIN32_ERROR = WIN32_ERROR(15650u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_PACKAGE: WIN32_ERROR = WIN32_ERROR(15649u32);
pub const ERROR_DEPLOYMENT_FAILED_CONFLICTING_MUTABLE_PACKAGE_DIRECTORY: WIN32_ERROR = WIN32_ERROR(15652u32);
pub const ERROR_DEPLOYMENT_OPTION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15645u32);
pub const ERROR_DESTINATION_ELEMENT_FULL: WIN32_ERROR = WIN32_ERROR(1161u32);
pub const ERROR_DESTROY_OBJECT_OF_OTHER_THREAD: WIN32_ERROR = WIN32_ERROR(1435u32);
pub const ERROR_DEVICE_ALREADY_ATTACHED: WIN32_ERROR = WIN32_ERROR(548u32);
pub const ERROR_DEVICE_ALREADY_REMEMBERED: WIN32_ERROR = WIN32_ERROR(1202u32);
pub const ERROR_DEVICE_DOOR_OPEN: WIN32_ERROR = WIN32_ERROR(1166u32);
pub const ERROR_DEVICE_ENUMERATION_ERROR: WIN32_ERROR = WIN32_ERROR(648u32);
pub const ERROR_DEVICE_FEATURE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(316u32);
pub const ERROR_DEVICE_HARDWARE_ERROR: WIN32_ERROR = WIN32_ERROR(483u32);
pub const ERROR_DEVICE_HINT_NAME_BUFFER_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(355u32);
pub const ERROR_DEVICE_INSTALLER_NOT_READY: WIN32_ERROR = WIN32_ERROR(3758096966u32);
pub const ERROR_DEVICE_INSTALL_BLOCKED: WIN32_ERROR = WIN32_ERROR(3758096968u32);
pub const ERROR_DEVICE_INTERFACE_ACTIVE: WIN32_ERROR = WIN32_ERROR(3758096923u32);
pub const ERROR_DEVICE_INTERFACE_REMOVED: WIN32_ERROR = WIN32_ERROR(3758096924u32);
pub const ERROR_DEVICE_IN_MAINTENANCE: WIN32_ERROR = WIN32_ERROR(359u32);
pub const ERROR_DEVICE_IN_USE: WIN32_ERROR = WIN32_ERROR(2404u32);
pub const ERROR_DEVICE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(4319u32);
pub const ERROR_DEVICE_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(1167u32);
pub const ERROR_DEVICE_NOT_PARTITIONED: WIN32_ERROR = WIN32_ERROR(1107u32);
pub const ERROR_DEVICE_NO_RESOURCES: WIN32_ERROR = WIN32_ERROR(322u32);
pub const ERROR_DEVICE_REINITIALIZATION_NEEDED: WIN32_ERROR = WIN32_ERROR(1164u32);
pub const ERROR_DEVICE_REMOVED: WIN32_ERROR = WIN32_ERROR(1617u32);
pub const ERROR_DEVICE_REQUIRES_CLEANING: WIN32_ERROR = WIN32_ERROR(1165u32);
pub const ERROR_DEVICE_RESET_REQUIRED: WIN32_ERROR = WIN32_ERROR(507u32);
pub const ERROR_DEVICE_SUPPORT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(171u32);
pub const ERROR_DEVICE_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(321u32);
pub const ERROR_DEVINFO_DATA_LOCKED: WIN32_ERROR = WIN32_ERROR(3758096915u32);
pub const ERROR_DEVINFO_LIST_LOCKED: WIN32_ERROR = WIN32_ERROR(3758096914u32);
pub const ERROR_DEVINFO_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(3758096904u32);
pub const ERROR_DEVINSTALL_QUEUE_NONNATIVE: WIN32_ERROR = WIN32_ERROR(3758096944u32);
pub const ERROR_DEVINST_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(3758096903u32);
pub const ERROR_DEV_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(55u32);
pub const ERROR_DEV_SIDELOAD_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15633u32);
pub const ERROR_DHCP_ADDRESS_CONFLICT: WIN32_ERROR = WIN32_ERROR(4100u32);
pub const ERROR_DIALIN_HOURS_RESTRICTION: u32 = 940u32;
pub const ERROR_DIALOUT_HOURS_RESTRICTION: u32 = 944u32;
pub const ERROR_DIFFERENT_PROFILE_RESOURCE_MANAGER_EXIST: WIN32_ERROR = WIN32_ERROR(15144u32);
pub const ERROR_DIFFERENT_SERVICE_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1079u32);
pub const ERROR_DIFFERENT_VERSION_OF_PACKAGED_SERVICE_INSTALLED: WIN32_ERROR = WIN32_ERROR(15654u32);
pub const ERROR_DIF_BINDING_API_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3199u32);
pub const ERROR_DIF_IOCALLBACK_NOT_REPLACED: WIN32_ERROR = WIN32_ERROR(3190u32);
pub const ERROR_DIF_LIVEDUMP_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(3191u32);
pub const ERROR_DIF_VOLATILE_DRIVER_HOTPATCHED: WIN32_ERROR = WIN32_ERROR(3193u32);
pub const ERROR_DIF_VOLATILE_DRIVER_IS_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(3195u32);
pub const ERROR_DIF_VOLATILE_INVALID_INFO: WIN32_ERROR = WIN32_ERROR(3194u32);
pub const ERROR_DIF_VOLATILE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(3198u32);
pub const ERROR_DIF_VOLATILE_PLUGIN_CHANGE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(3197u32);
pub const ERROR_DIF_VOLATILE_PLUGIN_IS_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(3196u32);
pub const ERROR_DIF_VOLATILE_SECTION_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(3192u32);
pub const ERROR_DIRECTORY: WIN32_ERROR = WIN32_ERROR(267u32);
pub const ERROR_DIRECTORY_NOT_RM: WIN32_ERROR = WIN32_ERROR(6803u32);
pub const ERROR_DIRECTORY_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(336u32);
pub const ERROR_DIRECT_ACCESS_HANDLE: WIN32_ERROR = WIN32_ERROR(130u32);
pub const ERROR_DIR_EFS_DISALLOWED: WIN32_ERROR = WIN32_ERROR(6010u32);
pub const ERROR_DIR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(145u32);
pub const ERROR_DIR_NOT_ROOT: WIN32_ERROR = WIN32_ERROR(144u32);
pub const ERROR_DISCARDED: WIN32_ERROR = WIN32_ERROR(157u32);
pub const ERROR_DISK_CHANGE: WIN32_ERROR = WIN32_ERROR(107u32);
pub const ERROR_DISK_CORRUPT: WIN32_ERROR = WIN32_ERROR(1393u32);
pub const ERROR_DISK_FULL: WIN32_ERROR = WIN32_ERROR(112u32);
pub const ERROR_DISK_NOT_CSV_CAPABLE: WIN32_ERROR = WIN32_ERROR(5964u32);
pub const ERROR_DISK_OPERATION_FAILED: WIN32_ERROR = WIN32_ERROR(1127u32);
pub const ERROR_DISK_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1295u32);
pub const ERROR_DISK_RECALIBRATE_FAILED: WIN32_ERROR = WIN32_ERROR(1126u32);
pub const ERROR_DISK_REPAIR_DISABLED: WIN32_ERROR = WIN32_ERROR(780u32);
pub const ERROR_DISK_REPAIR_REDIRECTED: WIN32_ERROR = WIN32_ERROR(792u32);
pub const ERROR_DISK_REPAIR_UNSUCCESSFUL: WIN32_ERROR = WIN32_ERROR(793u32);
pub const ERROR_DISK_RESET_FAILED: WIN32_ERROR = WIN32_ERROR(1128u32);
pub const ERROR_DISK_RESOURCES_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(314u32);
pub const ERROR_DISK_TOO_FRAGMENTED: WIN32_ERROR = WIN32_ERROR(302u32);
pub const ERROR_DI_BAD_PATH: WIN32_ERROR = WIN32_ERROR(3758096916u32);
pub const ERROR_DI_DONT_INSTALL: WIN32_ERROR = WIN32_ERROR(3758096939u32);
pub const ERROR_DI_DO_DEFAULT: WIN32_ERROR = WIN32_ERROR(3758096910u32);
pub const ERROR_DI_FUNCTION_OBSOLETE: WIN32_ERROR = WIN32_ERROR(3758096958u32);
pub const ERROR_DI_NOFILECOPY: WIN32_ERROR = WIN32_ERROR(3758096911u32);
pub const ERROR_DI_POSTPROCESSING_REQUIRED: WIN32_ERROR = WIN32_ERROR(3758096934u32);
pub const ERROR_DLL_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(1114u32);
pub const ERROR_DLL_INIT_FAILED_LOGOFF: WIN32_ERROR = WIN32_ERROR(624u32);
pub const ERROR_DLL_MIGHT_BE_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(687u32);
pub const ERROR_DLL_MIGHT_BE_INSECURE: WIN32_ERROR = WIN32_ERROR(686u32);
pub const ERROR_DLL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1157u32);
pub const ERROR_DLP_POLICY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(446u32);
pub const ERROR_DLP_POLICY_SILENTLY_FAIL: WIN32_ERROR = WIN32_ERROR(449u32);
pub const ERROR_DLP_POLICY_WARNS_AGAINST_OPERATION: WIN32_ERROR = WIN32_ERROR(445u32);
pub const ERROR_DM_OPERATION_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0xC0370600_u32 as _);
pub const ERROR_DOMAIN_CONTROLLER_EXISTS: WIN32_ERROR = WIN32_ERROR(1250u32);
pub const ERROR_DOMAIN_CONTROLLER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1908u32);
pub const ERROR_DOMAIN_CTRLR_CONFIG_ERROR: WIN32_ERROR = WIN32_ERROR(581u32);
pub const ERROR_DOMAIN_EXISTS: WIN32_ERROR = WIN32_ERROR(1356u32);
pub const ERROR_DOMAIN_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1357u32);
pub const ERROR_DOMAIN_SID_SAME_AS_LOCAL_WORKSTATION: WIN32_ERROR = WIN32_ERROR(8644u32);
pub const ERROR_DOMAIN_TRUST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(1810u32);
pub const ERROR_DOWNGRADE_DETECTED: WIN32_ERROR = WIN32_ERROR(1265u32);
pub const ERROR_DPL_NOT_SUPPORTED_FOR_USER: WIN32_ERROR = WIN32_ERROR(423u32);
pub const ERROR_DRIVERS_LEAKING_LOCKED_PAGES: WIN32_ERROR = WIN32_ERROR(729u32);
pub const ERROR_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(1275u32);
pub const ERROR_DRIVER_CANCEL_TIMEOUT: WIN32_ERROR = WIN32_ERROR(594u32);
pub const ERROR_DRIVER_DATABASE_ERROR: WIN32_ERROR = WIN32_ERROR(652u32);
pub const ERROR_DRIVER_FAILED_PRIOR_UNLOAD: WIN32_ERROR = WIN32_ERROR(654u32);
pub const ERROR_DRIVER_FAILED_SLEEP: WIN32_ERROR = WIN32_ERROR(633u32);
pub const ERROR_DRIVER_INSTALL_BLOCKED: WIN32_ERROR = WIN32_ERROR(3758096969u32);
pub const ERROR_DRIVER_NONNATIVE: WIN32_ERROR = WIN32_ERROR(3758096948u32);
pub const ERROR_DRIVER_PROCESS_TERMINATED: WIN32_ERROR = WIN32_ERROR(1291u32);
pub const ERROR_DRIVER_STORE_ADD_FAILED: WIN32_ERROR = WIN32_ERROR(3758096967u32);
pub const ERROR_DRIVER_STORE_DELETE_FAILED: WIN32_ERROR = WIN32_ERROR(3758096972u32);
pub const ERROR_DRIVE_LOCKED: WIN32_ERROR = WIN32_ERROR(108u32);
pub const ERROR_DRIVE_MEDIA_MISMATCH: WIN32_ERROR = WIN32_ERROR(4303u32);
pub const ERROR_DS_ADD_REPLICA_INHIBITED: WIN32_ERROR = WIN32_ERROR(8302u32);
pub const ERROR_DS_ADMIN_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8228u32);
pub const ERROR_DS_AFFECTS_MULTIPLE_DSAS: WIN32_ERROR = WIN32_ERROR(8249u32);
pub const ERROR_DS_AG_CANT_HAVE_UNIVERSAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8578u32);
pub const ERROR_DS_ALIASED_OBJ_MISSING: WIN32_ERROR = WIN32_ERROR(8334u32);
pub const ERROR_DS_ALIAS_DEREF_PROBLEM: WIN32_ERROR = WIN32_ERROR(8244u32);
pub const ERROR_DS_ALIAS_POINTS_TO_ALIAS: WIN32_ERROR = WIN32_ERROR(8336u32);
pub const ERROR_DS_ALIAS_PROBLEM: WIN32_ERROR = WIN32_ERROR(8241u32);
pub const ERROR_DS_ATTRIBUTE_OR_VALUE_EXISTS: WIN32_ERROR = WIN32_ERROR(8205u32);
pub const ERROR_DS_ATTRIBUTE_OWNED_BY_SAM: WIN32_ERROR = WIN32_ERROR(8346u32);
pub const ERROR_DS_ATTRIBUTE_TYPE_UNDEFINED: WIN32_ERROR = WIN32_ERROR(8204u32);
pub const ERROR_DS_ATT_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8318u32);
pub const ERROR_DS_ATT_IS_NOT_ON_OBJ: WIN32_ERROR = WIN32_ERROR(8310u32);
pub const ERROR_DS_ATT_NOT_DEF_FOR_CLASS: WIN32_ERROR = WIN32_ERROR(8317u32);
pub const ERROR_DS_ATT_NOT_DEF_IN_SCHEMA: WIN32_ERROR = WIN32_ERROR(8303u32);
pub const ERROR_DS_ATT_SCHEMA_REQ_ID: WIN32_ERROR = WIN32_ERROR(8399u32);
pub const ERROR_DS_ATT_SCHEMA_REQ_SYNTAX: WIN32_ERROR = WIN32_ERROR(8416u32);
pub const ERROR_DS_ATT_VAL_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8323u32);
pub const ERROR_DS_AUDIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8625u32);
pub const ERROR_DS_AUTHORIZATION_FAILED: WIN32_ERROR = WIN32_ERROR(8599u32);
pub const ERROR_DS_AUTH_METHOD_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8231u32);
pub const ERROR_DS_AUTH_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8234u32);
pub const ERROR_DS_AUX_CLS_TEST_FAIL: WIN32_ERROR = WIN32_ERROR(8389u32);
pub const ERROR_DS_BACKLINK_WITHOUT_LINK: WIN32_ERROR = WIN32_ERROR(8482u32);
pub const ERROR_DS_BAD_ATT_SCHEMA_SYNTAX: WIN32_ERROR = WIN32_ERROR(8400u32);
pub const ERROR_DS_BAD_HIERARCHY_FILE: WIN32_ERROR = WIN32_ERROR(8425u32);
pub const ERROR_DS_BAD_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8313u32);
pub const ERROR_DS_BAD_NAME_SYNTAX: WIN32_ERROR = WIN32_ERROR(8335u32);
pub const ERROR_DS_BAD_RDN_ATT_ID_SYNTAX: WIN32_ERROR = WIN32_ERROR(8392u32);
pub const ERROR_DS_BUILD_HIERARCHY_TABLE_FAILED: WIN32_ERROR = WIN32_ERROR(8426u32);
pub const ERROR_DS_BUSY: WIN32_ERROR = WIN32_ERROR(8206u32);
pub const ERROR_DS_CANT_ACCESS_REMOTE_PART_OF_AD: WIN32_ERROR = WIN32_ERROR(8585u32);
pub const ERROR_DS_CANT_ADD_ATT_VALUES: WIN32_ERROR = WIN32_ERROR(8320u32);
pub const ERROR_DS_CANT_ADD_SYSTEM_ONLY: WIN32_ERROR = WIN32_ERROR(8358u32);
pub const ERROR_DS_CANT_ADD_TO_GC: WIN32_ERROR = WIN32_ERROR(8550u32);
pub const ERROR_DS_CANT_CACHE_ATT: WIN32_ERROR = WIN32_ERROR(8401u32);
pub const ERROR_DS_CANT_CACHE_CLASS: WIN32_ERROR = WIN32_ERROR(8402u32);
pub const ERROR_DS_CANT_CREATE_IN_NONDOMAIN_NC: WIN32_ERROR = WIN32_ERROR(8553u32);
pub const ERROR_DS_CANT_CREATE_UNDER_SCHEMA: WIN32_ERROR = WIN32_ERROR(8510u32);
pub const ERROR_DS_CANT_DELETE: WIN32_ERROR = WIN32_ERROR(8398u32);
pub const ERROR_DS_CANT_DELETE_DSA_OBJ: WIN32_ERROR = WIN32_ERROR(8340u32);
pub const ERROR_DS_CANT_DEL_MASTER_CROSSREF: WIN32_ERROR = WIN32_ERROR(8375u32);
pub const ERROR_DS_CANT_DEMOTE_WITH_WRITEABLE_NC: WIN32_ERROR = WIN32_ERROR(8604u32);
pub const ERROR_DS_CANT_DEREF_ALIAS: WIN32_ERROR = WIN32_ERROR(8337u32);
pub const ERROR_DS_CANT_DERIVE_SPN_FOR_DELETED_DOMAIN: WIN32_ERROR = WIN32_ERROR(8603u32);
pub const ERROR_DS_CANT_DERIVE_SPN_WITHOUT_SERVER_REF: WIN32_ERROR = WIN32_ERROR(8589u32);
pub const ERROR_DS_CANT_FIND_DC_FOR_SRC_DOMAIN: WIN32_ERROR = WIN32_ERROR(8537u32);
pub const ERROR_DS_CANT_FIND_DSA_OBJ: WIN32_ERROR = WIN32_ERROR(8419u32);
pub const ERROR_DS_CANT_FIND_EXPECTED_NC: WIN32_ERROR = WIN32_ERROR(8420u32);
pub const ERROR_DS_CANT_FIND_NC_IN_CACHE: WIN32_ERROR = WIN32_ERROR(8421u32);
pub const ERROR_DS_CANT_MIX_MASTER_AND_REPS: WIN32_ERROR = WIN32_ERROR(8331u32);
pub const ERROR_DS_CANT_MOD_OBJ_CLASS: WIN32_ERROR = WIN32_ERROR(8215u32);
pub const ERROR_DS_CANT_MOD_PRIMARYGROUPID: WIN32_ERROR = WIN32_ERROR(8506u32);
pub const ERROR_DS_CANT_MOD_SYSTEM_ONLY: WIN32_ERROR = WIN32_ERROR(8369u32);
pub const ERROR_DS_CANT_MOVE_ACCOUNT_GROUP: WIN32_ERROR = WIN32_ERROR(8498u32);
pub const ERROR_DS_CANT_MOVE_APP_BASIC_GROUP: WIN32_ERROR = WIN32_ERROR(8608u32);
pub const ERROR_DS_CANT_MOVE_APP_QUERY_GROUP: WIN32_ERROR = WIN32_ERROR(8609u32);
pub const ERROR_DS_CANT_MOVE_DELETED_OBJECT: WIN32_ERROR = WIN32_ERROR(8489u32);
pub const ERROR_DS_CANT_MOVE_RESOURCE_GROUP: WIN32_ERROR = WIN32_ERROR(8499u32);
pub const ERROR_DS_CANT_ON_NON_LEAF: WIN32_ERROR = WIN32_ERROR(8213u32);
pub const ERROR_DS_CANT_ON_RDN: WIN32_ERROR = WIN32_ERROR(8214u32);
pub const ERROR_DS_CANT_REMOVE_ATT_CACHE: WIN32_ERROR = WIN32_ERROR(8403u32);
pub const ERROR_DS_CANT_REMOVE_CLASS_CACHE: WIN32_ERROR = WIN32_ERROR(8404u32);
pub const ERROR_DS_CANT_REM_MISSING_ATT: WIN32_ERROR = WIN32_ERROR(8324u32);
pub const ERROR_DS_CANT_REM_MISSING_ATT_VAL: WIN32_ERROR = WIN32_ERROR(8325u32);
pub const ERROR_DS_CANT_REPLACE_HIDDEN_REC: WIN32_ERROR = WIN32_ERROR(8424u32);
pub const ERROR_DS_CANT_RETRIEVE_ATTS: WIN32_ERROR = WIN32_ERROR(8481u32);
pub const ERROR_DS_CANT_RETRIEVE_CHILD: WIN32_ERROR = WIN32_ERROR(8422u32);
pub const ERROR_DS_CANT_RETRIEVE_DN: WIN32_ERROR = WIN32_ERROR(8405u32);
pub const ERROR_DS_CANT_RETRIEVE_INSTANCE: WIN32_ERROR = WIN32_ERROR(8407u32);
pub const ERROR_DS_CANT_RETRIEVE_SD: WIN32_ERROR = WIN32_ERROR(8526u32);
pub const ERROR_DS_CANT_START: WIN32_ERROR = WIN32_ERROR(8531u32);
pub const ERROR_DS_CANT_TREE_DELETE_CRITICAL_OBJ: WIN32_ERROR = WIN32_ERROR(8560u32);
pub const ERROR_DS_CANT_WITH_ACCT_GROUP_MEMBERSHPS: WIN32_ERROR = WIN32_ERROR(8493u32);
pub const ERROR_DS_CHILDREN_EXIST: WIN32_ERROR = WIN32_ERROR(8332u32);
pub const ERROR_DS_CLASS_MUST_BE_CONCRETE: WIN32_ERROR = WIN32_ERROR(8359u32);
pub const ERROR_DS_CLASS_NOT_DSA: WIN32_ERROR = WIN32_ERROR(8343u32);
pub const ERROR_DS_CLIENT_LOOP: WIN32_ERROR = WIN32_ERROR(8259u32);
pub const ERROR_DS_CODE_INCONSISTENCY: WIN32_ERROR = WIN32_ERROR(8408u32);
pub const ERROR_DS_COMPARE_FALSE: WIN32_ERROR = WIN32_ERROR(8229u32);
pub const ERROR_DS_COMPARE_TRUE: WIN32_ERROR = WIN32_ERROR(8230u32);
pub const ERROR_DS_CONFIDENTIALITY_REQUIRED: WIN32_ERROR = WIN32_ERROR(8237u32);
pub const ERROR_DS_CONFIG_PARAM_MISSING: WIN32_ERROR = WIN32_ERROR(8427u32);
pub const ERROR_DS_CONSTRAINT_VIOLATION: WIN32_ERROR = WIN32_ERROR(8239u32);
pub const ERROR_DS_CONSTRUCTED_ATT_MOD: WIN32_ERROR = WIN32_ERROR(8475u32);
pub const ERROR_DS_CONTROL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8258u32);
pub const ERROR_DS_COULDNT_CONTACT_FSMO: WIN32_ERROR = WIN32_ERROR(8367u32);
pub const ERROR_DS_COULDNT_IDENTIFY_OBJECTS_FOR_TREE_DELETE: WIN32_ERROR = WIN32_ERROR(8503u32);
pub const ERROR_DS_COULDNT_LOCK_TREE_FOR_DELETE: WIN32_ERROR = WIN32_ERROR(8502u32);
pub const ERROR_DS_COULDNT_UPDATE_SPNS: WIN32_ERROR = WIN32_ERROR(8525u32);
pub const ERROR_DS_COUNTING_AB_INDICES_FAILED: WIN32_ERROR = WIN32_ERROR(8428u32);
pub const ERROR_DS_CROSS_DOMAIN_CLEANUP_REQD: WIN32_ERROR = WIN32_ERROR(8491u32);
pub const ERROR_DS_CROSS_DOM_MOVE_ERROR: WIN32_ERROR = WIN32_ERROR(8216u32);
pub const ERROR_DS_CROSS_NC_DN_RENAME: WIN32_ERROR = WIN32_ERROR(8368u32);
pub const ERROR_DS_CROSS_REF_BUSY: WIN32_ERROR = WIN32_ERROR(8602u32);
pub const ERROR_DS_CROSS_REF_EXISTS: WIN32_ERROR = WIN32_ERROR(8374u32);
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE: WIN32_ERROR = WIN32_ERROR(8495u32);
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE_V2: WIN32_ERROR = WIN32_ERROR(8586u32);
pub const ERROR_DS_DATABASE_ERROR: WIN32_ERROR = WIN32_ERROR(8409u32);
pub const ERROR_DS_DECODING_ERROR: WIN32_ERROR = WIN32_ERROR(8253u32);
pub const ERROR_DS_DESTINATION_AUDITING_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(8536u32);
pub const ERROR_DS_DESTINATION_DOMAIN_NOT_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8535u32);
pub const ERROR_DS_DIFFERENT_REPL_EPOCHS: WIN32_ERROR = WIN32_ERROR(8593u32);
pub const ERROR_DS_DISALLOWED_IN_SYSTEM_CONTAINER: WIN32_ERROR = WIN32_ERROR(8615u32);
pub const ERROR_DS_DISALLOWED_NC_REDIRECT: WIN32_ERROR = WIN32_ERROR(8640u32);
pub const ERROR_DS_DNS_LOOKUP_FAILURE: WIN32_ERROR = WIN32_ERROR(8524u32);
pub const ERROR_DS_DOMAIN_NAME_EXISTS_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8634u32);
pub const ERROR_DS_DOMAIN_RENAME_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(8612u32);
pub const ERROR_DS_DOMAIN_VERSION_TOO_HIGH: WIN32_ERROR = WIN32_ERROR(8564u32);
pub const ERROR_DS_DOMAIN_VERSION_TOO_LOW: WIN32_ERROR = WIN32_ERROR(8566u32);
pub const ERROR_DS_DRA_ABANDON_SYNC: WIN32_ERROR = WIN32_ERROR(8462u32);
pub const ERROR_DS_DRA_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(8453u32);
pub const ERROR_DS_DRA_BAD_DN: WIN32_ERROR = WIN32_ERROR(8439u32);
pub const ERROR_DS_DRA_BAD_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8445u32);
pub const ERROR_DS_DRA_BAD_NC: WIN32_ERROR = WIN32_ERROR(8440u32);
pub const ERROR_DS_DRA_BUSY: WIN32_ERROR = WIN32_ERROR(8438u32);
pub const ERROR_DS_DRA_CONNECTION_FAILED: WIN32_ERROR = WIN32_ERROR(8444u32);
pub const ERROR_DS_DRA_CORRUPT_UTD_VECTOR: WIN32_ERROR = WIN32_ERROR(8629u32);
pub const ERROR_DS_DRA_DB_ERROR: WIN32_ERROR = WIN32_ERROR(8451u32);
pub const ERROR_DS_DRA_DN_EXISTS: WIN32_ERROR = WIN32_ERROR(8441u32);
pub const ERROR_DS_DRA_EARLIER_SCHEMA_CONFLICT: WIN32_ERROR = WIN32_ERROR(8544u32);
pub const ERROR_DS_DRA_EXTN_CONNECTION_FAILED: WIN32_ERROR = WIN32_ERROR(8466u32);
pub const ERROR_DS_DRA_GENERIC: WIN32_ERROR = WIN32_ERROR(8436u32);
pub const ERROR_DS_DRA_INCOMPATIBLE_PARTIAL_SET: WIN32_ERROR = WIN32_ERROR(8464u32);
pub const ERROR_DS_DRA_INCONSISTENT_DIT: WIN32_ERROR = WIN32_ERROR(8443u32);
pub const ERROR_DS_DRA_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(8442u32);
pub const ERROR_DS_DRA_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(8437u32);
pub const ERROR_DS_DRA_MAIL_PROBLEM: WIN32_ERROR = WIN32_ERROR(8447u32);
pub const ERROR_DS_DRA_MISSING_KRBTGT_SECRET: WIN32_ERROR = WIN32_ERROR(8633u32);
pub const ERROR_DS_DRA_MISSING_PARENT: WIN32_ERROR = WIN32_ERROR(8460u32);
pub const ERROR_DS_DRA_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(8458u32);
pub const ERROR_DS_DRA_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8454u32);
pub const ERROR_DS_DRA_NO_REPLICA: WIN32_ERROR = WIN32_ERROR(8452u32);
pub const ERROR_DS_DRA_OBJ_IS_REP_SOURCE: WIN32_ERROR = WIN32_ERROR(8450u32);
pub const ERROR_DS_DRA_OBJ_NC_MISMATCH: WIN32_ERROR = WIN32_ERROR(8545u32);
pub const ERROR_DS_DRA_OUT_OF_MEM: WIN32_ERROR = WIN32_ERROR(8446u32);
pub const ERROR_DS_DRA_OUT_SCHEDULE_WINDOW: WIN32_ERROR = WIN32_ERROR(8617u32);
pub const ERROR_DS_DRA_PREEMPTED: WIN32_ERROR = WIN32_ERROR(8461u32);
pub const ERROR_DS_DRA_RECYCLED_TARGET: WIN32_ERROR = WIN32_ERROR(8639u32);
pub const ERROR_DS_DRA_REF_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8448u32);
pub const ERROR_DS_DRA_REF_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8449u32);
pub const ERROR_DS_DRA_REPL_PENDING: WIN32_ERROR = WIN32_ERROR(8477u32);
pub const ERROR_DS_DRA_RPC_CANCELLED: WIN32_ERROR = WIN32_ERROR(8455u32);
pub const ERROR_DS_DRA_SCHEMA_CONFLICT: WIN32_ERROR = WIN32_ERROR(8543u32);
pub const ERROR_DS_DRA_SCHEMA_INFO_SHIP: WIN32_ERROR = WIN32_ERROR(8542u32);
pub const ERROR_DS_DRA_SCHEMA_MISMATCH: WIN32_ERROR = WIN32_ERROR(8418u32);
pub const ERROR_DS_DRA_SECRETS_DENIED: WIN32_ERROR = WIN32_ERROR(8630u32);
pub const ERROR_DS_DRA_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(8463u32);
pub const ERROR_DS_DRA_SINK_DISABLED: WIN32_ERROR = WIN32_ERROR(8457u32);
pub const ERROR_DS_DRA_SOURCE_DISABLED: WIN32_ERROR = WIN32_ERROR(8456u32);
pub const ERROR_DS_DRA_SOURCE_IS_PARTIAL_REPLICA: WIN32_ERROR = WIN32_ERROR(8465u32);
pub const ERROR_DS_DRA_SOURCE_REINSTALLED: WIN32_ERROR = WIN32_ERROR(8459u32);
pub const ERROR_DS_DRS_EXTENSIONS_CHANGED: WIN32_ERROR = WIN32_ERROR(8594u32);
pub const ERROR_DS_DSA_MUST_BE_INT_MASTER: WIN32_ERROR = WIN32_ERROR(8342u32);
pub const ERROR_DS_DST_DOMAIN_NOT_NATIVE: WIN32_ERROR = WIN32_ERROR(8496u32);
pub const ERROR_DS_DST_NC_MISMATCH: WIN32_ERROR = WIN32_ERROR(8486u32);
pub const ERROR_DS_DS_REQUIRED: WIN32_ERROR = WIN32_ERROR(8478u32);
pub const ERROR_DS_DUPLICATE_ID_FOUND: WIN32_ERROR = WIN32_ERROR(8605u32);
pub const ERROR_DS_DUP_LDAP_DISPLAY_NAME: WIN32_ERROR = WIN32_ERROR(8382u32);
pub const ERROR_DS_DUP_LINK_ID: WIN32_ERROR = WIN32_ERROR(8468u32);
pub const ERROR_DS_DUP_MAPI_ID: WIN32_ERROR = WIN32_ERROR(8380u32);
pub const ERROR_DS_DUP_MSDS_INTID: WIN32_ERROR = WIN32_ERROR(8597u32);
pub const ERROR_DS_DUP_OID: WIN32_ERROR = WIN32_ERROR(8379u32);
pub const ERROR_DS_DUP_RDN: WIN32_ERROR = WIN32_ERROR(8378u32);
pub const ERROR_DS_DUP_SCHEMA_ID_GUID: WIN32_ERROR = WIN32_ERROR(8381u32);
pub const ERROR_DS_ENCODING_ERROR: WIN32_ERROR = WIN32_ERROR(8252u32);
pub const ERROR_DS_EPOCH_MISMATCH: WIN32_ERROR = WIN32_ERROR(8483u32);
pub const ERROR_DS_EXISTING_AD_CHILD_NC: WIN32_ERROR = WIN32_ERROR(8613u32);
pub const ERROR_DS_EXISTS_IN_AUX_CLS: WIN32_ERROR = WIN32_ERROR(8393u32);
pub const ERROR_DS_EXISTS_IN_MAY_HAVE: WIN32_ERROR = WIN32_ERROR(8386u32);
pub const ERROR_DS_EXISTS_IN_MUST_HAVE: WIN32_ERROR = WIN32_ERROR(8385u32);
pub const ERROR_DS_EXISTS_IN_POSS_SUP: WIN32_ERROR = WIN32_ERROR(8395u32);
pub const ERROR_DS_EXISTS_IN_RDNATTID: WIN32_ERROR = WIN32_ERROR(8598u32);
pub const ERROR_DS_EXISTS_IN_SUB_CLS: WIN32_ERROR = WIN32_ERROR(8394u32);
pub const ERROR_DS_FILTER_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8254u32);
pub const ERROR_DS_FILTER_USES_CONTRUCTED_ATTRS: WIN32_ERROR = WIN32_ERROR(8555u32);
pub const ERROR_DS_FLAT_NAME_EXISTS_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8635u32);
pub const ERROR_DS_FOREST_VERSION_TOO_HIGH: WIN32_ERROR = WIN32_ERROR(8563u32);
pub const ERROR_DS_FOREST_VERSION_TOO_LOW: WIN32_ERROR = WIN32_ERROR(8565u32);
pub const ERROR_DS_GCVERIFY_ERROR: WIN32_ERROR = WIN32_ERROR(8417u32);
pub const ERROR_DS_GC_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8217u32);
pub const ERROR_DS_GC_REQUIRED: WIN32_ERROR = WIN32_ERROR(8547u32);
pub const ERROR_DS_GENERIC_ERROR: WIN32_ERROR = WIN32_ERROR(8341u32);
pub const ERROR_DS_GLOBAL_CANT_HAVE_CROSSDOMAIN_MEMBER: WIN32_ERROR = WIN32_ERROR(8519u32);
pub const ERROR_DS_GLOBAL_CANT_HAVE_LOCAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8516u32);
pub const ERROR_DS_GLOBAL_CANT_HAVE_UNIVERSAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8517u32);
pub const ERROR_DS_GOVERNSID_MISSING: WIN32_ERROR = WIN32_ERROR(8410u32);
pub const ERROR_DS_GROUP_CONVERSION_ERROR: WIN32_ERROR = WIN32_ERROR(8607u32);
pub const ERROR_DS_HAVE_PRIMARY_MEMBERS: WIN32_ERROR = WIN32_ERROR(8521u32);
pub const ERROR_DS_HIERARCHY_TABLE_MALLOC_FAILED: WIN32_ERROR = WIN32_ERROR(8429u32);
pub const ERROR_DS_HIERARCHY_TABLE_TOO_DEEP: WIN32_ERROR = WIN32_ERROR(8628u32);
pub const ERROR_DS_HIGH_ADLDS_FFL: WIN32_ERROR = WIN32_ERROR(8641u32);
pub const ERROR_DS_HIGH_DSA_VERSION: WIN32_ERROR = WIN32_ERROR(8642u32);
pub const ERROR_DS_ILLEGAL_BASE_SCHEMA_MOD: WIN32_ERROR = WIN32_ERROR(8507u32);
pub const ERROR_DS_ILLEGAL_MOD_OPERATION: WIN32_ERROR = WIN32_ERROR(8311u32);
pub const ERROR_DS_ILLEGAL_SUPERIOR: WIN32_ERROR = WIN32_ERROR(8345u32);
pub const ERROR_DS_ILLEGAL_XDOM_MOVE_OPERATION: WIN32_ERROR = WIN32_ERROR(8492u32);
pub const ERROR_DS_INAPPROPRIATE_AUTH: WIN32_ERROR = WIN32_ERROR(8233u32);
pub const ERROR_DS_INAPPROPRIATE_MATCHING: WIN32_ERROR = WIN32_ERROR(8238u32);
pub const ERROR_DS_INCOMPATIBLE_CONTROLS_USED: WIN32_ERROR = WIN32_ERROR(8574u32);
pub const ERROR_DS_INCOMPATIBLE_VERSION: WIN32_ERROR = WIN32_ERROR(8567u32);
pub const ERROR_DS_INCORRECT_ROLE_OWNER: WIN32_ERROR = WIN32_ERROR(8210u32);
pub const ERROR_DS_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8532u32);
pub const ERROR_DS_INIT_FAILURE_CONSOLE: WIN32_ERROR = WIN32_ERROR(8561u32);
pub const ERROR_DS_INSTALL_NO_SCH_VERSION_IN_INIFILE: WIN32_ERROR = WIN32_ERROR(8512u32);
pub const ERROR_DS_INSTALL_NO_SRC_SCH_VERSION: WIN32_ERROR = WIN32_ERROR(8511u32);
pub const ERROR_DS_INSTALL_SCHEMA_MISMATCH: WIN32_ERROR = WIN32_ERROR(8467u32);
pub const ERROR_DS_INSUFFICIENT_ATTR_TO_CREATE_OBJECT: WIN32_ERROR = WIN32_ERROR(8606u32);
pub const ERROR_DS_INSUFF_ACCESS_RIGHTS: WIN32_ERROR = WIN32_ERROR(8344u32);
pub const ERROR_DS_INTERNAL_FAILURE: WIN32_ERROR = WIN32_ERROR(8430u32);
pub const ERROR_DS_INVALID_ATTRIBUTE_SYNTAX: WIN32_ERROR = WIN32_ERROR(8203u32);
pub const ERROR_DS_INVALID_DMD: WIN32_ERROR = WIN32_ERROR(8360u32);
pub const ERROR_DS_INVALID_DN_SYNTAX: WIN32_ERROR = WIN32_ERROR(8242u32);
pub const ERROR_DS_INVALID_GROUP_TYPE: WIN32_ERROR = WIN32_ERROR(8513u32);
pub const ERROR_DS_INVALID_LDAP_DISPLAY_NAME: WIN32_ERROR = WIN32_ERROR(8479u32);
pub const ERROR_DS_INVALID_NAME_FOR_SPN: WIN32_ERROR = WIN32_ERROR(8554u32);
pub const ERROR_DS_INVALID_ROLE_OWNER: WIN32_ERROR = WIN32_ERROR(8366u32);
pub const ERROR_DS_INVALID_SCRIPT: WIN32_ERROR = WIN32_ERROR(8600u32);
pub const ERROR_DS_INVALID_SEARCH_FLAG: WIN32_ERROR = WIN32_ERROR(8500u32);
pub const ERROR_DS_INVALID_SEARCH_FLAG_SUBTREE: WIN32_ERROR = WIN32_ERROR(8626u32);
pub const ERROR_DS_INVALID_SEARCH_FLAG_TUPLE: WIN32_ERROR = WIN32_ERROR(8627u32);
pub const ERROR_DS_IS_LEAF: WIN32_ERROR = WIN32_ERROR(8243u32);
pub const ERROR_DS_KEY_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8527u32);
pub const ERROR_DS_LDAP_SEND_QUEUE_FULL: WIN32_ERROR = WIN32_ERROR(8616u32);
pub const ERROR_DS_LINK_ID_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8577u32);
pub const ERROR_DS_LOCAL_CANT_HAVE_CROSSDOMAIN_LOCAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8520u32);
pub const ERROR_DS_LOCAL_ERROR: WIN32_ERROR = WIN32_ERROR(8251u32);
pub const ERROR_DS_LOCAL_MEMBER_OF_LOCAL_ONLY: WIN32_ERROR = WIN32_ERROR(8548u32);
pub const ERROR_DS_LOOP_DETECT: WIN32_ERROR = WIN32_ERROR(8246u32);
pub const ERROR_DS_LOW_ADLDS_FFL: WIN32_ERROR = WIN32_ERROR(8643u32);
pub const ERROR_DS_LOW_DSA_VERSION: WIN32_ERROR = WIN32_ERROR(8568u32);
pub const ERROR_DS_MACHINE_ACCOUNT_CREATED_PRENT4: WIN32_ERROR = WIN32_ERROR(8572u32);
pub const ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8557u32);
pub const ERROR_DS_MAPI_ID_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8632u32);
pub const ERROR_DS_MASTERDSA_REQUIRED: WIN32_ERROR = WIN32_ERROR(8314u32);
pub const ERROR_DS_MAX_OBJ_SIZE_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8304u32);
pub const ERROR_DS_MEMBERSHIP_EVALUATED_LOCALLY: WIN32_ERROR = WIN32_ERROR(8201u32);
pub const ERROR_DS_MISSING_EXPECTED_ATT: WIN32_ERROR = WIN32_ERROR(8411u32);
pub const ERROR_DS_MISSING_FOREST_TRUST: WIN32_ERROR = WIN32_ERROR(8649u32);
pub const ERROR_DS_MISSING_FSMO_SETTINGS: WIN32_ERROR = WIN32_ERROR(8434u32);
pub const ERROR_DS_MISSING_INFRASTRUCTURE_CONTAINER: WIN32_ERROR = WIN32_ERROR(8497u32);
pub const ERROR_DS_MISSING_REQUIRED_ATT: WIN32_ERROR = WIN32_ERROR(8316u32);
pub const ERROR_DS_MISSING_SUPREF: WIN32_ERROR = WIN32_ERROR(8406u32);
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_FLAG: WIN32_ERROR = WIN32_ERROR(8581u32);
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8579u32);
pub const ERROR_DS_MODIFYDN_WRONG_GRANDPARENT: WIN32_ERROR = WIN32_ERROR(8582u32);
pub const ERROR_DS_MUST_BE_RUN_ON_DST_DC: WIN32_ERROR = WIN32_ERROR(8558u32);
pub const ERROR_DS_NAME_ERROR_DOMAIN_ONLY: WIN32_ERROR = WIN32_ERROR(8473u32);
pub const ERROR_DS_NAME_ERROR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8470u32);
pub const ERROR_DS_NAME_ERROR_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8471u32);
pub const ERROR_DS_NAME_ERROR_NO_MAPPING: WIN32_ERROR = WIN32_ERROR(8472u32);
pub const ERROR_DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: WIN32_ERROR = WIN32_ERROR(8474u32);
pub const ERROR_DS_NAME_ERROR_RESOLVING: WIN32_ERROR = WIN32_ERROR(8469u32);
pub const ERROR_DS_NAME_ERROR_TRUST_REFERRAL: WIN32_ERROR = WIN32_ERROR(8583u32);
pub const ERROR_DS_NAME_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8571u32);
pub const ERROR_DS_NAME_REFERENCE_INVALID: WIN32_ERROR = WIN32_ERROR(8373u32);
pub const ERROR_DS_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(8348u32);
pub const ERROR_DS_NAME_TOO_MANY_PARTS: WIN32_ERROR = WIN32_ERROR(8347u32);
pub const ERROR_DS_NAME_TYPE_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8351u32);
pub const ERROR_DS_NAME_UNPARSEABLE: WIN32_ERROR = WIN32_ERROR(8350u32);
pub const ERROR_DS_NAME_VALUE_TOO_LONG: WIN32_ERROR = WIN32_ERROR(8349u32);
pub const ERROR_DS_NAMING_MASTER_GC: WIN32_ERROR = WIN32_ERROR(8523u32);
pub const ERROR_DS_NAMING_VIOLATION: WIN32_ERROR = WIN32_ERROR(8247u32);
pub const ERROR_DS_NCNAME_MISSING_CR_REF: WIN32_ERROR = WIN32_ERROR(8412u32);
pub const ERROR_DS_NCNAME_MUST_BE_NC: WIN32_ERROR = WIN32_ERROR(8357u32);
pub const ERROR_DS_NC_MUST_HAVE_NC_PARENT: WIN32_ERROR = WIN32_ERROR(8494u32);
pub const ERROR_DS_NC_STILL_HAS_DSAS: WIN32_ERROR = WIN32_ERROR(8546u32);
pub const ERROR_DS_NONEXISTENT_MAY_HAVE: WIN32_ERROR = WIN32_ERROR(8387u32);
pub const ERROR_DS_NONEXISTENT_MUST_HAVE: WIN32_ERROR = WIN32_ERROR(8388u32);
pub const ERROR_DS_NONEXISTENT_POSS_SUP: WIN32_ERROR = WIN32_ERROR(8390u32);
pub const ERROR_DS_NONSAFE_SCHEMA_CHANGE: WIN32_ERROR = WIN32_ERROR(8508u32);
pub const ERROR_DS_NON_ASQ_SEARCH: WIN32_ERROR = WIN32_ERROR(8624u32);
pub const ERROR_DS_NON_BASE_SEARCH: WIN32_ERROR = WIN32_ERROR(8480u32);
pub const ERROR_DS_NOTIFY_FILTER_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(8377u32);
pub const ERROR_DS_NOT_AN_OBJECT: WIN32_ERROR = WIN32_ERROR(8352u32);
pub const ERROR_DS_NOT_AUTHORITIVE_FOR_DST_NC: WIN32_ERROR = WIN32_ERROR(8487u32);
pub const ERROR_DS_NOT_CLOSEST: WIN32_ERROR = WIN32_ERROR(8588u32);
pub const ERROR_DS_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(8200u32);
pub const ERROR_DS_NOT_ON_BACKLINK: WIN32_ERROR = WIN32_ERROR(8362u32);
pub const ERROR_DS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8256u32);
pub const ERROR_DS_NOT_SUPPORTED_SORT_ORDER: WIN32_ERROR = WIN32_ERROR(8570u32);
pub const ERROR_DS_NO_ATTRIBUTE_OR_VALUE: WIN32_ERROR = WIN32_ERROR(8202u32);
pub const ERROR_DS_NO_BEHAVIOR_VERSION_IN_MIXEDDOMAIN: WIN32_ERROR = WIN32_ERROR(8569u32);
pub const ERROR_DS_NO_CHAINED_EVAL: WIN32_ERROR = WIN32_ERROR(8328u32);
pub const ERROR_DS_NO_CHAINING: WIN32_ERROR = WIN32_ERROR(8327u32);
pub const ERROR_DS_NO_CHECKPOINT_WITH_PDC: WIN32_ERROR = WIN32_ERROR(8551u32);
pub const ERROR_DS_NO_CROSSREF_FOR_NC: WIN32_ERROR = WIN32_ERROR(8363u32);
pub const ERROR_DS_NO_DELETED_NAME: WIN32_ERROR = WIN32_ERROR(8355u32);
pub const ERROR_DS_NO_FPO_IN_UNIVERSAL_GROUPS: WIN32_ERROR = WIN32_ERROR(8549u32);
pub const ERROR_DS_NO_MORE_RIDS: WIN32_ERROR = WIN32_ERROR(8209u32);
pub const ERROR_DS_NO_MSDS_INTID: WIN32_ERROR = WIN32_ERROR(8596u32);
pub const ERROR_DS_NO_NEST_GLOBALGROUP_IN_MIXEDDOMAIN: WIN32_ERROR = WIN32_ERROR(8514u32);
pub const ERROR_DS_NO_NEST_LOCALGROUP_IN_MIXEDDOMAIN: WIN32_ERROR = WIN32_ERROR(8515u32);
pub const ERROR_DS_NO_NTDSA_OBJECT: WIN32_ERROR = WIN32_ERROR(8623u32);
pub const ERROR_DS_NO_OBJECT_MOVE_IN_SCHEMA_NC: WIN32_ERROR = WIN32_ERROR(8580u32);
pub const ERROR_DS_NO_PARENT_OBJECT: WIN32_ERROR = WIN32_ERROR(8329u32);
pub const ERROR_DS_NO_PKT_PRIVACY_ON_CONNECTION: WIN32_ERROR = WIN32_ERROR(8533u32);
pub const ERROR_DS_NO_RDN_DEFINED_IN_SCHEMA: WIN32_ERROR = WIN32_ERROR(8306u32);
pub const ERROR_DS_NO_REF_DOMAIN: WIN32_ERROR = WIN32_ERROR(8575u32);
pub const ERROR_DS_NO_REQUESTED_ATTS_FOUND: WIN32_ERROR = WIN32_ERROR(8308u32);
pub const ERROR_DS_NO_RESULTS_RETURNED: WIN32_ERROR = WIN32_ERROR(8257u32);
pub const ERROR_DS_NO_RIDS_ALLOCATED: WIN32_ERROR = WIN32_ERROR(8208u32);
pub const ERROR_DS_NO_SERVER_OBJECT: WIN32_ERROR = WIN32_ERROR(8622u32);
pub const ERROR_DS_NO_SUCH_OBJECT: WIN32_ERROR = WIN32_ERROR(8240u32);
pub const ERROR_DS_NO_TREE_DELETE_ABOVE_NC: WIN32_ERROR = WIN32_ERROR(8501u32);
pub const ERROR_DS_NTDSCRIPT_PROCESS_ERROR: WIN32_ERROR = WIN32_ERROR(8592u32);
pub const ERROR_DS_NTDSCRIPT_SYNTAX_ERROR: WIN32_ERROR = WIN32_ERROR(8591u32);
pub const ERROR_DS_OBJECT_BEING_REMOVED: WIN32_ERROR = WIN32_ERROR(8339u32);
pub const ERROR_DS_OBJECT_CLASS_REQUIRED: WIN32_ERROR = WIN32_ERROR(8315u32);
pub const ERROR_DS_OBJECT_RESULTS_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(8248u32);
pub const ERROR_DS_OBJ_CLASS_NOT_DEFINED: WIN32_ERROR = WIN32_ERROR(8371u32);
pub const ERROR_DS_OBJ_CLASS_NOT_SUBCLASS: WIN32_ERROR = WIN32_ERROR(8372u32);
pub const ERROR_DS_OBJ_CLASS_VIOLATION: WIN32_ERROR = WIN32_ERROR(8212u32);
pub const ERROR_DS_OBJ_GUID_EXISTS: WIN32_ERROR = WIN32_ERROR(8361u32);
pub const ERROR_DS_OBJ_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8333u32);
pub const ERROR_DS_OBJ_STRING_NAME_EXISTS: WIN32_ERROR = WIN32_ERROR(8305u32);
pub const ERROR_DS_OBJ_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(8312u32);
pub const ERROR_DS_OFFSET_RANGE_ERROR: WIN32_ERROR = WIN32_ERROR(8262u32);
pub const ERROR_DS_OID_MAPPED_GROUP_CANT_HAVE_MEMBERS: WIN32_ERROR = WIN32_ERROR(8637u32);
pub const ERROR_DS_OID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8638u32);
pub const ERROR_DS_OPERATIONS_ERROR: WIN32_ERROR = WIN32_ERROR(8224u32);
pub const ERROR_DS_OUT_OF_SCOPE: WIN32_ERROR = WIN32_ERROR(8338u32);
pub const ERROR_DS_OUT_OF_VERSION_STORE: WIN32_ERROR = WIN32_ERROR(8573u32);
pub const ERROR_DS_PARAM_ERROR: WIN32_ERROR = WIN32_ERROR(8255u32);
pub const ERROR_DS_PARENT_IS_AN_ALIAS: WIN32_ERROR = WIN32_ERROR(8330u32);
pub const ERROR_DS_PDC_OPERATION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(8490u32);
pub const ERROR_DS_PER_ATTRIBUTE_AUTHZ_FAILED_DURING_ADD: WIN32_ERROR = WIN32_ERROR(8652u32);
pub const ERROR_DS_POLICY_NOT_KNOWN: WIN32_ERROR = WIN32_ERROR(8618u32);
pub const ERROR_DS_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(8225u32);
pub const ERROR_DS_RANGE_CONSTRAINT: WIN32_ERROR = WIN32_ERROR(8322u32);
pub const ERROR_DS_RDN_DOESNT_MATCH_SCHEMA: WIN32_ERROR = WIN32_ERROR(8307u32);
pub const ERROR_DS_RECALCSCHEMA_FAILED: WIN32_ERROR = WIN32_ERROR(8396u32);
pub const ERROR_DS_REFERRAL: WIN32_ERROR = WIN32_ERROR(8235u32);
pub const ERROR_DS_REFERRAL_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8260u32);
pub const ERROR_DS_REFUSING_FSMO_ROLES: WIN32_ERROR = WIN32_ERROR(8433u32);
pub const ERROR_DS_REMOTE_CROSSREF_OP_FAILED: WIN32_ERROR = WIN32_ERROR(8601u32);
pub const ERROR_DS_REPLICATOR_ONLY: WIN32_ERROR = WIN32_ERROR(8370u32);
pub const ERROR_DS_REPLICA_SET_CHANGE_NOT_ALLOWED_ON_DISABLED_CR: WIN32_ERROR = WIN32_ERROR(8595u32);
pub const ERROR_DS_REPL_LIFETIME_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8614u32);
pub const ERROR_DS_RESERVED_LINK_ID: WIN32_ERROR = WIN32_ERROR(8576u32);
pub const ERROR_DS_RESERVED_MAPI_ID: WIN32_ERROR = WIN32_ERROR(8631u32);
pub const ERROR_DS_RIDMGR_DISABLED: WIN32_ERROR = WIN32_ERROR(8263u32);
pub const ERROR_DS_RIDMGR_INIT_ERROR: WIN32_ERROR = WIN32_ERROR(8211u32);
pub const ERROR_DS_ROLE_NOT_VERIFIED: WIN32_ERROR = WIN32_ERROR(8610u32);
pub const ERROR_DS_ROOT_CANT_BE_SUBREF: WIN32_ERROR = WIN32_ERROR(8326u32);
pub const ERROR_DS_ROOT_MUST_BE_NC: WIN32_ERROR = WIN32_ERROR(8301u32);
pub const ERROR_DS_ROOT_REQUIRES_CLASS_TOP: WIN32_ERROR = WIN32_ERROR(8432u32);
pub const ERROR_DS_SAM_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8504u32);
pub const ERROR_DS_SAM_INIT_FAILURE_CONSOLE: WIN32_ERROR = WIN32_ERROR(8562u32);
pub const ERROR_DS_SAM_NEED_BOOTKEY_FLOPPY: WIN32_ERROR = WIN32_ERROR(8530u32);
pub const ERROR_DS_SAM_NEED_BOOTKEY_PASSWORD: WIN32_ERROR = WIN32_ERROR(8529u32);
pub const ERROR_DS_SCHEMA_ALLOC_FAILED: WIN32_ERROR = WIN32_ERROR(8415u32);
pub const ERROR_DS_SCHEMA_NOT_LOADED: WIN32_ERROR = WIN32_ERROR(8414u32);
pub const ERROR_DS_SCHEMA_UPDATE_DISALLOWED: WIN32_ERROR = WIN32_ERROR(8509u32);
pub const ERROR_DS_SECURITY_CHECKING_ERROR: WIN32_ERROR = WIN32_ERROR(8413u32);
pub const ERROR_DS_SECURITY_ILLEGAL_MODIFY: WIN32_ERROR = WIN32_ERROR(8423u32);
pub const ERROR_DS_SEC_DESC_INVALID: WIN32_ERROR = WIN32_ERROR(8354u32);
pub const ERROR_DS_SEC_DESC_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(8353u32);
pub const ERROR_DS_SEMANTIC_ATT_TEST: WIN32_ERROR = WIN32_ERROR(8383u32);
pub const ERROR_DS_SENSITIVE_GROUP_VIOLATION: WIN32_ERROR = WIN32_ERROR(8505u32);
pub const ERROR_DS_SERVER_DOWN: WIN32_ERROR = WIN32_ERROR(8250u32);
pub const ERROR_DS_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(8364u32);
pub const ERROR_DS_SINGLE_USER_MODE_FAILED: WIN32_ERROR = WIN32_ERROR(8590u32);
pub const ERROR_DS_SINGLE_VALUE_CONSTRAINT: WIN32_ERROR = WIN32_ERROR(8321u32);
pub const ERROR_DS_SIZELIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8227u32);
pub const ERROR_DS_SORT_CONTROL_MISSING: WIN32_ERROR = WIN32_ERROR(8261u32);
pub const ERROR_DS_SOURCE_AUDITING_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(8552u32);
pub const ERROR_DS_SOURCE_DOMAIN_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8534u32);
pub const ERROR_DS_SPN_VALUE_NOT_UNIQUE_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8647u32);
pub const ERROR_DS_SRC_AND_DST_NC_IDENTICAL: WIN32_ERROR = WIN32_ERROR(8485u32);
pub const ERROR_DS_SRC_AND_DST_OBJECT_CLASS_MISMATCH: WIN32_ERROR = WIN32_ERROR(8540u32);
pub const ERROR_DS_SRC_DC_MUST_BE_SP4_OR_GREATER: WIN32_ERROR = WIN32_ERROR(8559u32);
pub const ERROR_DS_SRC_GUID_MISMATCH: WIN32_ERROR = WIN32_ERROR(8488u32);
pub const ERROR_DS_SRC_NAME_MISMATCH: WIN32_ERROR = WIN32_ERROR(8484u32);
pub const ERROR_DS_SRC_OBJ_NOT_GROUP_OR_USER: WIN32_ERROR = WIN32_ERROR(8538u32);
pub const ERROR_DS_SRC_SID_EXISTS_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8539u32);
pub const ERROR_DS_STRING_SD_CONVERSION_FAILED: WIN32_ERROR = WIN32_ERROR(8522u32);
pub const ERROR_DS_STRONG_AUTH_REQUIRED: WIN32_ERROR = WIN32_ERROR(8232u32);
pub const ERROR_DS_SUBREF_MUST_HAVE_PARENT: WIN32_ERROR = WIN32_ERROR(8356u32);
pub const ERROR_DS_SUBTREE_NOTIFY_NOT_NC_HEAD: WIN32_ERROR = WIN32_ERROR(8376u32);
pub const ERROR_DS_SUB_CLS_TEST_FAIL: WIN32_ERROR = WIN32_ERROR(8391u32);
pub const ERROR_DS_SYNTAX_MISMATCH: WIN32_ERROR = WIN32_ERROR(8384u32);
pub const ERROR_DS_THREAD_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8587u32);
pub const ERROR_DS_TIMELIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8226u32);
pub const ERROR_DS_TREE_DELETE_NOT_FINISHED: WIN32_ERROR = WIN32_ERROR(8397u32);
pub const ERROR_DS_UNABLE_TO_SURRENDER_ROLES: WIN32_ERROR = WIN32_ERROR(8435u32);
pub const ERROR_DS_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(8207u32);
pub const ERROR_DS_UNAVAILABLE_CRIT_EXTENSION: WIN32_ERROR = WIN32_ERROR(8236u32);
pub const ERROR_DS_UNDELETE_SAM_VALIDATION_FAILED: WIN32_ERROR = WIN32_ERROR(8645u32);
pub const ERROR_DS_UNICODEPWD_NOT_IN_QUOTES: WIN32_ERROR = WIN32_ERROR(8556u32);
pub const ERROR_DS_UNIVERSAL_CANT_HAVE_LOCAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8518u32);
pub const ERROR_DS_UNKNOWN_ERROR: WIN32_ERROR = WIN32_ERROR(8431u32);
pub const ERROR_DS_UNKNOWN_OPERATION: WIN32_ERROR = WIN32_ERROR(8365u32);
pub const ERROR_DS_UNWILLING_TO_PERFORM: WIN32_ERROR = WIN32_ERROR(8245u32);
pub const ERROR_DS_UPN_VALUE_NOT_UNIQUE_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8648u32);
pub const ERROR_DS_USER_BUFFER_TO_SMALL: WIN32_ERROR = WIN32_ERROR(8309u32);
pub const ERROR_DS_VALUE_KEY_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8650u32);
pub const ERROR_DS_VERSION_CHECK_FAILURE: WIN32_ERROR = WIN32_ERROR(643u32);
pub const ERROR_DS_WKO_CONTAINER_CANNOT_BE_SPECIAL: WIN32_ERROR = WIN32_ERROR(8611u32);
pub const ERROR_DS_WRONG_LINKED_ATT_SYNTAX: WIN32_ERROR = WIN32_ERROR(8528u32);
pub const ERROR_DS_WRONG_OM_OBJ_CLASS: WIN32_ERROR = WIN32_ERROR(8476u32);
pub const ERROR_DUPLICATE_FOUND: WIN32_ERROR = WIN32_ERROR(3758096898u32);
pub const ERROR_DUPLICATE_PRIVILEGES: WIN32_ERROR = WIN32_ERROR(311u32);
pub const ERROR_DUPLICATE_SERVICE_NAME: WIN32_ERROR = WIN32_ERROR(1078u32);
pub const ERROR_DUPLICATE_TAG: WIN32_ERROR = WIN32_ERROR(2014u32);
pub const ERROR_DUP_DOMAINNAME: WIN32_ERROR = WIN32_ERROR(1221u32);
pub const ERROR_DUP_NAME: WIN32_ERROR = WIN32_ERROR(52u32);
pub const ERROR_DYNAMIC_CODE_BLOCKED: WIN32_ERROR = WIN32_ERROR(1655u32);
pub const ERROR_DYNLINK_FROM_INVALID_RING: WIN32_ERROR = WIN32_ERROR(196u32);
pub const ERROR_EAS_DIDNT_FIT: WIN32_ERROR = WIN32_ERROR(275u32);
pub const ERROR_EAS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(282u32);
pub const ERROR_EA_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(994u32);
pub const ERROR_EA_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(276u32);
pub const ERROR_EA_LIST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(255u32);
pub const ERROR_EA_TABLE_FULL: WIN32_ERROR = WIN32_ERROR(277u32);
pub const ERROR_EC_CIRCULAR_FORWARDING: WIN32_ERROR = WIN32_ERROR(15082u32);
pub const ERROR_EC_CREDSTORE_FULL: WIN32_ERROR = WIN32_ERROR(15083u32);
pub const ERROR_EC_CRED_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15084u32);
pub const ERROR_EC_LOG_DISABLED: WIN32_ERROR = WIN32_ERROR(15081u32);
pub const ERROR_EC_NO_ACTIVE_CHANNEL: WIN32_ERROR = WIN32_ERROR(15085u32);
pub const ERROR_EC_SUBSCRIPTION_CANNOT_ACTIVATE: WIN32_ERROR = WIN32_ERROR(15080u32);
pub const ERROR_EDP_DPL_POLICY_CANT_BE_SATISFIED: WIN32_ERROR = WIN32_ERROR(357u32);
pub const ERROR_EDP_POLICY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(356u32);
pub const ERROR_EFS_ALG_BLOB_TOO_BIG: WIN32_ERROR = WIN32_ERROR(6013u32);
pub const ERROR_EFS_DISABLED: WIN32_ERROR = WIN32_ERROR(6015u32);
pub const ERROR_EFS_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6831u32);
pub const ERROR_EFS_SERVER_NOT_TRUSTED: WIN32_ERROR = WIN32_ERROR(6011u32);
pub const ERROR_EFS_VERSION_NOT_SUPPORT: WIN32_ERROR = WIN32_ERROR(6016u32);
pub const ERROR_ELEVATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(740u32);
pub const ERROR_EMPTY: WIN32_ERROR = WIN32_ERROR(4306u32);
pub const ERROR_ENCLAVE_FAILURE: WIN32_ERROR = WIN32_ERROR(349u32);
pub const ERROR_ENCLAVE_NOT_TERMINATED: WIN32_ERROR = WIN32_ERROR(814u32);
pub const ERROR_ENCLAVE_VIOLATION: WIN32_ERROR = WIN32_ERROR(815u32);
pub const ERROR_ENCRYPTED_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(489u32);
pub const ERROR_ENCRYPTED_IO_NOT_POSSIBLE: WIN32_ERROR = WIN32_ERROR(808u32);
pub const ERROR_ENCRYPTING_METADATA_DISALLOWED: WIN32_ERROR = WIN32_ERROR(431u32);
pub const ERROR_ENCRYPTION_DISABLED: WIN32_ERROR = WIN32_ERROR(430u32);
pub const ERROR_ENCRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6000u32);
pub const ERROR_ENCRYPTION_POLICY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(6022u32);
pub const ERROR_END_OF_MEDIA: WIN32_ERROR = WIN32_ERROR(1100u32);
pub const ERROR_ENLISTMENT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6717u32);
pub const ERROR_ENLISTMENT_NOT_SUPERIOR: WIN32_ERROR = WIN32_ERROR(6820u32);
pub const ERROR_ENVVAR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(203u32);
pub const ERROR_EOM_OVERFLOW: WIN32_ERROR = WIN32_ERROR(1129u32);
pub const ERROR_ERRORS_ENCOUNTERED: WIN32_ERROR = WIN32_ERROR(774u32);
pub const ERROR_EVALUATION_EXPIRATION: WIN32_ERROR = WIN32_ERROR(622u32);
pub const ERROR_EVENTLOG_CANT_START: WIN32_ERROR = WIN32_ERROR(1501u32);
pub const ERROR_EVENTLOG_FILE_CHANGED: WIN32_ERROR = WIN32_ERROR(1503u32);
pub const ERROR_EVENTLOG_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(1500u32);
pub const ERROR_EVENT_DONE: WIN32_ERROR = WIN32_ERROR(710u32);
pub const ERROR_EVENT_PENDING: WIN32_ERROR = WIN32_ERROR(711u32);
pub const ERROR_EVT_CANNOT_OPEN_CHANNEL_OF_QUERY: WIN32_ERROR = WIN32_ERROR(15036u32);
pub const ERROR_EVT_CHANNEL_CANNOT_ACTIVATE: WIN32_ERROR = WIN32_ERROR(15025u32);
pub const ERROR_EVT_CHANNEL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15007u32);
pub const ERROR_EVT_CONFIGURATION_ERROR: WIN32_ERROR = WIN32_ERROR(15010u32);
pub const ERROR_EVT_EVENT_DEFINITION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15032u32);
pub const ERROR_EVT_EVENT_TEMPLATE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15003u32);
pub const ERROR_EVT_FILTER_ALREADYSCOPED: WIN32_ERROR = WIN32_ERROR(15014u32);
pub const ERROR_EVT_FILTER_INVARG: WIN32_ERROR = WIN32_ERROR(15016u32);
pub const ERROR_EVT_FILTER_INVTEST: WIN32_ERROR = WIN32_ERROR(15017u32);
pub const ERROR_EVT_FILTER_INVTYPE: WIN32_ERROR = WIN32_ERROR(15018u32);
pub const ERROR_EVT_FILTER_NOTELTSET: WIN32_ERROR = WIN32_ERROR(15015u32);
pub const ERROR_EVT_FILTER_OUT_OF_RANGE: WIN32_ERROR = WIN32_ERROR(15038u32);
pub const ERROR_EVT_FILTER_PARSEERR: WIN32_ERROR = WIN32_ERROR(15019u32);
pub const ERROR_EVT_FILTER_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(15026u32);
pub const ERROR_EVT_FILTER_UNEXPECTEDTOKEN: WIN32_ERROR = WIN32_ERROR(15021u32);
pub const ERROR_EVT_FILTER_UNSUPPORTEDOP: WIN32_ERROR = WIN32_ERROR(15020u32);
pub const ERROR_EVT_INVALID_CHANNEL_PATH: WIN32_ERROR = WIN32_ERROR(15000u32);
pub const ERROR_EVT_INVALID_CHANNEL_PROPERTY_VALUE: WIN32_ERROR = WIN32_ERROR(15023u32);
pub const ERROR_EVT_INVALID_EVENT_DATA: WIN32_ERROR = WIN32_ERROR(15005u32);
pub const ERROR_EVT_INVALID_OPERATION_OVER_ENABLED_DIRECT_CHANNEL: WIN32_ERROR = WIN32_ERROR(15022u32);
pub const ERROR_EVT_INVALID_PUBLISHER_NAME: WIN32_ERROR = WIN32_ERROR(15004u32);
pub const ERROR_EVT_INVALID_PUBLISHER_PROPERTY_VALUE: WIN32_ERROR = WIN32_ERROR(15024u32);
pub const ERROR_EVT_INVALID_QUERY: WIN32_ERROR = WIN32_ERROR(15001u32);
pub const ERROR_EVT_MALFORMED_XML_TEXT: WIN32_ERROR = WIN32_ERROR(15008u32);
pub const ERROR_EVT_MAX_INSERTS_REACHED: WIN32_ERROR = WIN32_ERROR(15031u32);
pub const ERROR_EVT_MESSAGE_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15028u32);
pub const ERROR_EVT_MESSAGE_LOCALE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15033u32);
pub const ERROR_EVT_MESSAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15027u32);
pub const ERROR_EVT_NON_VALIDATING_MSXML: WIN32_ERROR = WIN32_ERROR(15013u32);
pub const ERROR_EVT_PUBLISHER_DISABLED: WIN32_ERROR = WIN32_ERROR(15037u32);
pub const ERROR_EVT_PUBLISHER_METADATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15002u32);
pub const ERROR_EVT_QUERY_RESULT_INVALID_POSITION: WIN32_ERROR = WIN32_ERROR(15012u32);
pub const ERROR_EVT_QUERY_RESULT_STALE: WIN32_ERROR = WIN32_ERROR(15011u32);
pub const ERROR_EVT_SUBSCRIPTION_TO_DIRECT_CHANNEL: WIN32_ERROR = WIN32_ERROR(15009u32);
pub const ERROR_EVT_UNRESOLVED_PARAMETER_INSERT: WIN32_ERROR = WIN32_ERROR(15030u32);
pub const ERROR_EVT_UNRESOLVED_VALUE_INSERT: WIN32_ERROR = WIN32_ERROR(15029u32);
pub const ERROR_EVT_VERSION_TOO_NEW: WIN32_ERROR = WIN32_ERROR(15035u32);
pub const ERROR_EVT_VERSION_TOO_OLD: WIN32_ERROR = WIN32_ERROR(15034u32);
pub const ERROR_EXCEPTION_IN_RESOURCE_CALL: WIN32_ERROR = WIN32_ERROR(5930u32);
pub const ERROR_EXCEPTION_IN_SERVICE: WIN32_ERROR = WIN32_ERROR(1064u32);
pub const ERROR_EXCL_SEM_ALREADY_OWNED: WIN32_ERROR = WIN32_ERROR(101u32);
pub const ERROR_EXE_CANNOT_MODIFY_SIGNED_BINARY: WIN32_ERROR = WIN32_ERROR(217u32);
pub const ERROR_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY: WIN32_ERROR = WIN32_ERROR(218u32);
pub const ERROR_EXE_MACHINE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(216u32);
pub const ERROR_EXE_MARKED_INVALID: WIN32_ERROR = WIN32_ERROR(192u32);
pub const ERROR_EXPECTED_SECTION_NAME: WIN32_ERROR = WIN32_ERROR(3758096384u32);
pub const ERROR_EXPIRED_HANDLE: WIN32_ERROR = WIN32_ERROR(6854u32);
pub const ERROR_EXTENDED_ERROR: WIN32_ERROR = WIN32_ERROR(1208u32);
pub const ERROR_EXTERNAL_BACKING_PROVIDER_UNKNOWN: WIN32_ERROR = WIN32_ERROR(343u32);
pub const ERROR_EXTERNAL_SYSKEY_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(399u32);
pub const ERROR_EXTRANEOUS_INFORMATION: WIN32_ERROR = WIN32_ERROR(677u32);
pub const ERROR_FAILED_DRIVER_ENTRY: WIN32_ERROR = WIN32_ERROR(647u32);
pub const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: WIN32_ERROR = WIN32_ERROR(1063u32);
pub const ERROR_FAIL_FAST_EXCEPTION: WIN32_ERROR = WIN32_ERROR(1653u32);
pub const ERROR_FAIL_I24: WIN32_ERROR = WIN32_ERROR(83u32);
pub const ERROR_FAIL_NOACTION_REBOOT: WIN32_ERROR = WIN32_ERROR(350u32);
pub const ERROR_FAIL_REBOOT_INITIATED: WIN32_ERROR = WIN32_ERROR(3018u32);
pub const ERROR_FAIL_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(3017u32);
pub const ERROR_FAIL_RESTART: WIN32_ERROR = WIN32_ERROR(352u32);
pub const ERROR_FAIL_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(351u32);
pub const ERROR_FATAL_APP_EXIT: WIN32_ERROR = WIN32_ERROR(713u32);
pub const ERROR_FILEMARK_DETECTED: WIN32_ERROR = WIN32_ERROR(1101u32);
pub const ERROR_FILENAME_EXCED_RANGE: WIN32_ERROR = WIN32_ERROR(206u32);
pub const ERROR_FILEQUEUE_LOCKED: WIN32_ERROR = WIN32_ERROR(3758096918u32);
pub const ERROR_FILE_CHECKED_OUT: WIN32_ERROR = WIN32_ERROR(220u32);
pub const ERROR_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(1392u32);
pub const ERROR_FILE_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(6002u32);
pub const ERROR_FILE_EXISTS: WIN32_ERROR = WIN32_ERROR(80u32);
pub const ERROR_FILE_HANDLE_REVOKED: WIN32_ERROR = WIN32_ERROR(806u32);
pub const ERROR_FILE_HASH_NOT_IN_CATALOG: WIN32_ERROR = WIN32_ERROR(3758096971u32);
pub const ERROR_FILE_IDENTITY_NOT_PERSISTENT: WIN32_ERROR = WIN32_ERROR(6823u32);
pub const ERROR_FILE_INVALID: WIN32_ERROR = WIN32_ERROR(1006u32);
pub const ERROR_FILE_LEVEL_TRIM_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(326u32);
pub const ERROR_FILE_METADATA_OPTIMIZATION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(809u32);
pub const ERROR_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(6007u32);
pub const ERROR_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2u32);
pub const ERROR_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(425u32);
pub const ERROR_FILE_OFFLINE: WIN32_ERROR = WIN32_ERROR(4350u32);
pub const ERROR_FILE_PROTECTED_UNDER_DPL: WIN32_ERROR = WIN32_ERROR(406u32);
pub const ERROR_FILE_READ_ONLY: WIN32_ERROR = WIN32_ERROR(6009u32);
pub const ERROR_FILE_SHARE_RESOURCE_CONFLICT: WIN32_ERROR = WIN32_ERROR(5938u32);
pub const ERROR_FILE_SNAP_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(440u32);
pub const ERROR_FILE_SNAP_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(435u32);
pub const ERROR_FILE_SNAP_IO_NOT_COORDINATED: WIN32_ERROR = WIN32_ERROR(438u32);
pub const ERROR_FILE_SNAP_MODIFY_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(437u32);
pub const ERROR_FILE_SNAP_UNEXPECTED_ERROR: WIN32_ERROR = WIN32_ERROR(439u32);
pub const ERROR_FILE_SNAP_USER_SECTION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(436u32);
pub const ERROR_FILE_SYSTEM_LIMITATION: WIN32_ERROR = WIN32_ERROR(665u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_BUSY: WIN32_ERROR = WIN32_ERROR(371u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_INVALID_OPERATION: WIN32_ERROR = WIN32_ERROR(385u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(370u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_PROVIDER_UNKNOWN: WIN32_ERROR = WIN32_ERROR(372u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(369u32);
pub const ERROR_FILE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(223u32);
pub const ERROR_FIRMWARE_UPDATED: WIN32_ERROR = WIN32_ERROR(728u32);
pub const ERROR_FLOATED_SECTION: WIN32_ERROR = WIN32_ERROR(6846u32);
pub const ERROR_FLOAT_MULTIPLE_FAULTS: WIN32_ERROR = WIN32_ERROR(630u32);
pub const ERROR_FLOAT_MULTIPLE_TRAPS: WIN32_ERROR = WIN32_ERROR(631u32);
pub const ERROR_FLOPPY_BAD_REGISTERS: WIN32_ERROR = WIN32_ERROR(1125u32);
pub const ERROR_FLOPPY_ID_MARK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1122u32);
pub const ERROR_FLOPPY_UNKNOWN_ERROR: WIN32_ERROR = WIN32_ERROR(1124u32);
pub const ERROR_FLOPPY_VOLUME: WIN32_ERROR = WIN32_ERROR(584u32);
pub const ERROR_FLOPPY_WRONG_CYLINDER: WIN32_ERROR = WIN32_ERROR(1123u32);
pub const ERROR_FLT_ALREADY_ENLISTED: windows_core::HRESULT = windows_core::HRESULT(0x801F001B_u32 as _);
pub const ERROR_FLT_CBDQ_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x801F000E_u32 as _);
pub const ERROR_FLT_CONTEXT_ALLOCATION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x801F0016_u32 as _);
pub const ERROR_FLT_CONTEXT_ALREADY_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x801F0002_u32 as _);
pub const ERROR_FLT_CONTEXT_ALREADY_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x801F001C_u32 as _);
pub const ERROR_FLT_DELETING_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x801F000B_u32 as _);
pub const ERROR_FLT_DISALLOW_FAST_IO: windows_core::HRESULT = windows_core::HRESULT(0x801F0004_u32 as _);
pub const ERROR_FLT_DO_NOT_ATTACH: windows_core::HRESULT = windows_core::HRESULT(0x801F000F_u32 as _);
pub const ERROR_FLT_DO_NOT_DETACH: windows_core::HRESULT = windows_core::HRESULT(0x801F0010_u32 as _);
pub const ERROR_FLT_DUPLICATE_ENTRY: windows_core::HRESULT = windows_core::HRESULT(0x801F000D_u32 as _);
pub const ERROR_FLT_FILTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x801F0013_u32 as _);
pub const ERROR_FLT_FILTER_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x801F0008_u32 as _);
pub const ERROR_FLT_INSTANCE_ALTITUDE_COLLISION: windows_core::HRESULT = windows_core::HRESULT(0x801F0011_u32 as _);
pub const ERROR_FLT_INSTANCE_NAME_COLLISION: windows_core::HRESULT = windows_core::HRESULT(0x801F0012_u32 as _);
pub const ERROR_FLT_INSTANCE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x801F0015_u32 as _);
pub const ERROR_FLT_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x801F000A_u32 as _);
pub const ERROR_FLT_INVALID_ASYNCHRONOUS_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x801F0003_u32 as _);
pub const ERROR_FLT_INVALID_CONTEXT_REGISTRATION: windows_core::HRESULT = windows_core::HRESULT(0x801F0017_u32 as _);
pub const ERROR_FLT_INVALID_NAME_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x801F0005_u32 as _);
pub const ERROR_FLT_IO_COMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x1F0001_u32 as _);
pub const ERROR_FLT_MUST_BE_NONPAGED_POOL: windows_core::HRESULT = windows_core::HRESULT(0x801F000C_u32 as _);
pub const ERROR_FLT_NAME_CACHE_MISS: windows_core::HRESULT = windows_core::HRESULT(0x801F0018_u32 as _);
pub const ERROR_FLT_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x801F0007_u32 as _);
pub const ERROR_FLT_NOT_SAFE_TO_POST_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x801F0006_u32 as _);
pub const ERROR_FLT_NO_DEVICE_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x801F0019_u32 as _);
pub const ERROR_FLT_NO_HANDLER_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x801F0001_u32 as _);
pub const ERROR_FLT_NO_WAITER_FOR_REPLY: windows_core::HRESULT = windows_core::HRESULT(0x801F0020_u32 as _);
pub const ERROR_FLT_POST_OPERATION_CLEANUP: windows_core::HRESULT = windows_core::HRESULT(0x801F0009_u32 as _);
pub const ERROR_FLT_REGISTRATION_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x801F0023_u32 as _);
pub const ERROR_FLT_VOLUME_ALREADY_MOUNTED: windows_core::HRESULT = windows_core::HRESULT(0x801F001A_u32 as _);
pub const ERROR_FLT_VOLUME_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x801F0014_u32 as _);
pub const ERROR_FLT_WCOS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x801F0024_u32 as _);
pub const ERROR_FORMS_AUTH_REQUIRED: WIN32_ERROR = WIN32_ERROR(224u32);
pub const ERROR_FOUND_OUT_OF_SCOPE: WIN32_ERROR = WIN32_ERROR(601u32);
pub const ERROR_FSFILTER_OP_COMPLETED_SUCCESSFULLY: WIN32_ERROR = WIN32_ERROR(762u32);
pub const ERROR_FS_DRIVER_REQUIRED: WIN32_ERROR = WIN32_ERROR(588u32);
pub const ERROR_FS_GUID_MISMATCH: WIN32_ERROR = WIN32_ERROR(477u32);
pub const ERROR_FS_METADATA_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(510u32);
pub const ERROR_FT_DI_SCAN_REQUIRED: WIN32_ERROR = WIN32_ERROR(339u32);
pub const ERROR_FT_READ_FAILURE: WIN32_ERROR = WIN32_ERROR(415u32);
pub const ERROR_FT_READ_FROM_COPY_FAILURE: WIN32_ERROR = WIN32_ERROR(818u32);
pub const ERROR_FT_READ_RECOVERY_FROM_BACKUP: WIN32_ERROR = WIN32_ERROR(704u32);
pub const ERROR_FT_WRITE_FAILURE: WIN32_ERROR = WIN32_ERROR(338u32);
pub const ERROR_FT_WRITE_RECOVERY: WIN32_ERROR = WIN32_ERROR(705u32);
pub const ERROR_FULLSCREEN_MODE: WIN32_ERROR = WIN32_ERROR(1007u32);
pub const ERROR_FULL_BACKUP: WIN32_ERROR = WIN32_ERROR(4004u32);
pub const ERROR_FUNCTION_FAILED: WIN32_ERROR = WIN32_ERROR(1627u32);
pub const ERROR_FUNCTION_NOT_CALLED: WIN32_ERROR = WIN32_ERROR(1626u32);
pub const ERROR_GDI_HANDLE_LEAK: WIN32_ERROR = WIN32_ERROR(373u32);
pub const ERROR_GENERAL_SYNTAX: WIN32_ERROR = WIN32_ERROR(3758096387u32);
pub const ERROR_GENERIC_COMMAND_FAILED: WIN32_ERROR = WIN32_ERROR(14109u32);
pub const ERROR_GENERIC_NOT_MAPPED: WIN32_ERROR = WIN32_ERROR(1360u32);
pub const ERROR_GEN_FAILURE: WIN32_ERROR = WIN32_ERROR(31u32);
pub const ERROR_GLOBAL_ONLY_HOOK: WIN32_ERROR = WIN32_ERROR(1429u32);
pub const ERROR_GPIO_CLIENT_INFORMATION_INVALID: WIN32_ERROR = WIN32_ERROR(15322u32);
pub const ERROR_GPIO_INCOMPATIBLE_CONNECT_MODE: WIN32_ERROR = WIN32_ERROR(15326u32);
pub const ERROR_GPIO_INTERRUPT_ALREADY_UNMASKED: WIN32_ERROR = WIN32_ERROR(15327u32);
pub const ERROR_GPIO_INVALID_REGISTRATION_PACKET: WIN32_ERROR = WIN32_ERROR(15324u32);
pub const ERROR_GPIO_OPERATION_DENIED: WIN32_ERROR = WIN32_ERROR(15325u32);
pub const ERROR_GPIO_VERSION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15323u32);
pub const ERROR_GRACEFUL_DISCONNECT: WIN32_ERROR = WIN32_ERROR(1226u32);
pub const ERROR_GRAPHICS_ADAPTER_ACCESS_NOT_EXCLUDED: windows_core::HRESULT = windows_core::HRESULT(0xC026243B_u32 as _);
pub const ERROR_GRAPHICS_ADAPTER_CHAIN_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0xC0262433_u32 as _);
pub const ERROR_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_SOURCE: windows_core::HRESULT = windows_core::HRESULT(0xC0262328_u32 as _);
pub const ERROR_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_TARGET: windows_core::HRESULT = windows_core::HRESULT(0xC0262329_u32 as _);
pub const ERROR_GRAPHICS_ADAPTER_WAS_RESET: windows_core::HRESULT = windows_core::HRESULT(0xC0262003_u32 as _);
pub const ERROR_GRAPHICS_ALLOCATION_BUSY: windows_core::HRESULT = windows_core::HRESULT(0xC0262102_u32 as _);
pub const ERROR_GRAPHICS_ALLOCATION_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0xC0262112_u32 as _);
pub const ERROR_GRAPHICS_ALLOCATION_CONTENT_LOST: windows_core::HRESULT = windows_core::HRESULT(0xC0262116_u32 as _);
pub const ERROR_GRAPHICS_ALLOCATION_INVALID: windows_core::HRESULT = windows_core::HRESULT(0xC0262106_u32 as _);
pub const ERROR_GRAPHICS_CANCEL_VIDPN_TOPOLOGY_AUGMENTATION: windows_core::HRESULT = windows_core::HRESULT(0xC026235A_u32 as _);
pub const ERROR_GRAPHICS_CANNOTCOLORCONVERT: windows_core::HRESULT = windows_core::HRESULT(0xC0262008_u32 as _);
pub const ERROR_GRAPHICS_CANT_ACCESS_ACTIVE_VIDPN: windows_core::HRESULT = windows_core::HRESULT(0xC0262343_u32 as _);
pub const ERROR_GRAPHICS_CANT_EVICT_PINNED_ALLOCATION: windows_core::HRESULT = windows_core::HRESULT(0xC0262109_u32 as _);
pub const ERROR_GRAPHICS_CANT_LOCK_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0xC0262101_u32 as _);
pub const ERROR_GRAPHICS_CANT_RENDER_LOCKED_ALLOCATION: windows_core::HRESULT = windows_core::HRESULT(0xC0262111_u32 as _);
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_ENUMERATED: windows_core::HRESULT = windows_core::HRESULT(0xC0262432_u32 as _);
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_POWERED_ON: windows_core::HRESULT = windows_core::HRESULT(0xC0262435_u32 as _);
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_STARTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262434_u32 as _);
pub const ERROR_GRAPHICS_CHILD_DESCRIPTOR_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262401_u32 as _);
pub const ERROR_GRAPHICS_CLIENTVIDPN_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0xC026235C_u32 as _);
pub const ERROR_GRAPHICS_COPP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262501_u32 as _);
pub const ERROR_GRAPHICS_DATASET_IS_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x26234B_u32 as _);
pub const ERROR_GRAPHICS_DDCCI_CURRENT_CURRENT_VALUE_GREATER_THAN_MAXIMUM_VALUE: windows_core::HRESULT = windows_core::HRESULT(0xC02625D8_u32 as _);
pub const ERROR_GRAPHICS_DDCCI_INVALID_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC0262585_u32 as _);
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_CHECKSUM: windows_core::HRESULT = windows_core::HRESULT(0xC026258B_u32 as _);
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_COMMAND: windows_core::HRESULT = windows_core::HRESULT(0xC0262589_u32 as _);
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC026258A_u32 as _);
pub const ERROR_GRAPHICS_DDCCI_MONITOR_RETURNED_INVALID_TIMING_STATUS_BYTE: windows_core::HRESULT = windows_core::HRESULT(0xC0262586_u32 as _);
pub const ERROR_GRAPHICS_DDCCI_VCP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262584_u32 as _);
pub const ERROR_GRAPHICS_DEPENDABLE_CHILD_STATUS: windows_core::HRESULT = windows_core::HRESULT(0x4026243C_u32 as _);
pub const ERROR_GRAPHICS_DISPLAY_DEVICE_NOT_ATTACHED_TO_DESKTOP: windows_core::HRESULT = windows_core::HRESULT(0xC02625E2_u32 as _);
pub const ERROR_GRAPHICS_DRIVER_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC0262009_u32 as _);
pub const ERROR_GRAPHICS_EMPTY_ADAPTER_MONITOR_MODE_SUPPORT_INTERSECTION: windows_core::HRESULT = windows_core::HRESULT(0xC0262325_u32 as _);
pub const ERROR_GRAPHICS_FREQUENCYRANGE_ALREADY_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0xC026231F_u32 as _);
pub const ERROR_GRAPHICS_FREQUENCYRANGE_NOT_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0xC026231D_u32 as _);
pub const ERROR_GRAPHICS_GAMMA_RAMP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262348_u32 as _);
pub const ERROR_GRAPHICS_GPU_EXCEPTION_ON_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0xC0262200_u32 as _);
pub const ERROR_GRAPHICS_I2C_DEVICE_DOES_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0xC0262581_u32 as _);
pub const ERROR_GRAPHICS_I2C_ERROR_RECEIVING_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC0262583_u32 as _);
pub const ERROR_GRAPHICS_I2C_ERROR_TRANSMITTING_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC0262582_u32 as _);
pub const ERROR_GRAPHICS_I2C_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262580_u32 as _);
pub const ERROR_GRAPHICS_INCOMPATIBLE_PRIVATE_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0xC0262355_u32 as _);
pub const ERROR_GRAPHICS_INCONSISTENT_DEVICE_LINK_STATE: windows_core::HRESULT = windows_core::HRESULT(0xC0262436_u32 as _);
pub const ERROR_GRAPHICS_INDIRECT_DISPLAY_ABANDON_SWAPCHAIN: windows_core::HRESULT = windows_core::HRESULT(0xC0262012_u32 as _);
pub const ERROR_GRAPHICS_INDIRECT_DISPLAY_DEVICE_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0xC0262013_u32 as _);
pub const ERROR_GRAPHICS_INSUFFICIENT_DMA_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0xC0262001_u32 as _);
pub const ERROR_GRAPHICS_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC02625E7_u32 as _);
pub const ERROR_GRAPHICS_INVALID_ACTIVE_REGION: windows_core::HRESULT = windows_core::HRESULT(0xC026230B_u32 as _);
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0xC0262114_u32 as _);
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_INSTANCE: windows_core::HRESULT = windows_core::HRESULT(0xC0262113_u32 as _);
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_USAGE: windows_core::HRESULT = windows_core::HRESULT(0xC0262110_u32 as _);
pub const ERROR_GRAPHICS_INVALID_CLIENT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC026235B_u32 as _);
pub const ERROR_GRAPHICS_INVALID_COLORBASIS: windows_core::HRESULT = windows_core::HRESULT(0xC026233E_u32 as _);
pub const ERROR_GRAPHICS_INVALID_COPYPROTECTION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC026234F_u32 as _);
pub const ERROR_GRAPHICS_INVALID_DISPLAY_ADAPTER: windows_core::HRESULT = windows_core::HRESULT(0xC0262002_u32 as _);
pub const ERROR_GRAPHICS_INVALID_DRIVER_MODEL: windows_core::HRESULT = windows_core::HRESULT(0xC0262004_u32 as _);
pub const ERROR_GRAPHICS_INVALID_FREQUENCY: windows_core::HRESULT = windows_core::HRESULT(0xC026230A_u32 as _);
pub const ERROR_GRAPHICS_INVALID_GAMMA_RAMP: windows_core::HRESULT = windows_core::HRESULT(0xC0262347_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MODE_PRUNING_ALGORITHM: windows_core::HRESULT = windows_core::HRESULT(0xC0262356_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITORDESCRIPTOR: windows_core::HRESULT = windows_core::HRESULT(0xC026232B_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITORDESCRIPTORSET: windows_core::HRESULT = windows_core::HRESULT(0xC026232A_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITOR_CAPABILITY_ORIGIN: windows_core::HRESULT = windows_core::HRESULT(0xC0262357_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE: windows_core::HRESULT = windows_core::HRESULT(0xC026231C_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGESET: windows_core::HRESULT = windows_core::HRESULT(0xC026231B_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE_CONSTRAINT: windows_core::HRESULT = windows_core::HRESULT(0xC0262358_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITOR_SOURCEMODESET: windows_core::HRESULT = windows_core::HRESULT(0xC0262321_u32 as _);
pub const ERROR_GRAPHICS_INVALID_MONITOR_SOURCE_MODE: windows_core::HRESULT = windows_core::HRESULT(0xC0262322_u32 as _);
pub const ERROR_GRAPHICS_INVALID_PATH_CONTENT_GEOMETRY_TRANSFORMATION: windows_core::HRESULT = windows_core::HRESULT(0xC0262345_u32 as _);
pub const ERROR_GRAPHICS_INVALID_PATH_CONTENT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC026234E_u32 as _);
pub const ERROR_GRAPHICS_INVALID_PATH_IMPORTANCE_ORDINAL: windows_core::HRESULT = windows_core::HRESULT(0xC0262344_u32 as _);
pub const ERROR_GRAPHICS_INVALID_PHYSICAL_MONITOR_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0xC026258C_u32 as _);
pub const ERROR_GRAPHICS_INVALID_PIXELFORMAT: windows_core::HRESULT = windows_core::HRESULT(0xC026233D_u32 as _);
pub const ERROR_GRAPHICS_INVALID_PIXELVALUEACCESSMODE: windows_core::HRESULT = windows_core::HRESULT(0xC026233F_u32 as _);
pub const ERROR_GRAPHICS_INVALID_POINTER: windows_core::HRESULT = windows_core::HRESULT(0xC02625E4_u32 as _);
pub const ERROR_GRAPHICS_INVALID_PRIMARYSURFACE_SIZE: windows_core::HRESULT = windows_core::HRESULT(0xC026233A_u32 as _);
pub const ERROR_GRAPHICS_INVALID_SCANLINE_ORDERING: windows_core::HRESULT = windows_core::HRESULT(0xC0262352_u32 as _);
pub const ERROR_GRAPHICS_INVALID_STRIDE: windows_core::HRESULT = windows_core::HRESULT(0xC026233C_u32 as _);
pub const ERROR_GRAPHICS_INVALID_TOTAL_REGION: windows_core::HRESULT = windows_core::HRESULT(0xC026230C_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDEOPRESENTSOURCESET: windows_core::HRESULT = windows_core::HRESULT(0xC0262315_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDEOPRESENTTARGETSET: windows_core::HRESULT = windows_core::HRESULT(0xC0262316_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE: windows_core::HRESULT = windows_core::HRESULT(0xC0262304_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE_MODE: windows_core::HRESULT = windows_core::HRESULT(0xC0262310_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET: windows_core::HRESULT = windows_core::HRESULT(0xC0262305_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET_MODE: windows_core::HRESULT = windows_core::HRESULT(0xC0262311_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDPN: windows_core::HRESULT = windows_core::HRESULT(0xC0262303_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDPN_PRESENT_PATH: windows_core::HRESULT = windows_core::HRESULT(0xC0262319_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDPN_SOURCEMODESET: windows_core::HRESULT = windows_core::HRESULT(0xC0262308_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TARGETMODESET: windows_core::HRESULT = windows_core::HRESULT(0xC0262309_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TARGET_SUBSET_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC026232F_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TOPOLOGY: windows_core::HRESULT = windows_core::HRESULT(0xC0262300_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TOPOLOGY_RECOMMENDATION_REASON: windows_core::HRESULT = windows_core::HRESULT(0xC026234D_u32 as _);
pub const ERROR_GRAPHICS_INVALID_VISIBLEREGION_SIZE: windows_core::HRESULT = windows_core::HRESULT(0xC026233B_u32 as _);
pub const ERROR_GRAPHICS_LEADLINK_NOT_ENUMERATED: windows_core::HRESULT = windows_core::HRESULT(0xC0262431_u32 as _);
pub const ERROR_GRAPHICS_LEADLINK_START_DEFERRED: windows_core::HRESULT = windows_core::HRESULT(0x40262437_u32 as _);
pub const ERROR_GRAPHICS_LINK_CONFIGURATION_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC0262017_u32 as _);
pub const ERROR_GRAPHICS_MAX_NUM_PATHS_REACHED: windows_core::HRESULT = windows_core::HRESULT(0xC0262359_u32 as _);
pub const ERROR_GRAPHICS_MCA_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC0262588_u32 as _);
pub const ERROR_GRAPHICS_MCA_INVALID_CAPABILITIES_STRING: windows_core::HRESULT = windows_core::HRESULT(0xC0262587_u32 as _);
pub const ERROR_GRAPHICS_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: windows_core::HRESULT = windows_core::HRESULT(0xC02625DE_u32 as _);
pub const ERROR_GRAPHICS_MCA_INVALID_VCP_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC02625D9_u32 as _);
pub const ERROR_GRAPHICS_MCA_MCCS_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC02625DB_u32 as _);
pub const ERROR_GRAPHICS_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: windows_core::HRESULT = windows_core::HRESULT(0xC02625DA_u32 as _);
pub const ERROR_GRAPHICS_MCA_UNSUPPORTED_COLOR_TEMPERATURE: windows_core::HRESULT = windows_core::HRESULT(0xC02625DF_u32 as _);
pub const ERROR_GRAPHICS_MCA_UNSUPPORTED_MCCS_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC02625DC_u32 as _);
pub const ERROR_GRAPHICS_MIRRORING_DEVICES_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC02625E3_u32 as _);
pub const ERROR_GRAPHICS_MODE_ALREADY_IN_MODESET: windows_core::HRESULT = windows_core::HRESULT(0xC0262314_u32 as _);
pub const ERROR_GRAPHICS_MODE_ID_MUST_BE_UNIQUE: windows_core::HRESULT = windows_core::HRESULT(0xC0262324_u32 as _);
pub const ERROR_GRAPHICS_MODE_NOT_IN_MODESET: windows_core::HRESULT = windows_core::HRESULT(0xC026234A_u32 as _);
pub const ERROR_GRAPHICS_MODE_NOT_PINNED: windows_core::HRESULT = windows_core::HRESULT(0x262307_u32 as _);
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_ALREADY_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0xC026232D_u32 as _);
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_ID_MUST_BE_UNIQUE: windows_core::HRESULT = windows_core::HRESULT(0xC026232E_u32 as _);
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_NOT_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0xC026232C_u32 as _);
pub const ERROR_GRAPHICS_MONITOR_COULD_NOT_BE_ASSOCIATED_WITH_ADAPTER: windows_core::HRESULT = windows_core::HRESULT(0xC0262334_u32 as _);
pub const ERROR_GRAPHICS_MONITOR_NOT_CONNECTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262338_u32 as _);
pub const ERROR_GRAPHICS_MONITOR_NO_LONGER_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC026258D_u32 as _);
pub const ERROR_GRAPHICS_MPO_ALLOCATION_UNPINNED: windows_core::HRESULT = windows_core::HRESULT(0xC0262018_u32 as _);
pub const ERROR_GRAPHICS_MULTISAMPLING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262349_u32 as _);
pub const ERROR_GRAPHICS_NOT_A_LINKED_ADAPTER: windows_core::HRESULT = windows_core::HRESULT(0xC0262430_u32 as _);
pub const ERROR_GRAPHICS_NOT_EXCLUSIVE_MODE_OWNER: windows_core::HRESULT = windows_core::HRESULT(0xC0262000_u32 as _);
pub const ERROR_GRAPHICS_NOT_POST_DEVICE_DRIVER: windows_core::HRESULT = windows_core::HRESULT(0xC0262438_u32 as _);
pub const ERROR_GRAPHICS_NO_ACTIVE_VIDPN: windows_core::HRESULT = windows_core::HRESULT(0xC0262336_u32 as _);
pub const ERROR_GRAPHICS_NO_AVAILABLE_IMPORTANCE_ORDINALS: windows_core::HRESULT = windows_core::HRESULT(0xC0262354_u32 as _);
pub const ERROR_GRAPHICS_NO_AVAILABLE_VIDPN_TARGET: windows_core::HRESULT = windows_core::HRESULT(0xC0262333_u32 as _);
pub const ERROR_GRAPHICS_NO_DISPLAY_DEVICE_CORRESPONDS_TO_NAME: windows_core::HRESULT = windows_core::HRESULT(0xC02625E1_u32 as _);
pub const ERROR_GRAPHICS_NO_DISPLAY_MODE_MANAGEMENT_SUPPORT: windows_core::HRESULT = windows_core::HRESULT(0xC0262341_u32 as _);
pub const ERROR_GRAPHICS_NO_MONITORS_CORRESPOND_TO_DISPLAY_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0xC02625E5_u32 as _);
pub const ERROR_GRAPHICS_NO_MORE_ELEMENTS_IN_DATASET: windows_core::HRESULT = windows_core::HRESULT(0x26234C_u32 as _);
pub const ERROR_GRAPHICS_NO_PREFERRED_MODE: windows_core::HRESULT = windows_core::HRESULT(0x26231E_u32 as _);
pub const ERROR_GRAPHICS_NO_RECOMMENDED_FUNCTIONAL_VIDPN: windows_core::HRESULT = windows_core::HRESULT(0xC0262323_u32 as _);
pub const ERROR_GRAPHICS_NO_RECOMMENDED_VIDPN_TOPOLOGY: windows_core::HRESULT = windows_core::HRESULT(0xC026231A_u32 as _);
pub const ERROR_GRAPHICS_NO_VIDEO_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0xC0262100_u32 as _);
pub const ERROR_GRAPHICS_NO_VIDPNMGR: windows_core::HRESULT = windows_core::HRESULT(0xC0262335_u32 as _);
pub const ERROR_GRAPHICS_ONLY_CONSOLE_SESSION_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC02625E0_u32 as _);
pub const ERROR_GRAPHICS_OPM_ALL_HDCP_HARDWARE_ALREADY_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0xC0262518_u32 as _);
pub const ERROR_GRAPHICS_OPM_DRIVER_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC026251E_u32 as _);
pub const ERROR_GRAPHICS_OPM_HDCP_SRM_NEVER_SET: windows_core::HRESULT = windows_core::HRESULT(0xC0262516_u32 as _);
pub const ERROR_GRAPHICS_OPM_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC026250B_u32 as _);
pub const ERROR_GRAPHICS_OPM_INVALID_CONFIGURATION_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0xC0262521_u32 as _);
pub const ERROR_GRAPHICS_OPM_INVALID_ENCRYPTED_PARAMETERS: windows_core::HRESULT = windows_core::HRESULT(0xC0262503_u32 as _);
pub const ERROR_GRAPHICS_OPM_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0xC026250C_u32 as _);
pub const ERROR_GRAPHICS_OPM_INVALID_INFORMATION_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0xC026251D_u32 as _);
pub const ERROR_GRAPHICS_OPM_INVALID_SRM: windows_core::HRESULT = windows_core::HRESULT(0xC0262512_u32 as _);
pub const ERROR_GRAPHICS_OPM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262500_u32 as _);
pub const ERROR_GRAPHICS_OPM_NO_VIDEO_OUTPUTS_EXIST: windows_core::HRESULT = windows_core::HRESULT(0xC0262505_u32 as _);
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_ACP: windows_core::HRESULT = windows_core::HRESULT(0xC0262514_u32 as _);
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_CGMSA: windows_core::HRESULT = windows_core::HRESULT(0xC0262515_u32 as _);
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_HDCP: windows_core::HRESULT = windows_core::HRESULT(0xC0262513_u32 as _);
pub const ERROR_GRAPHICS_OPM_RESOLUTION_TOO_HIGH: windows_core::HRESULT = windows_core::HRESULT(0xC0262517_u32 as _);
pub const ERROR_GRAPHICS_OPM_SESSION_TYPE_CHANGE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC026251B_u32 as _);
pub const ERROR_GRAPHICS_OPM_SIGNALING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262520_u32 as _);
pub const ERROR_GRAPHICS_OPM_SPANNING_MODE_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0xC026250F_u32 as _);
pub const ERROR_GRAPHICS_OPM_THEATER_MODE_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0xC0262510_u32 as _);
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_DOES_NOT_HAVE_COPP_SEMANTICS: windows_core::HRESULT = windows_core::HRESULT(0xC026251C_u32 as _);
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_DOES_NOT_HAVE_OPM_SEMANTICS: windows_core::HRESULT = windows_core::HRESULT(0xC026251F_u32 as _);
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_NO_LONGER_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC026251A_u32 as _);
pub const ERROR_GRAPHICS_PARAMETER_ARRAY_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC02625E6_u32 as _);
pub const ERROR_GRAPHICS_PARTIAL_DATA_POPULATED: windows_core::HRESULT = windows_core::HRESULT(0x4026200A_u32 as _);
pub const ERROR_GRAPHICS_PATH_ALREADY_IN_TOPOLOGY: windows_core::HRESULT = windows_core::HRESULT(0xC0262313_u32 as _);
pub const ERROR_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_PINNED: windows_core::HRESULT = windows_core::HRESULT(0x262351_u32 as _);
pub const ERROR_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262346_u32 as _);
pub const ERROR_GRAPHICS_PATH_NOT_IN_TOPOLOGY: windows_core::HRESULT = windows_core::HRESULT(0xC0262327_u32 as _);
pub const ERROR_GRAPHICS_PINNED_MODE_MUST_REMAIN_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0xC0262312_u32 as _);
pub const ERROR_GRAPHICS_POLLING_TOO_FREQUENTLY: windows_core::HRESULT = windows_core::HRESULT(0x40262439_u32 as _);
pub const ERROR_GRAPHICS_PRESENT_BUFFER_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0262010_u32 as _);
pub const ERROR_GRAPHICS_PRESENT_DENIED: windows_core::HRESULT = windows_core::HRESULT(0xC0262007_u32 as _);
pub const ERROR_GRAPHICS_PRESENT_INVALID_WINDOW: windows_core::HRESULT = windows_core::HRESULT(0xC026200F_u32 as _);
pub const ERROR_GRAPHICS_PRESENT_MODE_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0xC0262005_u32 as _);
pub const ERROR_GRAPHICS_PRESENT_OCCLUDED: windows_core::HRESULT = windows_core::HRESULT(0xC0262006_u32 as _);
pub const ERROR_GRAPHICS_PRESENT_REDIRECTION_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0xC026200B_u32 as _);
pub const ERROR_GRAPHICS_PRESENT_UNOCCLUDED: windows_core::HRESULT = windows_core::HRESULT(0xC026200C_u32 as _);
pub const ERROR_GRAPHICS_PVP_HFS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC0262511_u32 as _);
pub const ERROR_GRAPHICS_PVP_INVALID_CERTIFICATE_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC026250E_u32 as _);
pub const ERROR_GRAPHICS_RESOURCES_NOT_RELATED: windows_core::HRESULT = windows_core::HRESULT(0xC0262330_u32 as _);
pub const ERROR_GRAPHICS_SESSION_TYPE_CHANGE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC02605E8_u32 as _);
pub const ERROR_GRAPHICS_SKIP_ALLOCATION_PREPARATION: windows_core::HRESULT = windows_core::HRESULT(0x40262201_u32 as _);
pub const ERROR_GRAPHICS_SOURCE_ALREADY_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0xC0262317_u32 as _);
pub const ERROR_GRAPHICS_SOURCE_ID_MUST_BE_UNIQUE: windows_core::HRESULT = windows_core::HRESULT(0xC0262331_u32 as _);
pub const ERROR_GRAPHICS_SOURCE_NOT_IN_TOPOLOGY: windows_core::HRESULT = windows_core::HRESULT(0xC0262339_u32 as _);
pub const ERROR_GRAPHICS_SPECIFIED_CHILD_ALREADY_CONNECTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262400_u32 as _);
pub const ERROR_GRAPHICS_STALE_MODESET: windows_core::HRESULT = windows_core::HRESULT(0xC0262320_u32 as _);
pub const ERROR_GRAPHICS_STALE_VIDPN_TOPOLOGY: windows_core::HRESULT = windows_core::HRESULT(0xC0262337_u32 as _);
pub const ERROR_GRAPHICS_START_DEFERRED: windows_core::HRESULT = windows_core::HRESULT(0x4026243A_u32 as _);
pub const ERROR_GRAPHICS_TARGET_ALREADY_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0xC0262318_u32 as _);
pub const ERROR_GRAPHICS_TARGET_ID_MUST_BE_UNIQUE: windows_core::HRESULT = windows_core::HRESULT(0xC0262332_u32 as _);
pub const ERROR_GRAPHICS_TARGET_NOT_IN_TOPOLOGY: windows_core::HRESULT = windows_core::HRESULT(0xC0262340_u32 as _);
pub const ERROR_GRAPHICS_TOO_MANY_REFERENCES: windows_core::HRESULT = windows_core::HRESULT(0xC0262103_u32 as _);
pub const ERROR_GRAPHICS_TOPOLOGY_CHANGES_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC0262353_u32 as _);
pub const ERROR_GRAPHICS_TRY_AGAIN_LATER: windows_core::HRESULT = windows_core::HRESULT(0xC0262104_u32 as _);
pub const ERROR_GRAPHICS_TRY_AGAIN_NOW: windows_core::HRESULT = windows_core::HRESULT(0xC0262105_u32 as _);
pub const ERROR_GRAPHICS_UAB_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262502_u32 as _);
pub const ERROR_GRAPHICS_UNASSIGNED_MODESET_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC0262350_u32 as _);
pub const ERROR_GRAPHICS_UNKNOWN_CHILD_STATUS: windows_core::HRESULT = windows_core::HRESULT(0x4026242F_u32 as _);
pub const ERROR_GRAPHICS_UNSWIZZLING_APERTURE_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC0262107_u32 as _);
pub const ERROR_GRAPHICS_UNSWIZZLING_APERTURE_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262108_u32 as _);
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_COMPOSITION_WINDOW_DPI_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0xC0262016_u32 as _);
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_CREATE_SUPERWETINK_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0xC0262014_u32 as _);
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_DESTROY_SUPERWETINK_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0xC0262015_u32 as _);
pub const ERROR_GRAPHICS_VAIL_STATE_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0xC0262011_u32 as _);
pub const ERROR_GRAPHICS_VIDEO_PRESENT_TARGETS_LESS_THAN_SOURCES: windows_core::HRESULT = windows_core::HRESULT(0xC0262326_u32 as _);
pub const ERROR_GRAPHICS_VIDPN_MODALITY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262306_u32 as _);
pub const ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0xC0262342_u32 as _);
pub const ERROR_GRAPHICS_VIDPN_TOPOLOGY_CURRENTLY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262302_u32 as _);
pub const ERROR_GRAPHICS_VIDPN_TOPOLOGY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0262301_u32 as _);
pub const ERROR_GRAPHICS_WINDOWDC_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC026200D_u32 as _);
pub const ERROR_GRAPHICS_WINDOWLESS_PRESENT_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0xC026200E_u32 as _);
pub const ERROR_GRAPHICS_WRONG_ALLOCATION_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0xC0262115_u32 as _);
pub const ERROR_GROUPSET_CANT_PROVIDE: WIN32_ERROR = WIN32_ERROR(5993u32);
pub const ERROR_GROUPSET_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5991u32);
pub const ERROR_GROUPSET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5992u32);
pub const ERROR_GROUP_EXISTS: WIN32_ERROR = WIN32_ERROR(1318u32);
pub const ERROR_GROUP_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5012u32);
pub const ERROR_GROUP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5013u32);
pub const ERROR_GROUP_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(5014u32);
pub const ERROR_GUID_SUBSTITUTION_MADE: WIN32_ERROR = WIN32_ERROR(680u32);
pub const ERROR_HANDLES_CLOSED: WIN32_ERROR = WIN32_ERROR(676u32);
pub const ERROR_HANDLE_DISK_FULL: WIN32_ERROR = WIN32_ERROR(39u32);
pub const ERROR_HANDLE_EOF: WIN32_ERROR = WIN32_ERROR(38u32);
pub const ERROR_HANDLE_NO_LONGER_VALID: WIN32_ERROR = WIN32_ERROR(6815u32);
pub const ERROR_HANDLE_REVOKED: WIN32_ERROR = WIN32_ERROR(811u32);
pub const ERROR_HASH_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(15301u32);
pub const ERROR_HASH_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15300u32);
pub const ERROR_HAS_SYSTEM_CRITICAL_FILES: WIN32_ERROR = WIN32_ERROR(488u32);
pub const ERROR_HEURISTIC_DAMAGE_POSSIBLE: WIN32_ERROR = WIN32_ERROR(6731u32);
pub const ERROR_HIBERNATED: WIN32_ERROR = WIN32_ERROR(726u32);
pub const ERROR_HIBERNATION_FAILURE: WIN32_ERROR = WIN32_ERROR(656u32);
pub const ERROR_HOOK_NEEDS_HMOD: WIN32_ERROR = WIN32_ERROR(1428u32);
pub const ERROR_HOOK_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(1431u32);
pub const ERROR_HOOK_TYPE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(1458u32);
pub const ERROR_HOST_DOWN: WIN32_ERROR = WIN32_ERROR(1256u32);
pub const ERROR_HOST_NODE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5005u32);
pub const ERROR_HOST_NODE_NOT_GROUP_OWNER: WIN32_ERROR = WIN32_ERROR(5016u32);
pub const ERROR_HOST_NODE_NOT_RESOURCE_OWNER: WIN32_ERROR = WIN32_ERROR(5015u32);
pub const ERROR_HOST_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1232u32);
pub const ERROR_HOTKEY_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(1409u32);
pub const ERROR_HOTKEY_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(1419u32);
pub const ERROR_HUNG_DISPLAY_DRIVER_THREAD: windows_core::HRESULT = windows_core::HRESULT(0x80260001_u32 as _);
pub const ERROR_HV_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3224698886u32);
pub const ERROR_HV_ACKNOWLEDGED: WIN32_ERROR = WIN32_ERROR(3224698902u32);
pub const ERROR_HV_CPUID_FEATURE_VALIDATION: WIN32_ERROR = WIN32_ERROR(3224698940u32);
pub const ERROR_HV_CPUID_XSAVE_FEATURE_VALIDATION: WIN32_ERROR = WIN32_ERROR(3224698941u32);
pub const ERROR_HV_DEVICE_NOT_IN_DOMAIN: WIN32_ERROR = WIN32_ERROR(3224698998u32);
pub const ERROR_HV_EVENT_BUFFER_ALREADY_FREED: WIN32_ERROR = WIN32_ERROR(3224698996u32);
pub const ERROR_HV_FEATURE_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(3224698910u32);
pub const ERROR_HV_INACTIVE: WIN32_ERROR = WIN32_ERROR(3224698908u32);
pub const ERROR_HV_INSUFFICIENT_BUFFER: WIN32_ERROR = WIN32_ERROR(3224698931u32);
pub const ERROR_HV_INSUFFICIENT_BUFFERS: WIN32_ERROR = WIN32_ERROR(3224698899u32);
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_MEMORY: WIN32_ERROR = WIN32_ERROR(3224698997u32);
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_MEMORY_MIRRORING: WIN32_ERROR = WIN32_ERROR(3224699010u32);
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY: WIN32_ERROR = WIN32_ERROR(3224699011u32);
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY_MIRRORING: WIN32_ERROR = WIN32_ERROR(3224699013u32);
pub const ERROR_HV_INSUFFICIENT_DEVICE_DOMAINS: WIN32_ERROR = WIN32_ERROR(3224698936u32);
pub const ERROR_HV_INSUFFICIENT_MEMORY: WIN32_ERROR = WIN32_ERROR(3224698891u32);
pub const ERROR_HV_INSUFFICIENT_MEMORY_MIRRORING: WIN32_ERROR = WIN32_ERROR(3224699009u32);
pub const ERROR_HV_INSUFFICIENT_ROOT_MEMORY: WIN32_ERROR = WIN32_ERROR(3224698995u32);
pub const ERROR_HV_INSUFFICIENT_ROOT_MEMORY_MIRRORING: WIN32_ERROR = WIN32_ERROR(3224699012u32);
pub const ERROR_HV_INVALID_ALIGNMENT: WIN32_ERROR = WIN32_ERROR(3224698884u32);
pub const ERROR_HV_INVALID_CONNECTION_ID: WIN32_ERROR = WIN32_ERROR(3224698898u32);
pub const ERROR_HV_INVALID_CPU_GROUP_ID: WIN32_ERROR = WIN32_ERROR(3224698991u32);
pub const ERROR_HV_INVALID_CPU_GROUP_STATE: WIN32_ERROR = WIN32_ERROR(3224698992u32);
pub const ERROR_HV_INVALID_DEVICE_ID: WIN32_ERROR = WIN32_ERROR(3224698967u32);
pub const ERROR_HV_INVALID_DEVICE_STATE: WIN32_ERROR = WIN32_ERROR(3224698968u32);
pub const ERROR_HV_INVALID_HYPERCALL_CODE: WIN32_ERROR = WIN32_ERROR(3224698882u32);
pub const ERROR_HV_INVALID_HYPERCALL_INPUT: WIN32_ERROR = WIN32_ERROR(3224698883u32);
pub const ERROR_HV_INVALID_LP_INDEX: WIN32_ERROR = WIN32_ERROR(3224698945u32);
pub const ERROR_HV_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(3224698885u32);
pub const ERROR_HV_INVALID_PARTITION_ID: WIN32_ERROR = WIN32_ERROR(3224698893u32);
pub const ERROR_HV_INVALID_PARTITION_STATE: WIN32_ERROR = WIN32_ERROR(3224698887u32);
pub const ERROR_HV_INVALID_PORT_ID: WIN32_ERROR = WIN32_ERROR(3224698897u32);
pub const ERROR_HV_INVALID_PROXIMITY_DOMAIN_INFO: WIN32_ERROR = WIN32_ERROR(3224698906u32);
pub const ERROR_HV_INVALID_REGISTER_VALUE: WIN32_ERROR = WIN32_ERROR(3224698960u32);
pub const ERROR_HV_INVALID_SAVE_RESTORE_STATE: WIN32_ERROR = WIN32_ERROR(3224698903u32);
pub const ERROR_HV_INVALID_SYNIC_STATE: WIN32_ERROR = WIN32_ERROR(3224698904u32);
pub const ERROR_HV_INVALID_VP_INDEX: WIN32_ERROR = WIN32_ERROR(3224698894u32);
pub const ERROR_HV_INVALID_VP_STATE: WIN32_ERROR = WIN32_ERROR(3224698901u32);
pub const ERROR_HV_INVALID_VTL_STATE: WIN32_ERROR = WIN32_ERROR(3224698961u32);
pub const ERROR_HV_MSR_ACCESS_FAILED: WIN32_ERROR = WIN32_ERROR(3224699008u32);
pub const ERROR_HV_NESTED_VM_EXIT: WIN32_ERROR = WIN32_ERROR(3224698999u32);
pub const ERROR_HV_NOT_ACKNOWLEDGED: WIN32_ERROR = WIN32_ERROR(3224698900u32);
pub const ERROR_HV_NOT_ALLOWED_WITH_NESTED_VIRT_ACTIVE: WIN32_ERROR = WIN32_ERROR(3224698994u32);
pub const ERROR_HV_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(3224702976u32);
pub const ERROR_HV_NO_DATA: WIN32_ERROR = WIN32_ERROR(3224698907u32);
pub const ERROR_HV_NO_RESOURCES: WIN32_ERROR = WIN32_ERROR(3224698909u32);
pub const ERROR_HV_NX_NOT_DETECTED: WIN32_ERROR = WIN32_ERROR(3224698965u32);
pub const ERROR_HV_OBJECT_IN_USE: WIN32_ERROR = WIN32_ERROR(3224698905u32);
pub const ERROR_HV_OPERATION_DENIED: WIN32_ERROR = WIN32_ERROR(3224698888u32);
pub const ERROR_HV_OPERATION_FAILED: WIN32_ERROR = WIN32_ERROR(3224698993u32);
pub const ERROR_HV_PAGE_REQUEST_INVALID: WIN32_ERROR = WIN32_ERROR(3224698976u32);
pub const ERROR_HV_PARTITION_TOO_DEEP: WIN32_ERROR = WIN32_ERROR(3224698892u32);
pub const ERROR_HV_PENDING_PAGE_REQUESTS: WIN32_ERROR = WIN32_ERROR(3473497u32);
pub const ERROR_HV_PROCESSOR_STARTUP_TIMEOUT: WIN32_ERROR = WIN32_ERROR(3224698942u32);
pub const ERROR_HV_PROPERTY_VALUE_OUT_OF_RANGE: WIN32_ERROR = WIN32_ERROR(3224698890u32);
pub const ERROR_HV_SMX_ENABLED: WIN32_ERROR = WIN32_ERROR(3224698943u32);
pub const ERROR_HV_UNKNOWN_PROPERTY: WIN32_ERROR = WIN32_ERROR(3224698889u32);
pub const ERROR_HWNDS_HAVE_DIFF_PARENT: WIN32_ERROR = WIN32_ERROR(1441u32);
pub const ERROR_ICM_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(2018u32);
pub const ERROR_IDLE_DISCONNECTED: u32 = 926u32;
pub const ERROR_IEPORT_FULL: WIN32_ERROR = WIN32_ERROR(4341u32);
pub const ERROR_ILLEGAL_CHARACTER: WIN32_ERROR = WIN32_ERROR(582u32);
pub const ERROR_ILLEGAL_DLL_RELOCATION: WIN32_ERROR = WIN32_ERROR(623u32);
pub const ERROR_ILLEGAL_ELEMENT_ADDRESS: WIN32_ERROR = WIN32_ERROR(1162u32);
pub const ERROR_ILLEGAL_FLOAT_CONTEXT: WIN32_ERROR = WIN32_ERROR(579u32);
pub const ERROR_ILL_FORMED_PASSWORD: WIN32_ERROR = WIN32_ERROR(1324u32);
pub const ERROR_IMAGE_AT_DIFFERENT_BASE: WIN32_ERROR = WIN32_ERROR(807u32);
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(706u32);
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH_EXE: WIN32_ERROR = WIN32_ERROR(720u32);
pub const ERROR_IMAGE_NOT_AT_BASE: WIN32_ERROR = WIN32_ERROR(700u32);
pub const ERROR_IMAGE_SUBSYSTEM_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(308u32);
pub const ERROR_IMPLEMENTATION_LIMIT: WIN32_ERROR = WIN32_ERROR(1292u32);
pub const ERROR_IMPLICIT_TRANSACTION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(6725u32);
pub const ERROR_INCOMPATIBLE_SERVICE_PRIVILEGE: WIN32_ERROR = WIN32_ERROR(1297u32);
pub const ERROR_INCOMPATIBLE_SERVICE_SID_TYPE: WIN32_ERROR = WIN32_ERROR(1290u32);
pub const ERROR_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING: WIN32_ERROR = WIN32_ERROR(304u32);
pub const ERROR_INCORRECT_ACCOUNT_TYPE: WIN32_ERROR = WIN32_ERROR(8646u32);
pub const ERROR_INCORRECT_ADDRESS: WIN32_ERROR = WIN32_ERROR(1241u32);
pub const ERROR_INCORRECT_SIZE: WIN32_ERROR = WIN32_ERROR(1462u32);
pub const ERROR_INC_BACKUP: WIN32_ERROR = WIN32_ERROR(4003u32);
pub const ERROR_INDEX_ABSENT: WIN32_ERROR = WIN32_ERROR(1611u32);
pub const ERROR_INDEX_OUT_OF_BOUNDS: WIN32_ERROR = WIN32_ERROR(474u32);
pub const ERROR_INDIGENOUS_TYPE: WIN32_ERROR = WIN32_ERROR(4338u32);
pub const ERROR_INDOUBT_TRANSACTIONS_EXIST: WIN32_ERROR = WIN32_ERROR(6827u32);
pub const ERROR_INFLOOP_IN_RELOC_CHAIN: WIN32_ERROR = WIN32_ERROR(202u32);
pub const ERROR_INF_IN_USE_BY_DEVICES: WIN32_ERROR = WIN32_ERROR(3758096957u32);
pub const ERROR_INSTALL_ALREADY_RUNNING: WIN32_ERROR = WIN32_ERROR(1618u32);
pub const ERROR_INSTALL_CANCEL: WIN32_ERROR = WIN32_ERROR(15608u32);
pub const ERROR_INSTALL_DEREGISTRATION_FAILURE: WIN32_ERROR = WIN32_ERROR(15607u32);
pub const ERROR_INSTALL_FAILED: WIN32_ERROR = WIN32_ERROR(15609u32);
pub const ERROR_INSTALL_FAILURE: WIN32_ERROR = WIN32_ERROR(1603u32);
pub const ERROR_INSTALL_FIREWALL_SERVICE_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(15626u32);
pub const ERROR_INSTALL_FULLTRUST_HOSTRUNTIME_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY: WIN32_ERROR = WIN32_ERROR(15663u32);
pub const ERROR_INSTALL_INVALID_PACKAGE: WIN32_ERROR = WIN32_ERROR(15602u32);
pub const ERROR_INSTALL_INVALID_RELATED_SET_UPDATE: WIN32_ERROR = WIN32_ERROR(15639u32);
pub const ERROR_INSTALL_LANGUAGE_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1623u32);
pub const ERROR_INSTALL_LOG_FAILURE: WIN32_ERROR = WIN32_ERROR(1622u32);
pub const ERROR_INSTALL_NETWORK_FAILURE: WIN32_ERROR = WIN32_ERROR(15605u32);
pub const ERROR_INSTALL_NOTUSED: WIN32_ERROR = WIN32_ERROR(1634u32);
pub const ERROR_INSTALL_OPEN_PACKAGE_FAILED: WIN32_ERROR = WIN32_ERROR(15600u32);
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_APPLICATIONID_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(15637u32);
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE: WIN32_ERROR = WIN32_ERROR(15634u32);
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY: WIN32_ERROR = WIN32_ERROR(15640u32);
pub const ERROR_INSTALL_OUT_OF_DISK_SPACE: WIN32_ERROR = WIN32_ERROR(15604u32);
pub const ERROR_INSTALL_PACKAGE_DOWNGRADE: WIN32_ERROR = WIN32_ERROR(15622u32);
pub const ERROR_INSTALL_PACKAGE_INVALID: WIN32_ERROR = WIN32_ERROR(1620u32);
pub const ERROR_INSTALL_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15601u32);
pub const ERROR_INSTALL_PACKAGE_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(1619u32);
pub const ERROR_INSTALL_PACKAGE_REJECTED: WIN32_ERROR = WIN32_ERROR(1625u32);
pub const ERROR_INSTALL_PACKAGE_VERSION: WIN32_ERROR = WIN32_ERROR(1613u32);
pub const ERROR_INSTALL_PLATFORM_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1633u32);
pub const ERROR_INSTALL_POLICY_FAILURE: WIN32_ERROR = WIN32_ERROR(15615u32);
pub const ERROR_INSTALL_PREREQUISITE_FAILED: WIN32_ERROR = WIN32_ERROR(15613u32);
pub const ERROR_INSTALL_REGISTRATION_FAILURE: WIN32_ERROR = WIN32_ERROR(15606u32);
pub const ERROR_INSTALL_REJECTED: WIN32_ERROR = WIN32_ERROR(1654u32);
pub const ERROR_INSTALL_REMOTE_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1640u32);
pub const ERROR_INSTALL_REMOTE_PROHIBITED: WIN32_ERROR = WIN32_ERROR(1645u32);
pub const ERROR_INSTALL_RESOLVE_DEPENDENCY_FAILED: WIN32_ERROR = WIN32_ERROR(15603u32);
pub const ERROR_INSTALL_RESOLVE_HOSTRUNTIME_DEPENDENCY_FAILED: WIN32_ERROR = WIN32_ERROR(15665u32);
pub const ERROR_INSTALL_SERVICE_FAILURE: WIN32_ERROR = WIN32_ERROR(1601u32);
pub const ERROR_INSTALL_SERVICE_SAFEBOOT: WIN32_ERROR = WIN32_ERROR(1652u32);
pub const ERROR_INSTALL_SOURCE_ABSENT: WIN32_ERROR = WIN32_ERROR(1612u32);
pub const ERROR_INSTALL_SUSPEND: WIN32_ERROR = WIN32_ERROR(1604u32);
pub const ERROR_INSTALL_TEMP_UNWRITABLE: WIN32_ERROR = WIN32_ERROR(1632u32);
pub const ERROR_INSTALL_TRANSFORM_FAILURE: WIN32_ERROR = WIN32_ERROR(1624u32);
pub const ERROR_INSTALL_TRANSFORM_REJECTED: WIN32_ERROR = WIN32_ERROR(1644u32);
pub const ERROR_INSTALL_UI_FAILURE: WIN32_ERROR = WIN32_ERROR(1621u32);
pub const ERROR_INSTALL_USEREXIT: WIN32_ERROR = WIN32_ERROR(1602u32);
pub const ERROR_INSTALL_VOLUME_CORRUPT: WIN32_ERROR = WIN32_ERROR(15630u32);
pub const ERROR_INSTALL_VOLUME_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(15628u32);
pub const ERROR_INSTALL_VOLUME_OFFLINE: WIN32_ERROR = WIN32_ERROR(15629u32);
pub const ERROR_INSTALL_WRONG_PROCESSOR_ARCHITECTURE: WIN32_ERROR = WIN32_ERROR(15632u32);
pub const ERROR_INSTRUCTION_MISALIGNMENT: WIN32_ERROR = WIN32_ERROR(549u32);
pub const ERROR_INSUFFICIENT_BUFFER: WIN32_ERROR = WIN32_ERROR(122u32);
pub const ERROR_INSUFFICIENT_LOGON_INFO: WIN32_ERROR = WIN32_ERROR(608u32);
pub const ERROR_INSUFFICIENT_POWER: WIN32_ERROR = WIN32_ERROR(639u32);
pub const ERROR_INSUFFICIENT_RESOURCE_FOR_SPECIFIED_SHARED_SECTION_SIZE: WIN32_ERROR = WIN32_ERROR(781u32);
pub const ERROR_INSUFFICIENT_VIRTUAL_ADDR_RESOURCES: WIN32_ERROR = WIN32_ERROR(473u32);
pub const ERROR_INTERFACE_ALREADY_EXISTS: u32 = 904u32;
pub const ERROR_INTERFACE_CONFIGURATION: u32 = 912u32;
pub const ERROR_INTERFACE_CONNECTED: u32 = 908u32;
pub const ERROR_INTERFACE_DEVICE_ACTIVE: WIN32_ERROR = WIN32_ERROR(3758096923u32);
pub const ERROR_INTERFACE_DEVICE_REMOVED: WIN32_ERROR = WIN32_ERROR(3758096924u32);
pub const ERROR_INTERFACE_DISABLED: u32 = 916u32;
pub const ERROR_INTERFACE_DISCONNECTED: u32 = 929u32;
pub const ERROR_INTERFACE_HAS_NO_DEVICES: u32 = 925u32;
pub const ERROR_INTERFACE_NOT_CONNECTED: u32 = 906u32;
pub const ERROR_INTERFACE_UNREACHABLE: u32 = 927u32;
pub const ERROR_INTERMIXED_KERNEL_EA_OPERATION: WIN32_ERROR = WIN32_ERROR(324u32);
pub const ERROR_INTERNAL_DB_CORRUPTION: WIN32_ERROR = WIN32_ERROR(1358u32);
pub const ERROR_INTERNAL_DB_ERROR: WIN32_ERROR = WIN32_ERROR(1383u32);
pub const ERROR_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(1359u32);
pub const ERROR_INTERRUPT_STILL_CONNECTED: WIN32_ERROR = WIN32_ERROR(764u32);
pub const ERROR_INTERRUPT_VECTOR_ALREADY_CONNECTED: WIN32_ERROR = WIN32_ERROR(763u32);
pub const ERROR_INVALID_ACCEL_HANDLE: WIN32_ERROR = WIN32_ERROR(1403u32);
pub const ERROR_INVALID_ACCESS: WIN32_ERROR = WIN32_ERROR(12u32);
pub const ERROR_INVALID_ACCOUNT_NAME: WIN32_ERROR = WIN32_ERROR(1315u32);
pub const ERROR_INVALID_ACE_CONDITION: WIN32_ERROR = WIN32_ERROR(805u32);
pub const ERROR_INVALID_ACL: WIN32_ERROR = WIN32_ERROR(1336u32);
pub const ERROR_INVALID_ADDRESS: WIN32_ERROR = WIN32_ERROR(487u32);
pub const ERROR_INVALID_ATTRIBUTE_LENGTH: u32 = 953u32;
pub const ERROR_INVALID_AT_INTERRUPT_TIME: WIN32_ERROR = WIN32_ERROR(104u32);
pub const ERROR_INVALID_BLOCK: WIN32_ERROR = WIN32_ERROR(9u32);
pub const ERROR_INVALID_BLOCK_LENGTH: WIN32_ERROR = WIN32_ERROR(1106u32);
pub const ERROR_INVALID_CAP: WIN32_ERROR = WIN32_ERROR(320u32);
pub const ERROR_INVALID_CATEGORY: WIN32_ERROR = WIN32_ERROR(117u32);
pub const ERROR_INVALID_CLASS: WIN32_ERROR = WIN32_ERROR(3758096902u32);
pub const ERROR_INVALID_CLASS_INSTALLER: WIN32_ERROR = WIN32_ERROR(3758096909u32);
pub const ERROR_INVALID_CLEANER: WIN32_ERROR = WIN32_ERROR(4310u32);
pub const ERROR_INVALID_CLUSTER_IPV6_ADDRESS: WIN32_ERROR = WIN32_ERROR(5911u32);
pub const ERROR_INVALID_CMM: WIN32_ERROR = WIN32_ERROR(2010u32);
pub const ERROR_INVALID_COINSTALLER: WIN32_ERROR = WIN32_ERROR(3758096935u32);
pub const ERROR_INVALID_COLORINDEX: WIN32_ERROR = WIN32_ERROR(2022u32);
pub const ERROR_INVALID_COLORSPACE: WIN32_ERROR = WIN32_ERROR(2017u32);
pub const ERROR_INVALID_COMBOBOX_MESSAGE: WIN32_ERROR = WIN32_ERROR(1422u32);
pub const ERROR_INVALID_COMMAND_LINE: WIN32_ERROR = WIN32_ERROR(1639u32);
pub const ERROR_INVALID_COMPUTERNAME: WIN32_ERROR = WIN32_ERROR(1210u32);
pub const ERROR_INVALID_CONFIG_VALUE: WIN32_ERROR = WIN32_ERROR(479u32);
pub const ERROR_INVALID_CRUNTIME_PARAMETER: WIN32_ERROR = WIN32_ERROR(1288u32);
pub const ERROR_INVALID_CURSOR_HANDLE: WIN32_ERROR = WIN32_ERROR(1402u32);
pub const ERROR_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(13u32);
pub const ERROR_INVALID_DATATYPE: WIN32_ERROR = WIN32_ERROR(1804u32);
pub const ERROR_INVALID_DEVICE_OBJECT_PARAMETER: WIN32_ERROR = WIN32_ERROR(650u32);
pub const ERROR_INVALID_DEVINST_NAME: WIN32_ERROR = WIN32_ERROR(3758096901u32);
pub const ERROR_INVALID_DLL: WIN32_ERROR = WIN32_ERROR(1154u32);
pub const ERROR_INVALID_DOMAINNAME: WIN32_ERROR = WIN32_ERROR(1212u32);
pub const ERROR_INVALID_DOMAIN_ROLE: WIN32_ERROR = WIN32_ERROR(1354u32);
pub const ERROR_INVALID_DOMAIN_STATE: WIN32_ERROR = WIN32_ERROR(1353u32);
pub const ERROR_INVALID_DRIVE: WIN32_ERROR = WIN32_ERROR(15u32);
pub const ERROR_INVALID_DRIVE_OBJECT: WIN32_ERROR = WIN32_ERROR(4321u32);
pub const ERROR_INVALID_DWP_HANDLE: WIN32_ERROR = WIN32_ERROR(1405u32);
pub const ERROR_INVALID_EA_HANDLE: WIN32_ERROR = WIN32_ERROR(278u32);
pub const ERROR_INVALID_EA_NAME: WIN32_ERROR = WIN32_ERROR(254u32);
pub const ERROR_INVALID_EDIT_HEIGHT: WIN32_ERROR = WIN32_ERROR(1424u32);
pub const ERROR_INVALID_ENVIRONMENT: WIN32_ERROR = WIN32_ERROR(1805u32);
pub const ERROR_INVALID_EVENTNAME: WIN32_ERROR = WIN32_ERROR(1211u32);
pub const ERROR_INVALID_EVENT_COUNT: WIN32_ERROR = WIN32_ERROR(151u32);
pub const ERROR_INVALID_EXCEPTION_HANDLER: WIN32_ERROR = WIN32_ERROR(310u32);
pub const ERROR_INVALID_EXE_SIGNATURE: WIN32_ERROR = WIN32_ERROR(191u32);
pub const ERROR_INVALID_FIELD: WIN32_ERROR = WIN32_ERROR(1616u32);
pub const ERROR_INVALID_FIELD_IN_PARAMETER_LIST: WIN32_ERROR = WIN32_ERROR(328u32);
pub const ERROR_INVALID_FILTER_DRIVER: WIN32_ERROR = WIN32_ERROR(3758096940u32);
pub const ERROR_INVALID_FILTER_PROC: WIN32_ERROR = WIN32_ERROR(1427u32);
pub const ERROR_INVALID_FLAGS: WIN32_ERROR = WIN32_ERROR(1004u32);
pub const ERROR_INVALID_FLAG_NUMBER: WIN32_ERROR = WIN32_ERROR(186u32);
pub const ERROR_INVALID_FORM_NAME: WIN32_ERROR = WIN32_ERROR(1902u32);
pub const ERROR_INVALID_FORM_SIZE: WIN32_ERROR = WIN32_ERROR(1903u32);
pub const ERROR_INVALID_FUNCTION: WIN32_ERROR = WIN32_ERROR(1u32);
pub const ERROR_INVALID_GROUPNAME: WIN32_ERROR = WIN32_ERROR(1209u32);
pub const ERROR_INVALID_GROUP_ATTRIBUTES: WIN32_ERROR = WIN32_ERROR(1345u32);
pub const ERROR_INVALID_GW_COMMAND: WIN32_ERROR = WIN32_ERROR(1443u32);
pub const ERROR_INVALID_HANDLE: WIN32_ERROR = WIN32_ERROR(6u32);
pub const ERROR_INVALID_HANDLE_STATE: WIN32_ERROR = WIN32_ERROR(1609u32);
pub const ERROR_INVALID_HOOK_FILTER: WIN32_ERROR = WIN32_ERROR(1426u32);
pub const ERROR_INVALID_HOOK_HANDLE: WIN32_ERROR = WIN32_ERROR(1404u32);
pub const ERROR_INVALID_HWPROFILE: WIN32_ERROR = WIN32_ERROR(3758096912u32);
pub const ERROR_INVALID_HW_PROFILE: WIN32_ERROR = WIN32_ERROR(619u32);
pub const ERROR_INVALID_ICON_HANDLE: WIN32_ERROR = WIN32_ERROR(1414u32);
pub const ERROR_INVALID_ID_AUTHORITY: WIN32_ERROR = WIN32_ERROR(1343u32);
pub const ERROR_INVALID_IMAGE_HASH: WIN32_ERROR = WIN32_ERROR(577u32);
pub const ERROR_INVALID_IMPORT_OF_NON_DLL: WIN32_ERROR = WIN32_ERROR(1276u32);
pub const ERROR_INVALID_INDEX: WIN32_ERROR = WIN32_ERROR(1413u32);
pub const ERROR_INVALID_INF_LOGCONFIG: WIN32_ERROR = WIN32_ERROR(3758096938u32);
pub const ERROR_INVALID_KERNEL_INFO_VERSION: WIN32_ERROR = WIN32_ERROR(340u32);
pub const ERROR_INVALID_KEYBOARD_HANDLE: WIN32_ERROR = WIN32_ERROR(1457u32);
pub const ERROR_INVALID_LABEL: WIN32_ERROR = WIN32_ERROR(1299u32);
pub const ERROR_INVALID_LB_MESSAGE: WIN32_ERROR = WIN32_ERROR(1432u32);
pub const ERROR_INVALID_LDT_DESCRIPTOR: WIN32_ERROR = WIN32_ERROR(564u32);
pub const ERROR_INVALID_LDT_OFFSET: WIN32_ERROR = WIN32_ERROR(563u32);
pub const ERROR_INVALID_LDT_SIZE: WIN32_ERROR = WIN32_ERROR(561u32);
pub const ERROR_INVALID_LEVEL: WIN32_ERROR = WIN32_ERROR(124u32);
pub const ERROR_INVALID_LIBRARY: WIN32_ERROR = WIN32_ERROR(4301u32);
pub const ERROR_INVALID_LIST_FORMAT: WIN32_ERROR = WIN32_ERROR(153u32);
pub const ERROR_INVALID_LOCK_RANGE: WIN32_ERROR = WIN32_ERROR(307u32);
pub const ERROR_INVALID_LOGON_HOURS: WIN32_ERROR = WIN32_ERROR(1328u32);
pub const ERROR_INVALID_LOGON_TYPE: WIN32_ERROR = WIN32_ERROR(1367u32);
pub const ERROR_INVALID_MACHINENAME: WIN32_ERROR = WIN32_ERROR(3758096928u32);
pub const ERROR_INVALID_MEDIA: WIN32_ERROR = WIN32_ERROR(4300u32);
pub const ERROR_INVALID_MEDIA_POOL: WIN32_ERROR = WIN32_ERROR(4302u32);
pub const ERROR_INVALID_MEMBER: WIN32_ERROR = WIN32_ERROR(1388u32);
pub const ERROR_INVALID_MENU_HANDLE: WIN32_ERROR = WIN32_ERROR(1401u32);
pub const ERROR_INVALID_MESSAGE: WIN32_ERROR = WIN32_ERROR(1002u32);
pub const ERROR_INVALID_MESSAGEDEST: WIN32_ERROR = WIN32_ERROR(1218u32);
pub const ERROR_INVALID_MESSAGENAME: WIN32_ERROR = WIN32_ERROR(1217u32);
pub const ERROR_INVALID_MINALLOCSIZE: WIN32_ERROR = WIN32_ERROR(195u32);
pub const ERROR_INVALID_MODULETYPE: WIN32_ERROR = WIN32_ERROR(190u32);
pub const ERROR_INVALID_MONITOR_HANDLE: WIN32_ERROR = WIN32_ERROR(1461u32);
pub const ERROR_INVALID_MSGBOX_STYLE: WIN32_ERROR = WIN32_ERROR(1438u32);
pub const ERROR_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(123u32);
pub const ERROR_INVALID_NETNAME: WIN32_ERROR = WIN32_ERROR(1214u32);
pub const ERROR_INVALID_OPERATION: WIN32_ERROR = WIN32_ERROR(4317u32);
pub const ERROR_INVALID_OPERATION_ON_QUORUM: WIN32_ERROR = WIN32_ERROR(5068u32);
pub const ERROR_INVALID_OPLOCK_PROTOCOL: WIN32_ERROR = WIN32_ERROR(301u32);
pub const ERROR_INVALID_ORDINAL: WIN32_ERROR = WIN32_ERROR(182u32);
pub const ERROR_INVALID_OWNER: WIN32_ERROR = WIN32_ERROR(1307u32);
pub const ERROR_INVALID_PACKAGE_SID_LENGTH: WIN32_ERROR = WIN32_ERROR(4253u32);
pub const ERROR_INVALID_PACKET: u32 = 954u32;
pub const ERROR_INVALID_PACKET_LENGTH_OR_ID: u32 = 952u32;
pub const ERROR_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(87u32);
pub const ERROR_INVALID_PASSWORD: WIN32_ERROR = WIN32_ERROR(86u32);
pub const ERROR_INVALID_PASSWORDNAME: WIN32_ERROR = WIN32_ERROR(1216u32);
pub const ERROR_INVALID_PATCH_XML: WIN32_ERROR = WIN32_ERROR(1650u32);
pub const ERROR_INVALID_PEP_INFO_VERSION: WIN32_ERROR = WIN32_ERROR(341u32);
pub const ERROR_INVALID_PIXEL_FORMAT: WIN32_ERROR = WIN32_ERROR(2000u32);
pub const ERROR_INVALID_PLUGPLAY_DEVICE_PATH: WIN32_ERROR = WIN32_ERROR(620u32);
pub const ERROR_INVALID_PORT_ATTRIBUTES: WIN32_ERROR = WIN32_ERROR(545u32);
pub const ERROR_INVALID_PRIMARY_GROUP: WIN32_ERROR = WIN32_ERROR(1308u32);
pub const ERROR_INVALID_PRINTER_COMMAND: WIN32_ERROR = WIN32_ERROR(1803u32);
pub const ERROR_INVALID_PRINTER_DRIVER_MANIFEST: WIN32_ERROR = WIN32_ERROR(3021u32);
pub const ERROR_INVALID_PRINTER_NAME: WIN32_ERROR = WIN32_ERROR(1801u32);
pub const ERROR_INVALID_PRINTER_STATE: WIN32_ERROR = WIN32_ERROR(1906u32);
pub const ERROR_INVALID_PRINT_MONITOR: WIN32_ERROR = WIN32_ERROR(3007u32);
pub const ERROR_INVALID_PRIORITY: WIN32_ERROR = WIN32_ERROR(1800u32);
pub const ERROR_INVALID_PROFILE: WIN32_ERROR = WIN32_ERROR(2011u32);
pub const ERROR_INVALID_PROPPAGE_PROVIDER: WIN32_ERROR = WIN32_ERROR(3758096932u32);
pub const ERROR_INVALID_QUOTA_LOWER: WIN32_ERROR = WIN32_ERROR(547u32);
pub const ERROR_INVALID_RADIUS_RESPONSE: u32 = 939u32;
pub const ERROR_INVALID_REFERENCE_STRING: WIN32_ERROR = WIN32_ERROR(3758096927u32);
pub const ERROR_INVALID_REG_PROPERTY: WIN32_ERROR = WIN32_ERROR(3758096905u32);
pub const ERROR_INVALID_REPARSE_DATA: WIN32_ERROR = WIN32_ERROR(4392u32);
pub const ERROR_INVALID_RUNLEVEL_SETTING: WIN32_ERROR = WIN32_ERROR(15401u32);
pub const ERROR_INVALID_SCROLLBAR_RANGE: WIN32_ERROR = WIN32_ERROR(1448u32);
pub const ERROR_INVALID_SECURITY_DESCR: WIN32_ERROR = WIN32_ERROR(1338u32);
pub const ERROR_INVALID_SEGDPL: WIN32_ERROR = WIN32_ERROR(198u32);
pub const ERROR_INVALID_SEGMENT_NUMBER: WIN32_ERROR = WIN32_ERROR(180u32);
pub const ERROR_INVALID_SEPARATOR_FILE: WIN32_ERROR = WIN32_ERROR(1799u32);
pub const ERROR_INVALID_SERVER_STATE: WIN32_ERROR = WIN32_ERROR(1352u32);
pub const ERROR_INVALID_SERVICENAME: WIN32_ERROR = WIN32_ERROR(1213u32);
pub const ERROR_INVALID_SERVICE_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1057u32);
pub const ERROR_INVALID_SERVICE_CONTROL: WIN32_ERROR = WIN32_ERROR(1052u32);
pub const ERROR_INVALID_SERVICE_LOCK: WIN32_ERROR = WIN32_ERROR(1071u32);
pub const ERROR_INVALID_SHARENAME: WIN32_ERROR = WIN32_ERROR(1215u32);
pub const ERROR_INVALID_SHOWWIN_COMMAND: WIN32_ERROR = WIN32_ERROR(1449u32);
pub const ERROR_INVALID_SID: WIN32_ERROR = WIN32_ERROR(1337u32);
pub const ERROR_INVALID_SIGNAL_NUMBER: WIN32_ERROR = WIN32_ERROR(209u32);
pub const ERROR_INVALID_SIGNATURE: u32 = 950u32;
pub const ERROR_INVALID_SIGNATURE_LENGTH: u32 = 949u32;
pub const ERROR_INVALID_SPI_VALUE: WIN32_ERROR = WIN32_ERROR(1439u32);
pub const ERROR_INVALID_STACKSEG: WIN32_ERROR = WIN32_ERROR(189u32);
pub const ERROR_INVALID_STAGED_SIGNATURE: WIN32_ERROR = WIN32_ERROR(15620u32);
pub const ERROR_INVALID_STARTING_CODESEG: WIN32_ERROR = WIN32_ERROR(188u32);
pub const ERROR_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5023u32);
pub const ERROR_INVALID_SUB_AUTHORITY: WIN32_ERROR = WIN32_ERROR(1335u32);
pub const ERROR_INVALID_TABLE: WIN32_ERROR = WIN32_ERROR(1628u32);
pub const ERROR_INVALID_TARGET: WIN32_ERROR = WIN32_ERROR(3758096947u32);
pub const ERROR_INVALID_TARGET_HANDLE: WIN32_ERROR = WIN32_ERROR(114u32);
pub const ERROR_INVALID_TASK_INDEX: WIN32_ERROR = WIN32_ERROR(1551u32);
pub const ERROR_INVALID_TASK_NAME: WIN32_ERROR = WIN32_ERROR(1550u32);
pub const ERROR_INVALID_THREAD_ID: WIN32_ERROR = WIN32_ERROR(1444u32);
pub const ERROR_INVALID_TIME: WIN32_ERROR = WIN32_ERROR(1901u32);
pub const ERROR_INVALID_TOKEN: WIN32_ERROR = WIN32_ERROR(315u32);
pub const ERROR_INVALID_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6700u32);
pub const ERROR_INVALID_TRANSFORM: WIN32_ERROR = WIN32_ERROR(2020u32);
pub const ERROR_INVALID_UNWIND_TARGET: WIN32_ERROR = WIN32_ERROR(544u32);
pub const ERROR_INVALID_USER_BUFFER: WIN32_ERROR = WIN32_ERROR(1784u32);
pub const ERROR_INVALID_USER_PRINCIPAL_NAME: WIN32_ERROR = WIN32_ERROR(8636u32);
pub const ERROR_INVALID_VARIANT: WIN32_ERROR = WIN32_ERROR(604u32);
pub const ERROR_INVALID_VERIFY_SWITCH: WIN32_ERROR = WIN32_ERROR(118u32);
pub const ERROR_INVALID_WINDOW_HANDLE: WIN32_ERROR = WIN32_ERROR(1400u32);
pub const ERROR_INVALID_WINDOW_STYLE: WIN32_ERROR = WIN32_ERROR(2002u32);
pub const ERROR_INVALID_WORKSTATION: WIN32_ERROR = WIN32_ERROR(1329u32);
pub const ERROR_IN_WOW64: WIN32_ERROR = WIN32_ERROR(3758096949u32);
pub const ERROR_IOPL_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(197u32);
pub const ERROR_IO_DEVICE: WIN32_ERROR = WIN32_ERROR(1117u32);
pub const ERROR_IO_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(996u32);
pub const ERROR_IO_PENDING: WIN32_ERROR = WIN32_ERROR(997u32);
pub const ERROR_IO_PREEMPTED: windows_core::HRESULT = windows_core::HRESULT(0x89010001_u32 as _);
pub const ERROR_IO_PRIVILEGE_FAILED: WIN32_ERROR = WIN32_ERROR(571u32);
pub const ERROR_IO_REISSUE_AS_CACHED: WIN32_ERROR = WIN32_ERROR(3950u32);
pub const ERROR_IPSEC_AUTH_FIREWALL_DROP: WIN32_ERROR = WIN32_ERROR(13917u32);
pub const ERROR_IPSEC_BAD_SPI: WIN32_ERROR = WIN32_ERROR(13910u32);
pub const ERROR_IPSEC_CLEAR_TEXT_DROP: WIN32_ERROR = WIN32_ERROR(13916u32);
pub const ERROR_IPSEC_DEFAULT_MM_AUTH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13014u32);
pub const ERROR_IPSEC_DEFAULT_MM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13013u32);
pub const ERROR_IPSEC_DEFAULT_QM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13015u32);
pub const ERROR_IPSEC_DOSP_BLOCK: WIN32_ERROR = WIN32_ERROR(13925u32);
pub const ERROR_IPSEC_DOSP_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(13927u32);
pub const ERROR_IPSEC_DOSP_KEYMOD_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(13930u32);
pub const ERROR_IPSEC_DOSP_MAX_ENTRIES: WIN32_ERROR = WIN32_ERROR(13929u32);
pub const ERROR_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES: WIN32_ERROR = WIN32_ERROR(13932u32);
pub const ERROR_IPSEC_DOSP_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(13931u32);
pub const ERROR_IPSEC_DOSP_RECEIVED_MULTICAST: WIN32_ERROR = WIN32_ERROR(13926u32);
pub const ERROR_IPSEC_DOSP_STATE_LOOKUP_FAILED: WIN32_ERROR = WIN32_ERROR(13928u32);
pub const ERROR_IPSEC_IKE_ADD_UPDATE_KEY_FAILED: WIN32_ERROR = WIN32_ERROR(13860u32);
pub const ERROR_IPSEC_IKE_ATTRIB_FAIL: WIN32_ERROR = WIN32_ERROR(13802u32);
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE: WIN32_ERROR = WIN32_ERROR(13905u32);
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY: WIN32_ERROR = WIN32_ERROR(13907u32);
pub const ERROR_IPSEC_IKE_AUTH_FAIL: WIN32_ERROR = WIN32_ERROR(13801u32);
pub const ERROR_IPSEC_IKE_BENIGN_REINIT: WIN32_ERROR = WIN32_ERROR(13878u32);
pub const ERROR_IPSEC_IKE_CERT_CHAIN_POLICY_MISMATCH: WIN32_ERROR = WIN32_ERROR(13887u32);
pub const ERROR_IPSEC_IKE_CGA_AUTH_FAILED: WIN32_ERROR = WIN32_ERROR(13892u32);
pub const ERROR_IPSEC_IKE_COEXISTENCE_SUPPRESS: WIN32_ERROR = WIN32_ERROR(13902u32);
pub const ERROR_IPSEC_IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED: WIN32_ERROR = WIN32_ERROR(13823u32);
pub const ERROR_IPSEC_IKE_CRL_FAILED: WIN32_ERROR = WIN32_ERROR(13817u32);
pub const ERROR_IPSEC_IKE_DECRYPT: WIN32_ERROR = WIN32_ERROR(13867u32);
pub const ERROR_IPSEC_IKE_DH_FAIL: WIN32_ERROR = WIN32_ERROR(13822u32);
pub const ERROR_IPSEC_IKE_DH_FAILURE: WIN32_ERROR = WIN32_ERROR(13864u32);
pub const ERROR_IPSEC_IKE_DOS_COOKIE_SENT: WIN32_ERROR = WIN32_ERROR(13890u32);
pub const ERROR_IPSEC_IKE_DROP_NO_RESPONSE: WIN32_ERROR = WIN32_ERROR(13813u32);
pub const ERROR_IPSEC_IKE_ENCRYPT: WIN32_ERROR = WIN32_ERROR(13866u32);
pub const ERROR_IPSEC_IKE_ERROR: WIN32_ERROR = WIN32_ERROR(13816u32);
pub const ERROR_IPSEC_IKE_FAILQUERYSSP: WIN32_ERROR = WIN32_ERROR(13854u32);
pub const ERROR_IPSEC_IKE_FAILSSPINIT: WIN32_ERROR = WIN32_ERROR(13853u32);
pub const ERROR_IPSEC_IKE_GENERAL_PROCESSING_ERROR: WIN32_ERROR = WIN32_ERROR(13804u32);
pub const ERROR_IPSEC_IKE_GETSPIFAIL: WIN32_ERROR = WIN32_ERROR(13857u32);
pub const ERROR_IPSEC_IKE_INNER_IP_ASSIGNMENT_FAILURE: WIN32_ERROR = WIN32_ERROR(13899u32);
pub const ERROR_IPSEC_IKE_INVALID_AUTH_ALG: WIN32_ERROR = WIN32_ERROR(13874u32);
pub const ERROR_IPSEC_IKE_INVALID_AUTH_PAYLOAD: WIN32_ERROR = WIN32_ERROR(13889u32);
pub const ERROR_IPSEC_IKE_INVALID_CERT_KEYLEN: WIN32_ERROR = WIN32_ERROR(13881u32);
pub const ERROR_IPSEC_IKE_INVALID_CERT_TYPE: WIN32_ERROR = WIN32_ERROR(13819u32);
pub const ERROR_IPSEC_IKE_INVALID_COOKIE: WIN32_ERROR = WIN32_ERROR(13846u32);
pub const ERROR_IPSEC_IKE_INVALID_ENCRYPT_ALG: WIN32_ERROR = WIN32_ERROR(13873u32);
pub const ERROR_IPSEC_IKE_INVALID_FILTER: WIN32_ERROR = WIN32_ERROR(13858u32);
pub const ERROR_IPSEC_IKE_INVALID_GROUP: WIN32_ERROR = WIN32_ERROR(13865u32);
pub const ERROR_IPSEC_IKE_INVALID_HASH: WIN32_ERROR = WIN32_ERROR(13870u32);
pub const ERROR_IPSEC_IKE_INVALID_HASH_ALG: WIN32_ERROR = WIN32_ERROR(13871u32);
pub const ERROR_IPSEC_IKE_INVALID_HASH_SIZE: WIN32_ERROR = WIN32_ERROR(13872u32);
pub const ERROR_IPSEC_IKE_INVALID_HEADER: WIN32_ERROR = WIN32_ERROR(13824u32);
pub const ERROR_IPSEC_IKE_INVALID_KEY_USAGE: WIN32_ERROR = WIN32_ERROR(13818u32);
pub const ERROR_IPSEC_IKE_INVALID_MAJOR_VERSION: WIN32_ERROR = WIN32_ERROR(13880u32);
pub const ERROR_IPSEC_IKE_INVALID_MM_FOR_QM: WIN32_ERROR = WIN32_ERROR(13894u32);
pub const ERROR_IPSEC_IKE_INVALID_PAYLOAD: WIN32_ERROR = WIN32_ERROR(13843u32);
pub const ERROR_IPSEC_IKE_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(13861u32);
pub const ERROR_IPSEC_IKE_INVALID_RESPONDER_LIFETIME_NOTIFY: WIN32_ERROR = WIN32_ERROR(13879u32);
pub const ERROR_IPSEC_IKE_INVALID_SIG: WIN32_ERROR = WIN32_ERROR(13875u32);
pub const ERROR_IPSEC_IKE_INVALID_SIGNATURE: WIN32_ERROR = WIN32_ERROR(13826u32);
pub const ERROR_IPSEC_IKE_INVALID_SITUATION: WIN32_ERROR = WIN32_ERROR(13863u32);
pub const ERROR_IPSEC_IKE_KERBEROS_ERROR: WIN32_ERROR = WIN32_ERROR(13827u32);
pub const ERROR_IPSEC_IKE_KILL_DUMMY_NAP_TUNNEL: WIN32_ERROR = WIN32_ERROR(13898u32);
pub const ERROR_IPSEC_IKE_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(13876u32);
pub const ERROR_IPSEC_IKE_LOAD_SOFT_SA: WIN32_ERROR = WIN32_ERROR(13844u32);
pub const ERROR_IPSEC_IKE_MM_ACQUIRE_DROP: WIN32_ERROR = WIN32_ERROR(13809u32);
pub const ERROR_IPSEC_IKE_MM_DELAY_DROP: WIN32_ERROR = WIN32_ERROR(13814u32);
pub const ERROR_IPSEC_IKE_MM_EXPIRED: WIN32_ERROR = WIN32_ERROR(13885u32);
pub const ERROR_IPSEC_IKE_MM_LIMIT: WIN32_ERROR = WIN32_ERROR(13882u32);
pub const ERROR_IPSEC_IKE_NEGOTIATION_DISABLED: WIN32_ERROR = WIN32_ERROR(13883u32);
pub const ERROR_IPSEC_IKE_NEGOTIATION_PENDING: WIN32_ERROR = WIN32_ERROR(13803u32);
pub const ERROR_IPSEC_IKE_NEG_STATUS_BEGIN: WIN32_ERROR = WIN32_ERROR(13800u32);
pub const ERROR_IPSEC_IKE_NEG_STATUS_END: WIN32_ERROR = WIN32_ERROR(13897u32);
pub const ERROR_IPSEC_IKE_NEG_STATUS_EXTENDED_END: WIN32_ERROR = WIN32_ERROR(13909u32);
pub const ERROR_IPSEC_IKE_NOTCBPRIV: WIN32_ERROR = WIN32_ERROR(13851u32);
pub const ERROR_IPSEC_IKE_NO_CERT: WIN32_ERROR = WIN32_ERROR(13806u32);
pub const ERROR_IPSEC_IKE_NO_MM_POLICY: WIN32_ERROR = WIN32_ERROR(13850u32);
pub const ERROR_IPSEC_IKE_NO_PEER_CERT: WIN32_ERROR = WIN32_ERROR(13847u32);
pub const ERROR_IPSEC_IKE_NO_POLICY: WIN32_ERROR = WIN32_ERROR(13825u32);
pub const ERROR_IPSEC_IKE_NO_PRIVATE_KEY: WIN32_ERROR = WIN32_ERROR(13820u32);
pub const ERROR_IPSEC_IKE_NO_PUBLIC_KEY: WIN32_ERROR = WIN32_ERROR(13828u32);
pub const ERROR_IPSEC_IKE_OUT_OF_MEMORY: WIN32_ERROR = WIN32_ERROR(13859u32);
pub const ERROR_IPSEC_IKE_PEER_CRL_FAILED: WIN32_ERROR = WIN32_ERROR(13848u32);
pub const ERROR_IPSEC_IKE_PEER_DOESNT_SUPPORT_MOBIKE: WIN32_ERROR = WIN32_ERROR(13904u32);
pub const ERROR_IPSEC_IKE_PEER_MM_ASSUMED_INVALID: WIN32_ERROR = WIN32_ERROR(13886u32);
pub const ERROR_IPSEC_IKE_POLICY_CHANGE: WIN32_ERROR = WIN32_ERROR(13849u32);
pub const ERROR_IPSEC_IKE_POLICY_MATCH: WIN32_ERROR = WIN32_ERROR(13868u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR: WIN32_ERROR = WIN32_ERROR(13829u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT: WIN32_ERROR = WIN32_ERROR(13835u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT_REQ: WIN32_ERROR = WIN32_ERROR(13836u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_DELETE: WIN32_ERROR = WIN32_ERROR(13841u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_HASH: WIN32_ERROR = WIN32_ERROR(13837u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_ID: WIN32_ERROR = WIN32_ERROR(13834u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_KE: WIN32_ERROR = WIN32_ERROR(13833u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NATOA: WIN32_ERROR = WIN32_ERROR(13893u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NONCE: WIN32_ERROR = WIN32_ERROR(13839u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NOTIFY: WIN32_ERROR = WIN32_ERROR(13840u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_PROP: WIN32_ERROR = WIN32_ERROR(13831u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SA: WIN32_ERROR = WIN32_ERROR(13830u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SIG: WIN32_ERROR = WIN32_ERROR(13838u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_TRANS: WIN32_ERROR = WIN32_ERROR(13832u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_VENDOR: WIN32_ERROR = WIN32_ERROR(13842u32);
pub const ERROR_IPSEC_IKE_QM_ACQUIRE_DROP: WIN32_ERROR = WIN32_ERROR(13810u32);
pub const ERROR_IPSEC_IKE_QM_DELAY_DROP: WIN32_ERROR = WIN32_ERROR(13815u32);
pub const ERROR_IPSEC_IKE_QM_EXPIRED: WIN32_ERROR = WIN32_ERROR(13895u32);
pub const ERROR_IPSEC_IKE_QM_LIMIT: WIN32_ERROR = WIN32_ERROR(13884u32);
pub const ERROR_IPSEC_IKE_QUEUE_DROP_MM: WIN32_ERROR = WIN32_ERROR(13811u32);
pub const ERROR_IPSEC_IKE_QUEUE_DROP_NO_MM: WIN32_ERROR = WIN32_ERROR(13812u32);
pub const ERROR_IPSEC_IKE_RATELIMIT_DROP: WIN32_ERROR = WIN32_ERROR(13903u32);
pub const ERROR_IPSEC_IKE_REQUIRE_CP_PAYLOAD_MISSING: WIN32_ERROR = WIN32_ERROR(13900u32);
pub const ERROR_IPSEC_IKE_RPC_DELETE: WIN32_ERROR = WIN32_ERROR(13877u32);
pub const ERROR_IPSEC_IKE_SA_DELETED: WIN32_ERROR = WIN32_ERROR(13807u32);
pub const ERROR_IPSEC_IKE_SA_REAPED: WIN32_ERROR = WIN32_ERROR(13808u32);
pub const ERROR_IPSEC_IKE_SECLOADFAIL: WIN32_ERROR = WIN32_ERROR(13852u32);
pub const ERROR_IPSEC_IKE_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(13891u32);
pub const ERROR_IPSEC_IKE_SIMULTANEOUS_REKEY: WIN32_ERROR = WIN32_ERROR(13821u32);
pub const ERROR_IPSEC_IKE_SOFT_SA_TORN_DOWN: WIN32_ERROR = WIN32_ERROR(13845u32);
pub const ERROR_IPSEC_IKE_SRVACQFAIL: WIN32_ERROR = WIN32_ERROR(13855u32);
pub const ERROR_IPSEC_IKE_SRVQUERYCRED: WIN32_ERROR = WIN32_ERROR(13856u32);
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE: WIN32_ERROR = WIN32_ERROR(13908u32);
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_FAILURE: WIN32_ERROR = WIN32_ERROR(13906u32);
pub const ERROR_IPSEC_IKE_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(13805u32);
pub const ERROR_IPSEC_IKE_TOO_MANY_FILTERS: WIN32_ERROR = WIN32_ERROR(13896u32);
pub const ERROR_IPSEC_IKE_UNEXPECTED_MESSAGE_ID: WIN32_ERROR = WIN32_ERROR(13888u32);
pub const ERROR_IPSEC_IKE_UNKNOWN_DOI: WIN32_ERROR = WIN32_ERROR(13862u32);
pub const ERROR_IPSEC_IKE_UNSUPPORTED_ID: WIN32_ERROR = WIN32_ERROR(13869u32);
pub const ERROR_IPSEC_INTEGRITY_CHECK_FAILED: WIN32_ERROR = WIN32_ERROR(13915u32);
pub const ERROR_IPSEC_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(13914u32);
pub const ERROR_IPSEC_KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING: WIN32_ERROR = WIN32_ERROR(13901u32);
pub const ERROR_IPSEC_MM_AUTH_EXISTS: WIN32_ERROR = WIN32_ERROR(13010u32);
pub const ERROR_IPSEC_MM_AUTH_IN_USE: WIN32_ERROR = WIN32_ERROR(13012u32);
pub const ERROR_IPSEC_MM_AUTH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13011u32);
pub const ERROR_IPSEC_MM_AUTH_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13022u32);
pub const ERROR_IPSEC_MM_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13006u32);
pub const ERROR_IPSEC_MM_FILTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13007u32);
pub const ERROR_IPSEC_MM_FILTER_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13018u32);
pub const ERROR_IPSEC_MM_POLICY_EXISTS: WIN32_ERROR = WIN32_ERROR(13003u32);
pub const ERROR_IPSEC_MM_POLICY_IN_USE: WIN32_ERROR = WIN32_ERROR(13005u32);
pub const ERROR_IPSEC_MM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13004u32);
pub const ERROR_IPSEC_MM_POLICY_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13021u32);
pub const ERROR_IPSEC_QM_POLICY_EXISTS: WIN32_ERROR = WIN32_ERROR(13000u32);
pub const ERROR_IPSEC_QM_POLICY_IN_USE: WIN32_ERROR = WIN32_ERROR(13002u32);
pub const ERROR_IPSEC_QM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13001u32);
pub const ERROR_IPSEC_QM_POLICY_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13023u32);
pub const ERROR_IPSEC_REPLAY_CHECK_FAILED: WIN32_ERROR = WIN32_ERROR(13913u32);
pub const ERROR_IPSEC_SA_LIFETIME_EXPIRED: WIN32_ERROR = WIN32_ERROR(13911u32);
pub const ERROR_IPSEC_THROTTLE_DROP: WIN32_ERROR = WIN32_ERROR(13918u32);
pub const ERROR_IPSEC_TRANSPORT_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13008u32);
pub const ERROR_IPSEC_TRANSPORT_FILTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13009u32);
pub const ERROR_IPSEC_TRANSPORT_FILTER_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13019u32);
pub const ERROR_IPSEC_TUNNEL_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13016u32);
pub const ERROR_IPSEC_TUNNEL_FILTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13017u32);
pub const ERROR_IPSEC_TUNNEL_FILTER_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13020u32);
pub const ERROR_IPSEC_WRONG_SA: WIN32_ERROR = WIN32_ERROR(13912u32);
pub const ERROR_IP_ADDRESS_CONFLICT1: WIN32_ERROR = WIN32_ERROR(611u32);
pub const ERROR_IP_ADDRESS_CONFLICT2: WIN32_ERROR = WIN32_ERROR(612u32);
pub const ERROR_IRQ_BUSY: WIN32_ERROR = WIN32_ERROR(1119u32);
pub const ERROR_IS_JOINED: WIN32_ERROR = WIN32_ERROR(134u32);
pub const ERROR_IS_JOIN_PATH: WIN32_ERROR = WIN32_ERROR(147u32);
pub const ERROR_IS_JOIN_TARGET: WIN32_ERROR = WIN32_ERROR(133u32);
pub const ERROR_IS_SUBSTED: WIN32_ERROR = WIN32_ERROR(135u32);
pub const ERROR_IS_SUBST_PATH: WIN32_ERROR = WIN32_ERROR(146u32);
pub const ERROR_IS_SUBST_TARGET: WIN32_ERROR = WIN32_ERROR(149u32);
pub const ERROR_ITERATED_DATA_EXCEEDS_64k: WIN32_ERROR = WIN32_ERROR(194u32);
pub const ERROR_JOB_NO_CONTAINER: WIN32_ERROR = WIN32_ERROR(1505u32);
pub const ERROR_JOIN_TO_JOIN: WIN32_ERROR = WIN32_ERROR(138u32);
pub const ERROR_JOIN_TO_SUBST: WIN32_ERROR = WIN32_ERROR(140u32);
pub const ERROR_JOURNAL_DELETE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1178u32);
pub const ERROR_JOURNAL_ENTRY_DELETED: WIN32_ERROR = WIN32_ERROR(1181u32);
pub const ERROR_JOURNAL_HOOK_SET: WIN32_ERROR = WIN32_ERROR(1430u32);
pub const ERROR_JOURNAL_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(1179u32);
pub const ERROR_KERNEL_APC: WIN32_ERROR = WIN32_ERROR(738u32);
pub const ERROR_KEY_DELETED: WIN32_ERROR = WIN32_ERROR(1018u32);
pub const ERROR_KEY_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(3758096900u32);
pub const ERROR_KEY_HAS_CHILDREN: WIN32_ERROR = WIN32_ERROR(1020u32);
pub const ERROR_KM_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(1930u32);
pub const ERROR_LABEL_TOO_LONG: WIN32_ERROR = WIN32_ERROR(154u32);
pub const ERROR_LAPS_ENCRYPTION_REQUIRES_2016_DFL: WIN32_ERROR = WIN32_ERROR(8657u32);
pub const ERROR_LAPS_LEGACY_SCHEMA_MISSING: WIN32_ERROR = WIN32_ERROR(8655u32);
pub const ERROR_LAPS_SCHEMA_MISSING: WIN32_ERROR = WIN32_ERROR(8656u32);
pub const ERROR_LAST_ADMIN: WIN32_ERROR = WIN32_ERROR(1322u32);
pub const ERROR_LB_WITHOUT_TABSTOPS: WIN32_ERROR = WIN32_ERROR(1434u32);
pub const ERROR_LIBRARY_FULL: WIN32_ERROR = WIN32_ERROR(4322u32);
pub const ERROR_LIBRARY_OFFLINE: WIN32_ERROR = WIN32_ERROR(4305u32);
pub const ERROR_LICENSE_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1395u32);
pub const ERROR_LINE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3758096642u32);
pub const ERROR_LINUX_SUBSYSTEM_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(414u32);
pub const ERROR_LINUX_SUBSYSTEM_UPDATE_REQUIRED: WIN32_ERROR = WIN32_ERROR(444u32);
pub const ERROR_LISTBOX_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1416u32);
pub const ERROR_LM_CROSS_ENCRYPTION_REQUIRED: WIN32_ERROR = WIN32_ERROR(1390u32);
pub const ERROR_LOCAL_POLICY_MODIFICATION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8653u32);
pub const ERROR_LOCAL_USER_SESSION_KEY: WIN32_ERROR = WIN32_ERROR(1303u32);
pub const ERROR_LOCKED: WIN32_ERROR = WIN32_ERROR(212u32);
pub const ERROR_LOCK_FAILED: WIN32_ERROR = WIN32_ERROR(167u32);
pub const ERROR_LOCK_VIOLATION: WIN32_ERROR = WIN32_ERROR(33u32);
pub const ERROR_LOGIN_TIME_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1239u32);
pub const ERROR_LOGIN_WKSTA_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1240u32);
pub const ERROR_LOGON_FAILURE: WIN32_ERROR = WIN32_ERROR(1326u32);
pub const ERROR_LOGON_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(1380u32);
pub const ERROR_LOGON_SERVER_CONFLICT: WIN32_ERROR = WIN32_ERROR(568u32);
pub const ERROR_LOGON_SESSION_COLLISION: WIN32_ERROR = WIN32_ERROR(1366u32);
pub const ERROR_LOGON_SESSION_EXISTS: WIN32_ERROR = WIN32_ERROR(1363u32);
pub const ERROR_LOGON_TYPE_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(1385u32);
pub const ERROR_LOG_APPENDED_FLUSH_FAILED: WIN32_ERROR = WIN32_ERROR(6647u32);
pub const ERROR_LOG_ARCHIVE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6633u32);
pub const ERROR_LOG_ARCHIVE_NOT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6632u32);
pub const ERROR_LOG_BLOCKS_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(6605u32);
pub const ERROR_LOG_BLOCK_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(6603u32);
pub const ERROR_LOG_BLOCK_INVALID: WIN32_ERROR = WIN32_ERROR(6609u32);
pub const ERROR_LOG_BLOCK_VERSION: WIN32_ERROR = WIN32_ERROR(6608u32);
pub const ERROR_LOG_CANT_DELETE: WIN32_ERROR = WIN32_ERROR(6616u32);
pub const ERROR_LOG_CLIENT_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(6636u32);
pub const ERROR_LOG_CLIENT_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(6637u32);
pub const ERROR_LOG_CONTAINER_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(6617u32);
pub const ERROR_LOG_CONTAINER_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(6641u32);
pub const ERROR_LOG_CONTAINER_READ_FAILED: WIN32_ERROR = WIN32_ERROR(6639u32);
pub const ERROR_LOG_CONTAINER_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(6642u32);
pub const ERROR_LOG_CONTAINER_WRITE_FAILED: WIN32_ERROR = WIN32_ERROR(6640u32);
pub const ERROR_LOG_CORRUPTION_DETECTED: WIN32_ERROR = WIN32_ERROR(6817u32);
pub const ERROR_LOG_DEDICATED: WIN32_ERROR = WIN32_ERROR(6631u32);
pub const ERROR_LOG_EPHEMERAL: WIN32_ERROR = WIN32_ERROR(6634u32);
pub const ERROR_LOG_FILE_FULL: WIN32_ERROR = WIN32_ERROR(1502u32);
pub const ERROR_LOG_FULL: WIN32_ERROR = WIN32_ERROR(6628u32);
pub const ERROR_LOG_FULL_HANDLER_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6638u32);
pub const ERROR_LOG_GROWTH_FAILED: WIN32_ERROR = WIN32_ERROR(6833u32);
pub const ERROR_LOG_HARD_ERROR: WIN32_ERROR = WIN32_ERROR(718u32);
pub const ERROR_LOG_INCONSISTENT_SECURITY: WIN32_ERROR = WIN32_ERROR(6646u32);
pub const ERROR_LOG_INVALID_RANGE: WIN32_ERROR = WIN32_ERROR(6604u32);
pub const ERROR_LOG_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(6612u32);
pub const ERROR_LOG_METADATA_FLUSH_FAILED: WIN32_ERROR = WIN32_ERROR(6645u32);
pub const ERROR_LOG_METADATA_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(6614u32);
pub const ERROR_LOG_METADATA_INVALID: WIN32_ERROR = WIN32_ERROR(6613u32);
pub const ERROR_LOG_MULTIPLEXED: WIN32_ERROR = WIN32_ERROR(6630u32);
pub const ERROR_LOG_NOT_ENOUGH_CONTAINERS: WIN32_ERROR = WIN32_ERROR(6635u32);
pub const ERROR_LOG_NO_RESTART: WIN32_ERROR = WIN32_ERROR(6611u32);
pub const ERROR_LOG_PINNED: WIN32_ERROR = WIN32_ERROR(6644u32);
pub const ERROR_LOG_PINNED_ARCHIVE_TAIL: WIN32_ERROR = WIN32_ERROR(6623u32);
pub const ERROR_LOG_PINNED_RESERVATION: WIN32_ERROR = WIN32_ERROR(6648u32);
pub const ERROR_LOG_POLICY_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(6619u32);
pub const ERROR_LOG_POLICY_CONFLICT: WIN32_ERROR = WIN32_ERROR(6622u32);
pub const ERROR_LOG_POLICY_INVALID: WIN32_ERROR = WIN32_ERROR(6621u32);
pub const ERROR_LOG_POLICY_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(6620u32);
pub const ERROR_LOG_READ_CONTEXT_INVALID: WIN32_ERROR = WIN32_ERROR(6606u32);
pub const ERROR_LOG_READ_MODE_INVALID: WIN32_ERROR = WIN32_ERROR(6610u32);
pub const ERROR_LOG_RECORDS_RESERVED_INVALID: WIN32_ERROR = WIN32_ERROR(6625u32);
pub const ERROR_LOG_RECORD_NONEXISTENT: WIN32_ERROR = WIN32_ERROR(6624u32);
pub const ERROR_LOG_RESERVATION_INVALID: WIN32_ERROR = WIN32_ERROR(6615u32);
pub const ERROR_LOG_RESIZE_INVALID_SIZE: WIN32_ERROR = WIN32_ERROR(6806u32);
pub const ERROR_LOG_RESTART_INVALID: WIN32_ERROR = WIN32_ERROR(6607u32);
pub const ERROR_LOG_SECTOR_INVALID: WIN32_ERROR = WIN32_ERROR(6600u32);
pub const ERROR_LOG_SECTOR_PARITY_INVALID: WIN32_ERROR = WIN32_ERROR(6601u32);
pub const ERROR_LOG_SECTOR_REMAPPED: WIN32_ERROR = WIN32_ERROR(6602u32);
pub const ERROR_LOG_SPACE_RESERVED_INVALID: WIN32_ERROR = WIN32_ERROR(6626u32);
pub const ERROR_LOG_START_OF_LOG: WIN32_ERROR = WIN32_ERROR(6618u32);
pub const ERROR_LOG_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(6643u32);
pub const ERROR_LOG_TAIL_INVALID: WIN32_ERROR = WIN32_ERROR(6627u32);
pub const ERROR_LONGJUMP: WIN32_ERROR = WIN32_ERROR(682u32);
pub const ERROR_LOST_MODE_LOGON_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1939u32);
pub const ERROR_LOST_WRITEBEHIND_DATA: WIN32_ERROR = WIN32_ERROR(596u32);
pub const ERROR_LOST_WRITEBEHIND_DATA_LOCAL_DISK_ERROR: WIN32_ERROR = WIN32_ERROR(790u32);
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(788u32);
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_SERVER_ERROR: WIN32_ERROR = WIN32_ERROR(789u32);
pub const ERROR_LUIDS_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(1334u32);
pub const ERROR_MACHINE_LOCKED: WIN32_ERROR = WIN32_ERROR(1271u32);
pub const ERROR_MACHINE_SCOPE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15666u32);
pub const ERROR_MACHINE_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(3758096930u32);
pub const ERROR_MAGAZINE_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(1163u32);
pub const ERROR_MALFORMED_SUBSTITUTION_STRING: WIN32_ERROR = WIN32_ERROR(14094u32);
pub const ERROR_MAPPED_ALIGNMENT: WIN32_ERROR = WIN32_ERROR(1132u32);
pub const ERROR_MARKED_TO_DISALLOW_WRITES: WIN32_ERROR = WIN32_ERROR(348u32);
pub const ERROR_MARSHALL_OVERFLOW: WIN32_ERROR = WIN32_ERROR(603u32);
pub const ERROR_MAX_CLIENT_INTERFACE_LIMIT: u32 = 935u32;
pub const ERROR_MAX_LAN_INTERFACE_LIMIT: u32 = 933u32;
pub const ERROR_MAX_SESSIONS_REACHED: WIN32_ERROR = WIN32_ERROR(353u32);
pub const ERROR_MAX_THRDS_REACHED: WIN32_ERROR = WIN32_ERROR(164u32);
pub const ERROR_MAX_WAN_INTERFACE_LIMIT: u32 = 934u32;
pub const ERROR_MCA_EXCEPTION: WIN32_ERROR = WIN32_ERROR(784u32);
pub const ERROR_MCA_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(15205u32);
pub const ERROR_MCA_INVALID_CAPABILITIES_STRING: WIN32_ERROR = WIN32_ERROR(15200u32);
pub const ERROR_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: WIN32_ERROR = WIN32_ERROR(15206u32);
pub const ERROR_MCA_INVALID_VCP_VERSION: WIN32_ERROR = WIN32_ERROR(15201u32);
pub const ERROR_MCA_MCCS_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(15203u32);
pub const ERROR_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: WIN32_ERROR = WIN32_ERROR(15202u32);
pub const ERROR_MCA_OCCURED: WIN32_ERROR = WIN32_ERROR(651u32);
pub const ERROR_MCA_UNSUPPORTED_COLOR_TEMPERATURE: WIN32_ERROR = WIN32_ERROR(15207u32);
pub const ERROR_MCA_UNSUPPORTED_MCCS_VERSION: WIN32_ERROR = WIN32_ERROR(15204u32);
pub const ERROR_MEDIA_CHANGED: WIN32_ERROR = WIN32_ERROR(1110u32);
pub const ERROR_MEDIA_CHECK: WIN32_ERROR = WIN32_ERROR(679u32);
pub const ERROR_MEDIA_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(4315u32);
pub const ERROR_MEDIA_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(4318u32);
pub const ERROR_MEDIA_OFFLINE: WIN32_ERROR = WIN32_ERROR(4304u32);
pub const ERROR_MEDIA_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(4308u32);
pub const ERROR_MEDIUM_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(4323u32);
pub const ERROR_MEMBERS_PRIMARY_GROUP: WIN32_ERROR = WIN32_ERROR(1374u32);
pub const ERROR_MEMBER_IN_ALIAS: WIN32_ERROR = WIN32_ERROR(1378u32);
pub const ERROR_MEMBER_IN_GROUP: WIN32_ERROR = WIN32_ERROR(1320u32);
pub const ERROR_MEMBER_NOT_IN_ALIAS: WIN32_ERROR = WIN32_ERROR(1377u32);
pub const ERROR_MEMBER_NOT_IN_GROUP: WIN32_ERROR = WIN32_ERROR(1321u32);
pub const ERROR_MEMORY_HARDWARE: WIN32_ERROR = WIN32_ERROR(779u32);
pub const ERROR_MENU_ITEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1456u32);
pub const ERROR_MESSAGE_EXCEEDS_MAX_SIZE: WIN32_ERROR = WIN32_ERROR(4336u32);
pub const ERROR_MESSAGE_SYNC_ONLY: WIN32_ERROR = WIN32_ERROR(1159u32);
pub const ERROR_METAFILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2003u32);
pub const ERROR_META_EXPANSION_TOO_LONG: WIN32_ERROR = WIN32_ERROR(208u32);
pub const ERROR_MINIVERSION_INACCESSIBLE_FROM_SPECIFIED_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6810u32);
pub const ERROR_MISSING_SYSTEMFILE: WIN32_ERROR = WIN32_ERROR(573u32);
pub const ERROR_MOD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(126u32);
pub const ERROR_MONITOR_INVALID_DESCRIPTOR_CHECKSUM: windows_core::HRESULT = windows_core::HRESULT(0xC0261003_u32 as _);
pub const ERROR_MONITOR_INVALID_DETAILED_TIMING_BLOCK: windows_core::HRESULT = windows_core::HRESULT(0xC0261009_u32 as _);
pub const ERROR_MONITOR_INVALID_MANUFACTURE_DATE: windows_core::HRESULT = windows_core::HRESULT(0xC026100A_u32 as _);
pub const ERROR_MONITOR_INVALID_SERIAL_NUMBER_MONDSC_BLOCK: windows_core::HRESULT = windows_core::HRESULT(0xC0261006_u32 as _);
pub const ERROR_MONITOR_INVALID_STANDARD_TIMING_BLOCK: windows_core::HRESULT = windows_core::HRESULT(0xC0261004_u32 as _);
pub const ERROR_MONITOR_INVALID_USER_FRIENDLY_MONDSC_BLOCK: windows_core::HRESULT = windows_core::HRESULT(0xC0261007_u32 as _);
pub const ERROR_MONITOR_NO_DESCRIPTOR: windows_core::HRESULT = windows_core::HRESULT(0x261001_u32 as _);
pub const ERROR_MONITOR_NO_MORE_DESCRIPTOR_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC0261008_u32 as _);
pub const ERROR_MONITOR_UNKNOWN_DESCRIPTOR_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x261002_u32 as _);
pub const ERROR_MONITOR_WMI_DATABLOCK_REGISTRATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC0261005_u32 as _);
pub const ERROR_MORE_DATA: WIN32_ERROR = WIN32_ERROR(234u32);
pub const ERROR_MORE_WRITES: WIN32_ERROR = WIN32_ERROR(1120u32);
pub const ERROR_MOUNT_POINT_NOT_RESOLVED: WIN32_ERROR = WIN32_ERROR(649u32);
pub const ERROR_MP_PROCESSOR_MISMATCH: WIN32_ERROR = WIN32_ERROR(725u32);
pub const ERROR_MRM_AUTOMERGE_ENABLED: WIN32_ERROR = WIN32_ERROR(15139u32);
pub const ERROR_MRM_DIRECT_REF_TO_NON_DEFAULT_RESOURCE: WIN32_ERROR = WIN32_ERROR(15146u32);
pub const ERROR_MRM_DUPLICATE_ENTRY: WIN32_ERROR = WIN32_ERROR(15119u32);
pub const ERROR_MRM_DUPLICATE_MAP_NAME: WIN32_ERROR = WIN32_ERROR(15118u32);
pub const ERROR_MRM_FILEPATH_TOO_LONG: WIN32_ERROR = WIN32_ERROR(15121u32);
pub const ERROR_MRM_GENERATION_COUNT_MISMATCH: WIN32_ERROR = WIN32_ERROR(15147u32);
pub const ERROR_MRM_INDETERMINATE_QUALIFIER_VALUE: WIN32_ERROR = WIN32_ERROR(15138u32);
pub const ERROR_MRM_INVALID_FILE_TYPE: WIN32_ERROR = WIN32_ERROR(15112u32);
pub const ERROR_MRM_INVALID_PRICONFIG: WIN32_ERROR = WIN32_ERROR(15111u32);
pub const ERROR_MRM_INVALID_PRI_FILE: WIN32_ERROR = WIN32_ERROR(15126u32);
pub const ERROR_MRM_INVALID_QUALIFIER_OPERATOR: WIN32_ERROR = WIN32_ERROR(15137u32);
pub const ERROR_MRM_INVALID_QUALIFIER_VALUE: WIN32_ERROR = WIN32_ERROR(15114u32);
pub const ERROR_MRM_INVALID_RESOURCE_IDENTIFIER: WIN32_ERROR = WIN32_ERROR(15120u32);
pub const ERROR_MRM_MAP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15135u32);
pub const ERROR_MRM_MISSING_DEFAULT_LANGUAGE: WIN32_ERROR = WIN32_ERROR(15160u32);
pub const ERROR_MRM_NAMED_RESOURCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15127u32);
pub const ERROR_MRM_NO_CANDIDATE: WIN32_ERROR = WIN32_ERROR(15115u32);
pub const ERROR_MRM_NO_CURRENT_VIEW_ON_THREAD: WIN32_ERROR = WIN32_ERROR(15143u32);
pub const ERROR_MRM_NO_MATCH_OR_DEFAULT_CANDIDATE: WIN32_ERROR = WIN32_ERROR(15116u32);
pub const ERROR_MRM_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15159u32);
pub const ERROR_MRM_RESOURCE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(15117u32);
pub const ERROR_MRM_RUNTIME_NO_DEFAULT_OR_NEUTRAL_RESOURCE: WIN32_ERROR = WIN32_ERROR(15110u32);
pub const ERROR_MRM_SCOPE_ITEM_CONFLICT: WIN32_ERROR = WIN32_ERROR(15161u32);
pub const ERROR_MRM_TOO_MANY_RESOURCES: WIN32_ERROR = WIN32_ERROR(15140u32);
pub const ERROR_MRM_UNKNOWN_QUALIFIER: WIN32_ERROR = WIN32_ERROR(15113u32);
pub const ERROR_MRM_UNSUPPORTED_DIRECTORY_TYPE: WIN32_ERROR = WIN32_ERROR(15122u32);
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_LOAD_UNLOAD_PRI_FILE: WIN32_ERROR = WIN32_ERROR(15142u32);
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_MERGE: WIN32_ERROR = WIN32_ERROR(15141u32);
pub const ERROR_MRM_UNSUPPORTED_PROFILE_TYPE: WIN32_ERROR = WIN32_ERROR(15136u32);
pub const ERROR_MR_MID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(317u32);
pub const ERROR_MUI_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15100u32);
pub const ERROR_MUI_FILE_NOT_LOADED: WIN32_ERROR = WIN32_ERROR(15105u32);
pub const ERROR_MUI_INTLSETTINGS_INVALID_LOCALE_NAME: WIN32_ERROR = WIN32_ERROR(15108u32);
pub const ERROR_MUI_INTLSETTINGS_UILANG_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(15107u32);
pub const ERROR_MUI_INVALID_FILE: WIN32_ERROR = WIN32_ERROR(15101u32);
pub const ERROR_MUI_INVALID_LOCALE_NAME: WIN32_ERROR = WIN32_ERROR(15103u32);
pub const ERROR_MUI_INVALID_RC_CONFIG: WIN32_ERROR = WIN32_ERROR(15102u32);
pub const ERROR_MUI_INVALID_ULTIMATEFALLBACK_NAME: WIN32_ERROR = WIN32_ERROR(15104u32);
pub const ERROR_MULTIPLE_FAULT_VIOLATION: WIN32_ERROR = WIN32_ERROR(640u32);
pub const ERROR_MUTANT_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(587u32);
pub const ERROR_MUTUAL_AUTH_FAILED: WIN32_ERROR = WIN32_ERROR(1397u32);
pub const ERROR_NDIS_ADAPTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891526u32);
pub const ERROR_NDIS_ADAPTER_NOT_READY: WIN32_ERROR = WIN32_ERROR(2150891537u32);
pub const ERROR_NDIS_ADAPTER_REMOVED: WIN32_ERROR = WIN32_ERROR(2150891544u32);
pub const ERROR_NDIS_ALREADY_MAPPED: WIN32_ERROR = WIN32_ERROR(2150891549u32);
pub const ERROR_NDIS_BAD_CHARACTERISTICS: WIN32_ERROR = WIN32_ERROR(2150891525u32);
pub const ERROR_NDIS_BAD_VERSION: WIN32_ERROR = WIN32_ERROR(2150891524u32);
pub const ERROR_NDIS_BUFFER_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(2150891542u32);
pub const ERROR_NDIS_DEVICE_FAILED: WIN32_ERROR = WIN32_ERROR(2150891528u32);
pub const ERROR_NDIS_DOT11_AP_BAND_CURRENTLY_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(2150899718u32);
pub const ERROR_NDIS_DOT11_AP_BAND_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(2150899720u32);
pub const ERROR_NDIS_DOT11_AP_CHANNEL_CURRENTLY_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(2150899717u32);
pub const ERROR_NDIS_DOT11_AP_CHANNEL_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(2150899719u32);
pub const ERROR_NDIS_DOT11_AUTO_CONFIG_ENABLED: WIN32_ERROR = WIN32_ERROR(2150899712u32);
pub const ERROR_NDIS_DOT11_MEDIA_IN_USE: WIN32_ERROR = WIN32_ERROR(2150899713u32);
pub const ERROR_NDIS_DOT11_POWER_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(2150899714u32);
pub const ERROR_NDIS_ERROR_READING_FILE: WIN32_ERROR = WIN32_ERROR(2150891548u32);
pub const ERROR_NDIS_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891547u32);
pub const ERROR_NDIS_GROUP_ADDRESS_IN_USE: WIN32_ERROR = WIN32_ERROR(2150891546u32);
pub const ERROR_NDIS_INDICATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(3407873u32);
pub const ERROR_NDIS_INTERFACE_CLOSING: WIN32_ERROR = WIN32_ERROR(2150891522u32);
pub const ERROR_NDIS_INTERFACE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891563u32);
pub const ERROR_NDIS_INVALID_ADDRESS: WIN32_ERROR = WIN32_ERROR(2150891554u32);
pub const ERROR_NDIS_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(2150891541u32);
pub const ERROR_NDIS_INVALID_DEVICE_REQUEST: WIN32_ERROR = WIN32_ERROR(2150891536u32);
pub const ERROR_NDIS_INVALID_LENGTH: WIN32_ERROR = WIN32_ERROR(2150891540u32);
pub const ERROR_NDIS_INVALID_OID: WIN32_ERROR = WIN32_ERROR(2150891543u32);
pub const ERROR_NDIS_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(2150891535u32);
pub const ERROR_NDIS_INVALID_PORT: WIN32_ERROR = WIN32_ERROR(2150891565u32);
pub const ERROR_NDIS_INVALID_PORT_STATE: WIN32_ERROR = WIN32_ERROR(2150891566u32);
pub const ERROR_NDIS_LOW_POWER_STATE: WIN32_ERROR = WIN32_ERROR(2150891567u32);
pub const ERROR_NDIS_MEDIA_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(2150891551u32);
pub const ERROR_NDIS_MULTICAST_EXISTS: WIN32_ERROR = WIN32_ERROR(2150891530u32);
pub const ERROR_NDIS_MULTICAST_FULL: WIN32_ERROR = WIN32_ERROR(2150891529u32);
pub const ERROR_NDIS_MULTICAST_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891531u32);
pub const ERROR_NDIS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2150891707u32);
pub const ERROR_NDIS_NO_QUEUES: WIN32_ERROR = WIN32_ERROR(2150891569u32);
pub const ERROR_NDIS_OFFLOAD_CONNECTION_REJECTED: WIN32_ERROR = WIN32_ERROR(3224637458u32);
pub const ERROR_NDIS_OFFLOAD_PATH_REJECTED: WIN32_ERROR = WIN32_ERROR(3224637459u32);
pub const ERROR_NDIS_OFFLOAD_POLICY: WIN32_ERROR = WIN32_ERROR(3224637455u32);
pub const ERROR_NDIS_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(2150891527u32);
pub const ERROR_NDIS_PAUSED: WIN32_ERROR = WIN32_ERROR(2150891562u32);
pub const ERROR_NDIS_PM_PROTOCOL_OFFLOAD_LIST_FULL: WIN32_ERROR = WIN32_ERROR(2150899716u32);
pub const ERROR_NDIS_PM_WOL_PATTERN_LIST_FULL: WIN32_ERROR = WIN32_ERROR(2150899715u32);
pub const ERROR_NDIS_REINIT_REQUIRED: WIN32_ERROR = WIN32_ERROR(2150891568u32);
pub const ERROR_NDIS_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(2150891532u32);
pub const ERROR_NDIS_RESET_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(2150891533u32);
pub const ERROR_NDIS_RESOURCE_CONFLICT: WIN32_ERROR = WIN32_ERROR(2150891550u32);
pub const ERROR_NDIS_UNSUPPORTED_MEDIA: WIN32_ERROR = WIN32_ERROR(2150891545u32);
pub const ERROR_NDIS_UNSUPPORTED_REVISION: WIN32_ERROR = WIN32_ERROR(2150891564u32);
pub const ERROR_NEEDS_REGISTRATION: WIN32_ERROR = WIN32_ERROR(15631u32);
pub const ERROR_NEEDS_REMEDIATION: WIN32_ERROR = WIN32_ERROR(15612u32);
pub const ERROR_NEGATIVE_SEEK: WIN32_ERROR = WIN32_ERROR(131u32);
pub const ERROR_NESTING_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(215u32);
pub const ERROR_NETLOGON_NOT_STARTED: WIN32_ERROR = WIN32_ERROR(1792u32);
pub const ERROR_NETNAME_DELETED: WIN32_ERROR = WIN32_ERROR(64u32);
pub const ERROR_NETWORK_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(65u32);
pub const ERROR_NETWORK_ACCESS_DENIED_EDP: WIN32_ERROR = WIN32_ERROR(354u32);
pub const ERROR_NETWORK_AUTHENTICATION_PROMPT_CANCELED: WIN32_ERROR = WIN32_ERROR(3024u32);
pub const ERROR_NETWORK_BUSY: WIN32_ERROR = WIN32_ERROR(54u32);
pub const ERROR_NETWORK_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5035u32);
pub const ERROR_NETWORK_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1231u32);
pub const ERROR_NET_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(570u32);
pub const ERROR_NET_WRITE_FAULT: WIN32_ERROR = WIN32_ERROR(88u32);
pub const ERROR_NOACCESS: WIN32_ERROR = WIN32_ERROR(998u32);
pub const ERROR_NODE_CANNOT_BE_CLUSTERED: WIN32_ERROR = WIN32_ERROR(5898u32);
pub const ERROR_NODE_CANT_HOST_RESOURCE: WIN32_ERROR = WIN32_ERROR(5071u32);
pub const ERROR_NODE_NOT_ACTIVE_CLUSTER_MEMBER: WIN32_ERROR = WIN32_ERROR(5980u32);
pub const ERROR_NODE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5036u32);
pub const ERROR_NOINTERFACE: WIN32_ERROR = WIN32_ERROR(632u32);
pub const ERROR_NOLOGON_INTERDOMAIN_TRUST_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1807u32);
pub const ERROR_NOLOGON_SERVER_TRUST_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1809u32);
pub const ERROR_NOLOGON_WORKSTATION_TRUST_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1808u32);
pub const ERROR_NONCORE_GROUPS_FOUND: WIN32_ERROR = WIN32_ERROR(5937u32);
pub const ERROR_NONE_MAPPED: WIN32_ERROR = WIN32_ERROR(1332u32);
pub const ERROR_NONPAGED_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1451u32);
pub const ERROR_NON_ACCOUNT_SID: WIN32_ERROR = WIN32_ERROR(1257u32);
pub const ERROR_NON_CSV_PATH: WIN32_ERROR = WIN32_ERROR(5950u32);
pub const ERROR_NON_DOMAIN_SID: WIN32_ERROR = WIN32_ERROR(1258u32);
pub const ERROR_NON_MDICHILD_WINDOW: WIN32_ERROR = WIN32_ERROR(1445u32);
pub const ERROR_NON_WINDOWS_DRIVER: WIN32_ERROR = WIN32_ERROR(3758096942u32);
pub const ERROR_NON_WINDOWS_NT_DRIVER: WIN32_ERROR = WIN32_ERROR(3758096941u32);
pub const ERROR_NOTHING_TO_TERMINATE: WIN32_ERROR = WIN32_ERROR(758u32);
pub const ERROR_NOTIFICATION_GUID_ALREADY_DEFINED: WIN32_ERROR = WIN32_ERROR(309u32);
pub const ERROR_NOTIFY_CLEANUP: WIN32_ERROR = WIN32_ERROR(745u32);
pub const ERROR_NOTIFY_ENUM_DIR: WIN32_ERROR = WIN32_ERROR(1022u32);
pub const ERROR_NOT_ALLOWED_ON_SYSTEM_FILE: WIN32_ERROR = WIN32_ERROR(313u32);
pub const ERROR_NOT_ALL_ASSIGNED: WIN32_ERROR = WIN32_ERROR(1300u32);
pub const ERROR_NOT_AN_INSTALLED_OEM_INF: WIN32_ERROR = WIN32_ERROR(3758096956u32);
pub const ERROR_NOT_APPCONTAINER: WIN32_ERROR = WIN32_ERROR(4250u32);
pub const ERROR_NOT_AUTHENTICATED: WIN32_ERROR = WIN32_ERROR(1244u32);
pub const ERROR_NOT_A_CLOUD_FILE: WIN32_ERROR = WIN32_ERROR(376u32);
pub const ERROR_NOT_A_CLOUD_SYNC_ROOT: WIN32_ERROR = WIN32_ERROR(405u32);
pub const ERROR_NOT_A_DAX_VOLUME: WIN32_ERROR = WIN32_ERROR(420u32);
pub const ERROR_NOT_A_DEV_VOLUME: WIN32_ERROR = WIN32_ERROR(476u32);
pub const ERROR_NOT_A_REPARSE_POINT: WIN32_ERROR = WIN32_ERROR(4390u32);
pub const ERROR_NOT_A_TIERED_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80830009_u32 as _);
pub const ERROR_NOT_CAPABLE: WIN32_ERROR = WIN32_ERROR(775u32);
pub const ERROR_NOT_CHILD_WINDOW: WIN32_ERROR = WIN32_ERROR(1442u32);
pub const ERROR_NOT_CLIENT_PORT: u32 = 913u32;
pub const ERROR_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(2250u32);
pub const ERROR_NOT_CONTAINER: WIN32_ERROR = WIN32_ERROR(1207u32);
pub const ERROR_NOT_DAX_MAPPABLE: WIN32_ERROR = WIN32_ERROR(421u32);
pub const ERROR_NOT_DISABLEABLE: WIN32_ERROR = WIN32_ERROR(3758096945u32);
pub const ERROR_NOT_DOS_DISK: WIN32_ERROR = WIN32_ERROR(26u32);
pub const ERROR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(4307u32);
pub const ERROR_NOT_ENOUGH_MEMORY: WIN32_ERROR = WIN32_ERROR(8u32);
pub const ERROR_NOT_ENOUGH_QUOTA: WIN32_ERROR = WIN32_ERROR(1816u32);
pub const ERROR_NOT_ENOUGH_SERVER_MEMORY: WIN32_ERROR = WIN32_ERROR(1130u32);
pub const ERROR_NOT_EXPORT_FORMAT: WIN32_ERROR = WIN32_ERROR(6008u32);
pub const ERROR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1168u32);
pub const ERROR_NOT_GUI_PROCESS: WIN32_ERROR = WIN32_ERROR(1471u32);
pub const ERROR_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(3758100480u32);
pub const ERROR_NOT_JOINED: WIN32_ERROR = WIN32_ERROR(136u32);
pub const ERROR_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(158u32);
pub const ERROR_NOT_LOGGED_ON: WIN32_ERROR = WIN32_ERROR(1245u32);
pub const ERROR_NOT_LOGON_PROCESS: WIN32_ERROR = WIN32_ERROR(1362u32);
pub const ERROR_NOT_OWNER: WIN32_ERROR = WIN32_ERROR(288u32);
pub const ERROR_NOT_QUORUM_CAPABLE: WIN32_ERROR = WIN32_ERROR(5021u32);
pub const ERROR_NOT_QUORUM_CLASS: WIN32_ERROR = WIN32_ERROR(5025u32);
pub const ERROR_NOT_READY: WIN32_ERROR = WIN32_ERROR(21u32);
pub const ERROR_NOT_READ_FROM_COPY: WIN32_ERROR = WIN32_ERROR(337u32);
pub const ERROR_NOT_REDUNDANT_STORAGE: WIN32_ERROR = WIN32_ERROR(333u32);
pub const ERROR_NOT_REGISTRY_FILE: WIN32_ERROR = WIN32_ERROR(1017u32);
pub const ERROR_NOT_ROUTER_PORT: u32 = 914u32;
pub const ERROR_NOT_SAFEBOOT_SERVICE: WIN32_ERROR = WIN32_ERROR(1084u32);
pub const ERROR_NOT_SAFE_MODE_DRIVER: WIN32_ERROR = WIN32_ERROR(646u32);
pub const ERROR_NOT_SAME_DEVICE: WIN32_ERROR = WIN32_ERROR(17u32);
pub const ERROR_NOT_SAME_OBJECT: WIN32_ERROR = WIN32_ERROR(1656u32);
pub const ERROR_NOT_SNAPSHOT_VOLUME: WIN32_ERROR = WIN32_ERROR(6841u32);
pub const ERROR_NOT_SUBSTED: WIN32_ERROR = WIN32_ERROR(137u32);
pub const ERROR_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(50u32);
pub const ERROR_NOT_SUPPORTED_IN_APPCONTAINER: WIN32_ERROR = WIN32_ERROR(4252u32);
pub const ERROR_NOT_SUPPORTED_ON_DAX: WIN32_ERROR = WIN32_ERROR(360u32);
pub const ERROR_NOT_SUPPORTED_ON_SBS: WIN32_ERROR = WIN32_ERROR(1254u32);
pub const ERROR_NOT_SUPPORTED_ON_STANDARD_SERVER: WIN32_ERROR = WIN32_ERROR(8584u32);
pub const ERROR_NOT_SUPPORTED_WITH_AUDITING: WIN32_ERROR = WIN32_ERROR(499u32);
pub const ERROR_NOT_SUPPORTED_WITH_BTT: WIN32_ERROR = WIN32_ERROR(429u32);
pub const ERROR_NOT_SUPPORTED_WITH_BYPASSIO: WIN32_ERROR = WIN32_ERROR(493u32);
pub const ERROR_NOT_SUPPORTED_WITH_CACHED_HANDLE: WIN32_ERROR = WIN32_ERROR(509u32);
pub const ERROR_NOT_SUPPORTED_WITH_COMPRESSION: WIN32_ERROR = WIN32_ERROR(496u32);
pub const ERROR_NOT_SUPPORTED_WITH_DEDUPLICATION: WIN32_ERROR = WIN32_ERROR(498u32);
pub const ERROR_NOT_SUPPORTED_WITH_ENCRYPTION: WIN32_ERROR = WIN32_ERROR(495u32);
pub const ERROR_NOT_SUPPORTED_WITH_MONITORING: WIN32_ERROR = WIN32_ERROR(503u32);
pub const ERROR_NOT_SUPPORTED_WITH_REPLICATION: WIN32_ERROR = WIN32_ERROR(497u32);
pub const ERROR_NOT_SUPPORTED_WITH_SNAPSHOT: WIN32_ERROR = WIN32_ERROR(504u32);
pub const ERROR_NOT_SUPPORTED_WITH_VIRTUALIZATION: WIN32_ERROR = WIN32_ERROR(505u32);
pub const ERROR_NOT_TINY_STREAM: WIN32_ERROR = WIN32_ERROR(598u32);
pub const ERROR_NO_ACE_CONDITION: WIN32_ERROR = WIN32_ERROR(804u32);
pub const ERROR_NO_ADMIN_ACCESS_POINT: WIN32_ERROR = WIN32_ERROR(5090u32);
pub const ERROR_NO_APPLICABLE_APP_LICENSES_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0EA0001_u32 as _);
pub const ERROR_NO_ASSOCIATED_CLASS: WIN32_ERROR = WIN32_ERROR(3758096896u32);
pub const ERROR_NO_ASSOCIATED_SERVICE: WIN32_ERROR = WIN32_ERROR(3758096921u32);
pub const ERROR_NO_ASSOCIATION: WIN32_ERROR = WIN32_ERROR(1155u32);
pub const ERROR_NO_AUTHENTICODE_CATALOG: WIN32_ERROR = WIN32_ERROR(3758096959u32);
pub const ERROR_NO_AUTH_PROTOCOL_AVAILABLE: u32 = 918u32;
pub const ERROR_NO_BACKUP: WIN32_ERROR = WIN32_ERROR(3758096643u32);
pub const ERROR_NO_BROWSER_SERVERS_FOUND: WIN32_ERROR = WIN32_ERROR(6118u32);
pub const ERROR_NO_BYPASSIO_DRIVER_SUPPORT: WIN32_ERROR = WIN32_ERROR(494u32);
pub const ERROR_NO_CALLBACK_ACTIVE: WIN32_ERROR = WIN32_ERROR(614u32);
pub const ERROR_NO_CATALOG_FOR_OEM_INF: WIN32_ERROR = WIN32_ERROR(3758096943u32);
pub const ERROR_NO_CLASSINSTALL_PARAMS: WIN32_ERROR = WIN32_ERROR(3758096917u32);
pub const ERROR_NO_CLASS_DRIVER_LIST: WIN32_ERROR = WIN32_ERROR(3758096920u32);
pub const ERROR_NO_COMPAT_DRIVERS: WIN32_ERROR = WIN32_ERROR(3758096936u32);
pub const ERROR_NO_CONFIGMGR_SERVICES: WIN32_ERROR = WIN32_ERROR(3758096931u32);
pub const ERROR_NO_DATA: WIN32_ERROR = WIN32_ERROR(232u32);
pub const ERROR_NO_DATA_DETECTED: WIN32_ERROR = WIN32_ERROR(1104u32);
pub const ERROR_NO_DEFAULT_DEVICE_INTERFACE: WIN32_ERROR = WIN32_ERROR(3758096922u32);
pub const ERROR_NO_DEFAULT_INTERFACE_DEVICE: WIN32_ERROR = WIN32_ERROR(3758096922u32);
pub const ERROR_NO_DEVICE_ICON: WIN32_ERROR = WIN32_ERROR(3758096937u32);
pub const ERROR_NO_DEVICE_SELECTED: WIN32_ERROR = WIN32_ERROR(3758096913u32);
pub const ERROR_NO_DRIVER_SELECTED: WIN32_ERROR = WIN32_ERROR(3758096899u32);
pub const ERROR_NO_EFS: WIN32_ERROR = WIN32_ERROR(6004u32);
pub const ERROR_NO_EVENT_PAIR: WIN32_ERROR = WIN32_ERROR(580u32);
pub const ERROR_NO_GUID_TRANSLATION: WIN32_ERROR = WIN32_ERROR(560u32);
pub const ERROR_NO_IMPERSONATION_TOKEN: WIN32_ERROR = WIN32_ERROR(1309u32);
pub const ERROR_NO_INF: WIN32_ERROR = WIN32_ERROR(3758096906u32);
pub const ERROR_NO_INHERITANCE: WIN32_ERROR = WIN32_ERROR(1391u32);
pub const ERROR_NO_INTERFACE_CREDENTIALS_SET: u32 = 909u32;
pub const ERROR_NO_LINK_TRACKING_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6852u32);
pub const ERROR_NO_LOGON_SERVERS: WIN32_ERROR = WIN32_ERROR(1311u32);
pub const ERROR_NO_LOG_SPACE: WIN32_ERROR = WIN32_ERROR(1019u32);
pub const ERROR_NO_MATCH: WIN32_ERROR = WIN32_ERROR(1169u32);
pub const ERROR_NO_MEDIA_IN_DRIVE: WIN32_ERROR = WIN32_ERROR(1112u32);
pub const ERROR_NO_MORE_DEVICES: WIN32_ERROR = WIN32_ERROR(1248u32);
pub const ERROR_NO_MORE_FILES: WIN32_ERROR = WIN32_ERROR(18u32);
pub const ERROR_NO_MORE_ITEMS: WIN32_ERROR = WIN32_ERROR(259u32);
pub const ERROR_NO_MORE_MATCHES: WIN32_ERROR = WIN32_ERROR(626u32);
pub const ERROR_NO_MORE_SEARCH_HANDLES: WIN32_ERROR = WIN32_ERROR(113u32);
pub const ERROR_NO_MORE_USER_HANDLES: WIN32_ERROR = WIN32_ERROR(1158u32);
pub const ERROR_NO_NETWORK: WIN32_ERROR = WIN32_ERROR(1222u32);
pub const ERROR_NO_NET_OR_BAD_PATH: WIN32_ERROR = WIN32_ERROR(1203u32);
pub const ERROR_NO_NVRAM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1470u32);
pub const ERROR_NO_PAGEFILE: WIN32_ERROR = WIN32_ERROR(578u32);
pub const ERROR_NO_PHYSICALLY_ALIGNED_FREE_SPACE_FOUND: WIN32_ERROR = WIN32_ERROR(408u32);
pub const ERROR_NO_PROC_SLOTS: WIN32_ERROR = WIN32_ERROR(89u32);
pub const ERROR_NO_PROMOTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(8222u32);
pub const ERROR_NO_QUOTAS_FOR_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1302u32);
pub const ERROR_NO_RADIUS_SERVERS: u32 = 938u32;
pub const ERROR_NO_RANGES_PROCESSED: WIN32_ERROR = WIN32_ERROR(312u32);
pub const ERROR_NO_RECOVERY_POLICY: WIN32_ERROR = WIN32_ERROR(6003u32);
pub const ERROR_NO_RECOVERY_PROGRAM: WIN32_ERROR = WIN32_ERROR(1082u32);
pub const ERROR_NO_SAVEPOINT_WITH_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(6842u32);
pub const ERROR_NO_SCROLLBARS: WIN32_ERROR = WIN32_ERROR(1447u32);
pub const ERROR_NO_SECRETS: WIN32_ERROR = WIN32_ERROR(8620u32);
pub const ERROR_NO_SECURITY_ON_OBJECT: WIN32_ERROR = WIN32_ERROR(1350u32);
pub const ERROR_NO_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1116u32);
pub const ERROR_NO_SIGNAL_SENT: WIN32_ERROR = WIN32_ERROR(205u32);
pub const ERROR_NO_SIGNATURE: u32 = 951u32;
pub const ERROR_NO_SITENAME: WIN32_ERROR = WIN32_ERROR(1919u32);
pub const ERROR_NO_SITE_SETTINGS_OBJECT: WIN32_ERROR = WIN32_ERROR(8619u32);
pub const ERROR_NO_SPOOL_SPACE: WIN32_ERROR = WIN32_ERROR(62u32);
pub const ERROR_NO_SUCH_ALIAS: WIN32_ERROR = WIN32_ERROR(1376u32);
pub const ERROR_NO_SUCH_DEVICE: WIN32_ERROR = WIN32_ERROR(433u32);
pub const ERROR_NO_SUCH_DEVICE_INTERFACE: WIN32_ERROR = WIN32_ERROR(3758096933u32);
pub const ERROR_NO_SUCH_DEVINST: WIN32_ERROR = WIN32_ERROR(3758096907u32);
pub const ERROR_NO_SUCH_DOMAIN: WIN32_ERROR = WIN32_ERROR(1355u32);
pub const ERROR_NO_SUCH_GROUP: WIN32_ERROR = WIN32_ERROR(1319u32);
pub const ERROR_NO_SUCH_INTERFACE: u32 = 905u32;
pub const ERROR_NO_SUCH_INTERFACE_CLASS: WIN32_ERROR = WIN32_ERROR(3758096926u32);
pub const ERROR_NO_SUCH_INTERFACE_DEVICE: WIN32_ERROR = WIN32_ERROR(3758096933u32);
pub const ERROR_NO_SUCH_LOGON_SESSION: WIN32_ERROR = WIN32_ERROR(1312u32);
pub const ERROR_NO_SUCH_MEMBER: WIN32_ERROR = WIN32_ERROR(1387u32);
pub const ERROR_NO_SUCH_PACKAGE: WIN32_ERROR = WIN32_ERROR(1364u32);
pub const ERROR_NO_SUCH_PRIVILEGE: WIN32_ERROR = WIN32_ERROR(1313u32);
pub const ERROR_NO_SUCH_SITE: WIN32_ERROR = WIN32_ERROR(1249u32);
pub const ERROR_NO_SUCH_USER: WIN32_ERROR = WIN32_ERROR(1317u32);
pub const ERROR_NO_SUPPORTING_DRIVES: WIN32_ERROR = WIN32_ERROR(4339u32);
pub const ERROR_NO_SYSTEM_MENU: WIN32_ERROR = WIN32_ERROR(1437u32);
pub const ERROR_NO_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1450u32);
pub const ERROR_NO_TASK_QUEUE: WIN32_ERROR = WIN32_ERROR(427u32);
pub const ERROR_NO_TOKEN: WIN32_ERROR = WIN32_ERROR(1008u32);
pub const ERROR_NO_TRACKING_SERVICE: WIN32_ERROR = WIN32_ERROR(1172u32);
pub const ERROR_NO_TRUST_LSA_SECRET: WIN32_ERROR = WIN32_ERROR(1786u32);
pub const ERROR_NO_TRUST_SAM_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1787u32);
pub const ERROR_NO_TXF_METADATA: WIN32_ERROR = WIN32_ERROR(6816u32);
pub const ERROR_NO_UNICODE_TRANSLATION: WIN32_ERROR = WIN32_ERROR(1113u32);
pub const ERROR_NO_USER_KEYS: WIN32_ERROR = WIN32_ERROR(6006u32);
pub const ERROR_NO_USER_SESSION_KEY: WIN32_ERROR = WIN32_ERROR(1394u32);
pub const ERROR_NO_VOLUME_ID: WIN32_ERROR = WIN32_ERROR(1173u32);
pub const ERROR_NO_VOLUME_LABEL: WIN32_ERROR = WIN32_ERROR(125u32);
pub const ERROR_NO_WILDCARD_CHARACTERS: WIN32_ERROR = WIN32_ERROR(1417u32);
pub const ERROR_NO_WORK_DONE: WIN32_ERROR = WIN32_ERROR(235u32);
pub const ERROR_NO_WRITABLE_DC_FOUND: WIN32_ERROR = WIN32_ERROR(8621u32);
pub const ERROR_NO_YIELD_PERFORMED: WIN32_ERROR = WIN32_ERROR(721u32);
pub const ERROR_NTLM_BLOCKED: WIN32_ERROR = WIN32_ERROR(1937u32);
pub const ERROR_NT_CROSS_ENCRYPTION_REQUIRED: WIN32_ERROR = WIN32_ERROR(1386u32);
pub const ERROR_NULL_LM_PASSWORD: WIN32_ERROR = WIN32_ERROR(1304u32);
pub const ERROR_OBJECT_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(5010u32);
pub const ERROR_OBJECT_IN_LIST: WIN32_ERROR = WIN32_ERROR(5011u32);
pub const ERROR_OBJECT_IS_IMMUTABLE: WIN32_ERROR = WIN32_ERROR(4449u32);
pub const ERROR_OBJECT_NAME_EXISTS: WIN32_ERROR = WIN32_ERROR(698u32);
pub const ERROR_OBJECT_NOT_EXTERNALLY_BACKED: WIN32_ERROR = WIN32_ERROR(342u32);
pub const ERROR_OBJECT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4312u32);
pub const ERROR_OBJECT_NO_LONGER_EXISTS: WIN32_ERROR = WIN32_ERROR(6807u32);
pub const ERROR_OFFLOAD_READ_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4442u32);
pub const ERROR_OFFLOAD_READ_FLT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4440u32);
pub const ERROR_OFFLOAD_WRITE_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4443u32);
pub const ERROR_OFFLOAD_WRITE_FLT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4441u32);
pub const ERROR_OFFSET_ALIGNMENT_VIOLATION: WIN32_ERROR = WIN32_ERROR(327u32);
pub const ERROR_OLD_WIN_VERSION: WIN32_ERROR = WIN32_ERROR(1150u32);
pub const ERROR_ONLY_IF_CONNECTED: WIN32_ERROR = WIN32_ERROR(1251u32);
pub const ERROR_ONLY_VALIDATE_VIA_AUTHENTICODE: WIN32_ERROR = WIN32_ERROR(3758096965u32);
pub const ERROR_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(110u32);
pub const ERROR_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(2401u32);
pub const ERROR_OPERATION_ABORTED: WIN32_ERROR = WIN32_ERROR(995u32);
pub const ERROR_OPERATION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(329u32);
pub const ERROR_OPERATION_NOT_ALLOWED_FROM_SYSTEM_COMPONENT: WIN32_ERROR = WIN32_ERROR(15145u32);
pub const ERROR_OPERATION_NOT_SUPPORTED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6853u32);
pub const ERROR_OPLOCK_BREAK_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(742u32);
pub const ERROR_OPLOCK_HANDLE_CLOSED: WIN32_ERROR = WIN32_ERROR(803u32);
pub const ERROR_OPLOCK_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(300u32);
pub const ERROR_OPLOCK_SWITCHED_TO_NEW_HANDLE: WIN32_ERROR = WIN32_ERROR(800u32);
pub const ERROR_ORPHAN_NAME_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(799u32);
pub const ERROR_OUTOFMEMORY: WIN32_ERROR = WIN32_ERROR(14u32);
pub const ERROR_OUT_OF_PAPER: WIN32_ERROR = WIN32_ERROR(28u32);
pub const ERROR_OUT_OF_STRUCTURES: WIN32_ERROR = WIN32_ERROR(84u32);
pub const ERROR_OVERRIDE_NOCHANGES: WIN32_ERROR = WIN32_ERROR(1252u32);
pub const ERROR_PACKAGED_SERVICE_REQUIRES_ADMIN_PRIVILEGES: WIN32_ERROR = WIN32_ERROR(15656u32);
pub const ERROR_PACKAGES_IN_USE: WIN32_ERROR = WIN32_ERROR(15618u32);
pub const ERROR_PACKAGES_REPUTATION_CHECK_FAILED: WIN32_ERROR = WIN32_ERROR(15643u32);
pub const ERROR_PACKAGES_REPUTATION_CHECK_TIMEDOUT: WIN32_ERROR = WIN32_ERROR(15644u32);
pub const ERROR_PACKAGE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(15611u32);
pub const ERROR_PACKAGE_EXTERNAL_LOCATION_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15662u32);
pub const ERROR_PACKAGE_LACKS_CAPABILITY_FOR_MANDATORY_STARTUPTASKS: WIN32_ERROR = WIN32_ERROR(15664u32);
pub const ERROR_PACKAGE_LACKS_CAPABILITY_TO_DEPLOY_ON_HOST: WIN32_ERROR = WIN32_ERROR(15658u32);
pub const ERROR_PACKAGE_MOVE_BLOCKED_BY_STREAMING: WIN32_ERROR = WIN32_ERROR(15636u32);
pub const ERROR_PACKAGE_MOVE_FAILED: WIN32_ERROR = WIN32_ERROR(15627u32);
pub const ERROR_PACKAGE_NAME_MISMATCH: WIN32_ERROR = WIN32_ERROR(15670u32);
pub const ERROR_PACKAGE_NOT_REGISTERED_FOR_USER: WIN32_ERROR = WIN32_ERROR(15669u32);
pub const ERROR_PACKAGE_NOT_SUPPORTED_ON_FILESYSTEM: WIN32_ERROR = WIN32_ERROR(15635u32);
pub const ERROR_PACKAGE_REPOSITORY_CORRUPTED: WIN32_ERROR = WIN32_ERROR(15614u32);
pub const ERROR_PACKAGE_STAGING_ONHOLD: WIN32_ERROR = WIN32_ERROR(15638u32);
pub const ERROR_PACKAGE_UPDATING: WIN32_ERROR = WIN32_ERROR(15616u32);
pub const ERROR_PAGED_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1452u32);
pub const ERROR_PAGEFILE_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(576u32);
pub const ERROR_PAGEFILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(491u32);
pub const ERROR_PAGEFILE_QUOTA: WIN32_ERROR = WIN32_ERROR(1454u32);
pub const ERROR_PAGEFILE_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(567u32);
pub const ERROR_PAGE_FAULT_COPY_ON_WRITE: WIN32_ERROR = WIN32_ERROR(749u32);
pub const ERROR_PAGE_FAULT_DEMAND_ZERO: WIN32_ERROR = WIN32_ERROR(748u32);
pub const ERROR_PAGE_FAULT_GUARD_PAGE: WIN32_ERROR = WIN32_ERROR(750u32);
pub const ERROR_PAGE_FAULT_PAGING_FILE: WIN32_ERROR = WIN32_ERROR(751u32);
pub const ERROR_PAGE_FAULT_TRANSITION: WIN32_ERROR = WIN32_ERROR(747u32);
pub const ERROR_PARAMETER_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1283u32);
pub const ERROR_PARTIAL_COPY: WIN32_ERROR = WIN32_ERROR(299u32);
pub const ERROR_PARTITION_FAILURE: WIN32_ERROR = WIN32_ERROR(1105u32);
pub const ERROR_PARTITION_TERMINATING: WIN32_ERROR = WIN32_ERROR(1184u32);
pub const ERROR_PASSWORD_CHANGE_REQUIRED: WIN32_ERROR = WIN32_ERROR(1938u32);
pub const ERROR_PASSWORD_EXPIRED: WIN32_ERROR = WIN32_ERROR(1330u32);
pub const ERROR_PASSWORD_MUST_CHANGE: WIN32_ERROR = WIN32_ERROR(1907u32);
pub const ERROR_PASSWORD_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1325u32);
pub const ERROR_PATCH_MANAGED_ADVERTISED_PRODUCT: WIN32_ERROR = WIN32_ERROR(1651u32);
pub const ERROR_PATCH_NO_SEQUENCE: WIN32_ERROR = WIN32_ERROR(1648u32);
pub const ERROR_PATCH_PACKAGE_INVALID: WIN32_ERROR = WIN32_ERROR(1636u32);
pub const ERROR_PATCH_PACKAGE_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(1635u32);
pub const ERROR_PATCH_PACKAGE_REJECTED: WIN32_ERROR = WIN32_ERROR(1643u32);
pub const ERROR_PATCH_PACKAGE_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1637u32);
pub const ERROR_PATCH_REMOVAL_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1649u32);
pub const ERROR_PATCH_REMOVAL_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1646u32);
pub const ERROR_PATCH_TARGET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1642u32);
pub const ERROR_PATH_BUSY: WIN32_ERROR = WIN32_ERROR(148u32);
pub const ERROR_PATH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3u32);
pub const ERROR_PEER_REFUSED_AUTH: u32 = 919u32;
pub const ERROR_PER_USER_TRUST_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1932u32);
pub const ERROR_PIPE_BUSY: WIN32_ERROR = WIN32_ERROR(231u32);
pub const ERROR_PIPE_CONNECTED: WIN32_ERROR = WIN32_ERROR(535u32);
pub const ERROR_PIPE_LISTENING: WIN32_ERROR = WIN32_ERROR(536u32);
pub const ERROR_PIPE_LOCAL: WIN32_ERROR = WIN32_ERROR(229u32);
pub const ERROR_PIPE_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(233u32);
pub const ERROR_PKINIT_FAILURE: WIN32_ERROR = WIN32_ERROR(1263u32);
pub const ERROR_PLATFORM_MANIFEST_BINARY_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4574u32);
pub const ERROR_PLATFORM_MANIFEST_CATALOG_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4573u32);
pub const ERROR_PLATFORM_MANIFEST_FILE_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4572u32);
pub const ERROR_PLATFORM_MANIFEST_INVALID: WIN32_ERROR = WIN32_ERROR(4571u32);
pub const ERROR_PLATFORM_MANIFEST_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(4575u32);
pub const ERROR_PLATFORM_MANIFEST_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4570u32);
pub const ERROR_PLATFORM_MANIFEST_NOT_SIGNED: WIN32_ERROR = WIN32_ERROR(4576u32);
pub const ERROR_PLUGPLAY_QUERY_VETOED: WIN32_ERROR = WIN32_ERROR(683u32);
pub const ERROR_PNP_BAD_MPS_TABLE: WIN32_ERROR = WIN32_ERROR(671u32);
pub const ERROR_PNP_INVALID_ID: WIN32_ERROR = WIN32_ERROR(674u32);
pub const ERROR_PNP_IRQ_TRANSLATION_FAILED: WIN32_ERROR = WIN32_ERROR(673u32);
pub const ERROR_PNP_QUERY_REMOVE_DEVICE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(480u32);
pub const ERROR_PNP_QUERY_REMOVE_RELATED_DEVICE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(481u32);
pub const ERROR_PNP_QUERY_REMOVE_UNRELATED_DEVICE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(482u32);
pub const ERROR_PNP_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(638u32);
pub const ERROR_PNP_REGISTRY_ERROR: WIN32_ERROR = WIN32_ERROR(3758096954u32);
pub const ERROR_PNP_RESTART_ENUMERATION: WIN32_ERROR = WIN32_ERROR(636u32);
pub const ERROR_PNP_TRANSLATION_FAILED: WIN32_ERROR = WIN32_ERROR(672u32);
pub const ERROR_POINT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1171u32);
pub const ERROR_POLICY_CONTROLLED_ACCOUNT: WIN32_ERROR = WIN32_ERROR(8654u32);
pub const ERROR_POLICY_OBJECT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8219u32);
pub const ERROR_POLICY_ONLY_IN_DS: WIN32_ERROR = WIN32_ERROR(8220u32);
pub const ERROR_POPUP_ALREADY_ACTIVE: WIN32_ERROR = WIN32_ERROR(1446u32);
pub const ERROR_PORT_LIMIT_REACHED: u32 = 931u32;
pub const ERROR_PORT_MESSAGE_TOO_LONG: WIN32_ERROR = WIN32_ERROR(546u32);
pub const ERROR_PORT_NOT_SET: WIN32_ERROR = WIN32_ERROR(642u32);
pub const ERROR_PORT_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1234u32);
pub const ERROR_POSSIBLE_DEADLOCK: WIN32_ERROR = WIN32_ERROR(1131u32);
pub const ERROR_POTENTIAL_FILE_FOUND: WIN32_ERROR = WIN32_ERROR(1180u32);
pub const ERROR_PPP_SESSION_TIMEOUT: u32 = 932u32;
pub const ERROR_PREDEFINED_HANDLE: WIN32_ERROR = WIN32_ERROR(714u32);
pub const ERROR_PRIMARY_TRANSPORT_CONNECT_FAILED: WIN32_ERROR = WIN32_ERROR(746u32);
pub const ERROR_PRINTER_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(1802u32);
pub const ERROR_PRINTER_DELETED: WIN32_ERROR = WIN32_ERROR(1905u32);
pub const ERROR_PRINTER_DRIVER_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(1795u32);
pub const ERROR_PRINTER_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(3014u32);
pub const ERROR_PRINTER_DRIVER_DOWNLOAD_NEEDED: WIN32_ERROR = WIN32_ERROR(3019u32);
pub const ERROR_PRINTER_DRIVER_IN_USE: WIN32_ERROR = WIN32_ERROR(3001u32);
pub const ERROR_PRINTER_DRIVER_PACKAGE_IN_USE: WIN32_ERROR = WIN32_ERROR(3015u32);
pub const ERROR_PRINTER_DRIVER_WARNED: WIN32_ERROR = WIN32_ERROR(3013u32);
pub const ERROR_PRINTER_HAS_JOBS_QUEUED: WIN32_ERROR = WIN32_ERROR(3009u32);
pub const ERROR_PRINTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3012u32);
pub const ERROR_PRINTER_NOT_SHAREABLE: WIN32_ERROR = WIN32_ERROR(3022u32);
pub const ERROR_PRINTQ_FULL: WIN32_ERROR = WIN32_ERROR(61u32);
pub const ERROR_PRINT_CANCELLED: WIN32_ERROR = WIN32_ERROR(63u32);
pub const ERROR_PRINT_JOB_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(3020u32);
pub const ERROR_PRINT_MONITOR_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(3006u32);
pub const ERROR_PRINT_MONITOR_IN_USE: WIN32_ERROR = WIN32_ERROR(3008u32);
pub const ERROR_PRINT_PROCESSOR_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(3005u32);
pub const ERROR_PRIVATE_DIALOG_INDEX: WIN32_ERROR = WIN32_ERROR(1415u32);
pub const ERROR_PRIVILEGE_NOT_HELD: WIN32_ERROR = WIN32_ERROR(1314u32);
pub const ERROR_PRI_MERGE_ADD_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15151u32);
pub const ERROR_PRI_MERGE_BUNDLE_PACKAGES_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15155u32);
pub const ERROR_PRI_MERGE_INVALID_FILE_NAME: WIN32_ERROR = WIN32_ERROR(15158u32);
pub const ERROR_PRI_MERGE_LOAD_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15150u32);
pub const ERROR_PRI_MERGE_MAIN_PACKAGE_REQUIRED: WIN32_ERROR = WIN32_ERROR(15156u32);
pub const ERROR_PRI_MERGE_MISSING_SCHEMA: WIN32_ERROR = WIN32_ERROR(15149u32);
pub const ERROR_PRI_MERGE_MULTIPLE_MAIN_PACKAGES_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15154u32);
pub const ERROR_PRI_MERGE_MULTIPLE_PACKAGE_FAMILIES_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15153u32);
pub const ERROR_PRI_MERGE_RESOURCE_PACKAGE_REQUIRED: WIN32_ERROR = WIN32_ERROR(15157u32);
pub const ERROR_PRI_MERGE_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(15148u32);
pub const ERROR_PRI_MERGE_WRITE_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15152u32);
pub const ERROR_PROCESS_ABORTED: WIN32_ERROR = WIN32_ERROR(1067u32);
pub const ERROR_PROCESS_IN_JOB: WIN32_ERROR = WIN32_ERROR(760u32);
pub const ERROR_PROCESS_IS_PROTECTED: WIN32_ERROR = WIN32_ERROR(1293u32);
pub const ERROR_PROCESS_MODE_ALREADY_BACKGROUND: WIN32_ERROR = WIN32_ERROR(402u32);
pub const ERROR_PROCESS_MODE_NOT_BACKGROUND: WIN32_ERROR = WIN32_ERROR(403u32);
pub const ERROR_PROCESS_NOT_IN_JOB: WIN32_ERROR = WIN32_ERROR(759u32);
pub const ERROR_PROC_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(127u32);
pub const ERROR_PRODUCT_UNINSTALLED: WIN32_ERROR = WIN32_ERROR(1614u32);
pub const ERROR_PRODUCT_VERSION: WIN32_ERROR = WIN32_ERROR(1638u32);
pub const ERROR_PROFILE_DOES_NOT_MATCH_DEVICE: WIN32_ERROR = WIN32_ERROR(2023u32);
pub const ERROR_PROFILE_NOT_ASSOCIATED_WITH_DEVICE: WIN32_ERROR = WIN32_ERROR(2015u32);
pub const ERROR_PROFILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2016u32);
pub const ERROR_PROFILING_AT_LIMIT: WIN32_ERROR = WIN32_ERROR(553u32);
pub const ERROR_PROFILING_NOT_STARTED: WIN32_ERROR = WIN32_ERROR(550u32);
pub const ERROR_PROFILING_NOT_STOPPED: WIN32_ERROR = WIN32_ERROR(551u32);
pub const ERROR_PROMOTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(8221u32);
pub const ERROR_PROTOCOL_ALREADY_INSTALLED: u32 = 948u32;
pub const ERROR_PROTOCOL_STOP_PENDING: u32 = 907u32;
pub const ERROR_PROTOCOL_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1233u32);
pub const ERROR_PROVISION_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_PROVISIONED: WIN32_ERROR = WIN32_ERROR(15642u32);
pub const ERROR_PWD_HISTORY_CONFLICT: WIN32_ERROR = WIN32_ERROR(617u32);
pub const ERROR_PWD_TOO_LONG: WIN32_ERROR = WIN32_ERROR(657u32);
pub const ERROR_PWD_TOO_RECENT: WIN32_ERROR = WIN32_ERROR(616u32);
pub const ERROR_PWD_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(615u32);
pub const ERROR_QUERY_STORAGE_ERROR: WIN32_ERROR = WIN32_ERROR(2151284737u32);
pub const ERROR_QUIC_ALPN_NEG_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80410007_u32 as _);
pub const ERROR_QUIC_CONNECTION_IDLE: windows_core::HRESULT = windows_core::HRESULT(0x80410005_u32 as _);
pub const ERROR_QUIC_CONNECTION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80410006_u32 as _);
pub const ERROR_QUIC_HANDSHAKE_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80410000_u32 as _);
pub const ERROR_QUIC_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80410003_u32 as _);
pub const ERROR_QUIC_PROTOCOL_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80410004_u32 as _);
pub const ERROR_QUIC_USER_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80410002_u32 as _);
pub const ERROR_QUIC_VER_NEG_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80410001_u32 as _);
pub const ERROR_QUORUMLOG_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(5028u32);
pub const ERROR_QUORUM_DISK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5086u32);
pub const ERROR_QUORUM_NOT_ALLOWED_IN_THIS_GROUP: WIN32_ERROR = WIN32_ERROR(5928u32);
pub const ERROR_QUORUM_OWNER_ALIVE: WIN32_ERROR = WIN32_ERROR(5034u32);
pub const ERROR_QUORUM_RESOURCE: WIN32_ERROR = WIN32_ERROR(5020u32);
pub const ERROR_QUORUM_RESOURCE_ONLINE_FAILED: WIN32_ERROR = WIN32_ERROR(5027u32);
pub const ERROR_QUOTA_ACTIVITY: WIN32_ERROR = WIN32_ERROR(810u32);
pub const ERROR_QUOTA_LIST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(621u32);
pub const ERROR_RANGE_LIST_CONFLICT: WIN32_ERROR = WIN32_ERROR(627u32);
pub const ERROR_RANGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(644u32);
pub const ERROR_RDP_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(7065u32);
pub const ERROR_READ_FAULT: WIN32_ERROR = WIN32_ERROR(30u32);
pub const ERROR_RECEIVE_EXPEDITED: WIN32_ERROR = WIN32_ERROR(708u32);
pub const ERROR_RECEIVE_PARTIAL: WIN32_ERROR = WIN32_ERROR(707u32);
pub const ERROR_RECEIVE_PARTIAL_EXPEDITED: WIN32_ERROR = WIN32_ERROR(709u32);
pub const ERROR_RECOVERY_FAILURE: WIN32_ERROR = WIN32_ERROR(1279u32);
pub const ERROR_RECOVERY_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(15619u32);
pub const ERROR_RECOVERY_NOT_NEEDED: WIN32_ERROR = WIN32_ERROR(6821u32);
pub const ERROR_REC_NON_EXISTENT: WIN32_ERROR = WIN32_ERROR(4005u32);
pub const ERROR_REDIRECTION_TO_DEFAULT_ACCOUNT_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15657u32);
pub const ERROR_REDIRECTOR_HAS_OPEN_HANDLES: WIN32_ERROR = WIN32_ERROR(1794u32);
pub const ERROR_REDIR_PAUSED: WIN32_ERROR = WIN32_ERROR(72u32);
pub const ERROR_REGISTRATION_FROM_REMOTE_DRIVE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15647u32);
pub const ERROR_REGISTRY_CORRUPT: WIN32_ERROR = WIN32_ERROR(1015u32);
pub const ERROR_REGISTRY_HIVE_RECOVERED: WIN32_ERROR = WIN32_ERROR(685u32);
pub const ERROR_REGISTRY_IO_FAILED: WIN32_ERROR = WIN32_ERROR(1016u32);
pub const ERROR_REGISTRY_QUOTA_LIMIT: WIN32_ERROR = WIN32_ERROR(613u32);
pub const ERROR_REGISTRY_RECOVERED: WIN32_ERROR = WIN32_ERROR(1014u32);
pub const ERROR_REG_NAT_CONSUMPTION: WIN32_ERROR = WIN32_ERROR(1261u32);
pub const ERROR_RELOC_CHAIN_XEEDS_SEGLIM: WIN32_ERROR = WIN32_ERROR(201u32);
pub const ERROR_REMOTEACCESS_NOT_CONFIGURED: u32 = 956u32;
pub const ERROR_REMOTE_ACCT_DISABLED: u32 = 922u32;
pub const ERROR_REMOTE_AUTHENTICATION_FAILURE: u32 = 924u32;
pub const ERROR_REMOTE_COMM_FAILURE: WIN32_ERROR = WIN32_ERROR(3758096929u32);
pub const ERROR_REMOTE_FILE_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(6814u32);
pub const ERROR_REMOTE_NO_DIALIN_PERMISSION: u32 = 920u32;
pub const ERROR_REMOTE_PASSWD_EXPIRED: u32 = 921u32;
pub const ERROR_REMOTE_PRINT_CONNECTIONS_BLOCKED: WIN32_ERROR = WIN32_ERROR(1936u32);
pub const ERROR_REMOTE_REQUEST_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(3758096955u32);
pub const ERROR_REMOTE_RESTRICTED_LOGON_HOURS: u32 = 923u32;
pub const ERROR_REMOTE_SESSION_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1220u32);
pub const ERROR_REMOTE_STORAGE_MEDIA_ERROR: WIN32_ERROR = WIN32_ERROR(4352u32);
pub const ERROR_REMOTE_STORAGE_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(4351u32);
pub const ERROR_REMOVE_FAILED: WIN32_ERROR = WIN32_ERROR(15610u32);
pub const ERROR_REM_NOT_LIST: WIN32_ERROR = WIN32_ERROR(51u32);
pub const ERROR_REPARSE: WIN32_ERROR = WIN32_ERROR(741u32);
pub const ERROR_REPARSE_ATTRIBUTE_CONFLICT: WIN32_ERROR = WIN32_ERROR(4391u32);
pub const ERROR_REPARSE_OBJECT: WIN32_ERROR = WIN32_ERROR(755u32);
pub const ERROR_REPARSE_POINT_ENCOUNTERED: WIN32_ERROR = WIN32_ERROR(4395u32);
pub const ERROR_REPARSE_TAG_INVALID: WIN32_ERROR = WIN32_ERROR(4393u32);
pub const ERROR_REPARSE_TAG_MISMATCH: WIN32_ERROR = WIN32_ERROR(4394u32);
pub const ERROR_REPLY_MESSAGE_MISMATCH: WIN32_ERROR = WIN32_ERROR(595u32);
pub const ERROR_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(1235u32);
pub const ERROR_REQUEST_OUT_OF_SEQUENCE: WIN32_ERROR = WIN32_ERROR(776u32);
pub const ERROR_REQUEST_PAUSED: WIN32_ERROR = WIN32_ERROR(3050u32);
pub const ERROR_REQUEST_REFUSED: WIN32_ERROR = WIN32_ERROR(4320u32);
pub const ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION: WIN32_ERROR = WIN32_ERROR(1459u32);
pub const ERROR_REQ_NOT_ACCEP: WIN32_ERROR = WIN32_ERROR(71u32);
pub const ERROR_RESIDENT_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(334u32);
pub const ERROR_RESILIENCY_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(15625u32);
pub const ERROR_RESMON_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(5017u32);
pub const ERROR_RESMON_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5084u32);
pub const ERROR_RESMON_ONLINE_FAILED: WIN32_ERROR = WIN32_ERROR(5018u32);
pub const ERROR_RESMON_SYSTEM_RESOURCES_LACKING: WIN32_ERROR = WIN32_ERROR(5956u32);
pub const ERROR_RESOURCEMANAGER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6716u32);
pub const ERROR_RESOURCEMANAGER_READ_ONLY: WIN32_ERROR = WIN32_ERROR(6707u32);
pub const ERROR_RESOURCE_CALL_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(5910u32);
pub const ERROR_RESOURCE_DATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1812u32);
pub const ERROR_RESOURCE_DISABLED: WIN32_ERROR = WIN32_ERROR(4309u32);
pub const ERROR_RESOURCE_ENUM_USER_STOP: WIN32_ERROR = WIN32_ERROR(15106u32);
pub const ERROR_RESOURCE_FAILED: WIN32_ERROR = WIN32_ERROR(5038u32);
pub const ERROR_RESOURCE_LANG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1815u32);
pub const ERROR_RESOURCE_NAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1814u32);
pub const ERROR_RESOURCE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5006u32);
pub const ERROR_RESOURCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5007u32);
pub const ERROR_RESOURCE_NOT_IN_AVAILABLE_STORAGE: WIN32_ERROR = WIN32_ERROR(5965u32);
pub const ERROR_RESOURCE_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(5004u32);
pub const ERROR_RESOURCE_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(4316u32);
pub const ERROR_RESOURCE_ONLINE: WIN32_ERROR = WIN32_ERROR(5019u32);
pub const ERROR_RESOURCE_PROPERTIES_STORED: WIN32_ERROR = WIN32_ERROR(5024u32);
pub const ERROR_RESOURCE_PROPERTY_UNCHANGEABLE: WIN32_ERROR = WIN32_ERROR(5089u32);
pub const ERROR_RESOURCE_REQUIREMENTS_CHANGED: WIN32_ERROR = WIN32_ERROR(756u32);
pub const ERROR_RESOURCE_TYPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1813u32);
pub const ERROR_RESTART_APPLICATION: WIN32_ERROR = WIN32_ERROR(1467u32);
pub const ERROR_RESUME_HIBERNATION: WIN32_ERROR = WIN32_ERROR(727u32);
pub const ERROR_RETRY: WIN32_ERROR = WIN32_ERROR(1237u32);
pub const ERROR_RETURN_ADDRESS_HIJACK_ATTEMPT: WIN32_ERROR = WIN32_ERROR(1662u32);
pub const ERROR_REVISION_MISMATCH: WIN32_ERROR = WIN32_ERROR(1306u32);
pub const ERROR_RING2SEG_MUST_BE_MOVABLE: WIN32_ERROR = WIN32_ERROR(200u32);
pub const ERROR_RING2_STACK_IN_USE: WIN32_ERROR = WIN32_ERROR(207u32);
pub const ERROR_RMODE_APP: WIN32_ERROR = WIN32_ERROR(1153u32);
pub const ERROR_RM_ALREADY_STARTED: WIN32_ERROR = WIN32_ERROR(6822u32);
pub const ERROR_RM_CANNOT_BE_FROZEN_FOR_SNAPSHOT: WIN32_ERROR = WIN32_ERROR(6728u32);
pub const ERROR_RM_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(6819u32);
pub const ERROR_RM_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(6802u32);
pub const ERROR_RM_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(6801u32);
pub const ERROR_ROLLBACK_TIMER_EXPIRED: WIN32_ERROR = WIN32_ERROR(6829u32);
pub const ERROR_ROUTER_CONFIG_INCOMPATIBLE: u32 = 945u32;
pub const ERROR_ROUTER_STOPPED: u32 = 900u32;
pub const ERROR_ROWSNOTRELEASED: WIN32_ERROR = WIN32_ERROR(772u32);
pub const ERROR_RPL_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(4006u32);
pub const ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT: WIN32_ERROR = WIN32_ERROR(15403u32);
pub const ERROR_RUNLEVEL_SWITCH_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(15404u32);
pub const ERROR_RUNLEVEL_SWITCH_TIMEOUT: WIN32_ERROR = WIN32_ERROR(15402u32);
pub const ERROR_RWRAW_ENCRYPTED_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(410u32);
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILEOFFSET: WIN32_ERROR = WIN32_ERROR(411u32);
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILERANGE: WIN32_ERROR = WIN32_ERROR(412u32);
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_PARAMETER: WIN32_ERROR = WIN32_ERROR(413u32);
pub const ERROR_RXACT_COMMITTED: WIN32_ERROR = WIN32_ERROR(744u32);
pub const ERROR_RXACT_COMMIT_FAILURE: WIN32_ERROR = WIN32_ERROR(1370u32);
pub const ERROR_RXACT_COMMIT_NECESSARY: WIN32_ERROR = WIN32_ERROR(678u32);
pub const ERROR_RXACT_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(1369u32);
pub const ERROR_RXACT_STATE_CREATED: WIN32_ERROR = WIN32_ERROR(701u32);
pub const ERROR_SAME_DRIVE: WIN32_ERROR = WIN32_ERROR(143u32);
pub const ERROR_SAM_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8541u32);
pub const ERROR_SCE_DISABLED: WIN32_ERROR = WIN32_ERROR(3758096952u32);
pub const ERROR_SCOPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(318u32);
pub const ERROR_SCREEN_ALREADY_LOCKED: WIN32_ERROR = WIN32_ERROR(1440u32);
pub const ERROR_SCRUB_DATA_DISABLED: WIN32_ERROR = WIN32_ERROR(332u32);
pub const ERROR_SECCORE_INVALID_COMMAND: windows_core::HRESULT = windows_core::HRESULT(0xC0E80000_u32 as _);
pub const ERROR_SECONDARY_IC_PROVIDER_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(15321u32);
pub const ERROR_SECRET_TOO_LONG: WIN32_ERROR = WIN32_ERROR(1382u32);
pub const ERROR_SECTION_DIRECT_MAP_ONLY: WIN32_ERROR = WIN32_ERROR(819u32);
pub const ERROR_SECTION_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(3758096386u32);
pub const ERROR_SECTION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3758096641u32);
pub const ERROR_SECTOR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(27u32);
pub const ERROR_SECUREBOOT_FILE_REPLACED: WIN32_ERROR = WIN32_ERROR(4426u32);
pub const ERROR_SECUREBOOT_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(4422u32);
pub const ERROR_SECUREBOOT_NOT_BASE_POLICY: WIN32_ERROR = WIN32_ERROR(4434u32);
pub const ERROR_SECUREBOOT_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(4425u32);
pub const ERROR_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY: WIN32_ERROR = WIN32_ERROR(4435u32);
pub const ERROR_SECUREBOOT_PLATFORM_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(4430u32);
pub const ERROR_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION: WIN32_ERROR = WIN32_ERROR(4429u32);
pub const ERROR_SECUREBOOT_POLICY_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4427u32);
pub const ERROR_SECUREBOOT_POLICY_NOT_SIGNED: WIN32_ERROR = WIN32_ERROR(4424u32);
pub const ERROR_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4423u32);
pub const ERROR_SECUREBOOT_POLICY_ROLLBACK_DETECTED: WIN32_ERROR = WIN32_ERROR(4431u32);
pub const ERROR_SECUREBOOT_POLICY_UNKNOWN: WIN32_ERROR = WIN32_ERROR(4428u32);
pub const ERROR_SECUREBOOT_POLICY_UPGRADE_MISMATCH: WIN32_ERROR = WIN32_ERROR(4432u32);
pub const ERROR_SECUREBOOT_POLICY_VIOLATION: WIN32_ERROR = WIN32_ERROR(4421u32);
pub const ERROR_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING: WIN32_ERROR = WIN32_ERROR(4433u32);
pub const ERROR_SECUREBOOT_ROLLBACK_DETECTED: WIN32_ERROR = WIN32_ERROR(4420u32);
pub const ERROR_SECURITY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(447u32);
pub const ERROR_SECURITY_STREAM_IS_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(306u32);
pub const ERROR_SEEK: WIN32_ERROR = WIN32_ERROR(25u32);
pub const ERROR_SEEK_ON_DEVICE: WIN32_ERROR = WIN32_ERROR(132u32);
pub const ERROR_SEGMENT_NOTIFICATION: WIN32_ERROR = WIN32_ERROR(702u32);
pub const ERROR_SEM_IS_SET: WIN32_ERROR = WIN32_ERROR(102u32);
pub const ERROR_SEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(187u32);
pub const ERROR_SEM_OWNER_DIED: WIN32_ERROR = WIN32_ERROR(105u32);
pub const ERROR_SEM_TIMEOUT: WIN32_ERROR = WIN32_ERROR(121u32);
pub const ERROR_SEM_USER_LIMIT: WIN32_ERROR = WIN32_ERROR(106u32);
pub const ERROR_SERIAL_NO_DEVICE: WIN32_ERROR = WIN32_ERROR(1118u32);
pub const ERROR_SERVER_DISABLED: WIN32_ERROR = WIN32_ERROR(1341u32);
pub const ERROR_SERVER_HAS_OPEN_HANDLES: WIN32_ERROR = WIN32_ERROR(1811u32);
pub const ERROR_SERVER_NOT_DISABLED: WIN32_ERROR = WIN32_ERROR(1342u32);
pub const ERROR_SERVER_SERVICE_CALL_REQUIRES_SMB1: WIN32_ERROR = WIN32_ERROR(3023u32);
pub const ERROR_SERVER_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1255u32);
pub const ERROR_SERVER_SID_MISMATCH: WIN32_ERROR = WIN32_ERROR(628u32);
pub const ERROR_SERVER_TRANSPORT_CONFLICT: WIN32_ERROR = WIN32_ERROR(816u32);
pub const ERROR_SERVICES_FAILED_AUTOSTART: WIN32_ERROR = WIN32_ERROR(15405u32);
pub const ERROR_SERVICE_ALREADY_RUNNING: WIN32_ERROR = WIN32_ERROR(1056u32);
pub const ERROR_SERVICE_CANNOT_ACCEPT_CTRL: WIN32_ERROR = WIN32_ERROR(1061u32);
pub const ERROR_SERVICE_DATABASE_LOCKED: WIN32_ERROR = WIN32_ERROR(1055u32);
pub const ERROR_SERVICE_DEPENDENCY_DELETED: WIN32_ERROR = WIN32_ERROR(1075u32);
pub const ERROR_SERVICE_DEPENDENCY_FAIL: WIN32_ERROR = WIN32_ERROR(1068u32);
pub const ERROR_SERVICE_DISABLED: WIN32_ERROR = WIN32_ERROR(1058u32);
pub const ERROR_SERVICE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1060u32);
pub const ERROR_SERVICE_EXISTS: WIN32_ERROR = WIN32_ERROR(1073u32);
pub const ERROR_SERVICE_EXISTS_AS_NON_PACKAGED_SERVICE: WIN32_ERROR = WIN32_ERROR(15655u32);
pub const ERROR_SERVICE_IS_PAUSED: u32 = 928u32;
pub const ERROR_SERVICE_LOGON_FAILED: WIN32_ERROR = WIN32_ERROR(1069u32);
pub const ERROR_SERVICE_MARKED_FOR_DELETE: WIN32_ERROR = WIN32_ERROR(1072u32);
pub const ERROR_SERVICE_NEVER_STARTED: WIN32_ERROR = WIN32_ERROR(1077u32);
pub const ERROR_SERVICE_NOTIFICATION: WIN32_ERROR = WIN32_ERROR(716u32);
pub const ERROR_SERVICE_NOTIFY_CLIENT_LAGGING: WIN32_ERROR = WIN32_ERROR(1294u32);
pub const ERROR_SERVICE_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(1062u32);
pub const ERROR_SERVICE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1243u32);
pub const ERROR_SERVICE_NOT_IN_EXE: WIN32_ERROR = WIN32_ERROR(1083u32);
pub const ERROR_SERVICE_NO_THREAD: WIN32_ERROR = WIN32_ERROR(1054u32);
pub const ERROR_SERVICE_REQUEST_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1053u32);
pub const ERROR_SERVICE_SPECIFIC_ERROR: WIN32_ERROR = WIN32_ERROR(1066u32);
pub const ERROR_SERVICE_START_HANG: WIN32_ERROR = WIN32_ERROR(1070u32);
pub const ERROR_SESSION_CREDENTIAL_CONFLICT: WIN32_ERROR = WIN32_ERROR(1219u32);
pub const ERROR_SESSION_KEY_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(501u32);
pub const ERROR_SETCOUNT_ON_BAD_LB: WIN32_ERROR = WIN32_ERROR(1433u32);
pub const ERROR_SETMARK_DETECTED: WIN32_ERROR = WIN32_ERROR(1103u32);
pub const ERROR_SET_CONTEXT_DENIED: WIN32_ERROR = WIN32_ERROR(1660u32);
pub const ERROR_SET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1170u32);
pub const ERROR_SET_POWER_STATE_FAILED: WIN32_ERROR = WIN32_ERROR(1141u32);
pub const ERROR_SET_POWER_STATE_VETOED: WIN32_ERROR = WIN32_ERROR(1140u32);
pub const ERROR_SET_SYSTEM_RESTORE_POINT: WIN32_ERROR = WIN32_ERROR(3758096950u32);
pub const ERROR_SHARED_POLICY: WIN32_ERROR = WIN32_ERROR(8218u32);
pub const ERROR_SHARING_BUFFER_EXCEEDED: WIN32_ERROR = WIN32_ERROR(36u32);
pub const ERROR_SHARING_PAUSED: WIN32_ERROR = WIN32_ERROR(70u32);
pub const ERROR_SHARING_VIOLATION: WIN32_ERROR = WIN32_ERROR(32u32);
pub const ERROR_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: WIN32_ERROR = WIN32_ERROR(305u32);
pub const ERROR_SHUTDOWN_CLUSTER: WIN32_ERROR = WIN32_ERROR(5008u32);
pub const ERROR_SHUTDOWN_DISKS_NOT_IN_MAINTENANCE_MODE: WIN32_ERROR = WIN32_ERROR(1192u32);
pub const ERROR_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1115u32);
pub const ERROR_SHUTDOWN_IS_SCHEDULED: WIN32_ERROR = WIN32_ERROR(1190u32);
pub const ERROR_SHUTDOWN_USERS_LOGGED_ON: WIN32_ERROR = WIN32_ERROR(1191u32);
pub const ERROR_SIGNAL_PENDING: WIN32_ERROR = WIN32_ERROR(162u32);
pub const ERROR_SIGNAL_REFUSED: WIN32_ERROR = WIN32_ERROR(156u32);
pub const ERROR_SIGNATURE_OSATTRIBUTE_MISMATCH: WIN32_ERROR = WIN32_ERROR(3758096964u32);
pub const ERROR_SIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: WIN32_ERROR = WIN32_ERROR(15661u32);
pub const ERROR_SINGLETON_RESOURCE_INSTALLED_IN_ACTIVE_USER: WIN32_ERROR = WIN32_ERROR(15653u32);
pub const ERROR_SINGLE_INSTANCE_APP: WIN32_ERROR = WIN32_ERROR(1152u32);
pub const ERROR_SMARTCARD_SUBSYSTEM_FAILURE: WIN32_ERROR = WIN32_ERROR(1264u32);
pub const ERROR_SMB1_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(384u32);
pub const ERROR_SMB_BAD_CLUSTER_DIALECT: windows_core::HRESULT = windows_core::HRESULT(0xC05D0001_u32 as _);
pub const ERROR_SMB_GUEST_LOGON_BLOCKED: WIN32_ERROR = WIN32_ERROR(1272u32);
pub const ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0xC05D0000_u32 as _);
pub const ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0xC05D0002_u32 as _);
pub const ERROR_SMI_PRIMITIVE_INSTALLER_FAILED: WIN32_ERROR = WIN32_ERROR(14108u32);
pub const ERROR_SMR_GARBAGE_COLLECTION_REQUIRED: WIN32_ERROR = WIN32_ERROR(4445u32);
pub const ERROR_SOME_NOT_MAPPED: WIN32_ERROR = WIN32_ERROR(1301u32);
pub const ERROR_SOURCE_ELEMENT_EMPTY: WIN32_ERROR = WIN32_ERROR(1160u32);
pub const ERROR_SPACES_ALLOCATION_SIZE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E7000E_u32 as _);
pub const ERROR_SPACES_CACHE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80E70026_u32 as _);
pub const ERROR_SPACES_CORRUPT_METADATA: windows_core::HRESULT = windows_core::HRESULT(0x80E70018_u32 as _);
pub const ERROR_SPACES_DRIVE_LOST_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80E7001F_u32 as _);
pub const ERROR_SPACES_DRIVE_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80E7001D_u32 as _);
pub const ERROR_SPACES_DRIVE_OPERATIONAL_STATE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70012_u32 as _);
pub const ERROR_SPACES_DRIVE_REDUNDANCY_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70006_u32 as _);
pub const ERROR_SPACES_DRIVE_SECTOR_SIZE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70004_u32 as _);
pub const ERROR_SPACES_DRIVE_SPLIT: windows_core::HRESULT = windows_core::HRESULT(0x80E7001E_u32 as _);
pub const ERROR_SPACES_DRT_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80E70019_u32 as _);
pub const ERROR_SPACES_ENCLOSURE_AWARE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E7000F_u32 as _);
pub const ERROR_SPACES_ENTRY_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80E70013_u32 as _);
pub const ERROR_SPACES_ENTRY_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70014_u32 as _);
pub const ERROR_SPACES_EXTENDED_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80E7000C_u32 as _);
pub const ERROR_SPACES_FAULT_DOMAIN_TYPE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70001_u32 as _);
pub const ERROR_SPACES_FLUSH_METADATA: windows_core::HRESULT = windows_core::HRESULT(0x80E70025_u32 as _);
pub const ERROR_SPACES_INCONSISTENCY: windows_core::HRESULT = windows_core::HRESULT(0x80E7001A_u32 as _);
pub const ERROR_SPACES_INTERLEAVE_LENGTH_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70009_u32 as _);
pub const ERROR_SPACES_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80E70002_u32 as _);
pub const ERROR_SPACES_LOG_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80E7001B_u32 as _);
pub const ERROR_SPACES_MAP_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80E70016_u32 as _);
pub const ERROR_SPACES_MARK_DIRTY: windows_core::HRESULT = windows_core::HRESULT(0x80E70020_u32 as _);
pub const ERROR_SPACES_NOT_ENOUGH_DRIVES: windows_core::HRESULT = windows_core::HRESULT(0x80E7000B_u32 as _);
pub const ERROR_SPACES_NO_REDUNDANCY: windows_core::HRESULT = windows_core::HRESULT(0x80E7001C_u32 as _);
pub const ERROR_SPACES_NUMBER_OF_COLUMNS_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E7000A_u32 as _);
pub const ERROR_SPACES_NUMBER_OF_DATA_COPIES_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70007_u32 as _);
pub const ERROR_SPACES_NUMBER_OF_GROUPS_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70011_u32 as _);
pub const ERROR_SPACES_PARITY_LAYOUT_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70008_u32 as _);
pub const ERROR_SPACES_POOL_WAS_DELETED: windows_core::HRESULT = windows_core::HRESULT(0xE70001_u32 as _);
pub const ERROR_SPACES_PROVISIONING_TYPE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E7000D_u32 as _);
pub const ERROR_SPACES_REPAIR_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80E70027_u32 as _);
pub const ERROR_SPACES_RESILIENCY_TYPE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70003_u32 as _);
pub const ERROR_SPACES_UNSUPPORTED_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80E70017_u32 as _);
pub const ERROR_SPACES_UPDATE_COLUMN_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80E70015_u32 as _);
pub const ERROR_SPACES_WRITE_CACHE_SIZE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80E70010_u32 as _);
pub const ERROR_SPARSE_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(490u32);
pub const ERROR_SPARSE_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6844u32);
pub const ERROR_SPECIAL_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1371u32);
pub const ERROR_SPECIAL_GROUP: WIN32_ERROR = WIN32_ERROR(1372u32);
pub const ERROR_SPECIAL_USER: WIN32_ERROR = WIN32_ERROR(1373u32);
pub const ERROR_SPL_NO_ADDJOB: WIN32_ERROR = WIN32_ERROR(3004u32);
pub const ERROR_SPL_NO_STARTDOC: WIN32_ERROR = WIN32_ERROR(3003u32);
pub const ERROR_SPOOL_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3002u32);
pub const ERROR_SRC_SRV_DLL_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(428u32);
pub const ERROR_STACK_BUFFER_OVERRUN: WIN32_ERROR = WIN32_ERROR(1282u32);
pub const ERROR_STACK_OVERFLOW: WIN32_ERROR = WIN32_ERROR(1001u32);
pub const ERROR_STACK_OVERFLOW_READ: WIN32_ERROR = WIN32_ERROR(599u32);
pub const ERROR_STAGEFROMUPDATEAGENT_PACKAGE_NOT_APPLICABLE: WIN32_ERROR = WIN32_ERROR(15668u32);
pub const ERROR_STATE_COMPOSITE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15815u32);
pub const ERROR_STATE_CONTAINER_NAME_SIZE_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15818u32);
pub const ERROR_STATE_CREATE_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15805u32);
pub const ERROR_STATE_DELETE_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15806u32);
pub const ERROR_STATE_DELETE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15809u32);
pub const ERROR_STATE_ENUMERATE_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15813u32);
pub const ERROR_STATE_ENUMERATE_SETTINGS_FAILED: WIN32_ERROR = WIN32_ERROR(15814u32);
pub const ERROR_STATE_GET_VERSION_FAILED: WIN32_ERROR = WIN32_ERROR(15801u32);
pub const ERROR_STATE_LOAD_STORE_FAILED: WIN32_ERROR = WIN32_ERROR(15800u32);
pub const ERROR_STATE_OPEN_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15804u32);
pub const ERROR_STATE_QUERY_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15810u32);
pub const ERROR_STATE_READ_COMPOSITE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15811u32);
pub const ERROR_STATE_READ_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15807u32);
pub const ERROR_STATE_SETTING_NAME_SIZE_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15817u32);
pub const ERROR_STATE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15816u32);
pub const ERROR_STATE_SET_VERSION_FAILED: WIN32_ERROR = WIN32_ERROR(15802u32);
pub const ERROR_STATE_STRUCTURED_RESET_FAILED: WIN32_ERROR = WIN32_ERROR(15803u32);
pub const ERROR_STATE_WRITE_COMPOSITE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15812u32);
pub const ERROR_STATE_WRITE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15808u32);
pub const ERROR_STATIC_INIT: WIN32_ERROR = WIN32_ERROR(4002u32);
pub const ERROR_STOPPED_ON_SYMLINK: WIN32_ERROR = WIN32_ERROR(681u32);
pub const ERROR_STORAGE_LOST_DATA_PERSISTENCE: WIN32_ERROR = WIN32_ERROR(368u32);
pub const ERROR_STORAGE_RESERVE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(418u32);
pub const ERROR_STORAGE_RESERVE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(417u32);
pub const ERROR_STORAGE_RESERVE_ID_INVALID: WIN32_ERROR = WIN32_ERROR(416u32);
pub const ERROR_STORAGE_RESERVE_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(419u32);
pub const ERROR_STORAGE_STACK_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(472u32);
pub const ERROR_STORAGE_TOPOLOGY_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(345u32);
pub const ERROR_STREAM_MINIVERSION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6808u32);
pub const ERROR_STREAM_MINIVERSION_NOT_VALID: WIN32_ERROR = WIN32_ERROR(6809u32);
pub const ERROR_STRICT_CFG_VIOLATION: WIN32_ERROR = WIN32_ERROR(1657u32);
pub const ERROR_SUBST_TO_JOIN: WIN32_ERROR = WIN32_ERROR(141u32);
pub const ERROR_SUBST_TO_SUBST: WIN32_ERROR = WIN32_ERROR(139u32);
pub const ERROR_SUCCESS: WIN32_ERROR = WIN32_ERROR(0u32);
pub const ERROR_SUCCESS_REBOOT_INITIATED: WIN32_ERROR = WIN32_ERROR(1641u32);
pub const ERROR_SUCCESS_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(3010u32);
pub const ERROR_SUCCESS_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(3011u32);
pub const ERROR_SVHDX_ERROR_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF00_u32 as _);
pub const ERROR_SVHDX_ERROR_STORED: windows_core::HRESULT = windows_core::HRESULT(0xC05C0000_u32 as _);
pub const ERROR_SVHDX_NO_INITIATOR: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF0B_u32 as _);
pub const ERROR_SVHDX_RESERVATION_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF07_u32 as _);
pub const ERROR_SVHDX_UNIT_ATTENTION_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF01_u32 as _);
pub const ERROR_SVHDX_UNIT_ATTENTION_CAPACITY_DATA_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF02_u32 as _);
pub const ERROR_SVHDX_UNIT_ATTENTION_OPERATING_DEFINITION_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF06_u32 as _);
pub const ERROR_SVHDX_UNIT_ATTENTION_REGISTRATIONS_PREEMPTED: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF05_u32 as _);
pub const ERROR_SVHDX_UNIT_ATTENTION_RESERVATIONS_PREEMPTED: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF03_u32 as _);
pub const ERROR_SVHDX_UNIT_ATTENTION_RESERVATIONS_RELEASED: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF04_u32 as _);
pub const ERROR_SVHDX_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF09_u32 as _);
pub const ERROR_SVHDX_WRONG_FILE_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF08_u32 as _);
pub const ERROR_SWAPERROR: WIN32_ERROR = WIN32_ERROR(999u32);
pub const ERROR_SXS_ACTIVATION_CONTEXT_DISABLED: WIN32_ERROR = WIN32_ERROR(14006u32);
pub const ERROR_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT: WIN32_ERROR = WIN32_ERROR(14103u32);
pub const ERROR_SXS_ASSEMBLY_MISSING: WIN32_ERROR = WIN32_ERROR(14081u32);
pub const ERROR_SXS_ASSEMBLY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14003u32);
pub const ERROR_SXS_ASSEMBLY_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(14097u32);
pub const ERROR_SXS_CANT_GEN_ACTCTX: WIN32_ERROR = WIN32_ERROR(14001u32);
pub const ERROR_SXS_COMPONENT_STORE_CORRUPT: WIN32_ERROR = WIN32_ERROR(14098u32);
pub const ERROR_SXS_CORRUPTION: WIN32_ERROR = WIN32_ERROR(14083u32);
pub const ERROR_SXS_CORRUPT_ACTIVATION_STACK: WIN32_ERROR = WIN32_ERROR(14082u32);
pub const ERROR_SXS_DUPLICATE_ACTIVATABLE_CLASS: WIN32_ERROR = WIN32_ERROR(14111u32);
pub const ERROR_SXS_DUPLICATE_ASSEMBLY_NAME: WIN32_ERROR = WIN32_ERROR(14027u32);
pub const ERROR_SXS_DUPLICATE_CLSID: WIN32_ERROR = WIN32_ERROR(14023u32);
pub const ERROR_SXS_DUPLICATE_DLL_NAME: WIN32_ERROR = WIN32_ERROR(14021u32);
pub const ERROR_SXS_DUPLICATE_IID: WIN32_ERROR = WIN32_ERROR(14024u32);
pub const ERROR_SXS_DUPLICATE_PROGID: WIN32_ERROR = WIN32_ERROR(14026u32);
pub const ERROR_SXS_DUPLICATE_TLBID: WIN32_ERROR = WIN32_ERROR(14025u32);
pub const ERROR_SXS_DUPLICATE_WINDOWCLASS_NAME: WIN32_ERROR = WIN32_ERROR(14022u32);
pub const ERROR_SXS_EARLY_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14084u32);
pub const ERROR_SXS_FILE_HASH_MISMATCH: WIN32_ERROR = WIN32_ERROR(14028u32);
pub const ERROR_SXS_FILE_HASH_MISSING: WIN32_ERROR = WIN32_ERROR(14110u32);
pub const ERROR_SXS_FILE_NOT_PART_OF_ASSEMBLY: WIN32_ERROR = WIN32_ERROR(14104u32);
pub const ERROR_SXS_IDENTITIES_DIFFERENT: WIN32_ERROR = WIN32_ERROR(14102u32);
pub const ERROR_SXS_IDENTITY_DUPLICATE_ATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14092u32);
pub const ERROR_SXS_IDENTITY_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14093u32);
pub const ERROR_SXS_INCORRECT_PUBLIC_KEY_TOKEN: WIN32_ERROR = WIN32_ERROR(14095u32);
pub const ERROR_SXS_INVALID_ACTCTXDATA_FORMAT: WIN32_ERROR = WIN32_ERROR(14002u32);
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14017u32);
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME: WIN32_ERROR = WIN32_ERROR(14080u32);
pub const ERROR_SXS_INVALID_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14085u32);
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME: WIN32_ERROR = WIN32_ERROR(14091u32);
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE: WIN32_ERROR = WIN32_ERROR(14090u32);
pub const ERROR_SXS_INVALID_XML_NAMESPACE_URI: WIN32_ERROR = WIN32_ERROR(14014u32);
pub const ERROR_SXS_KEY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14007u32);
pub const ERROR_SXS_LEAF_MANIFEST_DEPENDENCY_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(14016u32);
pub const ERROR_SXS_MANIFEST_FORMAT_ERROR: WIN32_ERROR = WIN32_ERROR(14004u32);
pub const ERROR_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT: WIN32_ERROR = WIN32_ERROR(14101u32);
pub const ERROR_SXS_MANIFEST_INVALID_REQUIRED_DEFAULT_NAMESPACE: WIN32_ERROR = WIN32_ERROR(14019u32);
pub const ERROR_SXS_MANIFEST_MISSING_REQUIRED_DEFAULT_NAMESPACE: WIN32_ERROR = WIN32_ERROR(14018u32);
pub const ERROR_SXS_MANIFEST_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14005u32);
pub const ERROR_SXS_MANIFEST_TOO_BIG: WIN32_ERROR = WIN32_ERROR(14105u32);
pub const ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14079u32);
pub const ERROR_SXS_MULTIPLE_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14086u32);
pub const ERROR_SXS_POLICY_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14029u32);
pub const ERROR_SXS_PRIVATE_MANIFEST_CROSS_PATH_WITH_REPARSE_POINT: WIN32_ERROR = WIN32_ERROR(14020u32);
pub const ERROR_SXS_PROCESS_DEFAULT_ALREADY_SET: WIN32_ERROR = WIN32_ERROR(14011u32);
pub const ERROR_SXS_PROCESS_TERMINATION_REQUESTED: WIN32_ERROR = WIN32_ERROR(14087u32);
pub const ERROR_SXS_PROTECTION_CATALOG_FILE_MISSING: WIN32_ERROR = WIN32_ERROR(14078u32);
pub const ERROR_SXS_PROTECTION_CATALOG_NOT_VALID: WIN32_ERROR = WIN32_ERROR(14076u32);
pub const ERROR_SXS_PROTECTION_PUBLIC_KEY_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(14075u32);
pub const ERROR_SXS_PROTECTION_RECOVERY_FAILED: WIN32_ERROR = WIN32_ERROR(14074u32);
pub const ERROR_SXS_RELEASE_ACTIVATION_CONTEXT: WIN32_ERROR = WIN32_ERROR(14088u32);
pub const ERROR_SXS_ROOT_MANIFEST_DEPENDENCY_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(14015u32);
pub const ERROR_SXS_SECTION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14000u32);
pub const ERROR_SXS_SETTING_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(14106u32);
pub const ERROR_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY: WIN32_ERROR = WIN32_ERROR(14089u32);
pub const ERROR_SXS_THREAD_QUERIES_DISABLED: WIN32_ERROR = WIN32_ERROR(14010u32);
pub const ERROR_SXS_TRANSACTION_CLOSURE_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(14107u32);
pub const ERROR_SXS_UNKNOWN_ENCODING: WIN32_ERROR = WIN32_ERROR(14013u32);
pub const ERROR_SXS_UNKNOWN_ENCODING_GROUP: WIN32_ERROR = WIN32_ERROR(14012u32);
pub const ERROR_SXS_UNTRANSLATABLE_HRESULT: WIN32_ERROR = WIN32_ERROR(14077u32);
pub const ERROR_SXS_VERSION_CONFLICT: WIN32_ERROR = WIN32_ERROR(14008u32);
pub const ERROR_SXS_WRONG_SECTION_TYPE: WIN32_ERROR = WIN32_ERROR(14009u32);
pub const ERROR_SXS_XML_E_BADCHARDATA: WIN32_ERROR = WIN32_ERROR(14036u32);
pub const ERROR_SXS_XML_E_BADCHARINSTRING: WIN32_ERROR = WIN32_ERROR(14034u32);
pub const ERROR_SXS_XML_E_BADNAMECHAR: WIN32_ERROR = WIN32_ERROR(14033u32);
pub const ERROR_SXS_XML_E_BADPEREFINSUBSET: WIN32_ERROR = WIN32_ERROR(14059u32);
pub const ERROR_SXS_XML_E_BADSTARTNAMECHAR: WIN32_ERROR = WIN32_ERROR(14032u32);
pub const ERROR_SXS_XML_E_BADXMLCASE: WIN32_ERROR = WIN32_ERROR(14069u32);
pub const ERROR_SXS_XML_E_BADXMLDECL: WIN32_ERROR = WIN32_ERROR(14056u32);
pub const ERROR_SXS_XML_E_COMMENTSYNTAX: WIN32_ERROR = WIN32_ERROR(14031u32);
pub const ERROR_SXS_XML_E_DUPLICATEATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14053u32);
pub const ERROR_SXS_XML_E_EXPECTINGCLOSEQUOTE: WIN32_ERROR = WIN32_ERROR(14045u32);
pub const ERROR_SXS_XML_E_EXPECTINGTAGEND: WIN32_ERROR = WIN32_ERROR(14038u32);
pub const ERROR_SXS_XML_E_INCOMPLETE_ENCODING: WIN32_ERROR = WIN32_ERROR(14043u32);
pub const ERROR_SXS_XML_E_INTERNALERROR: WIN32_ERROR = WIN32_ERROR(14041u32);
pub const ERROR_SXS_XML_E_INVALIDATROOTLEVEL: WIN32_ERROR = WIN32_ERROR(14055u32);
pub const ERROR_SXS_XML_E_INVALIDENCODING: WIN32_ERROR = WIN32_ERROR(14067u32);
pub const ERROR_SXS_XML_E_INVALIDSWITCH: WIN32_ERROR = WIN32_ERROR(14068u32);
pub const ERROR_SXS_XML_E_INVALID_DECIMAL: WIN32_ERROR = WIN32_ERROR(14047u32);
pub const ERROR_SXS_XML_E_INVALID_HEXIDECIMAL: WIN32_ERROR = WIN32_ERROR(14048u32);
pub const ERROR_SXS_XML_E_INVALID_STANDALONE: WIN32_ERROR = WIN32_ERROR(14070u32);
pub const ERROR_SXS_XML_E_INVALID_UNICODE: WIN32_ERROR = WIN32_ERROR(14049u32);
pub const ERROR_SXS_XML_E_INVALID_VERSION: WIN32_ERROR = WIN32_ERROR(14072u32);
pub const ERROR_SXS_XML_E_MISSINGEQUALS: WIN32_ERROR = WIN32_ERROR(14073u32);
pub const ERROR_SXS_XML_E_MISSINGQUOTE: WIN32_ERROR = WIN32_ERROR(14030u32);
pub const ERROR_SXS_XML_E_MISSINGROOT: WIN32_ERROR = WIN32_ERROR(14057u32);
pub const ERROR_SXS_XML_E_MISSINGSEMICOLON: WIN32_ERROR = WIN32_ERROR(14039u32);
pub const ERROR_SXS_XML_E_MISSINGWHITESPACE: WIN32_ERROR = WIN32_ERROR(14037u32);
pub const ERROR_SXS_XML_E_MISSING_PAREN: WIN32_ERROR = WIN32_ERROR(14044u32);
pub const ERROR_SXS_XML_E_MULTIPLEROOTS: WIN32_ERROR = WIN32_ERROR(14054u32);
pub const ERROR_SXS_XML_E_MULTIPLE_COLONS: WIN32_ERROR = WIN32_ERROR(14046u32);
pub const ERROR_SXS_XML_E_RESERVEDNAMESPACE: WIN32_ERROR = WIN32_ERROR(14066u32);
pub const ERROR_SXS_XML_E_UNBALANCEDPAREN: WIN32_ERROR = WIN32_ERROR(14040u32);
pub const ERROR_SXS_XML_E_UNCLOSEDCDATA: WIN32_ERROR = WIN32_ERROR(14065u32);
pub const ERROR_SXS_XML_E_UNCLOSEDCOMMENT: WIN32_ERROR = WIN32_ERROR(14063u32);
pub const ERROR_SXS_XML_E_UNCLOSEDDECL: WIN32_ERROR = WIN32_ERROR(14064u32);
pub const ERROR_SXS_XML_E_UNCLOSEDENDTAG: WIN32_ERROR = WIN32_ERROR(14061u32);
pub const ERROR_SXS_XML_E_UNCLOSEDSTARTTAG: WIN32_ERROR = WIN32_ERROR(14060u32);
pub const ERROR_SXS_XML_E_UNCLOSEDSTRING: WIN32_ERROR = WIN32_ERROR(14062u32);
pub const ERROR_SXS_XML_E_UNCLOSEDTAG: WIN32_ERROR = WIN32_ERROR(14052u32);
pub const ERROR_SXS_XML_E_UNEXPECTEDENDTAG: WIN32_ERROR = WIN32_ERROR(14051u32);
pub const ERROR_SXS_XML_E_UNEXPECTEDEOF: WIN32_ERROR = WIN32_ERROR(14058u32);
pub const ERROR_SXS_XML_E_UNEXPECTED_STANDALONE: WIN32_ERROR = WIN32_ERROR(14071u32);
pub const ERROR_SXS_XML_E_UNEXPECTED_WHITESPACE: WIN32_ERROR = WIN32_ERROR(14042u32);
pub const ERROR_SXS_XML_E_WHITESPACEORQUESTIONMARK: WIN32_ERROR = WIN32_ERROR(14050u32);
pub const ERROR_SXS_XML_E_XMLDECLSYNTAX: WIN32_ERROR = WIN32_ERROR(14035u32);
pub const ERROR_SYMLINK_CLASS_DISABLED: WIN32_ERROR = WIN32_ERROR(1463u32);
pub const ERROR_SYMLINK_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(1464u32);
pub const ERROR_SYNCHRONIZATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(569u32);
pub const ERROR_SYNC_FOREGROUND_REFRESH_REQUIRED: WIN32_ERROR = WIN32_ERROR(1274u32);
pub const ERROR_SYSTEM_DEVICE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15299u32);
pub const ERROR_SYSTEM_HIVE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(653u32);
pub const ERROR_SYSTEM_IMAGE_BAD_SIGNATURE: WIN32_ERROR = WIN32_ERROR(637u32);
pub const ERROR_SYSTEM_INTEGRITY_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(4552u32);
pub const ERROR_SYSTEM_INTEGRITY_POLICY_NOT_SIGNED: WIN32_ERROR = WIN32_ERROR(4553u32);
pub const ERROR_SYSTEM_INTEGRITY_POLICY_VIOLATION: WIN32_ERROR = WIN32_ERROR(4551u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_DANGEROUS_EXT: WIN32_ERROR = WIN32_ERROR(4558u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_EXPLICIT_DENY_FILE: WIN32_ERROR = WIN32_ERROR(4582u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_MALICIOUS: WIN32_ERROR = WIN32_ERROR(4556u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_OFFLINE: WIN32_ERROR = WIN32_ERROR(4559u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_PUA: WIN32_ERROR = WIN32_ERROR(4557u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_UNATTAINABLE: WIN32_ERROR = WIN32_ERROR(4581u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_UNFRIENDLY_FILE: WIN32_ERROR = WIN32_ERROR(4580u32);
pub const ERROR_SYSTEM_INTEGRITY_ROLLBACK_DETECTED: WIN32_ERROR = WIN32_ERROR(4550u32);
pub const ERROR_SYSTEM_INTEGRITY_SUPPLEMENTAL_POLICY_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4555u32);
pub const ERROR_SYSTEM_INTEGRITY_TOO_MANY_POLICIES: WIN32_ERROR = WIN32_ERROR(4554u32);
pub const ERROR_SYSTEM_NEEDS_REMEDIATION: WIN32_ERROR = WIN32_ERROR(15623u32);
pub const ERROR_SYSTEM_POWERSTATE_COMPLEX_TRANSITION: WIN32_ERROR = WIN32_ERROR(783u32);
pub const ERROR_SYSTEM_POWERSTATE_TRANSITION: WIN32_ERROR = WIN32_ERROR(782u32);
pub const ERROR_SYSTEM_PROCESS_TERMINATED: WIN32_ERROR = WIN32_ERROR(591u32);
pub const ERROR_SYSTEM_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(641u32);
pub const ERROR_SYSTEM_TRACE: WIN32_ERROR = WIN32_ERROR(150u32);
pub const ERROR_TAG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2012u32);
pub const ERROR_TAG_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(2013u32);
pub const ERROR_THREAD_1_INACTIVE: WIN32_ERROR = WIN32_ERROR(210u32);
pub const ERROR_THREAD_ALREADY_IN_TASK: WIN32_ERROR = WIN32_ERROR(1552u32);
pub const ERROR_THREAD_MODE_ALREADY_BACKGROUND: WIN32_ERROR = WIN32_ERROR(400u32);
pub const ERROR_THREAD_MODE_NOT_BACKGROUND: WIN32_ERROR = WIN32_ERROR(401u32);
pub const ERROR_THREAD_NOT_IN_PROCESS: WIN32_ERROR = WIN32_ERROR(566u32);
pub const ERROR_THREAD_WAS_SUSPENDED: WIN32_ERROR = WIN32_ERROR(699u32);
pub const ERROR_TIERING_ALREADY_PROCESSING: windows_core::HRESULT = windows_core::HRESULT(0x80830006_u32 as _);
pub const ERROR_TIERING_CANNOT_PIN_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80830007_u32 as _);
pub const ERROR_TIERING_FILE_IS_NOT_PINNED: windows_core::HRESULT = windows_core::HRESULT(0x80830008_u32 as _);
pub const ERROR_TIERING_INVALID_FILE_ID: windows_core::HRESULT = windows_core::HRESULT(0x80830004_u32 as _);
pub const ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80830001_u32 as _);
pub const ERROR_TIERING_STORAGE_TIER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80830003_u32 as _);
pub const ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80830002_u32 as _);
pub const ERROR_TIERING_WRONG_CLUSTER_NODE: windows_core::HRESULT = windows_core::HRESULT(0x80830005_u32 as _);
pub const ERROR_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1460u32);
pub const ERROR_TIMER_NOT_CANCELED: WIN32_ERROR = WIN32_ERROR(541u32);
pub const ERROR_TIMER_RESOLUTION_NOT_SET: WIN32_ERROR = WIN32_ERROR(607u32);
pub const ERROR_TIMER_RESUME_IGNORED: WIN32_ERROR = WIN32_ERROR(722u32);
pub const ERROR_TIME_SENSITIVE_THREAD: WIN32_ERROR = WIN32_ERROR(422u32);
pub const ERROR_TIME_SKEW: WIN32_ERROR = WIN32_ERROR(1398u32);
pub const ERROR_TLW_WITH_WSCHILD: WIN32_ERROR = WIN32_ERROR(1406u32);
pub const ERROR_TM_IDENTITY_MISMATCH: WIN32_ERROR = WIN32_ERROR(6845u32);
pub const ERROR_TM_INITIALIZATION_FAILED: WIN32_ERROR = WIN32_ERROR(6706u32);
pub const ERROR_TM_VOLATILE: WIN32_ERROR = WIN32_ERROR(6828u32);
pub const ERROR_TOKEN_ALREADY_IN_USE: WIN32_ERROR = WIN32_ERROR(1375u32);
pub const ERROR_TOO_MANY_CMDS: WIN32_ERROR = WIN32_ERROR(56u32);
pub const ERROR_TOO_MANY_CONTEXT_IDS: WIN32_ERROR = WIN32_ERROR(1384u32);
pub const ERROR_TOO_MANY_DESCRIPTORS: WIN32_ERROR = WIN32_ERROR(331u32);
pub const ERROR_TOO_MANY_LINKS: WIN32_ERROR = WIN32_ERROR(1142u32);
pub const ERROR_TOO_MANY_LUIDS_REQUESTED: WIN32_ERROR = WIN32_ERROR(1333u32);
pub const ERROR_TOO_MANY_MODULES: WIN32_ERROR = WIN32_ERROR(214u32);
pub const ERROR_TOO_MANY_MUXWAITERS: WIN32_ERROR = WIN32_ERROR(152u32);
pub const ERROR_TOO_MANY_NAMES: WIN32_ERROR = WIN32_ERROR(68u32);
pub const ERROR_TOO_MANY_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(4u32);
pub const ERROR_TOO_MANY_POSTS: WIN32_ERROR = WIN32_ERROR(298u32);
pub const ERROR_TOO_MANY_SECRETS: WIN32_ERROR = WIN32_ERROR(1381u32);
pub const ERROR_TOO_MANY_SEMAPHORES: WIN32_ERROR = WIN32_ERROR(100u32);
pub const ERROR_TOO_MANY_SEM_REQUESTS: WIN32_ERROR = WIN32_ERROR(103u32);
pub const ERROR_TOO_MANY_SESS: WIN32_ERROR = WIN32_ERROR(69u32);
pub const ERROR_TOO_MANY_SIDS: WIN32_ERROR = WIN32_ERROR(1389u32);
pub const ERROR_TOO_MANY_TCBS: WIN32_ERROR = WIN32_ERROR(155u32);
pub const ERROR_TOO_MANY_THREADS: WIN32_ERROR = WIN32_ERROR(565u32);
pub const ERROR_TRANSACTED_MAPPING_UNSUPPORTED_REMOTE: WIN32_ERROR = WIN32_ERROR(6834u32);
pub const ERROR_TRANSACTIONAL_CONFLICT: WIN32_ERROR = WIN32_ERROR(6800u32);
pub const ERROR_TRANSACTIONAL_OPEN_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(6832u32);
pub const ERROR_TRANSACTIONMANAGER_IDENTITY_MISMATCH: WIN32_ERROR = WIN32_ERROR(6727u32);
pub const ERROR_TRANSACTIONMANAGER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6718u32);
pub const ERROR_TRANSACTIONMANAGER_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(6719u32);
pub const ERROR_TRANSACTIONMANAGER_RECOVERY_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(6720u32);
pub const ERROR_TRANSACTIONS_NOT_FROZEN: WIN32_ERROR = WIN32_ERROR(6839u32);
pub const ERROR_TRANSACTIONS_UNSUPPORTED_REMOTE: WIN32_ERROR = WIN32_ERROR(6805u32);
pub const ERROR_TRANSACTION_ALREADY_ABORTED: WIN32_ERROR = WIN32_ERROR(6704u32);
pub const ERROR_TRANSACTION_ALREADY_COMMITTED: WIN32_ERROR = WIN32_ERROR(6705u32);
pub const ERROR_TRANSACTION_FREEZE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6840u32);
pub const ERROR_TRANSACTION_INTEGRITY_VIOLATED: WIN32_ERROR = WIN32_ERROR(6726u32);
pub const ERROR_TRANSACTION_INVALID_MARSHALL_BUFFER: WIN32_ERROR = WIN32_ERROR(6713u32);
pub const ERROR_TRANSACTION_MUST_WRITETHROUGH: WIN32_ERROR = WIN32_ERROR(6729u32);
pub const ERROR_TRANSACTION_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(6701u32);
pub const ERROR_TRANSACTION_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(6855u32);
pub const ERROR_TRANSACTION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6715u32);
pub const ERROR_TRANSACTION_NOT_JOINED: WIN32_ERROR = WIN32_ERROR(6708u32);
pub const ERROR_TRANSACTION_NOT_REQUESTED: WIN32_ERROR = WIN32_ERROR(6703u32);
pub const ERROR_TRANSACTION_NOT_ROOT: WIN32_ERROR = WIN32_ERROR(6721u32);
pub const ERROR_TRANSACTION_NO_SUPERIOR: WIN32_ERROR = WIN32_ERROR(6730u32);
pub const ERROR_TRANSACTION_OBJECT_EXPIRED: WIN32_ERROR = WIN32_ERROR(6722u32);
pub const ERROR_TRANSACTION_PROPAGATION_FAILED: WIN32_ERROR = WIN32_ERROR(6711u32);
pub const ERROR_TRANSACTION_RECORD_TOO_LONG: WIN32_ERROR = WIN32_ERROR(6724u32);
pub const ERROR_TRANSACTION_REQUEST_NOT_VALID: WIN32_ERROR = WIN32_ERROR(6702u32);
pub const ERROR_TRANSACTION_REQUIRED_PROMOTION: WIN32_ERROR = WIN32_ERROR(6837u32);
pub const ERROR_TRANSACTION_RESPONSE_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(6723u32);
pub const ERROR_TRANSACTION_SCOPE_CALLBACKS_NOT_SET: WIN32_ERROR = WIN32_ERROR(6836u32);
pub const ERROR_TRANSACTION_SUPERIOR_EXISTS: WIN32_ERROR = WIN32_ERROR(6709u32);
pub const ERROR_TRANSFORM_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2004u32);
pub const ERROR_TRANSLATION_COMPLETE: WIN32_ERROR = WIN32_ERROR(757u32);
pub const ERROR_TRANSPORT_FULL: WIN32_ERROR = WIN32_ERROR(4328u32);
pub const ERROR_TRUSTED_DOMAIN_FAILURE: WIN32_ERROR = WIN32_ERROR(1788u32);
pub const ERROR_TRUSTED_RELATIONSHIP_FAILURE: WIN32_ERROR = WIN32_ERROR(1789u32);
pub const ERROR_TRUST_FAILURE: WIN32_ERROR = WIN32_ERROR(1790u32);
pub const ERROR_TS_INCOMPATIBLE_SESSIONS: WIN32_ERROR = WIN32_ERROR(7069u32);
pub const ERROR_TS_VIDEO_SUBSYSTEM_ERROR: WIN32_ERROR = WIN32_ERROR(7070u32);
pub const ERROR_TXF_ATTRIBUTE_CORRUPT: WIN32_ERROR = WIN32_ERROR(6830u32);
pub const ERROR_TXF_DIR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(6826u32);
pub const ERROR_TXF_METADATA_ALREADY_PRESENT: WIN32_ERROR = WIN32_ERROR(6835u32);
pub const ERROR_UNABLE_TO_CLEAN: WIN32_ERROR = WIN32_ERROR(4311u32);
pub const ERROR_UNABLE_TO_EJECT_MOUNTED_MEDIA: WIN32_ERROR = WIN32_ERROR(4330u32);
pub const ERROR_UNABLE_TO_INVENTORY_DRIVE: WIN32_ERROR = WIN32_ERROR(4325u32);
pub const ERROR_UNABLE_TO_INVENTORY_SLOT: WIN32_ERROR = WIN32_ERROR(4326u32);
pub const ERROR_UNABLE_TO_INVENTORY_TRANSPORT: WIN32_ERROR = WIN32_ERROR(4327u32);
pub const ERROR_UNABLE_TO_LOAD_MEDIUM: WIN32_ERROR = WIN32_ERROR(4324u32);
pub const ERROR_UNABLE_TO_LOCK_MEDIA: WIN32_ERROR = WIN32_ERROR(1108u32);
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT: WIN32_ERROR = WIN32_ERROR(1176u32);
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT_2: WIN32_ERROR = WIN32_ERROR(1177u32);
pub const ERROR_UNABLE_TO_REMOVE_REPLACED: WIN32_ERROR = WIN32_ERROR(1175u32);
pub const ERROR_UNABLE_TO_UNLOAD_MEDIA: WIN32_ERROR = WIN32_ERROR(1109u32);
pub const ERROR_UNDEFINED_CHARACTER: WIN32_ERROR = WIN32_ERROR(583u32);
pub const ERROR_UNDEFINED_SCOPE: WIN32_ERROR = WIN32_ERROR(319u32);
pub const ERROR_UNEXPECTED_MM_CREATE_ERR: WIN32_ERROR = WIN32_ERROR(556u32);
pub const ERROR_UNEXPECTED_MM_EXTEND_ERR: WIN32_ERROR = WIN32_ERROR(558u32);
pub const ERROR_UNEXPECTED_MM_MAP_ERROR: WIN32_ERROR = WIN32_ERROR(557u32);
pub const ERROR_UNEXPECTED_NTCACHEMANAGER_ERROR: WIN32_ERROR = WIN32_ERROR(443u32);
pub const ERROR_UNEXPECTED_OMID: WIN32_ERROR = WIN32_ERROR(4334u32);
pub const ERROR_UNEXP_NET_ERR: WIN32_ERROR = WIN32_ERROR(59u32);
pub const ERROR_UNHANDLED_EXCEPTION: WIN32_ERROR = WIN32_ERROR(574u32);
pub const ERROR_UNIDENTIFIED_ERROR: WIN32_ERROR = WIN32_ERROR(1287u32);
pub const ERROR_UNKNOWN_COMPONENT: WIN32_ERROR = WIN32_ERROR(1607u32);
pub const ERROR_UNKNOWN_EXCEPTION: WIN32_ERROR = WIN32_ERROR(3758096953u32);
pub const ERROR_UNKNOWN_FEATURE: WIN32_ERROR = WIN32_ERROR(1606u32);
pub const ERROR_UNKNOWN_PATCH: WIN32_ERROR = WIN32_ERROR(1647u32);
pub const ERROR_UNKNOWN_PORT: WIN32_ERROR = WIN32_ERROR(1796u32);
pub const ERROR_UNKNOWN_PRINTER_DRIVER: WIN32_ERROR = WIN32_ERROR(1797u32);
pub const ERROR_UNKNOWN_PRINTPROCESSOR: WIN32_ERROR = WIN32_ERROR(1798u32);
pub const ERROR_UNKNOWN_PRINT_MONITOR: WIN32_ERROR = WIN32_ERROR(3000u32);
pub const ERROR_UNKNOWN_PRODUCT: WIN32_ERROR = WIN32_ERROR(1605u32);
pub const ERROR_UNKNOWN_PROPERTY: WIN32_ERROR = WIN32_ERROR(1608u32);
pub const ERROR_UNKNOWN_PROTOCOL_ID: u32 = 902u32;
pub const ERROR_UNKNOWN_REVISION: WIN32_ERROR = WIN32_ERROR(1305u32);
pub const ERROR_UNMAPPED_SUBSTITUTION_STRING: WIN32_ERROR = WIN32_ERROR(14096u32);
pub const ERROR_UNRECOGNIZED_MEDIA: WIN32_ERROR = WIN32_ERROR(1785u32);
pub const ERROR_UNRECOGNIZED_VOLUME: WIN32_ERROR = WIN32_ERROR(1005u32);
pub const ERROR_UNRECOVERABLE_STACK_OVERFLOW: WIN32_ERROR = WIN32_ERROR(3758097152u32);
pub const ERROR_UNSATISFIED_DEPENDENCIES: WIN32_ERROR = WIN32_ERROR(441u32);
pub const ERROR_UNSIGNED_PACKAGE_INVALID_CONTENT: WIN32_ERROR = WIN32_ERROR(15659u32);
pub const ERROR_UNSIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: WIN32_ERROR = WIN32_ERROR(15660u32);
pub const ERROR_UNSUPPORTED_COMPRESSION: WIN32_ERROR = WIN32_ERROR(618u32);
pub const ERROR_UNSUPPORTED_TYPE: WIN32_ERROR = WIN32_ERROR(1630u32);
pub const ERROR_UNTRUSTED_MOUNT_POINT: WIN32_ERROR = WIN32_ERROR(448u32);
pub const ERROR_UNWIND: WIN32_ERROR = WIN32_ERROR(542u32);
pub const ERROR_UNWIND_CONSOLIDATE: WIN32_ERROR = WIN32_ERROR(684u32);
pub const ERROR_UPDATE_IN_PROGRESS: u32 = 911u32;
pub const ERROR_USER_APC: WIN32_ERROR = WIN32_ERROR(737u32);
pub const ERROR_USER_DELETE_TRUST_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1934u32);
pub const ERROR_USER_EXISTS: WIN32_ERROR = WIN32_ERROR(1316u32);
pub const ERROR_USER_LIMIT: u32 = 937u32;
pub const ERROR_USER_MAPPED_FILE: WIN32_ERROR = WIN32_ERROR(1224u32);
pub const ERROR_USER_PROFILE_LOAD: WIN32_ERROR = WIN32_ERROR(500u32);
pub const ERROR_VALIDATE_CONTINUE: WIN32_ERROR = WIN32_ERROR(625u32);
pub const ERROR_VC_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(240u32);
pub const ERROR_VDM_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1286u32);
pub const ERROR_VDM_HARD_ERROR: WIN32_ERROR = WIN32_ERROR(593u32);
pub const ERROR_VERIFIER_STOP: WIN32_ERROR = WIN32_ERROR(537u32);
pub const ERROR_VERSION_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(777u32);
pub const ERROR_VHDSET_BACKING_STORAGE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF0C_u32 as _);
pub const ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE: WIN32_ERROR = WIN32_ERROR(3225026599u32);
pub const ERROR_VHD_BITMAP_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026572u32);
pub const ERROR_VHD_BLOCK_ALLOCATION_FAILURE: WIN32_ERROR = WIN32_ERROR(3225026569u32);
pub const ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT: WIN32_ERROR = WIN32_ERROR(3225026570u32);
pub const ERROR_VHD_CHANGE_TRACKING_DISABLED: WIN32_ERROR = WIN32_ERROR(3225026602u32);
pub const ERROR_VHD_CHILD_PARENT_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026574u32);
pub const ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026583u32);
pub const ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026575u32);
pub const ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE: WIN32_ERROR = WIN32_ERROR(3225026598u32);
pub const ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED: WIN32_ERROR = WIN32_ERROR(3225026584u32);
pub const ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT: WIN32_ERROR = WIN32_ERROR(3225026585u32);
pub const ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026562u32);
pub const ERROR_VHD_DRIVE_FOOTER_CORRUPT: WIN32_ERROR = WIN32_ERROR(3225026563u32);
pub const ERROR_VHD_DRIVE_FOOTER_MISSING: WIN32_ERROR = WIN32_ERROR(3225026561u32);
pub const ERROR_VHD_FORMAT_UNKNOWN: WIN32_ERROR = WIN32_ERROR(3225026564u32);
pub const ERROR_VHD_FORMAT_UNSUPPORTED_VERSION: WIN32_ERROR = WIN32_ERROR(3225026565u32);
pub const ERROR_VHD_INVALID_BLOCK_SIZE: WIN32_ERROR = WIN32_ERROR(3225026571u32);
pub const ERROR_VHD_INVALID_CHANGE_TRACKING_ID: WIN32_ERROR = WIN32_ERROR(3225026601u32);
pub const ERROR_VHD_INVALID_FILE_SIZE: WIN32_ERROR = WIN32_ERROR(3225026579u32);
pub const ERROR_VHD_INVALID_SIZE: WIN32_ERROR = WIN32_ERROR(3225026578u32);
pub const ERROR_VHD_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(3225026588u32);
pub const ERROR_VHD_INVALID_TYPE: WIN32_ERROR = WIN32_ERROR(3225026587u32);
pub const ERROR_VHD_METADATA_FULL: WIN32_ERROR = WIN32_ERROR(3225026600u32);
pub const ERROR_VHD_METADATA_READ_FAILURE: WIN32_ERROR = WIN32_ERROR(3225026576u32);
pub const ERROR_VHD_METADATA_WRITE_FAILURE: WIN32_ERROR = WIN32_ERROR(3225026577u32);
pub const ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION: WIN32_ERROR = WIN32_ERROR(3225026608u32);
pub const ERROR_VHD_PARENT_VHD_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3225026582u32);
pub const ERROR_VHD_PARENT_VHD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3225026573u32);
pub const ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA: WIN32_ERROR = WIN32_ERROR(3225026597u32);
pub const ERROR_VHD_SHARED: windows_core::HRESULT = windows_core::HRESULT(0xC05CFF0A_u32 as _);
pub const ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026566u32);
pub const ERROR_VHD_SPARSE_HEADER_CORRUPT: WIN32_ERROR = WIN32_ERROR(3225026568u32);
pub const ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION: WIN32_ERROR = WIN32_ERROR(3225026567u32);
pub const ERROR_VHD_UNEXPECTED_ID: WIN32_ERROR = WIN32_ERROR(3225026612u32);
pub const ERROR_VID_CHILD_GPA_PAGE_SET_CORRUPTED: WIN32_ERROR = WIN32_ERROR(3224829966u32);
pub const ERROR_VID_DUPLICATE_HANDLER: WIN32_ERROR = WIN32_ERROR(3224829953u32);
pub const ERROR_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT: WIN32_ERROR = WIN32_ERROR(3224829982u32);
pub const ERROR_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT: WIN32_ERROR = WIN32_ERROR(3224829964u32);
pub const ERROR_VID_HANDLER_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(3224829956u32);
pub const ERROR_VID_INSUFFICIENT_RESOURCES_HV_DEPOSIT: WIN32_ERROR = WIN32_ERROR(3224829997u32);
pub const ERROR_VID_INSUFFICIENT_RESOURCES_PHYSICAL_BUFFER: WIN32_ERROR = WIN32_ERROR(3224829996u32);
pub const ERROR_VID_INSUFFICIENT_RESOURCES_RESERVE: WIN32_ERROR = WIN32_ERROR(3224829995u32);
pub const ERROR_VID_INSUFFICIENT_RESOURCES_WITHDRAW: WIN32_ERROR = WIN32_ERROR(3224829999u32);
pub const ERROR_VID_INVALID_CHILD_GPA_PAGE_SET: WIN32_ERROR = WIN32_ERROR(3224829986u32);
pub const ERROR_VID_INVALID_GPA_RANGE_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829973u32);
pub const ERROR_VID_INVALID_MEMORY_BLOCK_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829970u32);
pub const ERROR_VID_INVALID_MESSAGE_QUEUE_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829972u32);
pub const ERROR_VID_INVALID_NUMA_NODE_INDEX: WIN32_ERROR = WIN32_ERROR(3224829968u32);
pub const ERROR_VID_INVALID_NUMA_SETTINGS: WIN32_ERROR = WIN32_ERROR(3224829967u32);
pub const ERROR_VID_INVALID_OBJECT_NAME: WIN32_ERROR = WIN32_ERROR(3224829957u32);
pub const ERROR_VID_INVALID_PPM_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829976u32);
pub const ERROR_VID_INVALID_PROCESSOR_STATE: WIN32_ERROR = WIN32_ERROR(3224829981u32);
pub const ERROR_VID_KM_INTERFACE_ALREADY_INITIALIZED: WIN32_ERROR = WIN32_ERROR(3224829983u32);
pub const ERROR_VID_MBPS_ARE_LOCKED: WIN32_ERROR = WIN32_ERROR(3224829977u32);
pub const ERROR_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE: WIN32_ERROR = WIN32_ERROR(3224829989u32);
pub const ERROR_VID_MBP_COUNT_EXCEEDED_LIMIT: WIN32_ERROR = WIN32_ERROR(3224829990u32);
pub const ERROR_VID_MB_PROPERTY_ALREADY_SET_RESET: WIN32_ERROR = WIN32_ERROR(3224829984u32);
pub const ERROR_VID_MB_STILL_REFERENCED: WIN32_ERROR = WIN32_ERROR(3224829965u32);
pub const ERROR_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(3224829975u32);
pub const ERROR_VID_MEMORY_TYPE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224829998u32);
pub const ERROR_VID_MESSAGE_QUEUE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(3224829963u32);
pub const ERROR_VID_MESSAGE_QUEUE_CLOSED: WIN32_ERROR = WIN32_ERROR(3224829978u32);
pub const ERROR_VID_MESSAGE_QUEUE_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(3224829959u32);
pub const ERROR_VID_MMIO_RANGE_DESTROYED: WIN32_ERROR = WIN32_ERROR(3224829985u32);
pub const ERROR_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(3224829969u32);
pub const ERROR_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE: WIN32_ERROR = WIN32_ERROR(3224829974u32);
pub const ERROR_VID_PAGE_RANGE_OVERFLOW: WIN32_ERROR = WIN32_ERROR(3224829971u32);
pub const ERROR_VID_PARTITION_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(3224829960u32);
pub const ERROR_VID_PARTITION_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(3224829961u32);
pub const ERROR_VID_PARTITION_NAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224829962u32);
pub const ERROR_VID_PARTITION_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(3224829958u32);
pub const ERROR_VID_PROCESS_ALREADY_SET: WIN32_ERROR = WIN32_ERROR(3224830000u32);
pub const ERROR_VID_QUEUE_FULL: WIN32_ERROR = WIN32_ERROR(3224829955u32);
pub const ERROR_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED: WIN32_ERROR = WIN32_ERROR(2151088129u32);
pub const ERROR_VID_RESERVE_PAGE_SET_IS_BEING_USED: WIN32_ERROR = WIN32_ERROR(3224829987u32);
pub const ERROR_VID_RESERVE_PAGE_SET_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(3224829988u32);
pub const ERROR_VID_SAVED_STATE_CORRUPT: WIN32_ERROR = WIN32_ERROR(3224829991u32);
pub const ERROR_VID_SAVED_STATE_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(3224829993u32);
pub const ERROR_VID_SAVED_STATE_UNRECOGNIZED_ITEM: WIN32_ERROR = WIN32_ERROR(3224829992u32);
pub const ERROR_VID_STOP_PENDING: WIN32_ERROR = WIN32_ERROR(3224829980u32);
pub const ERROR_VID_TOO_MANY_HANDLERS: WIN32_ERROR = WIN32_ERROR(3224829954u32);
pub const ERROR_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(3224829979u32);
pub const ERROR_VID_VTL_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3224829994u32);
pub const ERROR_VIRTDISK_DISK_ALREADY_OWNED: WIN32_ERROR = WIN32_ERROR(3225026590u32);
pub const ERROR_VIRTDISK_DISK_ONLINE_AND_WRITABLE: WIN32_ERROR = WIN32_ERROR(3225026591u32);
pub const ERROR_VIRTDISK_NOT_VIRTUAL_DISK: WIN32_ERROR = WIN32_ERROR(3225026581u32);
pub const ERROR_VIRTDISK_PROVIDER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3225026580u32);
pub const ERROR_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE: WIN32_ERROR = WIN32_ERROR(3225026589u32);
pub const ERROR_VIRTUAL_DISK_LIMITATION: WIN32_ERROR = WIN32_ERROR(3225026586u32);
pub const ERROR_VIRUS_DELETED: WIN32_ERROR = WIN32_ERROR(226u32);
pub const ERROR_VIRUS_INFECTED: WIN32_ERROR = WIN32_ERROR(225u32);
pub const ERROR_VMCOMPUTE_CONNECTION_CLOSED: WIN32_ERROR = WIN32_ERROR(3224830218u32);
pub const ERROR_VMCOMPUTE_CONNECT_FAILED: WIN32_ERROR = WIN32_ERROR(3224830216u32);
pub const ERROR_VMCOMPUTE_HYPERV_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(3224830210u32);
pub const ERROR_VMCOMPUTE_IMAGE_MISMATCH: WIN32_ERROR = WIN32_ERROR(3224830209u32);
pub const ERROR_VMCOMPUTE_INVALID_JSON: WIN32_ERROR = WIN32_ERROR(3224830221u32);
pub const ERROR_VMCOMPUTE_INVALID_LAYER: WIN32_ERROR = WIN32_ERROR(3224830226u32);
pub const ERROR_VMCOMPUTE_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(3224830213u32);
pub const ERROR_VMCOMPUTE_OPERATION_PENDING: WIN32_ERROR = WIN32_ERROR(3224830211u32);
pub const ERROR_VMCOMPUTE_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(3224830225u32);
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(3224830223u32);
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_STOPPED: WIN32_ERROR = WIN32_ERROR(3224830224u32);
pub const ERROR_VMCOMPUTE_SYSTEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224830222u32);
pub const ERROR_VMCOMPUTE_TERMINATED: WIN32_ERROR = WIN32_ERROR(3224830215u32);
pub const ERROR_VMCOMPUTE_TERMINATED_DURING_START: WIN32_ERROR = WIN32_ERROR(3224830208u32);
pub const ERROR_VMCOMPUTE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(3224830217u32);
pub const ERROR_VMCOMPUTE_TOO_MANY_NOTIFICATIONS: WIN32_ERROR = WIN32_ERROR(3224830212u32);
pub const ERROR_VMCOMPUTE_UNEXPECTED_EXIT: WIN32_ERROR = WIN32_ERROR(3224830214u32);
pub const ERROR_VMCOMPUTE_UNKNOWN_MESSAGE: WIN32_ERROR = WIN32_ERROR(3224830219u32);
pub const ERROR_VMCOMPUTE_UNSUPPORTED_PROTOCOL_VERSION: WIN32_ERROR = WIN32_ERROR(3224830220u32);
pub const ERROR_VMCOMPUTE_WINDOWS_INSIDER_REQUIRED: WIN32_ERROR = WIN32_ERROR(3224830227u32);
pub const ERROR_VNET_VIRTUAL_SWITCH_NAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224830464u32);
pub const ERROR_VOLMGR_ALL_DISKS_FAILED: WIN32_ERROR = WIN32_ERROR(3224895529u32);
pub const ERROR_VOLMGR_BAD_BOOT_DISK: WIN32_ERROR = WIN32_ERROR(3224895567u32);
pub const ERROR_VOLMGR_DATABASE_FULL: WIN32_ERROR = WIN32_ERROR(3224895489u32);
pub const ERROR_VOLMGR_DIFFERENT_SECTOR_SIZE: WIN32_ERROR = WIN32_ERROR(3224895566u32);
pub const ERROR_VOLMGR_DISK_CONFIGURATION_CORRUPTED: WIN32_ERROR = WIN32_ERROR(3224895490u32);
pub const ERROR_VOLMGR_DISK_CONFIGURATION_NOT_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895491u32);
pub const ERROR_VOLMGR_DISK_CONTAINS_NON_SIMPLE_VOLUME: WIN32_ERROR = WIN32_ERROR(3224895493u32);
pub const ERROR_VOLMGR_DISK_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895494u32);
pub const ERROR_VOLMGR_DISK_DYNAMIC: WIN32_ERROR = WIN32_ERROR(3224895495u32);
pub const ERROR_VOLMGR_DISK_ID_INVALID: WIN32_ERROR = WIN32_ERROR(3224895496u32);
pub const ERROR_VOLMGR_DISK_INVALID: WIN32_ERROR = WIN32_ERROR(3224895497u32);
pub const ERROR_VOLMGR_DISK_LAST_VOTER: WIN32_ERROR = WIN32_ERROR(3224895498u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_INVALID: WIN32_ERROR = WIN32_ERROR(3224895499u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_NON_BASIC_BETWEEN_BASIC_PARTITIONS: WIN32_ERROR = WIN32_ERROR(3224895500u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_NOT_CYLINDER_ALIGNED: WIN32_ERROR = WIN32_ERROR(3224895501u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_PARTITIONS_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(3224895502u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_PRIMARY_BETWEEN_LOGICAL_PARTITIONS: WIN32_ERROR = WIN32_ERROR(3224895503u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_TOO_MANY_PARTITIONS: WIN32_ERROR = WIN32_ERROR(3224895504u32);
pub const ERROR_VOLMGR_DISK_MISSING: WIN32_ERROR = WIN32_ERROR(3224895505u32);
pub const ERROR_VOLMGR_DISK_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(3224895506u32);
pub const ERROR_VOLMGR_DISK_NOT_ENOUGH_SPACE: WIN32_ERROR = WIN32_ERROR(3224895507u32);
pub const ERROR_VOLMGR_DISK_REVECTORING_FAILED: WIN32_ERROR = WIN32_ERROR(3224895508u32);
pub const ERROR_VOLMGR_DISK_SECTOR_SIZE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895509u32);
pub const ERROR_VOLMGR_DISK_SET_NOT_CONTAINED: WIN32_ERROR = WIN32_ERROR(3224895510u32);
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_MEMBERS: WIN32_ERROR = WIN32_ERROR(3224895511u32);
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_PLEXES: WIN32_ERROR = WIN32_ERROR(3224895512u32);
pub const ERROR_VOLMGR_DYNAMIC_DISK_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895513u32);
pub const ERROR_VOLMGR_EXTENT_ALREADY_USED: WIN32_ERROR = WIN32_ERROR(3224895514u32);
pub const ERROR_VOLMGR_EXTENT_NOT_CONTIGUOUS: WIN32_ERROR = WIN32_ERROR(3224895515u32);
pub const ERROR_VOLMGR_EXTENT_NOT_IN_PUBLIC_REGION: WIN32_ERROR = WIN32_ERROR(3224895516u32);
pub const ERROR_VOLMGR_EXTENT_NOT_SECTOR_ALIGNED: WIN32_ERROR = WIN32_ERROR(3224895517u32);
pub const ERROR_VOLMGR_EXTENT_OVERLAPS_EBR_PARTITION: WIN32_ERROR = WIN32_ERROR(3224895518u32);
pub const ERROR_VOLMGR_EXTENT_VOLUME_LENGTHS_DO_NOT_MATCH: WIN32_ERROR = WIN32_ERROR(3224895519u32);
pub const ERROR_VOLMGR_FAULT_TOLERANT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895520u32);
pub const ERROR_VOLMGR_INCOMPLETE_DISK_MIGRATION: WIN32_ERROR = WIN32_ERROR(2151153666u32);
pub const ERROR_VOLMGR_INCOMPLETE_REGENERATION: WIN32_ERROR = WIN32_ERROR(2151153665u32);
pub const ERROR_VOLMGR_INTERLEAVE_LENGTH_INVALID: WIN32_ERROR = WIN32_ERROR(3224895521u32);
pub const ERROR_VOLMGR_MAXIMUM_REGISTERED_USERS: WIN32_ERROR = WIN32_ERROR(3224895522u32);
pub const ERROR_VOLMGR_MEMBER_INDEX_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895524u32);
pub const ERROR_VOLMGR_MEMBER_INDEX_INVALID: WIN32_ERROR = WIN32_ERROR(3224895525u32);
pub const ERROR_VOLMGR_MEMBER_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895523u32);
pub const ERROR_VOLMGR_MEMBER_MISSING: WIN32_ERROR = WIN32_ERROR(3224895526u32);
pub const ERROR_VOLMGR_MEMBER_NOT_DETACHED: WIN32_ERROR = WIN32_ERROR(3224895527u32);
pub const ERROR_VOLMGR_MEMBER_REGENERATING: WIN32_ERROR = WIN32_ERROR(3224895528u32);
pub const ERROR_VOLMGR_MIRROR_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895579u32);
pub const ERROR_VOLMGR_NOTIFICATION_RESET: WIN32_ERROR = WIN32_ERROR(3224895532u32);
pub const ERROR_VOLMGR_NOT_PRIMARY_PACK: WIN32_ERROR = WIN32_ERROR(3224895570u32);
pub const ERROR_VOLMGR_NO_REGISTERED_USERS: WIN32_ERROR = WIN32_ERROR(3224895530u32);
pub const ERROR_VOLMGR_NO_SUCH_USER: WIN32_ERROR = WIN32_ERROR(3224895531u32);
pub const ERROR_VOLMGR_NO_VALID_LOG_COPIES: WIN32_ERROR = WIN32_ERROR(3224895576u32);
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_INVALID: WIN32_ERROR = WIN32_ERROR(3224895578u32);
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_MEMBER_INVALID: WIN32_ERROR = WIN32_ERROR(3224895573u32);
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_PLEX_INVALID: WIN32_ERROR = WIN32_ERROR(3224895572u32);
pub const ERROR_VOLMGR_NUMBER_OF_EXTENTS_INVALID: WIN32_ERROR = WIN32_ERROR(3224895565u32);
pub const ERROR_VOLMGR_NUMBER_OF_MEMBERS_INVALID: WIN32_ERROR = WIN32_ERROR(3224895533u32);
pub const ERROR_VOLMGR_NUMBER_OF_PLEXES_INVALID: WIN32_ERROR = WIN32_ERROR(3224895534u32);
pub const ERROR_VOLMGR_PACK_CONFIG_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895568u32);
pub const ERROR_VOLMGR_PACK_CONFIG_ONLINE: WIN32_ERROR = WIN32_ERROR(3224895569u32);
pub const ERROR_VOLMGR_PACK_CONFIG_UPDATE_FAILED: WIN32_ERROR = WIN32_ERROR(3224895492u32);
pub const ERROR_VOLMGR_PACK_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895535u32);
pub const ERROR_VOLMGR_PACK_HAS_QUORUM: WIN32_ERROR = WIN32_ERROR(3224895540u32);
pub const ERROR_VOLMGR_PACK_ID_INVALID: WIN32_ERROR = WIN32_ERROR(3224895536u32);
pub const ERROR_VOLMGR_PACK_INVALID: WIN32_ERROR = WIN32_ERROR(3224895537u32);
pub const ERROR_VOLMGR_PACK_LOG_UPDATE_FAILED: WIN32_ERROR = WIN32_ERROR(3224895571u32);
pub const ERROR_VOLMGR_PACK_NAME_INVALID: WIN32_ERROR = WIN32_ERROR(3224895538u32);
pub const ERROR_VOLMGR_PACK_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895539u32);
pub const ERROR_VOLMGR_PACK_WITHOUT_QUORUM: WIN32_ERROR = WIN32_ERROR(3224895541u32);
pub const ERROR_VOLMGR_PARTITION_STYLE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895542u32);
pub const ERROR_VOLMGR_PARTITION_UPDATE_FAILED: WIN32_ERROR = WIN32_ERROR(3224895543u32);
pub const ERROR_VOLMGR_PLEX_INDEX_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895545u32);
pub const ERROR_VOLMGR_PLEX_INDEX_INVALID: WIN32_ERROR = WIN32_ERROR(3224895546u32);
pub const ERROR_VOLMGR_PLEX_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895544u32);
pub const ERROR_VOLMGR_PLEX_LAST_ACTIVE: WIN32_ERROR = WIN32_ERROR(3224895547u32);
pub const ERROR_VOLMGR_PLEX_MISSING: WIN32_ERROR = WIN32_ERROR(3224895548u32);
pub const ERROR_VOLMGR_PLEX_NOT_RAID5: WIN32_ERROR = WIN32_ERROR(3224895551u32);
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE: WIN32_ERROR = WIN32_ERROR(3224895552u32);
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE_SPANNED: WIN32_ERROR = WIN32_ERROR(3224895575u32);
pub const ERROR_VOLMGR_PLEX_REGENERATING: WIN32_ERROR = WIN32_ERROR(3224895549u32);
pub const ERROR_VOLMGR_PLEX_TYPE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895550u32);
pub const ERROR_VOLMGR_PRIMARY_PACK_PRESENT: WIN32_ERROR = WIN32_ERROR(3224895577u32);
pub const ERROR_VOLMGR_RAID5_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895580u32);
pub const ERROR_VOLMGR_STRUCTURE_SIZE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895553u32);
pub const ERROR_VOLMGR_TOO_MANY_NOTIFICATION_REQUESTS: WIN32_ERROR = WIN32_ERROR(3224895554u32);
pub const ERROR_VOLMGR_TRANSACTION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(3224895555u32);
pub const ERROR_VOLMGR_UNEXPECTED_DISK_LAYOUT_CHANGE: WIN32_ERROR = WIN32_ERROR(3224895556u32);
pub const ERROR_VOLMGR_VOLUME_CONTAINS_MISSING_DISK: WIN32_ERROR = WIN32_ERROR(3224895557u32);
pub const ERROR_VOLMGR_VOLUME_ID_INVALID: WIN32_ERROR = WIN32_ERROR(3224895558u32);
pub const ERROR_VOLMGR_VOLUME_LENGTH_INVALID: WIN32_ERROR = WIN32_ERROR(3224895559u32);
pub const ERROR_VOLMGR_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: WIN32_ERROR = WIN32_ERROR(3224895560u32);
pub const ERROR_VOLMGR_VOLUME_MIRRORED: WIN32_ERROR = WIN32_ERROR(3224895574u32);
pub const ERROR_VOLMGR_VOLUME_NOT_MIRRORED: WIN32_ERROR = WIN32_ERROR(3224895561u32);
pub const ERROR_VOLMGR_VOLUME_NOT_RETAINED: WIN32_ERROR = WIN32_ERROR(3224895562u32);
pub const ERROR_VOLMGR_VOLUME_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895563u32);
pub const ERROR_VOLMGR_VOLUME_RETAINED: WIN32_ERROR = WIN32_ERROR(3224895564u32);
pub const ERROR_VOLSNAP_ACTIVATION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80820002_u32 as _);
pub const ERROR_VOLSNAP_BOOTFILE_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0x80820001_u32 as _);
pub const ERROR_VOLSNAP_HIBERNATE_READY: WIN32_ERROR = WIN32_ERROR(761u32);
pub const ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT: windows_core::HRESULT = windows_core::HRESULT(0x80820003_u32 as _);
pub const ERROR_VOLSNAP_PREPARE_HIBERNATE: WIN32_ERROR = WIN32_ERROR(655u32);
pub const ERROR_VOLUME_CONTAINS_SYS_FILES: WIN32_ERROR = WIN32_ERROR(4337u32);
pub const ERROR_VOLUME_DIRTY: WIN32_ERROR = WIN32_ERROR(6851u32);
pub const ERROR_VOLUME_MOUNTED: WIN32_ERROR = WIN32_ERROR(743u32);
pub const ERROR_VOLUME_NOT_CLUSTER_ALIGNED: WIN32_ERROR = WIN32_ERROR(407u32);
pub const ERROR_VOLUME_NOT_SIS_ENABLED: WIN32_ERROR = WIN32_ERROR(4500u32);
pub const ERROR_VOLUME_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(492u32);
pub const ERROR_VOLUME_NOT_SUPPORT_EFS: WIN32_ERROR = WIN32_ERROR(6014u32);
pub const ERROR_VOLUME_UPGRADE_DISABLED: WIN32_ERROR = WIN32_ERROR(517u32);
pub const ERROR_VOLUME_UPGRADE_DISABLED_TILL_OS_DOWNGRADE_EXPIRED: WIN32_ERROR = WIN32_ERROR(518u32);
pub const ERROR_VOLUME_UPGRADE_NOT_NEEDED: WIN32_ERROR = WIN32_ERROR(515u32);
pub const ERROR_VOLUME_UPGRADE_PENDING: WIN32_ERROR = WIN32_ERROR(516u32);
pub const ERROR_VOLUME_WRITE_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(508u32);
pub const ERROR_VRF_VOLATILE_CFG_AND_IO_ENABLED: WIN32_ERROR = WIN32_ERROR(3080u32);
pub const ERROR_VRF_VOLATILE_NMI_REGISTERED: WIN32_ERROR = WIN32_ERROR(3086u32);
pub const ERROR_VRF_VOLATILE_NOT_RUNNABLE_SYSTEM: WIN32_ERROR = WIN32_ERROR(3083u32);
pub const ERROR_VRF_VOLATILE_NOT_STOPPABLE: WIN32_ERROR = WIN32_ERROR(3081u32);
pub const ERROR_VRF_VOLATILE_NOT_SUPPORTED_RULECLASS: WIN32_ERROR = WIN32_ERROR(3084u32);
pub const ERROR_VRF_VOLATILE_PROTECTED_DRIVER: WIN32_ERROR = WIN32_ERROR(3085u32);
pub const ERROR_VRF_VOLATILE_SAFE_MODE: WIN32_ERROR = WIN32_ERROR(3082u32);
pub const ERROR_VRF_VOLATILE_SETTINGS_CONFLICT: WIN32_ERROR = WIN32_ERROR(3087u32);
pub const ERROR_VSMB_SAVED_STATE_CORRUPT: WIN32_ERROR = WIN32_ERROR(3224830977u32);
pub const ERROR_VSMB_SAVED_STATE_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224830976u32);
pub const ERROR_VSM_DMA_PROTECTION_NOT_IN_USE: WIN32_ERROR = WIN32_ERROR(4561u32);
pub const ERROR_VSM_NOT_INITIALIZED: WIN32_ERROR = WIN32_ERROR(4560u32);
pub const ERROR_WAIT_1: WIN32_ERROR = WIN32_ERROR(731u32);
pub const ERROR_WAIT_2: WIN32_ERROR = WIN32_ERROR(732u32);
pub const ERROR_WAIT_3: WIN32_ERROR = WIN32_ERROR(733u32);
pub const ERROR_WAIT_63: WIN32_ERROR = WIN32_ERROR(734u32);
pub const ERROR_WAIT_FOR_OPLOCK: WIN32_ERROR = WIN32_ERROR(765u32);
pub const ERROR_WAIT_NO_CHILDREN: WIN32_ERROR = WIN32_ERROR(128u32);
pub const ERROR_WAKE_SYSTEM: WIN32_ERROR = WIN32_ERROR(730u32);
pub const ERROR_WAKE_SYSTEM_DEBUGGER: WIN32_ERROR = WIN32_ERROR(675u32);
pub const ERROR_WAS_LOCKED: WIN32_ERROR = WIN32_ERROR(717u32);
pub const ERROR_WAS_UNLOCKED: WIN32_ERROR = WIN32_ERROR(715u32);
pub const ERROR_WEAK_WHFBKEY_BLOCKED: WIN32_ERROR = WIN32_ERROR(8651u32);
pub const ERROR_WINDOW_NOT_COMBOBOX: WIN32_ERROR = WIN32_ERROR(1423u32);
pub const ERROR_WINDOW_NOT_DIALOG: WIN32_ERROR = WIN32_ERROR(1420u32);
pub const ERROR_WINDOW_OF_OTHER_THREAD: WIN32_ERROR = WIN32_ERROR(1408u32);
pub const ERROR_WINS_INTERNAL: WIN32_ERROR = WIN32_ERROR(4000u32);
pub const ERROR_WIP_ENCRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6023u32);
pub const ERROR_WMI_ALREADY_DISABLED: WIN32_ERROR = WIN32_ERROR(4212u32);
pub const ERROR_WMI_ALREADY_ENABLED: WIN32_ERROR = WIN32_ERROR(4206u32);
pub const ERROR_WMI_DP_FAILED: WIN32_ERROR = WIN32_ERROR(4209u32);
pub const ERROR_WMI_DP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4204u32);
pub const ERROR_WMI_GUID_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(4207u32);
pub const ERROR_WMI_GUID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4200u32);
pub const ERROR_WMI_INSTANCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4201u32);
pub const ERROR_WMI_INVALID_MOF: WIN32_ERROR = WIN32_ERROR(4210u32);
pub const ERROR_WMI_INVALID_REGINFO: WIN32_ERROR = WIN32_ERROR(4211u32);
pub const ERROR_WMI_ITEMID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4202u32);
pub const ERROR_WMI_READ_ONLY: WIN32_ERROR = WIN32_ERROR(4213u32);
pub const ERROR_WMI_SERVER_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(4208u32);
pub const ERROR_WMI_SET_FAILURE: WIN32_ERROR = WIN32_ERROR(4214u32);
pub const ERROR_WMI_TRY_AGAIN: WIN32_ERROR = WIN32_ERROR(4203u32);
pub const ERROR_WMI_UNRESOLVED_INSTANCE_REF: WIN32_ERROR = WIN32_ERROR(4205u32);
pub const ERROR_WOF_FILE_RESOURCE_TABLE_CORRUPT: WIN32_ERROR = WIN32_ERROR(4448u32);
pub const ERROR_WOF_WIM_HEADER_CORRUPT: WIN32_ERROR = WIN32_ERROR(4446u32);
pub const ERROR_WOF_WIM_RESOURCE_TABLE_CORRUPT: WIN32_ERROR = WIN32_ERROR(4447u32);
pub const ERROR_WORKING_SET_QUOTA: WIN32_ERROR = WIN32_ERROR(1453u32);
pub const ERROR_WOW_ASSERTION: WIN32_ERROR = WIN32_ERROR(670u32);
pub const ERROR_WRITE_FAULT: WIN32_ERROR = WIN32_ERROR(29u32);
pub const ERROR_WRITE_PROTECT: WIN32_ERROR = WIN32_ERROR(19u32);
pub const ERROR_WRONG_COMPARTMENT: WIN32_ERROR = WIN32_ERROR(1468u32);
pub const ERROR_WRONG_DISK: WIN32_ERROR = WIN32_ERROR(34u32);
pub const ERROR_WRONG_EFS: WIN32_ERROR = WIN32_ERROR(6005u32);
pub const ERROR_WRONG_INF_STYLE: WIN32_ERROR = WIN32_ERROR(3758096640u32);
pub const ERROR_WRONG_INF_TYPE: WIN32_ERROR = WIN32_ERROR(3758096970u32);
pub const ERROR_WRONG_PASSWORD: WIN32_ERROR = WIN32_ERROR(1323u32);
pub const ERROR_WRONG_TARGET_NAME: WIN32_ERROR = WIN32_ERROR(1396u32);
pub const ERROR_WX86_ERROR: WIN32_ERROR = WIN32_ERROR(540u32);
pub const ERROR_WX86_WARNING: WIN32_ERROR = WIN32_ERROR(539u32);
pub const ERROR_XMLDSIG_ERROR: WIN32_ERROR = WIN32_ERROR(1466u32);
pub const ERROR_XML_ENCODING_MISMATCH: WIN32_ERROR = WIN32_ERROR(14100u32);
pub const ERROR_XML_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(1465u32);
pub const EVENT_E_ALL_SUBSCRIBERS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const EVENT_E_CANT_MODIFY_OR_DELETE_CONFIGURED_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x8004020E_u32 as _);
pub const EVENT_E_CANT_MODIFY_OR_DELETE_UNCONFIGURED_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x8004020D_u32 as _);
pub const EVENT_E_COMPLUS_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x8004020C_u32 as _);
pub const EVENT_E_FIRST: i32 = -2147220992i32;
pub const EVENT_E_INTERNALERROR: windows_core::HRESULT = windows_core::HRESULT(0x80040206_u32 as _);
pub const EVENT_E_INTERNALEXCEPTION: windows_core::HRESULT = windows_core::HRESULT(0x80040205_u32 as _);
pub const EVENT_E_INVALID_EVENT_CLASS_PARTITION: windows_core::HRESULT = windows_core::HRESULT(0x8004020F_u32 as _);
pub const EVENT_E_INVALID_PER_USER_SID: windows_core::HRESULT = windows_core::HRESULT(0x80040207_u32 as _);
pub const EVENT_E_LAST: i32 = -2147220961i32;
pub const EVENT_E_MISSING_EVENTCLASS: windows_core::HRESULT = windows_core::HRESULT(0x8004020A_u32 as _);
pub const EVENT_E_NOT_ALL_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x8004020B_u32 as _);
pub const EVENT_E_PER_USER_SID_NOT_LOGGED_ON: windows_core::HRESULT = windows_core::HRESULT(0x80040210_u32 as _);
pub const EVENT_E_QUERYFIELD: windows_core::HRESULT = windows_core::HRESULT(0x80040204_u32 as _);
pub const EVENT_E_QUERYSYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x80040203_u32 as _);
pub const EVENT_E_TOO_MANY_METHODS: windows_core::HRESULT = windows_core::HRESULT(0x80040209_u32 as _);
pub const EVENT_E_USER_EXCEPTION: windows_core::HRESULT = windows_core::HRESULT(0x80040208_u32 as _);
pub const EVENT_S_FIRST: i32 = 262656i32;
pub const EVENT_S_LAST: i32 = 262687i32;
pub const EVENT_S_NOSUBSCRIBERS: windows_core::HRESULT = windows_core::HRESULT(0x40202_u32 as _);
pub const EVENT_S_SOME_SUBSCRIBERS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x40200_u32 as _);
pub const EXCEPTION_ACCESS_VIOLATION: NTSTATUS = NTSTATUS(0xC0000005_u32 as _);
pub const EXCEPTION_ARRAY_BOUNDS_EXCEEDED: NTSTATUS = NTSTATUS(0xC000008C_u32 as _);
pub const EXCEPTION_BREAKPOINT: NTSTATUS = NTSTATUS(0x80000003_u32 as _);
pub const EXCEPTION_DATATYPE_MISALIGNMENT: NTSTATUS = NTSTATUS(0x80000002_u32 as _);
pub const EXCEPTION_FLT_DENORMAL_OPERAND: NTSTATUS = NTSTATUS(0xC000008D_u32 as _);
pub const EXCEPTION_FLT_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(0xC000008E_u32 as _);
pub const EXCEPTION_FLT_INEXACT_RESULT: NTSTATUS = NTSTATUS(0xC000008F_u32 as _);
pub const EXCEPTION_FLT_INVALID_OPERATION: NTSTATUS = NTSTATUS(0xC0000090_u32 as _);
pub const EXCEPTION_FLT_OVERFLOW: NTSTATUS = NTSTATUS(0xC0000091_u32 as _);
pub const EXCEPTION_FLT_STACK_CHECK: NTSTATUS = NTSTATUS(0xC0000092_u32 as _);
pub const EXCEPTION_FLT_UNDERFLOW: NTSTATUS = NTSTATUS(0xC0000093_u32 as _);
pub const EXCEPTION_GUARD_PAGE: NTSTATUS = NTSTATUS(0x80000001_u32 as _);
pub const EXCEPTION_ILLEGAL_INSTRUCTION: NTSTATUS = NTSTATUS(0xC000001D_u32 as _);
pub const EXCEPTION_INT_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(0xC0000094_u32 as _);
pub const EXCEPTION_INT_OVERFLOW: NTSTATUS = NTSTATUS(0xC0000095_u32 as _);
pub const EXCEPTION_INVALID_DISPOSITION: NTSTATUS = NTSTATUS(0xC0000026_u32 as _);
pub const EXCEPTION_INVALID_HANDLE: NTSTATUS = NTSTATUS(0xC0000008_u32 as _);
pub const EXCEPTION_IN_PAGE_ERROR: NTSTATUS = NTSTATUS(0xC0000006_u32 as _);
pub const EXCEPTION_NONCONTINUABLE_EXCEPTION: NTSTATUS = NTSTATUS(0xC0000025_u32 as _);
pub const EXCEPTION_POSSIBLE_DEADLOCK: NTSTATUS = NTSTATUS(0xC0000194_u32 as _);
pub const EXCEPTION_PRIV_INSTRUCTION: NTSTATUS = NTSTATUS(0xC0000096_u32 as _);
pub const EXCEPTION_SINGLE_STEP: NTSTATUS = NTSTATUS(0x80000004_u32 as _);
pub const EXCEPTION_SPAPI_UNRECOVERABLE_STACK_OVERFLOW: NTSTATUS = NTSTATUS(0xE0000300_u32 as _);
pub const EXCEPTION_STACK_OVERFLOW: NTSTATUS = NTSTATUS(0xC00000FD_u32 as _);
pub const E_ABORT: windows_core::HRESULT = windows_core::HRESULT(0x80004004_u32 as _);
pub const E_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80070005_u32 as _);
pub const E_APPLICATION_ACTIVATION_EXEC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8027025B_u32 as _);
pub const E_APPLICATION_ACTIVATION_TIMED_OUT: windows_core::HRESULT = windows_core::HRESULT(0x8027025A_u32 as _);
pub const E_APPLICATION_EXITING: windows_core::HRESULT = windows_core::HRESULT(0x8000001A_u32 as _);
pub const E_APPLICATION_MANAGER_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80270257_u32 as _);
pub const E_APPLICATION_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80270254_u32 as _);
pub const E_APPLICATION_TEMPORARY_LICENSE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8027025C_u32 as _);
pub const E_APPLICATION_TRIAL_LICENSE_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x8027025D_u32 as _);
pub const E_APPLICATION_VIEW_EXITING: windows_core::HRESULT = windows_core::HRESULT(0x8000001B_u32 as _);
pub const E_ASYNC_OPERATION_NOT_STARTED: windows_core::HRESULT = windows_core::HRESULT(0x80000019_u32 as _);
pub const E_AUDIO_ENGINE_NODE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80660001_u32 as _);
pub const E_BLUETOOTH_ATT_ATTRIBUTE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8065000A_u32 as _);
pub const E_BLUETOOTH_ATT_ATTRIBUTE_NOT_LONG: windows_core::HRESULT = windows_core::HRESULT(0x8065000B_u32 as _);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_AUTHENTICATION: windows_core::HRESULT = windows_core::HRESULT(0x80650005_u32 as _);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_AUTHORIZATION: windows_core::HRESULT = windows_core::HRESULT(0x80650008_u32 as _);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION: windows_core::HRESULT = windows_core::HRESULT(0x8065000F_u32 as _);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x8065000C_u32 as _);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x80650011_u32 as _);
pub const E_BLUETOOTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x8065000D_u32 as _);
pub const E_BLUETOOTH_ATT_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80650001_u32 as _);
pub const E_BLUETOOTH_ATT_INVALID_OFFSET: windows_core::HRESULT = windows_core::HRESULT(0x80650007_u32 as _);
pub const E_BLUETOOTH_ATT_INVALID_PDU: windows_core::HRESULT = windows_core::HRESULT(0x80650004_u32 as _);
pub const E_BLUETOOTH_ATT_PREPARE_QUEUE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80650009_u32 as _);
pub const E_BLUETOOTH_ATT_READ_NOT_PERMITTED: windows_core::HRESULT = windows_core::HRESULT(0x80650002_u32 as _);
pub const E_BLUETOOTH_ATT_REQUEST_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80650006_u32 as _);
pub const E_BLUETOOTH_ATT_UNKNOWN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80651000_u32 as _);
pub const E_BLUETOOTH_ATT_UNLIKELY: windows_core::HRESULT = windows_core::HRESULT(0x8065000E_u32 as _);
pub const E_BLUETOOTH_ATT_UNSUPPORTED_GROUP_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80650010_u32 as _);
pub const E_BLUETOOTH_ATT_WRITE_NOT_PERMITTED: windows_core::HRESULT = windows_core::HRESULT(0x80650003_u32 as _);
pub const E_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x8000000B_u32 as _);
pub const E_CHANGED_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8000000C_u32 as _);
pub const E_ELEVATED_ACTIVATION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80270251_u32 as _);
pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
pub const E_FULL_ADMIN_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80270253_u32 as _);
pub const E_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80070006_u32 as _);
pub const E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80660003_u32 as _);
pub const E_HDAUDIO_EMPTY_CONNECTION_LIST: windows_core::HRESULT = windows_core::HRESULT(0x80660002_u32 as _);
pub const E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED: windows_core::HRESULT = windows_core::HRESULT(0x80660004_u32 as _);
pub const E_HDAUDIO_NULL_LINKED_LIST_ENTRY: windows_core::HRESULT = windows_core::HRESULT(0x80660005_u32 as _);
pub const E_ILLEGAL_DELEGATE_ASSIGNMENT: windows_core::HRESULT = windows_core::HRESULT(0x80000018_u32 as _);
pub const E_ILLEGAL_METHOD_CALL: windows_core::HRESULT = windows_core::HRESULT(0x8000000E_u32 as _);
pub const E_ILLEGAL_STATE_CHANGE: windows_core::HRESULT = windows_core::HRESULT(0x8000000D_u32 as _);
pub const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const E_INVALID_PROTOCOL_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x83760002_u32 as _);
pub const E_INVALID_PROTOCOL_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x83760001_u32 as _);
pub const E_MBN_BAD_SIM: windows_core::HRESULT = windows_core::HRESULT(0x80548202_u32 as _);
pub const E_MBN_CONTEXT_NOT_ACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0x80548201_u32 as _);
pub const E_MBN_DATA_CLASS_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80548203_u32 as _);
pub const E_MBN_DEFAULT_PROFILE_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80548219_u32 as _);
pub const E_MBN_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80548212_u32 as _);
pub const E_MBN_INVALID_ACCESS_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80548204_u32 as _);
pub const E_MBN_INVALID_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x8054820C_u32 as _);
pub const E_MBN_INVALID_PROFILE: windows_core::HRESULT = windows_core::HRESULT(0x80548218_u32 as _);
pub const E_MBN_MAX_ACTIVATED_CONTEXTS: windows_core::HRESULT = windows_core::HRESULT(0x80548205_u32 as _);
pub const E_MBN_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8054820D_u32 as _);
pub const E_MBN_PACKET_SVC_DETACHED: windows_core::HRESULT = windows_core::HRESULT(0x80548206_u32 as _);
pub const E_MBN_PIN_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80548211_u32 as _);
pub const E_MBN_PIN_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8054820F_u32 as _);
pub const E_MBN_PIN_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80548210_u32 as _);
pub const E_MBN_PROVIDERS_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8054820E_u32 as _);
pub const E_MBN_PROVIDER_NOT_VISIBLE: windows_core::HRESULT = windows_core::HRESULT(0x80548207_u32 as _);
pub const E_MBN_RADIO_POWER_OFF: windows_core::HRESULT = windows_core::HRESULT(0x80548208_u32 as _);
pub const E_MBN_SERVICE_NOT_ACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0x80548209_u32 as _);
pub const E_MBN_SIM_NOT_INSERTED: windows_core::HRESULT = windows_core::HRESULT(0x8054820A_u32 as _);
pub const E_MBN_SMS_ENCODING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80548220_u32 as _);
pub const E_MBN_SMS_FILTER_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80548221_u32 as _);
pub const E_MBN_SMS_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80548227_u32 as _);
pub const E_MBN_SMS_INVALID_MEMORY_INDEX: windows_core::HRESULT = windows_core::HRESULT(0x80548222_u32 as _);
pub const E_MBN_SMS_LANG_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80548223_u32 as _);
pub const E_MBN_SMS_MEMORY_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80548224_u32 as _);
pub const E_MBN_SMS_MEMORY_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80548229_u32 as _);
pub const E_MBN_SMS_NETWORK_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80548225_u32 as _);
pub const E_MBN_SMS_OPERATION_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80548228_u32 as _);
pub const E_MBN_SMS_UNKNOWN_SMSC_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80548226_u32 as _);
pub const E_MBN_VOICE_CALL_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8054820B_u32 as _);
pub const E_MONITOR_RESOLUTION_TOO_LOW: windows_core::HRESULT = windows_core::HRESULT(0x80270250_u32 as _);
pub const E_MULTIPLE_EXTENSIONS_FOR_APPLICATION: windows_core::HRESULT = windows_core::HRESULT(0x80270255_u32 as _);
pub const E_MULTIPLE_PACKAGES_FOR_FAMILY: windows_core::HRESULT = windows_core::HRESULT(0x80270256_u32 as _);
pub const E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80004002_u32 as _);
pub const E_NOTIMPL: windows_core::HRESULT = windows_core::HRESULT(0x80004001_u32 as _);
pub const E_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8007000E_u32 as _);
pub const E_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80004003_u32 as _);
pub const E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x83760003_u32 as _);
pub const E_PROTOCOL_VERSION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x83760005_u32 as _);
pub const E_SKYDRIVE_FILE_NOT_UPLOADED: windows_core::HRESULT = windows_core::HRESULT(0x80270263_u32 as _);
pub const E_SKYDRIVE_ROOT_TARGET_CANNOT_INDEX: windows_core::HRESULT = windows_core::HRESULT(0x80270262_u32 as _);
pub const E_SKYDRIVE_ROOT_TARGET_FILE_SYSTEM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80270260_u32 as _);
pub const E_SKYDRIVE_ROOT_TARGET_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0x80270261_u32 as _);
pub const E_SKYDRIVE_ROOT_TARGET_VOLUME_ROOT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80270265_u32 as _);
pub const E_SKYDRIVE_UPDATE_AVAILABILITY_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80270264_u32 as _);
pub const E_STRING_NOT_NULL_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x80000017_u32 as _);
pub const E_SUBPROTOCOL_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x83760004_u32 as _);
pub const E_SYNCENGINE_CLIENT_UPDATE_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8802D006_u32 as _);
pub const E_SYNCENGINE_FILE_IDENTIFIER_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x8802C002_u32 as _);
pub const E_SYNCENGINE_FILE_SIZE_EXCEEDS_REMAINING_QUOTA: windows_core::HRESULT = windows_core::HRESULT(0x8802B002_u32 as _);
pub const E_SYNCENGINE_FILE_SIZE_OVER_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x8802B001_u32 as _);
pub const E_SYNCENGINE_FILE_SYNC_PARTNER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8802B005_u32 as _);
pub const E_SYNCENGINE_FOLDER_INACCESSIBLE: windows_core::HRESULT = windows_core::HRESULT(0x8802D001_u32 as _);
pub const E_SYNCENGINE_FOLDER_IN_REDIRECTION: windows_core::HRESULT = windows_core::HRESULT(0x8802D00B_u32 as _);
pub const E_SYNCENGINE_FOLDER_ITEM_COUNT_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8802B004_u32 as _);
pub const E_SYNCENGINE_PATH_LENGTH_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8802D004_u32 as _);
pub const E_SYNCENGINE_PROXY_AUTHENTICATION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8802D007_u32 as _);
pub const E_SYNCENGINE_REMOTE_PATH_LENGTH_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8802D005_u32 as _);
pub const E_SYNCENGINE_REQUEST_BLOCKED_BY_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x8802C006_u32 as _);
pub const E_SYNCENGINE_REQUEST_BLOCKED_DUE_TO_CLIENT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8802C007_u32 as _);
pub const E_SYNCENGINE_SERVICE_AUTHENTICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8802C003_u32 as _);
pub const E_SYNCENGINE_SERVICE_RETURNED_UNEXPECTED_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x8802C005_u32 as _);
pub const E_SYNCENGINE_STORAGE_SERVICE_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8802D00A_u32 as _);
pub const E_SYNCENGINE_STORAGE_SERVICE_PROVISIONING_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8802D008_u32 as _);
pub const E_SYNCENGINE_SYNC_PAUSED_BY_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x8802B006_u32 as _);
pub const E_SYNCENGINE_UNKNOWN_SERVICE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8802C004_u32 as _);
pub const E_SYNCENGINE_UNSUPPORTED_FILE_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8802B003_u32 as _);
pub const E_SYNCENGINE_UNSUPPORTED_FOLDER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8802D002_u32 as _);
pub const E_SYNCENGINE_UNSUPPORTED_MARKET: windows_core::HRESULT = windows_core::HRESULT(0x8802D003_u32 as _);
pub const E_SYNCENGINE_UNSUPPORTED_REPARSE_POINT: windows_core::HRESULT = windows_core::HRESULT(0x8802D009_u32 as _);
pub const E_UAC_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80270252_u32 as _);
pub const E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8000FFFF_u32 as _);
pub const FACILITY_ACPI_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(20u32);
pub const FACILITY_APP_EXEC: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(236u32);
pub const FACILITY_AUDIO_KERNEL: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(68u32);
pub const FACILITY_BCD_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(57u32);
pub const FACILITY_BTH_ATT: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(66u32);
pub const FACILITY_CLUSTER_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(19u32);
pub const FACILITY_CODCLASS_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(6u32);
pub const FACILITY_COMMONLOG: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(26u32);
pub const FACILITY_DEBUGGER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(1u32);
pub const FACILITY_DRIVER_FRAMEWORK: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(32u32);
pub const FACILITY_FILTER_MANAGER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(28u32);
pub const FACILITY_FIREWIRE_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(18u32);
pub const FACILITY_FVE_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(33u32);
pub const FACILITY_FWP_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(34u32);
pub const FACILITY_GRAPHICS_KERNEL: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(30u32);
pub const FACILITY_HID_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(17u32);
pub const FACILITY_HYPERVISOR: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(53u32);
pub const FACILITY_INTERIX: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(153u32);
pub const FACILITY_IO_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(4u32);
pub const FACILITY_IPSEC: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(54u32);
pub const FACILITY_LICENSING: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(234u32);
pub const FACILITY_MAXIMUM_VALUE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(237u32);
pub const FACILITY_MCA_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(5u32);
pub const FACILITY_MONITOR: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(29u32);
pub const FACILITY_NDIS_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(35u32);
pub const FACILITY_NTCERT: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(8u32);
pub const FACILITY_NTSSPI: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(9u32);
pub const FACILITY_NTWIN32: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(7u32);
pub const FACILITY_NT_IORING: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(70u32);
pub const FACILITY_PLATFORM_MANIFEST: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(235u32);
pub const FACILITY_QUIC_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(36u32);
pub const FACILITY_RDBSS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(65u32);
pub const FACILITY_RESUME_KEY_FILTER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(64u32);
pub const FACILITY_RPC_RUNTIME: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(2u32);
pub const FACILITY_RPC_STUBS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(3u32);
pub const FACILITY_RTPM: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(42u32);
pub const FACILITY_SDBUS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(81u32);
pub const FACILITY_SECUREBOOT: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(67u32);
pub const FACILITY_SECURITY_CORE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(232u32);
pub const FACILITY_SHARED_VHDX: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(92u32);
pub const FACILITY_SMB: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(93u32);
pub const FACILITY_SPACES: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(231u32);
pub const FACILITY_SXS_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(21u32);
pub const FACILITY_SYSTEM_INTEGRITY: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(233u32);
pub const FACILITY_TERMINAL_SERVER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(10u32);
pub const FACILITY_TPM: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(41u32);
pub const FACILITY_TRANSACTION: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(25u32);
pub const FACILITY_USB_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(16u32);
pub const FACILITY_VIDEO: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(27u32);
pub const FACILITY_VIRTUALIZATION: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(55u32);
pub const FACILITY_VOLMGR: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(56u32);
pub const FACILITY_VOLSNAP: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(80u32);
pub const FACILITY_VSM: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(69u32);
pub const FACILITY_WIN32K_NTGDI: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(63u32);
pub const FACILITY_WIN32K_NTUSER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(62u32);
pub const FACILITY_XVS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(94u32);
pub const FACILTIY_MUI_ERROR_CODE: u32 = 11u32;
pub const FALSE: BOOL = BOOL(0i32);
pub const FA_E_HOMEGROUP_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80270222_u32 as _);
pub const FA_E_MAX_PERSISTED_ITEMS_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x80270220_u32 as _);
pub const FDAEMON_E_CHANGEUPDATEFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80041684_u32 as _);
pub const FDAEMON_E_FATALERROR: windows_core::HRESULT = windows_core::HRESULT(0x80041682_u32 as _);
pub const FDAEMON_E_LOWRESOURCE: windows_core::HRESULT = windows_core::HRESULT(0x80041681_u32 as _);
pub const FDAEMON_E_NOWORDLIST: windows_core::HRESULT = windows_core::HRESULT(0x80041687_u32 as _);
pub const FDAEMON_E_PARTITIONDELETED: windows_core::HRESULT = windows_core::HRESULT(0x80041683_u32 as _);
pub const FDAEMON_E_TOOMANYFILTEREDBLOCKS: windows_core::HRESULT = windows_core::HRESULT(0x80041688_u32 as _);
pub const FDAEMON_E_WORDLISTCOMMITFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80041686_u32 as _);
pub const FDAEMON_W_EMPTYWORDLIST: windows_core::HRESULT = windows_core::HRESULT(0x41685_u32 as _);
pub const FDAEMON_W_WORDLISTFULL: windows_core::HRESULT = windows_core::HRESULT(0x41680_u32 as _);
pub const FILTER_E_ALREADY_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80041736_u32 as _);
pub const FILTER_E_CONTENTINDEXCORRUPT: windows_core::HRESULT = windows_core::HRESULT(0xC0041734_u32 as _);
pub const FILTER_E_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80041738_u32 as _);
pub const FILTER_E_NOT_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80041739_u32 as _);
pub const FILTER_E_NO_SUCH_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004173B_u32 as _);
pub const FILTER_E_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x8004173D_u32 as _);
pub const FILTER_E_PARTIALLY_FILTERED: windows_core::HRESULT = windows_core::HRESULT(0x8004173E_u32 as _);
pub const FILTER_E_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x80041730_u32 as _);
pub const FILTER_E_UNREACHABLE: windows_core::HRESULT = windows_core::HRESULT(0x80041737_u32 as _);
pub const FILTER_S_CONTENTSCAN_DELAYED: windows_core::HRESULT = windows_core::HRESULT(0x41733_u32 as _);
pub const FILTER_S_DISK_FULL: windows_core::HRESULT = windows_core::HRESULT(0x41735_u32 as _);
pub const FILTER_S_FULL_CONTENTSCAN_IMMEDIATE: windows_core::HRESULT = windows_core::HRESULT(0x41732_u32 as _);
pub const FILTER_S_NO_PROPSETS: windows_core::HRESULT = windows_core::HRESULT(0x4173A_u32 as _);
pub const FILTER_S_NO_SECURITY_DESCRIPTOR: windows_core::HRESULT = windows_core::HRESULT(0x4173C_u32 as _);
pub const FILTER_S_PARTIAL_CONTENTSCAN_IMMEDIATE: windows_core::HRESULT = windows_core::HRESULT(0x41731_u32 as _);
pub const FRS_ERR_AUTHENTICATION: i32 = 8008i32;
pub const FRS_ERR_CHILD_TO_PARENT_COMM: i32 = 8011i32;
pub const FRS_ERR_INSUFFICIENT_PRIV: i32 = 8007i32;
pub const FRS_ERR_INTERNAL: i32 = 8005i32;
pub const FRS_ERR_INTERNAL_API: i32 = 8004i32;
pub const FRS_ERR_INVALID_API_SEQUENCE: i32 = 8001i32;
pub const FRS_ERR_INVALID_SERVICE_PARAMETER: i32 = 8017i32;
pub const FRS_ERR_PARENT_AUTHENTICATION: i32 = 8010i32;
pub const FRS_ERR_PARENT_INSUFFICIENT_PRIV: i32 = 8009i32;
pub const FRS_ERR_PARENT_TO_CHILD_COMM: i32 = 8012i32;
pub const FRS_ERR_SERVICE_COMM: i32 = 8006i32;
pub const FRS_ERR_STARTING_SERVICE: i32 = 8002i32;
pub const FRS_ERR_STOPPING_SERVICE: i32 = 8003i32;
pub const FRS_ERR_SYSVOL_DEMOTE: i32 = 8016i32;
pub const FRS_ERR_SYSVOL_IS_BUSY: i32 = 8015i32;
pub const FRS_ERR_SYSVOL_POPULATE: i32 = 8013i32;
pub const FRS_ERR_SYSVOL_POPULATE_TIMEOUT: i32 = 8014i32;
pub const FVE_E_AAD_ENDPOINT_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x803100E1_u32 as _);
pub const FVE_E_AAD_SERVER_FAIL_BACKOFF: windows_core::HRESULT = windows_core::HRESULT(0x803100EA_u32 as _);
pub const FVE_E_AAD_SERVER_FAIL_RETRY_AFTER: windows_core::HRESULT = windows_core::HRESULT(0x803100E9_u32 as _);
pub const FVE_E_ACTION_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310009_u32 as _);
pub const FVE_E_ADBACKUP_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x803100D5_u32 as _);
pub const FVE_E_AD_ATTR_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x8031000E_u32 as _);
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_FIXED_DRIVE: windows_core::HRESULT = windows_core::HRESULT(0x803100DB_u32 as _);
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_OS_DRIVE: windows_core::HRESULT = windows_core::HRESULT(0x803100DA_u32 as _);
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_REMOVABLE_DRIVE: windows_core::HRESULT = windows_core::HRESULT(0x803100DC_u32 as _);
pub const FVE_E_AD_GUID_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8031000F_u32 as _);
pub const FVE_E_AD_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x8031001A_u32 as _);
pub const FVE_E_AD_INVALID_DATASIZE: windows_core::HRESULT = windows_core::HRESULT(0x8031000C_u32 as _);
pub const FVE_E_AD_INVALID_DATATYPE: windows_core::HRESULT = windows_core::HRESULT(0x8031000B_u32 as _);
pub const FVE_E_AD_NO_VALUES: windows_core::HRESULT = windows_core::HRESULT(0x8031000D_u32 as _);
pub const FVE_E_AD_SCHEMA_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x8031000A_u32 as _);
pub const FVE_E_AUTH_INVALID_APPLICATION: windows_core::HRESULT = windows_core::HRESULT(0x80310044_u32 as _);
pub const FVE_E_AUTH_INVALID_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80310045_u32 as _);
pub const FVE_E_AUTOUNLOCK_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80310029_u32 as _);
pub const FVE_E_BAD_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80310016_u32 as _);
pub const FVE_E_BAD_INFORMATION: windows_core::HRESULT = windows_core::HRESULT(0x80310010_u32 as _);
pub const FVE_E_BAD_PARTITION_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80310014_u32 as _);
pub const FVE_E_BCD_APPLICATIONS_PATH_INCORRECT: windows_core::HRESULT = windows_core::HRESULT(0x80310052_u32 as _);
pub const FVE_E_BOOTABLE_CDDVD: windows_core::HRESULT = windows_core::HRESULT(0x80310030_u32 as _);
pub const FVE_E_BUFFER_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x803100CF_u32 as _);
pub const FVE_E_CANNOT_ENCRYPT_NO_KEY: windows_core::HRESULT = windows_core::HRESULT(0x8031002E_u32 as _);
pub const FVE_E_CANNOT_SET_FVEK_ENCRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x8031002D_u32 as _);
pub const FVE_E_CANT_LOCK_AUTOUNLOCK_ENABLED_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310097_u32 as _);
pub const FVE_E_CLUSTERING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8031001E_u32 as _);
pub const FVE_E_CONV_READ: windows_core::HRESULT = windows_core::HRESULT(0x8031001B_u32 as _);
pub const FVE_E_CONV_RECOVERY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80310088_u32 as _);
pub const FVE_E_CONV_WRITE: windows_core::HRESULT = windows_core::HRESULT(0x8031001C_u32 as _);
pub const FVE_E_DATASET_FULL: windows_core::HRESULT = windows_core::HRESULT(0x803100EB_u32 as _);
pub const FVE_E_DEBUGGER_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x8031004F_u32 as _);
pub const FVE_E_DEVICELOCKOUT_COUNTER_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x803100CE_u32 as _);
pub const FVE_E_DEVICE_LOCKOUT_COUNTER_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803100CD_u32 as _);
pub const FVE_E_DEVICE_NOT_JOINED: windows_core::HRESULT = windows_core::HRESULT(0x803100E0_u32 as _);
pub const FVE_E_DE_DEVICE_LOCKEDOUT: windows_core::HRESULT = windows_core::HRESULT(0x803100CA_u32 as _);
pub const FVE_E_DE_FIXED_DATA_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803100C5_u32 as _);
pub const FVE_E_DE_HARDWARE_NOT_COMPLIANT: windows_core::HRESULT = windows_core::HRESULT(0x803100C6_u32 as _);
pub const FVE_E_DE_OS_VOLUME_NOT_PROTECTED: windows_core::HRESULT = windows_core::HRESULT(0x803100C9_u32 as _);
pub const FVE_E_DE_PREVENTED_FOR_OS: windows_core::HRESULT = windows_core::HRESULT(0x803100D1_u32 as _);
pub const FVE_E_DE_PROTECTION_NOT_YET_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x803100CB_u32 as _);
pub const FVE_E_DE_PROTECTION_SUSPENDED: windows_core::HRESULT = windows_core::HRESULT(0x803100C8_u32 as _);
pub const FVE_E_DE_VOLUME_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803100D3_u32 as _);
pub const FVE_E_DE_VOLUME_OPTED_OUT: windows_core::HRESULT = windows_core::HRESULT(0x803100D2_u32 as _);
pub const FVE_E_DE_WINRE_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x803100C7_u32 as _);
pub const FVE_E_DRY_RUN_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8031004D_u32 as _);
pub const FVE_E_DV_NOT_ALLOWED_BY_GP: windows_core::HRESULT = windows_core::HRESULT(0x80310071_u32 as _);
pub const FVE_E_DV_NOT_SUPPORTED_ON_FS: windows_core::HRESULT = windows_core::HRESULT(0x80310070_u32 as _);
pub const FVE_E_EDRIVE_BAND_ENUMERATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x803100E3_u32 as _);
pub const FVE_E_EDRIVE_BAND_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x803100B0_u32 as _);
pub const FVE_E_EDRIVE_DISALLOWED_BY_GP: windows_core::HRESULT = windows_core::HRESULT(0x803100B1_u32 as _);
pub const FVE_E_EDRIVE_DRY_RUN_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x803100BC_u32 as _);
pub const FVE_E_EDRIVE_DV_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803100B4_u32 as _);
pub const FVE_E_EDRIVE_INCOMPATIBLE_FIRMWARE: windows_core::HRESULT = windows_core::HRESULT(0x803100BF_u32 as _);
pub const FVE_E_EDRIVE_INCOMPATIBLE_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x803100B2_u32 as _);
pub const FVE_E_EDRIVE_NO_FAILOVER_TO_SW: windows_core::HRESULT = windows_core::HRESULT(0x803100AF_u32 as _);
pub const FVE_E_EFI_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x8031009C_u32 as _);
pub const FVE_E_ENH_PIN_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80310099_u32 as _);
pub const FVE_E_EOW_NOT_SUPPORTED_IN_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x803100D4_u32 as _);
pub const FVE_E_EXECUTE_REQUEST_SENT_TOO_SOON: windows_core::HRESULT = windows_core::HRESULT(0x803100DE_u32 as _);
pub const FVE_E_FAILED_AUTHENTICATION: windows_core::HRESULT = windows_core::HRESULT(0x80310027_u32 as _);
pub const FVE_E_FAILED_SECTOR_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80310026_u32 as _);
pub const FVE_E_FAILED_WRONG_FS: windows_core::HRESULT = windows_core::HRESULT(0x80310013_u32 as _);
pub const FVE_E_FIPS_DISABLE_PROTECTION_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310046_u32 as _);
pub const FVE_E_FIPS_HASH_KDF_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310098_u32 as _);
pub const FVE_E_FIPS_PREVENTS_EXTERNAL_KEY_EXPORT: windows_core::HRESULT = windows_core::HRESULT(0x80310038_u32 as _);
pub const FVE_E_FIPS_PREVENTS_PASSPHRASE: windows_core::HRESULT = windows_core::HRESULT(0x8031006C_u32 as _);
pub const FVE_E_FIPS_PREVENTS_RECOVERY_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80310037_u32 as _);
pub const FVE_E_FIPS_RNG_CHECK_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80310036_u32 as _);
pub const FVE_E_FIRMWARE_TYPE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80310048_u32 as _);
pub const FVE_E_FOREIGN_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310023_u32 as _);
pub const FVE_E_FS_MOUNTED: windows_core::HRESULT = windows_core::HRESULT(0x8031004B_u32 as _);
pub const FVE_E_FS_NOT_EXTENDED: windows_core::HRESULT = windows_core::HRESULT(0x80310047_u32 as _);
pub const FVE_E_FULL_ENCRYPTION_NOT_ALLOWED_ON_TP_STORAGE: windows_core::HRESULT = windows_core::HRESULT(0x803100A5_u32 as _);
pub const FVE_E_HIDDEN_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310056_u32 as _);
pub const FVE_E_INVALID_BITLOCKER_OID: windows_core::HRESULT = windows_core::HRESULT(0x8031006E_u32 as _);
pub const FVE_E_INVALID_DATUM_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8031009B_u32 as _);
pub const FVE_E_INVALID_KEY_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80310034_u32 as _);
pub const FVE_E_INVALID_NBP_CERT: windows_core::HRESULT = windows_core::HRESULT(0x803100E2_u32 as _);
pub const FVE_E_INVALID_NKP_CERT: windows_core::HRESULT = windows_core::HRESULT(0x8031009F_u32 as _);
pub const FVE_E_INVALID_PASSWORD_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80310035_u32 as _);
pub const FVE_E_INVALID_PIN_CHARS: windows_core::HRESULT = windows_core::HRESULT(0x8031009A_u32 as _);
pub const FVE_E_INVALID_PIN_CHARS_DETAILED: windows_core::HRESULT = windows_core::HRESULT(0x803100CC_u32 as _);
pub const FVE_E_INVALID_PROTECTOR_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8031003A_u32 as _);
pub const FVE_E_INVALID_STARTUP_OPTIONS: windows_core::HRESULT = windows_core::HRESULT(0x8031005B_u32 as _);
pub const FVE_E_KEYFILE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x8031003D_u32 as _);
pub const FVE_E_KEYFILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8031003C_u32 as _);
pub const FVE_E_KEYFILE_NO_VMK: windows_core::HRESULT = windows_core::HRESULT(0x8031003E_u32 as _);
pub const FVE_E_KEY_LENGTH_NOT_SUPPORTED_BY_EDRIVE: windows_core::HRESULT = windows_core::HRESULT(0x803100A7_u32 as _);
pub const FVE_E_KEY_PROTECTOR_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80310069_u32 as _);
pub const FVE_E_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8031001D_u32 as _);
pub const FVE_E_KEY_ROTATION_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x803100DF_u32 as _);
pub const FVE_E_KEY_ROTATION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803100DD_u32 as _);
pub const FVE_E_LIVEID_ACCOUNT_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x803100C3_u32 as _);
pub const FVE_E_LIVEID_ACCOUNT_SUSPENDED: windows_core::HRESULT = windows_core::HRESULT(0x803100C2_u32 as _);
pub const FVE_E_LOCKED_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310000_u32 as _);
pub const FVE_E_METADATA_FULL: windows_core::HRESULT = windows_core::HRESULT(0x803100EC_u32 as _);
pub const FVE_E_MOR_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80310055_u32 as _);
pub const FVE_E_MULTIPLE_NKP_CERTS: windows_core::HRESULT = windows_core::HRESULT(0x8031009D_u32 as _);
pub const FVE_E_NON_BITLOCKER_KU: windows_core::HRESULT = windows_core::HRESULT(0x80310093_u32 as _);
pub const FVE_E_NON_BITLOCKER_OID: windows_core::HRESULT = windows_core::HRESULT(0x80310085_u32 as _);
pub const FVE_E_NOT_ACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0x80310008_u32 as _);
pub const FVE_E_NOT_ALLOWED_IN_SAFE_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80310040_u32 as _);
pub const FVE_E_NOT_ALLOWED_IN_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80310053_u32 as _);
pub const FVE_E_NOT_ALLOWED_ON_CLUSTER: windows_core::HRESULT = windows_core::HRESULT(0x803100AE_u32 as _);
pub const FVE_E_NOT_ALLOWED_ON_CSV_STACK: windows_core::HRESULT = windows_core::HRESULT(0x803100AD_u32 as _);
pub const FVE_E_NOT_ALLOWED_TO_UPGRADE_WHILE_CONVERTING: windows_core::HRESULT = windows_core::HRESULT(0x803100B3_u32 as _);
pub const FVE_E_NOT_DATA_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310019_u32 as _);
pub const FVE_E_NOT_DECRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x80310039_u32 as _);
pub const FVE_E_NOT_DE_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x803100D7_u32 as _);
pub const FVE_E_NOT_ENCRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x80310001_u32 as _);
pub const FVE_E_NOT_ON_STACK: windows_core::HRESULT = windows_core::HRESULT(0x8031004A_u32 as _);
pub const FVE_E_NOT_OS_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310028_u32 as _);
pub const FVE_E_NOT_PROVISIONED_ON_ALL_VOLUMES: windows_core::HRESULT = windows_core::HRESULT(0x803100C4_u32 as _);
pub const FVE_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80310015_u32 as _);
pub const FVE_E_NO_AUTOUNLOCK_MASTER_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80310054_u32 as _);
pub const FVE_E_NO_BOOTMGR_METRIC: windows_core::HRESULT = windows_core::HRESULT(0x80310005_u32 as _);
pub const FVE_E_NO_BOOTSECTOR_METRIC: windows_core::HRESULT = windows_core::HRESULT(0x80310004_u32 as _);
pub const FVE_E_NO_EXISTING_PASSPHRASE: windows_core::HRESULT = windows_core::HRESULT(0x803100A8_u32 as _);
pub const FVE_E_NO_EXISTING_PIN: windows_core::HRESULT = windows_core::HRESULT(0x803100A0_u32 as _);
pub const FVE_E_NO_FEATURE_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0x8031005A_u32 as _);
pub const FVE_E_NO_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0x80310049_u32 as _);
pub const FVE_E_NO_MBR_METRIC: windows_core::HRESULT = windows_core::HRESULT(0x80310003_u32 as _);
pub const FVE_E_NO_PASSPHRASE_WITH_TPM: windows_core::HRESULT = windows_core::HRESULT(0x803100AB_u32 as _);
pub const FVE_E_NO_PREBOOT_KEYBOARD_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x803100B5_u32 as _);
pub const FVE_E_NO_PREBOOT_KEYBOARD_OR_WINRE_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x803100B6_u32 as _);
pub const FVE_E_NO_PROTECTORS_TO_TEST: windows_core::HRESULT = windows_core::HRESULT(0x8031003B_u32 as _);
pub const FVE_E_NO_SUCH_CAPABILITY_ON_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x803100D0_u32 as _);
pub const FVE_E_NO_TPM_BIOS: windows_core::HRESULT = windows_core::HRESULT(0x80310002_u32 as _);
pub const FVE_E_NO_TPM_WITH_PASSPHRASE: windows_core::HRESULT = windows_core::HRESULT(0x803100AC_u32 as _);
pub const FVE_E_OPERATION_NOT_SUPPORTED_ON_VISTA_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310096_u32 as _);
pub const FVE_E_OSV_KSR_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x803100D9_u32 as _);
pub const FVE_E_OS_NOT_PROTECTED: windows_core::HRESULT = windows_core::HRESULT(0x80310020_u32 as _);
pub const FVE_E_OS_VOLUME_PASSPHRASE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8031006D_u32 as _);
pub const FVE_E_OVERLAPPED_UPDATE: windows_core::HRESULT = windows_core::HRESULT(0x80310024_u32 as _);
pub const FVE_E_PASSPHRASE_PROTECTOR_CHANGE_BY_STD_USER_DISALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x803100C1_u32 as _);
pub const FVE_E_PASSPHRASE_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x803100AA_u32 as _);
pub const FVE_E_PIN_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80310043_u32 as _);
pub const FVE_E_PIN_PROTECTOR_CHANGE_BY_STD_USER_DISALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x803100A2_u32 as _);
pub const FVE_E_POLICY_CONFLICT_FDV_RK_OFF_AUK_ON: windows_core::HRESULT = windows_core::HRESULT(0x80310083_u32 as _);
pub const FVE_E_POLICY_CONFLICT_FDV_RP_OFF_ADB_ON: windows_core::HRESULT = windows_core::HRESULT(0x80310091_u32 as _);
pub const FVE_E_POLICY_CONFLICT_OSV_RP_OFF_ADB_ON: windows_core::HRESULT = windows_core::HRESULT(0x80310090_u32 as _);
pub const FVE_E_POLICY_CONFLICT_RDV_RK_OFF_AUK_ON: windows_core::HRESULT = windows_core::HRESULT(0x80310084_u32 as _);
pub const FVE_E_POLICY_CONFLICT_RDV_RP_OFF_ADB_ON: windows_core::HRESULT = windows_core::HRESULT(0x80310092_u32 as _);
pub const FVE_E_POLICY_CONFLICT_RO_AND_STARTUP_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310087_u32 as _);
pub const FVE_E_POLICY_INVALID_ENHANCED_BCD_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x803100BE_u32 as _);
pub const FVE_E_POLICY_INVALID_PASSPHRASE_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x80310080_u32 as _);
pub const FVE_E_POLICY_INVALID_PIN_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x80310068_u32 as _);
pub const FVE_E_POLICY_ON_RDV_EXCLUSION_LIST: windows_core::HRESULT = windows_core::HRESULT(0x803100E4_u32 as _);
pub const FVE_E_POLICY_PASSPHRASE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8031006A_u32 as _);
pub const FVE_E_POLICY_PASSPHRASE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8031006B_u32 as _);
pub const FVE_E_POLICY_PASSPHRASE_REQUIRES_ASCII: windows_core::HRESULT = windows_core::HRESULT(0x803100A4_u32 as _);
pub const FVE_E_POLICY_PASSPHRASE_TOO_SIMPLE: windows_core::HRESULT = windows_core::HRESULT(0x80310081_u32 as _);
pub const FVE_E_POLICY_PASSWORD_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8031002C_u32 as _);
pub const FVE_E_POLICY_PROHIBITS_SELFSIGNED: windows_core::HRESULT = windows_core::HRESULT(0x80310086_u32 as _);
pub const FVE_E_POLICY_RECOVERY_KEY_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8031005E_u32 as _);
pub const FVE_E_POLICY_RECOVERY_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8031005F_u32 as _);
pub const FVE_E_POLICY_RECOVERY_PASSWORD_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8031005C_u32 as _);
pub const FVE_E_POLICY_RECOVERY_PASSWORD_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8031005D_u32 as _);
pub const FVE_E_POLICY_REQUIRES_RECOVERY_PASSWORD_ON_TOUCH_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x803100B8_u32 as _);
pub const FVE_E_POLICY_REQUIRES_STARTUP_PIN_ON_TOUCH_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x803100B7_u32 as _);
pub const FVE_E_POLICY_STARTUP_KEY_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310062_u32 as _);
pub const FVE_E_POLICY_STARTUP_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310063_u32 as _);
pub const FVE_E_POLICY_STARTUP_PIN_KEY_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310064_u32 as _);
pub const FVE_E_POLICY_STARTUP_PIN_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310065_u32 as _);
pub const FVE_E_POLICY_STARTUP_PIN_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310060_u32 as _);
pub const FVE_E_POLICY_STARTUP_PIN_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310061_u32 as _);
pub const FVE_E_POLICY_STARTUP_TPM_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310066_u32 as _);
pub const FVE_E_POLICY_STARTUP_TPM_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310067_u32 as _);
pub const FVE_E_POLICY_USER_CERTIFICATE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310072_u32 as _);
pub const FVE_E_POLICY_USER_CERTIFICATE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310073_u32 as _);
pub const FVE_E_POLICY_USER_CERT_MUST_BE_HW: windows_core::HRESULT = windows_core::HRESULT(0x80310074_u32 as _);
pub const FVE_E_POLICY_USER_CONFIGURE_FDV_AUTOUNLOCK_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310075_u32 as _);
pub const FVE_E_POLICY_USER_CONFIGURE_RDV_AUTOUNLOCK_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310076_u32 as _);
pub const FVE_E_POLICY_USER_CONFIGURE_RDV_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310077_u32 as _);
pub const FVE_E_POLICY_USER_DISABLE_RDV_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310079_u32 as _);
pub const FVE_E_POLICY_USER_ENABLE_RDV_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310078_u32 as _);
pub const FVE_E_PREDICTED_TPM_PROTECTOR_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803100E5_u32 as _);
pub const FVE_E_PRIVATEKEY_AUTH_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80310094_u32 as _);
pub const FVE_E_PROTECTION_CANNOT_BE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803100D8_u32 as _);
pub const FVE_E_PROTECTION_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80310021_u32 as _);
pub const FVE_E_PROTECTOR_CHANGE_MAX_PASSPHRASE_CHANGE_ATTEMPTS_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x803100C0_u32 as _);
pub const FVE_E_PROTECTOR_CHANGE_MAX_PIN_CHANGE_ATTEMPTS_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x803100A3_u32 as _);
pub const FVE_E_PROTECTOR_CHANGE_PASSPHRASE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x803100A9_u32 as _);
pub const FVE_E_PROTECTOR_CHANGE_PIN_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x803100A1_u32 as _);
pub const FVE_E_PROTECTOR_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80310031_u32 as _);
pub const FVE_E_PROTECTOR_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80310033_u32 as _);
pub const FVE_E_PUBKEY_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80310058_u32 as _);
pub const FVE_E_RAW_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x80310050_u32 as _);
pub const FVE_E_RAW_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80310051_u32 as _);
pub const FVE_E_REBOOT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8031004E_u32 as _);
pub const FVE_E_RECOVERY_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310022_u32 as _);
pub const FVE_E_RECOVERY_PARTITION: windows_core::HRESULT = windows_core::HRESULT(0x80310082_u32 as _);
pub const FVE_E_RELATIVE_PATH: windows_core::HRESULT = windows_core::HRESULT(0x80310032_u32 as _);
pub const FVE_E_REMOVAL_OF_DRA_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80310095_u32 as _);
pub const FVE_E_REMOVAL_OF_NKP_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8031009E_u32 as _);
pub const FVE_E_SECUREBOOT_CONFIGURATION_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x803100BB_u32 as _);
pub const FVE_E_SECUREBOOT_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803100BA_u32 as _);
pub const FVE_E_SECURE_KEY_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80310007_u32 as _);
pub const FVE_E_SETUP_TPM_CALLBACK_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803100E6_u32 as _);
pub const FVE_E_SHADOW_COPY_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x803100BD_u32 as _);
pub const FVE_E_SYSTEM_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80310012_u32 as _);
pub const FVE_E_TOKEN_NOT_IMPERSONATED: windows_core::HRESULT = windows_core::HRESULT(0x8031004C_u32 as _);
pub const FVE_E_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80310011_u32 as _);
pub const FVE_E_TPM_CONTEXT_SETUP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803100E7_u32 as _);
pub const FVE_E_TPM_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8031003F_u32 as _);
pub const FVE_E_TPM_INVALID_PCR: windows_core::HRESULT = windows_core::HRESULT(0x80310041_u32 as _);
pub const FVE_E_TPM_NOT_OWNED: windows_core::HRESULT = windows_core::HRESULT(0x80310018_u32 as _);
pub const FVE_E_TPM_NO_VMK: windows_core::HRESULT = windows_core::HRESULT(0x80310042_u32 as _);
pub const FVE_E_TPM_SRK_AUTH_NOT_ZERO: windows_core::HRESULT = windows_core::HRESULT(0x80310025_u32 as _);
pub const FVE_E_TRANSIENT_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80310057_u32 as _);
pub const FVE_E_UPDATE_INVALID_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x803100E8_u32 as _);
pub const FVE_E_VIRTUALIZED_SPACE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x80310089_u32 as _);
pub const FVE_E_VOLUME_BOUND_ALREADY: windows_core::HRESULT = windows_core::HRESULT(0x8031001F_u32 as _);
pub const FVE_E_VOLUME_EXTEND_PREVENTS_EOW_DECRYPT: windows_core::HRESULT = windows_core::HRESULT(0x803100D6_u32 as _);
pub const FVE_E_VOLUME_HANDLE_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80310059_u32 as _);
pub const FVE_E_VOLUME_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x80310017_u32 as _);
pub const FVE_E_VOLUME_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x8031006F_u32 as _);
pub const FVE_E_WIPE_CANCEL_NOT_APPLICABLE: windows_core::HRESULT = windows_core::HRESULT(0x803100B9_u32 as _);
pub const FVE_E_WIPE_NOT_ALLOWED_ON_TP_STORAGE: windows_core::HRESULT = windows_core::HRESULT(0x803100A6_u32 as _);
pub const FVE_E_WRONG_BOOTMGR: windows_core::HRESULT = windows_core::HRESULT(0x80310006_u32 as _);
pub const FVE_E_WRONG_BOOTSECTOR: windows_core::HRESULT = windows_core::HRESULT(0x8031002A_u32 as _);
pub const FVE_E_WRONG_SYSTEM_FS: windows_core::HRESULT = windows_core::HRESULT(0x8031002B_u32 as _);
pub const FWP_E_ACTION_INCOMPATIBLE_WITH_LAYER: windows_core::HRESULT = windows_core::HRESULT(0x8032002C_u32 as _);
pub const FWP_E_ACTION_INCOMPATIBLE_WITH_SUBLAYER: windows_core::HRESULT = windows_core::HRESULT(0x8032002D_u32 as _);
pub const FWP_E_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80320009_u32 as _);
pub const FWP_E_BUILTIN_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80320017_u32 as _);
pub const FWP_E_CALLOUT_NOTIFICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80320037_u32 as _);
pub const FWP_E_CALLOUT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320001_u32 as _);
pub const FWP_E_CONDITION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320002_u32 as _);
pub const FWP_E_CONNECTIONS_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80320041_u32 as _);
pub const FWP_E_CONTEXT_INCOMPATIBLE_WITH_CALLOUT: windows_core::HRESULT = windows_core::HRESULT(0x8032002F_u32 as _);
pub const FWP_E_CONTEXT_INCOMPATIBLE_WITH_LAYER: windows_core::HRESULT = windows_core::HRESULT(0x8032002E_u32 as _);
pub const FWP_E_DROP_NOICMP: windows_core::HRESULT = windows_core::HRESULT(0x80320104_u32 as _);
pub const FWP_E_DUPLICATE_AUTH_METHOD: windows_core::HRESULT = windows_core::HRESULT(0x8032003C_u32 as _);
pub const FWP_E_DUPLICATE_CONDITION: windows_core::HRESULT = windows_core::HRESULT(0x8032002A_u32 as _);
pub const FWP_E_DUPLICATE_KEYMOD: windows_core::HRESULT = windows_core::HRESULT(0x8032002B_u32 as _);
pub const FWP_E_DYNAMIC_SESSION_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8032000B_u32 as _);
pub const FWP_E_EM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80320032_u32 as _);
pub const FWP_E_FILTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320003_u32 as _);
pub const FWP_E_IKEEXT_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80320044_u32 as _);
pub const FWP_E_INCOMPATIBLE_AUTH_METHOD: windows_core::HRESULT = windows_core::HRESULT(0x80320030_u32 as _);
pub const FWP_E_INCOMPATIBLE_CIPHER_TRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x8032003A_u32 as _);
pub const FWP_E_INCOMPATIBLE_DH_GROUP: windows_core::HRESULT = windows_core::HRESULT(0x80320031_u32 as _);
pub const FWP_E_INCOMPATIBLE_LAYER: windows_core::HRESULT = windows_core::HRESULT(0x80320014_u32 as _);
pub const FWP_E_INCOMPATIBLE_SA_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8032001B_u32 as _);
pub const FWP_E_INCOMPATIBLE_TXN: windows_core::HRESULT = windows_core::HRESULT(0x80320011_u32 as _);
pub const FWP_E_INVALID_ACTION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80320024_u32 as _);
pub const FWP_E_INVALID_AUTH_TRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x80320038_u32 as _);
pub const FWP_E_INVALID_CIPHER_TRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x80320039_u32 as _);
pub const FWP_E_INVALID_DNS_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80320042_u32 as _);
pub const FWP_E_INVALID_ENUMERATOR: windows_core::HRESULT = windows_core::HRESULT(0x8032001D_u32 as _);
pub const FWP_E_INVALID_FLAGS: windows_core::HRESULT = windows_core::HRESULT(0x8032001E_u32 as _);
pub const FWP_E_INVALID_INTERVAL: windows_core::HRESULT = windows_core::HRESULT(0x80320021_u32 as _);
pub const FWP_E_INVALID_NET_MASK: windows_core::HRESULT = windows_core::HRESULT(0x8032001F_u32 as _);
pub const FWP_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80320035_u32 as _);
pub const FWP_E_INVALID_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80320020_u32 as _);
pub const FWP_E_INVALID_TRANSFORM_COMBINATION: windows_core::HRESULT = windows_core::HRESULT(0x8032003B_u32 as _);
pub const FWP_E_INVALID_TUNNEL_ENDPOINT: windows_core::HRESULT = windows_core::HRESULT(0x8032003D_u32 as _);
pub const FWP_E_INVALID_WEIGHT: windows_core::HRESULT = windows_core::HRESULT(0x80320025_u32 as _);
pub const FWP_E_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8032000A_u32 as _);
pub const FWP_E_KEY_DICTATION_INVALID_KEYING_MATERIAL: windows_core::HRESULT = windows_core::HRESULT(0x80320040_u32 as _);
pub const FWP_E_KEY_DICTATOR_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8032003F_u32 as _);
pub const FWP_E_KM_CLIENTS_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80320015_u32 as _);
pub const FWP_E_L2_DRIVER_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x8032003E_u32 as _);
pub const FWP_E_LAYER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320004_u32 as _);
pub const FWP_E_LIFETIME_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80320016_u32 as _);
pub const FWP_E_MATCH_TYPE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80320026_u32 as _);
pub const FWP_E_NET_EVENTS_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80320013_u32 as _);
pub const FWP_E_NEVER_MATCH: windows_core::HRESULT = windows_core::HRESULT(0x80320033_u32 as _);
pub const FWP_E_NOTIFICATION_DROPPED: windows_core::HRESULT = windows_core::HRESULT(0x80320019_u32 as _);
pub const FWP_E_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320008_u32 as _);
pub const FWP_E_NO_TXN_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8032000D_u32 as _);
pub const FWP_E_NULL_DISPLAY_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80320023_u32 as _);
pub const FWP_E_NULL_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x8032001C_u32 as _);
pub const FWP_E_OUT_OF_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x80320028_u32 as _);
pub const FWP_E_PROVIDER_CONTEXT_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80320034_u32 as _);
pub const FWP_E_PROVIDER_CONTEXT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320006_u32 as _);
pub const FWP_E_PROVIDER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320005_u32 as _);
pub const FWP_E_RESERVED: windows_core::HRESULT = windows_core::HRESULT(0x80320029_u32 as _);
pub const FWP_E_SESSION_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x80320010_u32 as _);
pub const FWP_E_STILL_ON: windows_core::HRESULT = windows_core::HRESULT(0x80320043_u32 as _);
pub const FWP_E_SUBLAYER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80320007_u32 as _);
pub const FWP_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80320012_u32 as _);
pub const FWP_E_TOO_MANY_CALLOUTS: windows_core::HRESULT = windows_core::HRESULT(0x80320018_u32 as _);
pub const FWP_E_TOO_MANY_SUBLAYERS: windows_core::HRESULT = windows_core::HRESULT(0x80320036_u32 as _);
pub const FWP_E_TRAFFIC_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8032001A_u32 as _);
pub const FWP_E_TXN_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x8032000F_u32 as _);
pub const FWP_E_TXN_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8032000E_u32 as _);
pub const FWP_E_TYPE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80320027_u32 as _);
pub const FWP_E_WRONG_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x8032000C_u32 as _);
pub const FWP_E_ZERO_LENGTH_ARRAY: windows_core::HRESULT = windows_core::HRESULT(0x80320022_u32 as _);
pub const GCN_E_DEFAULTNAMESPACE_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x803B0029_u32 as _);
pub const GCN_E_MODULE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0021_u32 as _);
pub const GCN_E_NETADAPTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0026_u32 as _);
pub const GCN_E_NETADAPTER_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x803B0025_u32 as _);
pub const GCN_E_NETCOMPARTMENT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0027_u32 as _);
pub const GCN_E_NETINTERFACE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0028_u32 as _);
pub const GCN_E_NO_REQUEST_HANDLERS: windows_core::HRESULT = windows_core::HRESULT(0x803B0022_u32 as _);
pub const GCN_E_REQUEST_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803B0023_u32 as _);
pub const GCN_E_RUNTIMEKEYS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x803B0024_u32 as _);
pub const GENERIC_ALL: GENERIC_ACCESS_RIGHTS = GENERIC_ACCESS_RIGHTS(268435456u32);
pub const GENERIC_EXECUTE: GENERIC_ACCESS_RIGHTS = GENERIC_ACCESS_RIGHTS(536870912u32);
pub const GENERIC_READ: GENERIC_ACCESS_RIGHTS = GENERIC_ACCESS_RIGHTS(2147483648u32);
pub const GENERIC_WRITE: GENERIC_ACCESS_RIGHTS = GENERIC_ACCESS_RIGHTS(1073741824u32);
pub const HANDLE_FLAG_INHERIT: HANDLE_FLAGS = HANDLE_FLAGS(1u32);
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: HANDLE_FLAGS = HANDLE_FLAGS(2u32);
pub const HCN_E_ADAPTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0006_u32 as _);
pub const HCN_E_ADDR_INVALID_OR_RESERVED: windows_core::HRESULT = windows_core::HRESULT(0x803B002F_u32 as _);
pub const HCN_E_DEGRADED_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x803B0017_u32 as _);
pub const HCN_E_ENDPOINT_ALREADY_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x803B0014_u32 as _);
pub const HCN_E_ENDPOINT_NAMESPACE_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x803B002B_u32 as _);
pub const HCN_E_ENDPOINT_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x803B0034_u32 as _);
pub const HCN_E_ENDPOINT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0002_u32 as _);
pub const HCN_E_ENDPOINT_NOT_LOCAL: windows_core::HRESULT = windows_core::HRESULT(0x803B0035_u32 as _);
pub const HCN_E_ENDPOINT_SHARING_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803B001D_u32 as _);
pub const HCN_E_ENTITY_HAS_REFERENCES: windows_core::HRESULT = windows_core::HRESULT(0x803B002C_u32 as _);
pub const HCN_E_GUID_CONVERSION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x803B0019_u32 as _);
pub const HCN_E_ICS_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803B002A_u32 as _);
pub const HCN_E_INVALID_ENDPOINT: windows_core::HRESULT = windows_core::HRESULT(0x803B000C_u32 as _);
pub const HCN_E_INVALID_INTERNAL_PORT: windows_core::HRESULT = windows_core::HRESULT(0x803B002D_u32 as _);
pub const HCN_E_INVALID_IP: windows_core::HRESULT = windows_core::HRESULT(0x803B001E_u32 as _);
pub const HCN_E_INVALID_IP_SUBNET: windows_core::HRESULT = windows_core::HRESULT(0x803B0033_u32 as _);
pub const HCN_E_INVALID_JSON: windows_core::HRESULT = windows_core::HRESULT(0x803B001B_u32 as _);
pub const HCN_E_INVALID_JSON_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x803B001C_u32 as _);
pub const HCN_E_INVALID_NETWORK: windows_core::HRESULT = windows_core::HRESULT(0x803B000A_u32 as _);
pub const HCN_E_INVALID_NETWORK_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x803B000B_u32 as _);
pub const HCN_E_INVALID_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x803B000D_u32 as _);
pub const HCN_E_INVALID_POLICY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x803B000E_u32 as _);
pub const HCN_E_INVALID_PREFIX: windows_core::HRESULT = windows_core::HRESULT(0x803B0030_u32 as _);
pub const HCN_E_INVALID_REMOTE_ENDPOINT_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x803B000F_u32 as _);
pub const HCN_E_INVALID_SUBNET: windows_core::HRESULT = windows_core::HRESULT(0x803B0032_u32 as _);
pub const HCN_E_LAYER_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x803B0011_u32 as _);
pub const HCN_E_LAYER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0003_u32 as _);
pub const HCN_E_MANAGER_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0x803B0020_u32 as _);
pub const HCN_E_MAPPING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803B0016_u32 as _);
pub const HCN_E_NAMESPACE_ATTACH_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x803B002E_u32 as _);
pub const HCN_E_NETWORK_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x803B0010_u32 as _);
pub const HCN_E_NETWORK_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0001_u32 as _);
pub const HCN_E_OBJECT_USED_AFTER_UNLOAD: windows_core::HRESULT = windows_core::HRESULT(0x803B0031_u32 as _);
pub const HCN_E_POLICY_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x803B0012_u32 as _);
pub const HCN_E_POLICY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0008_u32 as _);
pub const HCN_E_PORT_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x803B0013_u32 as _);
pub const HCN_E_PORT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0007_u32 as _);
pub const HCN_E_REGKEY_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x803B001A_u32 as _);
pub const HCN_E_REQUEST_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803B0015_u32 as _);
pub const HCN_E_SHARED_SWITCH_MODIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x803B0018_u32 as _);
pub const HCN_E_SUBNET_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0005_u32 as _);
pub const HCN_E_SWITCH_EXTENSION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B001F_u32 as _);
pub const HCN_E_SWITCH_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0004_u32 as _);
pub const HCN_E_VFP_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x803B0037_u32 as _);
pub const HCN_E_VFP_PORTSETTING_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803B0009_u32 as _);
pub const HCN_INTERFACEPARAMETERS_ALREADY_APPLIED: windows_core::HRESULT = windows_core::HRESULT(0x803B0036_u32 as _);
pub const HCS_E_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x8037011B_u32 as _);
pub const HCS_E_CONNECTION_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0x8037010A_u32 as _);
pub const HCS_E_CONNECTION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80370109_u32 as _);
pub const HCS_E_CONNECT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80370108_u32 as _);
pub const HCS_E_GUEST_CRITICAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8037011C_u32 as _);
pub const HCS_E_HYPERV_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80370102_u32 as _);
pub const HCS_E_IMAGE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80370101_u32 as _);
pub const HCS_E_INVALID_JSON: windows_core::HRESULT = windows_core::HRESULT(0x8037010D_u32 as _);
pub const HCS_E_INVALID_LAYER: windows_core::HRESULT = windows_core::HRESULT(0x80370112_u32 as _);
pub const HCS_E_INVALID_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80370105_u32 as _);
pub const HCS_E_OPERATION_ALREADY_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80370121_u32 as _);
pub const HCS_E_OPERATION_ALREADY_STARTED: windows_core::HRESULT = windows_core::HRESULT(0x80370116_u32 as _);
pub const HCS_E_OPERATION_NOT_STARTED: windows_core::HRESULT = windows_core::HRESULT(0x80370115_u32 as _);
pub const HCS_E_OPERATION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80370117_u32 as _);
pub const HCS_E_OPERATION_RESULT_ALLOCATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8037011A_u32 as _);
pub const HCS_E_OPERATION_SYSTEM_CALLBACK_ALREADY_SET: windows_core::HRESULT = windows_core::HRESULT(0x80370119_u32 as _);
pub const HCS_E_OPERATION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80370118_u32 as _);
pub const HCS_E_PROCESS_ALREADY_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0x8037011F_u32 as _);
pub const HCS_E_PROCESS_INFO_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8037011D_u32 as _);
pub const HCS_E_PROTOCOL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80370111_u32 as _);
pub const HCS_E_SERVICE_DISCONNECT: windows_core::HRESULT = windows_core::HRESULT(0x8037011E_u32 as _);
pub const HCS_E_SERVICE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80370114_u32 as _);
pub const HCS_E_SYSTEM_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8037010F_u32 as _);
pub const HCS_E_SYSTEM_ALREADY_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0x80370110_u32 as _);
pub const HCS_E_SYSTEM_NOT_CONFIGURED_FOR_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80370120_u32 as _);
pub const HCS_E_SYSTEM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8037010E_u32 as _);
pub const HCS_E_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x80370107_u32 as _);
pub const HCS_E_TERMINATED_DURING_START: windows_core::HRESULT = windows_core::HRESULT(0x80370100_u32 as _);
pub const HCS_E_UNEXPECTED_EXIT: windows_core::HRESULT = windows_core::HRESULT(0x80370106_u32 as _);
pub const HCS_E_UNKNOWN_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x8037010B_u32 as _);
pub const HCS_E_UNSUPPORTED_PROTOCOL_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8037010C_u32 as _);
pub const HCS_E_WINDOWS_INSIDER_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80370113_u32 as _);
pub const HSP_BASE_ERROR_MASK: windows_core::HRESULT = windows_core::HRESULT(0x81290100_u32 as _);
pub const HSP_BASE_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x812901FF_u32 as _);
pub const HSP_BS_ERROR_MASK: windows_core::HRESULT = windows_core::HRESULT(0x81281000_u32 as _);
pub const HSP_BS_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x812810FF_u32 as _);
pub const HSP_DRV_ERROR_MASK: windows_core::HRESULT = windows_core::HRESULT(0x81290000_u32 as _);
pub const HSP_DRV_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x812900FF_u32 as _);
pub const HSP_E_ERROR_MASK: windows_core::HRESULT = windows_core::HRESULT(0x81280000_u32 as _);
pub const HSP_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x81280FFF_u32 as _);
pub const HSP_KSP_ALGORITHM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x81290209_u32 as _);
pub const HSP_KSP_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x81290205_u32 as _);
pub const HSP_KSP_DEVICE_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x81290201_u32 as _);
pub const HSP_KSP_ERROR_MASK: windows_core::HRESULT = windows_core::HRESULT(0x81290200_u32 as _);
pub const HSP_KSP_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x812902FF_u32 as _);
pub const HSP_KSP_INVALID_DATA: windows_core::HRESULT = windows_core::HRESULT(0x81290207_u32 as _);
pub const HSP_KSP_INVALID_FLAGS: windows_core::HRESULT = windows_core::HRESULT(0x81290208_u32 as _);
pub const HSP_KSP_INVALID_KEY_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x81290203_u32 as _);
pub const HSP_KSP_INVALID_KEY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8129020C_u32 as _);
pub const HSP_KSP_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x81290204_u32 as _);
pub const HSP_KSP_INVALID_PROVIDER_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x81290202_u32 as _);
pub const HSP_KSP_KEY_ALREADY_FINALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8129020A_u32 as _);
pub const HSP_KSP_KEY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x81290215_u32 as _);
pub const HSP_KSP_KEY_LOAD_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x81290217_u32 as _);
pub const HSP_KSP_KEY_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x81290216_u32 as _);
pub const HSP_KSP_KEY_NOT_FINALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8129020B_u32 as _);
pub const HSP_KSP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x81290206_u32 as _);
pub const HSP_KSP_NO_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x81290210_u32 as _);
pub const HSP_KSP_NO_MORE_ITEMS: windows_core::HRESULT = windows_core::HRESULT(0x81290218_u32 as _);
pub const HSP_KSP_PARAMETER_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x81290211_u32 as _);
pub const HTTP_E_STATUS_AMBIGUOUS: windows_core::HRESULT = windows_core::HRESULT(0x8019012C_u32 as _);
pub const HTTP_E_STATUS_BAD_GATEWAY: windows_core::HRESULT = windows_core::HRESULT(0x801901F6_u32 as _);
pub const HTTP_E_STATUS_BAD_METHOD: windows_core::HRESULT = windows_core::HRESULT(0x80190195_u32 as _);
pub const HTTP_E_STATUS_BAD_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80190190_u32 as _);
pub const HTTP_E_STATUS_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x80190199_u32 as _);
pub const HTTP_E_STATUS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80190191_u32 as _);
pub const HTTP_E_STATUS_EXPECTATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x801901A1_u32 as _);
pub const HTTP_E_STATUS_FORBIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x80190193_u32 as _);
pub const HTTP_E_STATUS_GATEWAY_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x801901F8_u32 as _);
pub const HTTP_E_STATUS_GONE: windows_core::HRESULT = windows_core::HRESULT(0x8019019A_u32 as _);
pub const HTTP_E_STATUS_LENGTH_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8019019B_u32 as _);
pub const HTTP_E_STATUS_MOVED: windows_core::HRESULT = windows_core::HRESULT(0x8019012D_u32 as _);
pub const HTTP_E_STATUS_NONE_ACCEPTABLE: windows_core::HRESULT = windows_core::HRESULT(0x80190196_u32 as _);
pub const HTTP_E_STATUS_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80190194_u32 as _);
pub const HTTP_E_STATUS_NOT_MODIFIED: windows_core::HRESULT = windows_core::HRESULT(0x80190130_u32 as _);
pub const HTTP_E_STATUS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x801901F5_u32 as _);
pub const HTTP_E_STATUS_PAYMENT_REQ: windows_core::HRESULT = windows_core::HRESULT(0x80190192_u32 as _);
pub const HTTP_E_STATUS_PRECOND_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8019019C_u32 as _);
pub const HTTP_E_STATUS_PROXY_AUTH_REQ: windows_core::HRESULT = windows_core::HRESULT(0x80190197_u32 as _);
pub const HTTP_E_STATUS_RANGE_NOT_SATISFIABLE: windows_core::HRESULT = windows_core::HRESULT(0x801901A0_u32 as _);
pub const HTTP_E_STATUS_REDIRECT: windows_core::HRESULT = windows_core::HRESULT(0x8019012E_u32 as _);
pub const HTTP_E_STATUS_REDIRECT_KEEP_VERB: windows_core::HRESULT = windows_core::HRESULT(0x80190133_u32 as _);
pub const HTTP_E_STATUS_REDIRECT_METHOD: windows_core::HRESULT = windows_core::HRESULT(0x8019012F_u32 as _);
pub const HTTP_E_STATUS_REQUEST_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80190198_u32 as _);
pub const HTTP_E_STATUS_REQUEST_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x8019019D_u32 as _);
pub const HTTP_E_STATUS_SERVER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x801901F4_u32 as _);
pub const HTTP_E_STATUS_SERVICE_UNAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x801901F7_u32 as _);
pub const HTTP_E_STATUS_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80190001_u32 as _);
pub const HTTP_E_STATUS_UNEXPECTED_CLIENT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190004_u32 as _);
pub const HTTP_E_STATUS_UNEXPECTED_REDIRECTION: windows_core::HRESULT = windows_core::HRESULT(0x80190003_u32 as _);
pub const HTTP_E_STATUS_UNEXPECTED_SERVER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80190005_u32 as _);
pub const HTTP_E_STATUS_UNSUPPORTED_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x8019019F_u32 as _);
pub const HTTP_E_STATUS_URI_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x8019019E_u32 as _);
pub const HTTP_E_STATUS_USE_PROXY: windows_core::HRESULT = windows_core::HRESULT(0x80190131_u32 as _);
pub const HTTP_E_STATUS_VERSION_NOT_SUP: windows_core::HRESULT = windows_core::HRESULT(0x801901F9_u32 as _);
pub const INPLACE_E_FIRST: i32 = -2147221088i32;
pub const INPLACE_E_LAST: i32 = -2147221073i32;
pub const INPLACE_E_NOTOOLSPACE: windows_core::HRESULT = windows_core::HRESULT(0x800401A1_u32 as _);
pub const INPLACE_E_NOTUNDOABLE: windows_core::HRESULT = windows_core::HRESULT(0x800401A0_u32 as _);
pub const INPLACE_S_FIRST: i32 = 262560i32;
pub const INPLACE_S_LAST: i32 = 262575i32;
pub const INPLACE_S_TRUNCATED: windows_core::HRESULT = windows_core::HRESULT(0x401A0_u32 as _);
pub const INPUT_E_DEVICE_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80400006_u32 as _);
pub const INPUT_E_DEVICE_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80400008_u32 as _);
pub const INPUT_E_FRAME: windows_core::HRESULT = windows_core::HRESULT(0x80400004_u32 as _);
pub const INPUT_E_HISTORY: windows_core::HRESULT = windows_core::HRESULT(0x80400005_u32 as _);
pub const INPUT_E_MULTIMODAL: windows_core::HRESULT = windows_core::HRESULT(0x80400002_u32 as _);
pub const INPUT_E_OUT_OF_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x80400000_u32 as _);
pub const INPUT_E_PACKET: windows_core::HRESULT = windows_core::HRESULT(0x80400003_u32 as _);
pub const INPUT_E_REENTRANCY: windows_core::HRESULT = windows_core::HRESULT(0x80400001_u32 as _);
pub const INPUT_E_TRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x80400007_u32 as _);
pub const INVALID_HANDLE_VALUE: HANDLE = HANDLE(-1i32 as _);
pub const IORING_E_COMPLETION_QUEUE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x80460005_u32 as _);
pub const IORING_E_COMPLETION_QUEUE_TOO_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80460008_u32 as _);
pub const IORING_E_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80460007_u32 as _);
pub const IORING_E_REQUIRED_FLAG_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80460001_u32 as _);
pub const IORING_E_SUBMISSION_QUEUE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80460002_u32 as _);
pub const IORING_E_SUBMISSION_QUEUE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x80460004_u32 as _);
pub const IORING_E_SUBMIT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80460006_u32 as _);
pub const IORING_E_VERSION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80460003_u32 as _);
pub const IO_BAD_BLOCK_WITH_NAME: NTSTATUS = NTSTATUS(0xC004001F_u32 as _);
pub const IO_CDROM_EXCLUSIVE_LOCK: NTSTATUS = NTSTATUS(0x40040085_u32 as _);
pub const IO_DRIVER_CANCEL_TIMEOUT: NTSTATUS = NTSTATUS(0x80040036_u32 as _);
pub const IO_DUMP_CALLBACK_EXCEPTION: NTSTATUS = NTSTATUS(0xC00400A3_u32 as _);
pub const IO_DUMP_CREATION_SUCCESS: NTSTATUS = NTSTATUS(0x400A2_u32 as _);
pub const IO_DUMP_DIRECT_CONFIG_FAILED: NTSTATUS = NTSTATUS(0xC0040030_u32 as _);
pub const IO_DUMP_DRIVER_LOAD_FAILURE: NTSTATUS = NTSTATUS(0xC004002D_u32 as _);
pub const IO_DUMP_DUMPFILE_CONFLICT: NTSTATUS = NTSTATUS(0xC004002F_u32 as _);
pub const IO_DUMP_INITIALIZATION_FAILURE: NTSTATUS = NTSTATUS(0xC004002E_u32 as _);
pub const IO_DUMP_INIT_DEDICATED_DUMP_FAILURE: NTSTATUS = NTSTATUS(0xC00400A4_u32 as _);
pub const IO_DUMP_PAGE_CONFIG_FAILED: NTSTATUS = NTSTATUS(0xC0040031_u32 as _);
pub const IO_DUMP_POINTER_FAILURE: NTSTATUS = NTSTATUS(0xC004002C_u32 as _);
pub const IO_ERROR_DISK_RESOURCES_EXHAUSTED: NTSTATUS = NTSTATUS(0xC0040096_u32 as _);
pub const IO_ERROR_DUMP_CREATION_ERROR: NTSTATUS = NTSTATUS(0xC00400A1_u32 as _);
pub const IO_ERROR_IO_HARDWARE_ERROR: NTSTATUS = NTSTATUS(0xC004009A_u32 as _);
pub const IO_ERR_BAD_BLOCK: NTSTATUS = NTSTATUS(0xC0040007_u32 as _);
pub const IO_ERR_BAD_FIRMWARE: NTSTATUS = NTSTATUS(0xC0040019_u32 as _);
pub const IO_ERR_CONFIGURATION_ERROR: NTSTATUS = NTSTATUS(0xC0040003_u32 as _);
pub const IO_ERR_CONTROLLER_ERROR: NTSTATUS = NTSTATUS(0xC004000B_u32 as _);
pub const IO_ERR_DMA_CONFLICT_DETECTED: NTSTATUS = NTSTATUS(0xC0040017_u32 as _);
pub const IO_ERR_DMA_RESOURCE_CONFLICT: NTSTATUS = NTSTATUS(0xC004001B_u32 as _);
pub const IO_ERR_DRIVER_ERROR: NTSTATUS = NTSTATUS(0xC0040004_u32 as _);
pub const IO_ERR_INCORRECT_IRQL: NTSTATUS = NTSTATUS(0xC004000D_u32 as _);
pub const IO_ERR_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(0xC0040002_u32 as _);
pub const IO_ERR_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC004000C_u32 as _);
pub const IO_ERR_INTERRUPT_RESOURCE_CONFLICT: NTSTATUS = NTSTATUS(0xC004001C_u32 as _);
pub const IO_ERR_INVALID_IOBASE: NTSTATUS = NTSTATUS(0xC004000E_u32 as _);
pub const IO_ERR_INVALID_REQUEST: NTSTATUS = NTSTATUS(0xC0040010_u32 as _);
pub const IO_ERR_IRQ_CONFLICT_DETECTED: NTSTATUS = NTSTATUS(0xC0040018_u32 as _);
pub const IO_ERR_LAYERED_FAILURE: NTSTATUS = NTSTATUS(0xC0040012_u32 as _);
pub const IO_ERR_MEMORY_CONFLICT_DETECTED: NTSTATUS = NTSTATUS(0xC0040015_u32 as _);
pub const IO_ERR_MEMORY_RESOURCE_CONFLICT: NTSTATUS = NTSTATUS(0xC004001D_u32 as _);
pub const IO_ERR_NOT_READY: NTSTATUS = NTSTATUS(0xC004000F_u32 as _);
pub const IO_ERR_OVERRUN_ERROR: NTSTATUS = NTSTATUS(0xC0040008_u32 as _);
pub const IO_ERR_PARITY: NTSTATUS = NTSTATUS(0xC0040005_u32 as _);
pub const IO_ERR_PORT_CONFLICT_DETECTED: NTSTATUS = NTSTATUS(0xC0040016_u32 as _);
pub const IO_ERR_PORT_RESOURCE_CONFLICT: NTSTATUS = NTSTATUS(0xC004001E_u32 as _);
pub const IO_ERR_PORT_TIMEOUT: NTSTATUS = NTSTATUS(0xC0040075_u32 as _);
pub const IO_ERR_PROTOCOL: NTSTATUS = NTSTATUS(0xC0040014_u32 as _);
pub const IO_ERR_RESET: NTSTATUS = NTSTATUS(0xC0040013_u32 as _);
pub const IO_ERR_RETRY_SUCCEEDED: NTSTATUS = NTSTATUS(0x40001_u32 as _);
pub const IO_ERR_SEEK_ERROR: NTSTATUS = NTSTATUS(0xC0040006_u32 as _);
pub const IO_ERR_SEQUENCE: NTSTATUS = NTSTATUS(0xC004000A_u32 as _);
pub const IO_ERR_THREAD_STUCK_IN_DEVICE_DRIVER: NTSTATUS = NTSTATUS(0xC004006C_u32 as _);
pub const IO_ERR_TIMEOUT: NTSTATUS = NTSTATUS(0xC0040009_u32 as _);
pub const IO_ERR_VERSION: NTSTATUS = NTSTATUS(0xC0040011_u32 as _);
pub const IO_FILE_QUOTA_CORRUPT: NTSTATUS = NTSTATUS(0xC004002A_u32 as _);
pub const IO_FILE_QUOTA_FAILED: NTSTATUS = NTSTATUS(0x80040028_u32 as _);
pub const IO_FILE_QUOTA_LIMIT: NTSTATUS = NTSTATUS(0x40040025_u32 as _);
pub const IO_FILE_QUOTA_STARTED: NTSTATUS = NTSTATUS(0x40040026_u32 as _);
pub const IO_FILE_QUOTA_SUCCEEDED: NTSTATUS = NTSTATUS(0x40040027_u32 as _);
pub const IO_FILE_QUOTA_THRESHOLD: NTSTATUS = NTSTATUS(0x40040024_u32 as _);
pub const IO_FILE_SYSTEM_CORRUPT: NTSTATUS = NTSTATUS(0xC0040029_u32 as _);
pub const IO_FILE_SYSTEM_CORRUPT_WITH_NAME: NTSTATUS = NTSTATUS(0xC0040037_u32 as _);
pub const IO_INFO_THROTTLE_COMPLETE: NTSTATUS = NTSTATUS(0x40040077_u32 as _);
pub const IO_LOST_DELAYED_WRITE: NTSTATUS = NTSTATUS(0x80040032_u32 as _);
pub const IO_LOST_DELAYED_WRITE_NETWORK_DISCONNECTED: NTSTATUS = NTSTATUS(0x8004008B_u32 as _);
pub const IO_LOST_DELAYED_WRITE_NETWORK_LOCAL_DISK_ERROR: NTSTATUS = NTSTATUS(0x8004008D_u32 as _);
pub const IO_LOST_DELAYED_WRITE_NETWORK_SERVER_ERROR: NTSTATUS = NTSTATUS(0x8004008C_u32 as _);
pub const IO_RECOVERED_VIA_ECC: NTSTATUS = NTSTATUS(0x80040021_u32 as _);
pub const IO_SYSTEM_SLEEP_FAILED: NTSTATUS = NTSTATUS(0xC004002B_u32 as _);
pub const IO_WARNING_ADAPTER_FIRMWARE_UPDATED: NTSTATUS = NTSTATUS(0x400400A0_u32 as _);
pub const IO_WARNING_ALLOCATION_FAILED: NTSTATUS = NTSTATUS(0x80040038_u32 as _);
pub const IO_WARNING_BUS_RESET: NTSTATUS = NTSTATUS(0x80040076_u32 as _);
pub const IO_WARNING_COMPLETION_TIME: NTSTATUS = NTSTATUS(0x8004009B_u32 as _);
pub const IO_WARNING_DEVICE_HAS_INTERNAL_DUMP: NTSTATUS = NTSTATUS(0x8004008F_u32 as _);
pub const IO_WARNING_DISK_CAPACITY_CHANGED: NTSTATUS = NTSTATUS(0x80040097_u32 as _);
pub const IO_WARNING_DISK_FIRMWARE_UPDATED: NTSTATUS = NTSTATUS(0x4004009F_u32 as _);
pub const IO_WARNING_DISK_PROVISIONING_TYPE_CHANGED: NTSTATUS = NTSTATUS(0x80040098_u32 as _);
pub const IO_WARNING_DISK_SURPRISE_REMOVED: NTSTATUS = NTSTATUS(0x8004009D_u32 as _);
pub const IO_WARNING_DUMP_DISABLED_DEVICE_GONE: NTSTATUS = NTSTATUS(0x8004009C_u32 as _);
pub const IO_WARNING_DUPLICATE_PATH: NTSTATUS = NTSTATUS(0x8004003B_u32 as _);
pub const IO_WARNING_DUPLICATE_SIGNATURE: NTSTATUS = NTSTATUS(0x8004003A_u32 as _);
pub const IO_WARNING_INTERRUPT_STILL_PENDING: NTSTATUS = NTSTATUS(0x80040035_u32 as _);
pub const IO_WARNING_IO_OPERATION_RETRIED: NTSTATUS = NTSTATUS(0x80040099_u32 as _);
pub const IO_WARNING_LOG_FLUSH_FAILED: NTSTATUS = NTSTATUS(0x80040039_u32 as _);
pub const IO_WARNING_PAGING_FAILURE: NTSTATUS = NTSTATUS(0x80040033_u32 as _);
pub const IO_WARNING_REPEATED_DISK_GUID: NTSTATUS = NTSTATUS(0x8004009E_u32 as _);
pub const IO_WARNING_RESET: NTSTATUS = NTSTATUS(0x80040081_u32 as _);
pub const IO_WARNING_SOFT_THRESHOLD_REACHED: NTSTATUS = NTSTATUS(0x80040090_u32 as _);
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX: NTSTATUS = NTSTATUS(0x80040091_u32 as _);
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_LUN: NTSTATUS = NTSTATUS(0x80040092_u32 as _);
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_POOL: NTSTATUS = NTSTATUS(0x80040093_u32 as _);
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_LUN: NTSTATUS = NTSTATUS(0x80040094_u32 as _);
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_POOL: NTSTATUS = NTSTATUS(0x80040095_u32 as _);
pub const IO_WARNING_VOLUME_LOST_DISK_EXTENT: NTSTATUS = NTSTATUS(0x8004008E_u32 as _);
pub const IO_WARNING_WRITE_FUA_PROBLEM: NTSTATUS = NTSTATUS(0x80040084_u32 as _);
pub const IO_WRITE_CACHE_DISABLED: NTSTATUS = NTSTATUS(0x80040022_u32 as _);
pub const IO_WRITE_CACHE_ENABLED: NTSTATUS = NTSTATUS(0x80040020_u32 as _);
pub const IO_WRN_BAD_FIRMWARE: NTSTATUS = NTSTATUS(0x8004001A_u32 as _);
pub const IO_WRN_FAILURE_PREDICTED: NTSTATUS = NTSTATUS(0x80040034_u32 as _);
pub const JSCRIPT_E_CANTEXECUTE: windows_core::HRESULT = windows_core::HRESULT(0x89020001_u32 as _);
pub const LANGUAGE_E_DATABASE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80041784_u32 as _);
pub const LANGUAGE_S_LARGE_WORD: windows_core::HRESULT = windows_core::HRESULT(0x41781_u32 as _);
pub const MARSHAL_E_FIRST: i32 = -2147221216i32;
pub const MARSHAL_E_LAST: i32 = -2147221201i32;
pub const MARSHAL_S_FIRST: i32 = 262432i32;
pub const MARSHAL_S_LAST: i32 = 262447i32;
pub const MAX_PATH: u32 = 260u32;
pub const MCA_BUS_ERROR: NTSTATUS = NTSTATUS(0xC005007A_u32 as _);
pub const MCA_BUS_TIMEOUT_ERROR: NTSTATUS = NTSTATUS(0xC005007B_u32 as _);
pub const MCA_ERROR_CACHE: NTSTATUS = NTSTATUS(0xC005003D_u32 as _);
pub const MCA_ERROR_CPU: NTSTATUS = NTSTATUS(0xC0050072_u32 as _);
pub const MCA_ERROR_CPU_BUS: NTSTATUS = NTSTATUS(0xC0050041_u32 as _);
pub const MCA_ERROR_MAS: NTSTATUS = NTSTATUS(0xC0050045_u32 as _);
pub const MCA_ERROR_MEM_1_2: NTSTATUS = NTSTATUS(0xC0050049_u32 as _);
pub const MCA_ERROR_MEM_1_2_5: NTSTATUS = NTSTATUS(0xC005004B_u32 as _);
pub const MCA_ERROR_MEM_1_2_5_4: NTSTATUS = NTSTATUS(0xC005004D_u32 as _);
pub const MCA_ERROR_MEM_UNKNOWN: NTSTATUS = NTSTATUS(0xC0050047_u32 as _);
pub const MCA_ERROR_PCI_BUS_MASTER_ABORT: NTSTATUS = NTSTATUS(0xC0050059_u32 as _);
pub const MCA_ERROR_PCI_BUS_MASTER_ABORT_NO_INFO: NTSTATUS = NTSTATUS(0xC005005B_u32 as _);
pub const MCA_ERROR_PCI_BUS_PARITY: NTSTATUS = NTSTATUS(0xC0050051_u32 as _);
pub const MCA_ERROR_PCI_BUS_PARITY_NO_INFO: NTSTATUS = NTSTATUS(0xC0050053_u32 as _);
pub const MCA_ERROR_PCI_BUS_SERR: NTSTATUS = NTSTATUS(0xC0050055_u32 as _);
pub const MCA_ERROR_PCI_BUS_SERR_NO_INFO: NTSTATUS = NTSTATUS(0xC0050057_u32 as _);
pub const MCA_ERROR_PCI_BUS_TIMEOUT: NTSTATUS = NTSTATUS(0xC005005D_u32 as _);
pub const MCA_ERROR_PCI_BUS_TIMEOUT_NO_INFO: NTSTATUS = NTSTATUS(0xC005005F_u32 as _);
pub const MCA_ERROR_PCI_BUS_UNKNOWN: NTSTATUS = NTSTATUS(0xC0050061_u32 as _);
pub const MCA_ERROR_PCI_DEVICE: NTSTATUS = NTSTATUS(0xC0050063_u32 as _);
pub const MCA_ERROR_PLATFORM_SPECIFIC: NTSTATUS = NTSTATUS(0xC0050067_u32 as _);
pub const MCA_ERROR_REGISTER_FILE: NTSTATUS = NTSTATUS(0xC0050043_u32 as _);
pub const MCA_ERROR_SMBIOS: NTSTATUS = NTSTATUS(0xC0050065_u32 as _);
pub const MCA_ERROR_SYSTEM_EVENT: NTSTATUS = NTSTATUS(0xC005004F_u32 as _);
pub const MCA_ERROR_TLB: NTSTATUS = NTSTATUS(0xC005003F_u32 as _);
pub const MCA_ERROR_UNKNOWN: NTSTATUS = NTSTATUS(0xC0050069_u32 as _);
pub const MCA_ERROR_UNKNOWN_NO_CPU: NTSTATUS = NTSTATUS(0xC005006B_u32 as _);
pub const MCA_EXTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC005007F_u32 as _);
pub const MCA_FRC_ERROR: NTSTATUS = NTSTATUS(0xC0050080_u32 as _);
pub const MCA_INFO_CPU_THERMAL_THROTTLING_REMOVED: NTSTATUS = NTSTATUS(0x40050070_u32 as _);
pub const MCA_INFO_MEMORY_PAGE_MARKED_BAD: NTSTATUS = NTSTATUS(0x40050074_u32 as _);
pub const MCA_INFO_NO_MORE_CORRECTED_ERROR_LOGS: NTSTATUS = NTSTATUS(0x40050073_u32 as _);
pub const MCA_INTERNALTIMER_ERROR: NTSTATUS = NTSTATUS(0xC005007C_u32 as _);
pub const MCA_MEMORYHIERARCHY_ERROR: NTSTATUS = NTSTATUS(0xC0050078_u32 as _);
pub const MCA_MICROCODE_ROM_PARITY_ERROR: NTSTATUS = NTSTATUS(0xC005007E_u32 as _);
pub const MCA_TLB_ERROR: NTSTATUS = NTSTATUS(0xC0050079_u32 as _);
pub const MCA_WARNING_CACHE: NTSTATUS = NTSTATUS(0x8005003C_u32 as _);
pub const MCA_WARNING_CMC_THRESHOLD_EXCEEDED: NTSTATUS = NTSTATUS(0x8005006D_u32 as _);
pub const MCA_WARNING_CPE_THRESHOLD_EXCEEDED: NTSTATUS = NTSTATUS(0x8005006E_u32 as _);
pub const MCA_WARNING_CPU: NTSTATUS = NTSTATUS(0x80050071_u32 as _);
pub const MCA_WARNING_CPU_BUS: NTSTATUS = NTSTATUS(0x80050040_u32 as _);
pub const MCA_WARNING_CPU_THERMAL_THROTTLED: NTSTATUS = NTSTATUS(0x8005006F_u32 as _);
pub const MCA_WARNING_MAS: NTSTATUS = NTSTATUS(0x80050044_u32 as _);
pub const MCA_WARNING_MEM_1_2: NTSTATUS = NTSTATUS(0x80050048_u32 as _);
pub const MCA_WARNING_MEM_1_2_5: NTSTATUS = NTSTATUS(0x8005004A_u32 as _);
pub const MCA_WARNING_MEM_1_2_5_4: NTSTATUS = NTSTATUS(0x8005004C_u32 as _);
pub const MCA_WARNING_MEM_UNKNOWN: NTSTATUS = NTSTATUS(0x80050046_u32 as _);
pub const MCA_WARNING_PCI_BUS_MASTER_ABORT: NTSTATUS = NTSTATUS(0x80050058_u32 as _);
pub const MCA_WARNING_PCI_BUS_MASTER_ABORT_NO_INFO: NTSTATUS = NTSTATUS(0x8005005A_u32 as _);
pub const MCA_WARNING_PCI_BUS_PARITY: NTSTATUS = NTSTATUS(0x80050050_u32 as _);
pub const MCA_WARNING_PCI_BUS_PARITY_NO_INFO: NTSTATUS = NTSTATUS(0x80050052_u32 as _);
pub const MCA_WARNING_PCI_BUS_SERR: NTSTATUS = NTSTATUS(0x80050054_u32 as _);
pub const MCA_WARNING_PCI_BUS_SERR_NO_INFO: NTSTATUS = NTSTATUS(0x80050056_u32 as _);
pub const MCA_WARNING_PCI_BUS_TIMEOUT: NTSTATUS = NTSTATUS(0x8005005C_u32 as _);
pub const MCA_WARNING_PCI_BUS_TIMEOUT_NO_INFO: NTSTATUS = NTSTATUS(0x8005005E_u32 as _);
pub const MCA_WARNING_PCI_BUS_UNKNOWN: NTSTATUS = NTSTATUS(0x80050060_u32 as _);
pub const MCA_WARNING_PCI_DEVICE: NTSTATUS = NTSTATUS(0x80050062_u32 as _);
pub const MCA_WARNING_PLATFORM_SPECIFIC: NTSTATUS = NTSTATUS(0x80050066_u32 as _);
pub const MCA_WARNING_REGISTER_FILE: NTSTATUS = NTSTATUS(0x80050042_u32 as _);
pub const MCA_WARNING_SMBIOS: NTSTATUS = NTSTATUS(0x80050064_u32 as _);
pub const MCA_WARNING_SYSTEM_EVENT: NTSTATUS = NTSTATUS(0x8005004E_u32 as _);
pub const MCA_WARNING_TLB: NTSTATUS = NTSTATUS(0x8005003E_u32 as _);
pub const MCA_WARNING_UNKNOWN: NTSTATUS = NTSTATUS(0x80050068_u32 as _);
pub const MCA_WARNING_UNKNOWN_NO_CPU: NTSTATUS = NTSTATUS(0x8005006A_u32 as _);
pub const MEM_E_INVALID_LINK: windows_core::HRESULT = windows_core::HRESULT(0x80080010_u32 as _);
pub const MEM_E_INVALID_ROOT: windows_core::HRESULT = windows_core::HRESULT(0x80080009_u32 as _);
pub const MEM_E_INVALID_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80080011_u32 as _);
pub const MENROLL_S_ENROLLMENT_SUSPENDED: windows_core::HRESULT = windows_core::HRESULT(0x180011_u32 as _);
pub const MILAVERR_INSUFFICIENTVIDEORESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x88980508_u32 as _);
pub const MILAVERR_INVALIDWMPVERSION: windows_core::HRESULT = windows_core::HRESULT(0x88980507_u32 as _);
pub const MILAVERR_MEDIAPLAYERCLOSED: windows_core::HRESULT = windows_core::HRESULT(0x8898050D_u32 as _);
pub const MILAVERR_MODULENOTLOADED: windows_core::HRESULT = windows_core::HRESULT(0x88980505_u32 as _);
pub const MILAVERR_NOCLOCK: windows_core::HRESULT = windows_core::HRESULT(0x88980500_u32 as _);
pub const MILAVERR_NOMEDIATYPE: windows_core::HRESULT = windows_core::HRESULT(0x88980501_u32 as _);
pub const MILAVERR_NOREADYFRAMES: windows_core::HRESULT = windows_core::HRESULT(0x88980504_u32 as _);
pub const MILAVERR_NOVIDEOMIXER: windows_core::HRESULT = windows_core::HRESULT(0x88980502_u32 as _);
pub const MILAVERR_NOVIDEOPRESENTER: windows_core::HRESULT = windows_core::HRESULT(0x88980503_u32 as _);
pub const MILAVERR_REQUESTEDTEXTURETOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x8898050A_u32 as _);
pub const MILAVERR_SEEKFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8898050B_u32 as _);
pub const MILAVERR_UNEXPECTEDWMPFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8898050C_u32 as _);
pub const MILAVERR_UNKNOWNHARDWAREERROR: windows_core::HRESULT = windows_core::HRESULT(0x8898050E_u32 as _);
pub const MILAVERR_VIDEOACCELERATIONNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88980509_u32 as _);
pub const MILAVERR_WMPFACTORYNOTREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x88980506_u32 as _);
pub const MILEFFECTSERR_ALREADYATTACHEDTOLISTENER: windows_core::HRESULT = windows_core::HRESULT(0x88980618_u32 as _);
pub const MILEFFECTSERR_CONNECTORNOTASSOCIATEDWITHEFFECT: windows_core::HRESULT = windows_core::HRESULT(0x88980612_u32 as _);
pub const MILEFFECTSERR_CONNECTORNOTCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x88980611_u32 as _);
pub const MILEFFECTSERR_CYCLEDETECTED: windows_core::HRESULT = windows_core::HRESULT(0x88980614_u32 as _);
pub const MILEFFECTSERR_EFFECTALREADYINAGRAPH: windows_core::HRESULT = windows_core::HRESULT(0x88980616_u32 as _);
pub const MILEFFECTSERR_EFFECTHASNOCHILDREN: windows_core::HRESULT = windows_core::HRESULT(0x88980617_u32 as _);
pub const MILEFFECTSERR_EFFECTINMORETHANONEGRAPH: windows_core::HRESULT = windows_core::HRESULT(0x88980615_u32 as _);
pub const MILEFFECTSERR_EFFECTNOTPARTOFGROUP: windows_core::HRESULT = windows_core::HRESULT(0x8898060F_u32 as _);
pub const MILEFFECTSERR_EMPTYBOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x8898061A_u32 as _);
pub const MILEFFECTSERR_NOINPUTSOURCEATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x88980610_u32 as _);
pub const MILEFFECTSERR_NOTAFFINETRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x88980619_u32 as _);
pub const MILEFFECTSERR_OUTPUTSIZETOOLARGE: windows_core::HRESULT = windows_core::HRESULT(0x8898061B_u32 as _);
pub const MILEFFECTSERR_RESERVED: windows_core::HRESULT = windows_core::HRESULT(0x88980613_u32 as _);
pub const MILEFFECTSERR_UNKNOWNPROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8898060E_u32 as _);
pub const MILERR_ADAPTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8898009E_u32 as _);
pub const MILERR_ALREADYLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x88980086_u32 as _);
pub const MILERR_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8898008F_u32 as _);
pub const MILERR_BADNUMBER: windows_core::HRESULT = windows_core::HRESULT(0x8898000A_u32 as _);
pub const MILERR_COLORSPACE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8898009F_u32 as _);
pub const MILERR_DEVICECANNOTRENDERTEXT: windows_core::HRESULT = windows_core::HRESULT(0x88980088_u32 as _);
pub const MILERR_DISPLAYFORMATNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88980084_u32 as _);
pub const MILERR_DISPLAYID_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x889800A1_u32 as _);
pub const MILERR_DISPLAYSTATEINVALID: windows_core::HRESULT = windows_core::HRESULT(0x88980006_u32 as _);
pub const MILERR_DXGI_ENUMERATION_OUT_OF_SYNC: windows_core::HRESULT = windows_core::HRESULT(0x8898009D_u32 as _);
pub const MILERR_GENERIC_IGNORE: windows_core::HRESULT = windows_core::HRESULT(0x8898008B_u32 as _);
pub const MILERR_GLYPHBITMAPMISSED: windows_core::HRESULT = windows_core::HRESULT(0x88980089_u32 as _);
pub const MILERR_INSUFFICIENTBUFFER: windows_core::HRESULT = windows_core::HRESULT(0x88980002_u32 as _);
pub const MILERR_INTERNALERROR: windows_core::HRESULT = windows_core::HRESULT(0x88980080_u32 as _);
pub const MILERR_INVALIDCALL: windows_core::HRESULT = windows_core::HRESULT(0x88980085_u32 as _);
pub const MILERR_MALFORMEDGLYPHCACHE: windows_core::HRESULT = windows_core::HRESULT(0x8898008A_u32 as _);
pub const MILERR_MALFORMED_GUIDELINE_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8898008C_u32 as _);
pub const MILERR_MAX_TEXTURE_SIZE_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8898009A_u32 as _);
pub const MILERR_MISMATCHED_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x88980090_u32 as _);
pub const MILERR_MROW_READLOCK_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980097_u32 as _);
pub const MILERR_MROW_UPDATE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980098_u32 as _);
pub const MILERR_NEED_RECREATE_AND_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x8898008E_u32 as _);
pub const MILERR_NONINVERTIBLEMATRIX: windows_core::HRESULT = windows_core::HRESULT(0x88980007_u32 as _);
pub const MILERR_NOTLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x88980087_u32 as _);
pub const MILERR_NOT_QUEUING_PRESENTS: windows_core::HRESULT = windows_core::HRESULT(0x88980094_u32 as _);
pub const MILERR_NO_HARDWARE_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x8898008D_u32 as _);
pub const MILERR_NO_REDIRECTION_SURFACE_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88980091_u32 as _);
pub const MILERR_NO_REDIRECTION_SURFACE_RETRY_LATER: windows_core::HRESULT = windows_core::HRESULT(0x88980095_u32 as _);
pub const MILERR_OBJECTBUSY: windows_core::HRESULT = windows_core::HRESULT(0x88980001_u32 as _);
pub const MILERR_PREFILTER_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x889800A0_u32 as _);
pub const MILERR_QPC_TIME_WENT_BACKWARD: windows_core::HRESULT = windows_core::HRESULT(0x8898009B_u32 as _);
pub const MILERR_QUEUED_PRESENT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88980093_u32 as _);
pub const MILERR_REMOTING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88980092_u32 as _);
pub const MILERR_SCANNER_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980004_u32 as _);
pub const MILERR_SCREENACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x88980005_u32 as _);
pub const MILERR_SHADER_COMPILE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980099_u32 as _);
pub const MILERR_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x88980009_u32 as _);
pub const MILERR_TOOMANYSHADERELEMNTS: windows_core::HRESULT = windows_core::HRESULT(0x88980096_u32 as _);
pub const MILERR_WIN32ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88980003_u32 as _);
pub const MILERR_ZEROVECTOR: windows_core::HRESULT = windows_core::HRESULT(0x88980008_u32 as _);
pub const MK_E_CANTOPENFILE: windows_core::HRESULT = windows_core::HRESULT(0x800401EA_u32 as _);
pub const MK_E_CONNECTMANUALLY: windows_core::HRESULT = windows_core::HRESULT(0x800401E0_u32 as _);
pub const MK_E_ENUMERATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800401EF_u32 as _);
pub const MK_E_EXCEEDEDDEADLINE: windows_core::HRESULT = windows_core::HRESULT(0x800401E1_u32 as _);
pub const MK_E_FIRST: i32 = -2147221024i32;
pub const MK_E_INTERMEDIATEINTERFACENOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x800401E7_u32 as _);
pub const MK_E_INVALIDEXTENSION: windows_core::HRESULT = windows_core::HRESULT(0x800401E6_u32 as _);
pub const MK_E_LAST: i32 = -2147221009i32;
pub const MK_E_MUSTBOTHERUSER: windows_core::HRESULT = windows_core::HRESULT(0x800401EB_u32 as _);
pub const MK_E_NEEDGENERIC: windows_core::HRESULT = windows_core::HRESULT(0x800401E2_u32 as _);
pub const MK_E_NOINVERSE: windows_core::HRESULT = windows_core::HRESULT(0x800401EC_u32 as _);
pub const MK_E_NOOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x800401E5_u32 as _);
pub const MK_E_NOPREFIX: windows_core::HRESULT = windows_core::HRESULT(0x800401EE_u32 as _);
pub const MK_E_NOSTORAGE: windows_core::HRESULT = windows_core::HRESULT(0x800401ED_u32 as _);
pub const MK_E_NOTBINDABLE: windows_core::HRESULT = windows_core::HRESULT(0x800401E8_u32 as _);
pub const MK_E_NOTBOUND: windows_core::HRESULT = windows_core::HRESULT(0x800401E9_u32 as _);
pub const MK_E_NO_NORMALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80080007_u32 as _);
pub const MK_E_SYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x800401E4_u32 as _);
pub const MK_E_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x800401E3_u32 as _);
pub const MK_S_FIRST: i32 = 262624i32;
pub const MK_S_HIM: windows_core::HRESULT = windows_core::HRESULT(0x401E5_u32 as _);
pub const MK_S_LAST: i32 = 262639i32;
pub const MK_S_ME: windows_core::HRESULT = windows_core::HRESULT(0x401E4_u32 as _);
pub const MK_S_MONIKERALREADYREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x401E7_u32 as _);
pub const MK_S_REDUCED_TO_SELF: windows_core::HRESULT = windows_core::HRESULT(0x401E2_u32 as _);
pub const MK_S_US: windows_core::HRESULT = windows_core::HRESULT(0x401E6_u32 as _);
pub const MSDTC_E_DUPLICATE_RESOURCE: windows_core::HRESULT = windows_core::HRESULT(0x80110701_u32 as _);
pub const MSSIPOTF_E_BADVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80097015_u32 as _);
pub const MSSIPOTF_E_BAD_FIRST_TABLE_PLACEMENT: windows_core::HRESULT = windows_core::HRESULT(0x80097008_u32 as _);
pub const MSSIPOTF_E_BAD_MAGICNUMBER: windows_core::HRESULT = windows_core::HRESULT(0x80097004_u32 as _);
pub const MSSIPOTF_E_BAD_OFFSET_TABLE: windows_core::HRESULT = windows_core::HRESULT(0x80097005_u32 as _);
pub const MSSIPOTF_E_CANTGETOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80097002_u32 as _);
pub const MSSIPOTF_E_CRYPT: windows_core::HRESULT = windows_core::HRESULT(0x80097014_u32 as _);
pub const MSSIPOTF_E_DSIG_STRUCTURE: windows_core::HRESULT = windows_core::HRESULT(0x80097016_u32 as _);
pub const MSSIPOTF_E_FAILED_HINTS_CHECK: windows_core::HRESULT = windows_core::HRESULT(0x80097011_u32 as _);
pub const MSSIPOTF_E_FAILED_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x80097010_u32 as _);
pub const MSSIPOTF_E_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80097013_u32 as _);
pub const MSSIPOTF_E_FILETOOSMALL: windows_core::HRESULT = windows_core::HRESULT(0x8009700B_u32 as _);
pub const MSSIPOTF_E_FILE_CHECKSUM: windows_core::HRESULT = windows_core::HRESULT(0x8009700D_u32 as _);
pub const MSSIPOTF_E_NOHEADTABLE: windows_core::HRESULT = windows_core::HRESULT(0x80097003_u32 as _);
pub const MSSIPOTF_E_NOT_OPENTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80097012_u32 as _);
pub const MSSIPOTF_E_OUTOFMEMRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80097001_u32 as _);
pub const MSSIPOTF_E_PCONST_CHECK: windows_core::HRESULT = windows_core::HRESULT(0x80097017_u32 as _);
pub const MSSIPOTF_E_STRUCTURE: windows_core::HRESULT = windows_core::HRESULT(0x80097018_u32 as _);
pub const MSSIPOTF_E_TABLES_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0x80097009_u32 as _);
pub const MSSIPOTF_E_TABLE_CHECKSUM: windows_core::HRESULT = windows_core::HRESULT(0x8009700C_u32 as _);
pub const MSSIPOTF_E_TABLE_LONGWORD: windows_core::HRESULT = windows_core::HRESULT(0x80097007_u32 as _);
pub const MSSIPOTF_E_TABLE_PADBYTES: windows_core::HRESULT = windows_core::HRESULT(0x8009700A_u32 as _);
pub const MSSIPOTF_E_TABLE_TAGORDER: windows_core::HRESULT = windows_core::HRESULT(0x80097006_u32 as _);
pub const NAP_E_CONFLICTING_ID: windows_core::HRESULT = windows_core::HRESULT(0x80270003_u32 as _);
pub const NAP_E_ENTITY_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8027000E_u32 as _);
pub const NAP_E_ID_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8027000A_u32 as _);
pub const NAP_E_INVALID_PACKET: windows_core::HRESULT = windows_core::HRESULT(0x80270001_u32 as _);
pub const NAP_E_MAXSIZE_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x8027000B_u32 as _);
pub const NAP_E_MISMATCHED_ID: windows_core::HRESULT = windows_core::HRESULT(0x80270008_u32 as _);
pub const NAP_E_MISSING_SOH: windows_core::HRESULT = windows_core::HRESULT(0x80270002_u32 as _);
pub const NAP_E_NETSH_GROUPPOLICY_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8027000F_u32 as _);
pub const NAP_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80270007_u32 as _);
pub const NAP_E_NOT_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80270009_u32 as _);
pub const NAP_E_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80270006_u32 as _);
pub const NAP_E_NO_CACHED_SOH: windows_core::HRESULT = windows_core::HRESULT(0x80270004_u32 as _);
pub const NAP_E_SERVICE_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8027000C_u32 as _);
pub const NAP_E_SHV_CONFIG_EXISTED: windows_core::HRESULT = windows_core::HRESULT(0x80270011_u32 as _);
pub const NAP_E_SHV_CONFIG_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80270012_u32 as _);
pub const NAP_E_SHV_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80270013_u32 as _);
pub const NAP_E_STILL_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x80270005_u32 as _);
pub const NAP_E_TOO_MANY_CALLS: windows_core::HRESULT = windows_core::HRESULT(0x80270010_u32 as _);
pub const NAP_S_CERT_ALREADY_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x27000D_u32 as _);
pub const NOERROR: u32 = 0u32;
pub const NOT_AN_ERROR1: windows_core::HRESULT = windows_core::HRESULT(0x81600_u32 as _);
pub const NO_ERROR: WIN32_ERROR = WIN32_ERROR(0u32);
pub const NTDDI_MAXVER: u32 = 2560u32;
pub const NTE_AUTHENTICATION_IGNORED: windows_core::HRESULT = windows_core::HRESULT(0x80090031_u32 as _);
pub const NTE_BAD_ALGID: windows_core::HRESULT = windows_core::HRESULT(0x80090008_u32 as _);
pub const NTE_BAD_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80090005_u32 as _);
pub const NTE_BAD_FLAGS: windows_core::HRESULT = windows_core::HRESULT(0x80090009_u32 as _);
pub const NTE_BAD_HASH: windows_core::HRESULT = windows_core::HRESULT(0x80090002_u32 as _);
pub const NTE_BAD_HASH_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8009000C_u32 as _);
pub const NTE_BAD_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80090003_u32 as _);
pub const NTE_BAD_KEYSET: windows_core::HRESULT = windows_core::HRESULT(0x80090016_u32 as _);
pub const NTE_BAD_KEYSET_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x8009001F_u32 as _);
pub const NTE_BAD_KEY_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8009000B_u32 as _);
pub const NTE_BAD_LEN: windows_core::HRESULT = windows_core::HRESULT(0x80090004_u32 as _);
pub const NTE_BAD_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80090013_u32 as _);
pub const NTE_BAD_PROV_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80090014_u32 as _);
pub const NTE_BAD_PUBLIC_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80090015_u32 as _);
pub const NTE_BAD_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80090006_u32 as _);
pub const NTE_BAD_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8009000A_u32 as _);
pub const NTE_BAD_UID: windows_core::HRESULT = windows_core::HRESULT(0x80090001_u32 as _);
pub const NTE_BAD_VER: windows_core::HRESULT = windows_core::HRESULT(0x80090007_u32 as _);
pub const NTE_BUFFERS_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0x8009002B_u32 as _);
pub const NTE_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80090028_u32 as _);
pub const NTE_DECRYPTION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8009002C_u32 as _);
pub const NTE_DEVICE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80090035_u32 as _);
pub const NTE_DEVICE_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80090030_u32 as _);
pub const NTE_DOUBLE_ENCRYPT: windows_core::HRESULT = windows_core::HRESULT(0x80090012_u32 as _);
pub const NTE_ENCRYPTION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80090034_u32 as _);
pub const NTE_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8009000F_u32 as _);
pub const NTE_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80090020_u32 as _);
pub const NTE_FIXEDPARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80090025_u32 as _);
pub const NTE_HMAC_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8009002F_u32 as _);
pub const NTE_INCORRECT_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80090033_u32 as _);
pub const NTE_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8009002D_u32 as _);
pub const NTE_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80090026_u32 as _);
pub const NTE_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80090027_u32 as _);
pub const NTE_KEYSET_ENTRY_BAD: windows_core::HRESULT = windows_core::HRESULT(0x8009001A_u32 as _);
pub const NTE_KEYSET_NOT_DEF: windows_core::HRESULT = windows_core::HRESULT(0x80090019_u32 as _);
pub const NTE_NOT_ACTIVE_CONSOLE: windows_core::HRESULT = windows_core::HRESULT(0x80090038_u32 as _);
pub const NTE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80090011_u32 as _);
pub const NTE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80090029_u32 as _);
pub const NTE_NO_KEY: windows_core::HRESULT = windows_core::HRESULT(0x8009000D_u32 as _);
pub const NTE_NO_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8009000E_u32 as _);
pub const NTE_NO_MORE_ITEMS: windows_core::HRESULT = windows_core::HRESULT(0x8009002A_u32 as _);
pub const NTE_OP_OK: u32 = 0u32;
pub const NTE_PASSWORD_CHANGE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80090037_u32 as _);
pub const NTE_PERM: windows_core::HRESULT = windows_core::HRESULT(0x80090010_u32 as _);
pub const NTE_PROVIDER_DLL_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8009001D_u32 as _);
pub const NTE_PROV_DLL_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8009001E_u32 as _);
pub const NTE_PROV_TYPE_ENTRY_BAD: windows_core::HRESULT = windows_core::HRESULT(0x80090018_u32 as _);
pub const NTE_PROV_TYPE_NOT_DEF: windows_core::HRESULT = windows_core::HRESULT(0x80090017_u32 as _);
pub const NTE_PROV_TYPE_NO_MATCH: windows_core::HRESULT = windows_core::HRESULT(0x8009001B_u32 as _);
pub const NTE_SIGNATURE_FILE_BAD: windows_core::HRESULT = windows_core::HRESULT(0x8009001C_u32 as _);
pub const NTE_SILENT_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80090022_u32 as _);
pub const NTE_SYS_ERR: windows_core::HRESULT = windows_core::HRESULT(0x80090021_u32 as _);
pub const NTE_TEMPORARY_PROFILE: windows_core::HRESULT = windows_core::HRESULT(0x80090024_u32 as _);
pub const NTE_TOKEN_KEYSET_STORAGE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80090023_u32 as _);
pub const NTE_UI_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8009002E_u32 as _);
pub const NTE_USER_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80090036_u32 as _);
pub const NTE_VALIDATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80090032_u32 as _);
pub const OLEOBJ_E_FIRST: i32 = -2147221120i32;
pub const OLEOBJ_E_INVALIDVERB: windows_core::HRESULT = windows_core::HRESULT(0x80040181_u32 as _);
pub const OLEOBJ_E_LAST: i32 = -2147221105i32;
pub const OLEOBJ_E_NOVERBS: windows_core::HRESULT = windows_core::HRESULT(0x80040180_u32 as _);
pub const OLEOBJ_S_CANNOT_DOVERB_NOW: windows_core::HRESULT = windows_core::HRESULT(0x40181_u32 as _);
pub const OLEOBJ_S_FIRST: i32 = 262528i32;
pub const OLEOBJ_S_INVALIDHWND: windows_core::HRESULT = windows_core::HRESULT(0x40182_u32 as _);
pub const OLEOBJ_S_INVALIDVERB: windows_core::HRESULT = windows_core::HRESULT(0x40180_u32 as _);
pub const OLEOBJ_S_LAST: i32 = 262543i32;
pub const OLE_E_ADVF: windows_core::HRESULT = windows_core::HRESULT(0x80040001_u32 as _);
pub const OLE_E_ADVISENOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040003_u32 as _);
pub const OLE_E_BLANK: windows_core::HRESULT = windows_core::HRESULT(0x80040007_u32 as _);
pub const OLE_E_CANTCONVERT: windows_core::HRESULT = windows_core::HRESULT(0x80040011_u32 as _);
pub const OLE_E_CANT_BINDTOSOURCE: windows_core::HRESULT = windows_core::HRESULT(0x8004000A_u32 as _);
pub const OLE_E_CANT_GETMONIKER: windows_core::HRESULT = windows_core::HRESULT(0x80040009_u32 as _);
pub const OLE_E_CLASSDIFF: windows_core::HRESULT = windows_core::HRESULT(0x80040008_u32 as _);
pub const OLE_E_ENUM_NOMORE: windows_core::HRESULT = windows_core::HRESULT(0x80040002_u32 as _);
pub const OLE_E_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x80040000_u32 as _);
pub const OLE_E_INVALIDHWND: windows_core::HRESULT = windows_core::HRESULT(0x8004000F_u32 as _);
pub const OLE_E_INVALIDRECT: windows_core::HRESULT = windows_core::HRESULT(0x8004000D_u32 as _);
pub const OLE_E_LAST: windows_core::HRESULT = windows_core::HRESULT(0x800400FF_u32 as _);
pub const OLE_E_NOCACHE: windows_core::HRESULT = windows_core::HRESULT(0x80040006_u32 as _);
pub const OLE_E_NOCONNECTION: windows_core::HRESULT = windows_core::HRESULT(0x80040004_u32 as _);
pub const OLE_E_NOSTORAGE: windows_core::HRESULT = windows_core::HRESULT(0x80040012_u32 as _);
pub const OLE_E_NOTRUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80040005_u32 as _);
pub const OLE_E_NOT_INPLACEACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x80040010_u32 as _);
pub const OLE_E_OLEVERB: windows_core::HRESULT = windows_core::HRESULT(0x80040000_u32 as _);
pub const OLE_E_PROMPTSAVECANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x8004000C_u32 as _);
pub const OLE_E_STATIC: windows_core::HRESULT = windows_core::HRESULT(0x8004000B_u32 as _);
pub const OLE_E_WRONGCOMPOBJ: windows_core::HRESULT = windows_core::HRESULT(0x8004000E_u32 as _);
pub const OLE_S_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x40000_u32 as _);
pub const OLE_S_LAST: windows_core::HRESULT = windows_core::HRESULT(0x400FF_u32 as _);
pub const OLE_S_MAC_CLIPFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x40002_u32 as _);
pub const OLE_S_STATIC: windows_core::HRESULT = windows_core::HRESULT(0x40001_u32 as _);
pub const OLE_S_USEREG: windows_core::HRESULT = windows_core::HRESULT(0x40000_u32 as _);
pub const ONL_CONNECTION_COUNT_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x8086000D_u32 as _);
pub const ONL_E_ACCESS_DENIED_BY_TOU: windows_core::HRESULT = windows_core::HRESULT(0x80860002_u32 as _);
pub const ONL_E_ACCOUNT_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80860007_u32 as _);
pub const ONL_E_ACCOUNT_SUSPENDED_ABUSE: windows_core::HRESULT = windows_core::HRESULT(0x8086000B_u32 as _);
pub const ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE: windows_core::HRESULT = windows_core::HRESULT(0x8086000A_u32 as _);
pub const ONL_E_ACCOUNT_UPDATE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80860005_u32 as _);
pub const ONL_E_ACTION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8086000C_u32 as _);
pub const ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT: windows_core::HRESULT = windows_core::HRESULT(0x8086000E_u32 as _);
pub const ONL_E_EMAIL_VERIFICATION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80860009_u32 as _);
pub const ONL_E_FORCESIGNIN: windows_core::HRESULT = windows_core::HRESULT(0x80860006_u32 as _);
pub const ONL_E_INVALID_APPLICATION: windows_core::HRESULT = windows_core::HRESULT(0x80860003_u32 as _);
pub const ONL_E_INVALID_AUTHENTICATION_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x80860001_u32 as _);
pub const ONL_E_PARENTAL_CONSENT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80860008_u32 as _);
pub const ONL_E_PASSWORD_UPDATE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80860004_u32 as _);
pub const ONL_E_REQUEST_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x80860010_u32 as _);
pub const ONL_E_USER_AUTHENTICATION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8086000F_u32 as _);
pub const OR_INVALID_OID: i32 = 1911i32;
pub const OR_INVALID_OXID: i32 = 1910i32;
pub const OR_INVALID_SET: i32 = 1912i32;
pub const OSS_ACCESS_SERIALIZATION_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80093013_u32 as _);
pub const OSS_API_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093029_u32 as _);
pub const OSS_BAD_ARG: windows_core::HRESULT = windows_core::HRESULT(0x80093006_u32 as _);
pub const OSS_BAD_ENCRULES: windows_core::HRESULT = windows_core::HRESULT(0x80093016_u32 as _);
pub const OSS_BAD_PTR: windows_core::HRESULT = windows_core::HRESULT(0x8009300B_u32 as _);
pub const OSS_BAD_TABLE: windows_core::HRESULT = windows_core::HRESULT(0x8009300F_u32 as _);
pub const OSS_BAD_TIME: windows_core::HRESULT = windows_core::HRESULT(0x8009300C_u32 as _);
pub const OSS_BAD_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80093007_u32 as _);
pub const OSS_BERDER_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x8009302A_u32 as _);
pub const OSS_CANT_CLOSE_TRACE_FILE: windows_core::HRESULT = windows_core::HRESULT(0x8009302E_u32 as _);
pub const OSS_CANT_OPEN_TRACE_FILE: windows_core::HRESULT = windows_core::HRESULT(0x8009301B_u32 as _);
pub const OSS_CANT_OPEN_TRACE_WINDOW: windows_core::HRESULT = windows_core::HRESULT(0x80093018_u32 as _);
pub const OSS_COMPARATOR_CODE_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093025_u32 as _);
pub const OSS_COMPARATOR_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093024_u32 as _);
pub const OSS_CONSTRAINT_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093023_u32 as _);
pub const OSS_CONSTRAINT_VIOLATED: windows_core::HRESULT = windows_core::HRESULT(0x80093011_u32 as _);
pub const OSS_COPIER_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093022_u32 as _);
pub const OSS_DATA_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80093005_u32 as _);
pub const OSS_FATAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80093012_u32 as _);
pub const OSS_INDEFINITE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8009300D_u32 as _);
pub const OSS_LIMITED: windows_core::HRESULT = windows_core::HRESULT(0x8009300A_u32 as _);
pub const OSS_MEM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8009300E_u32 as _);
pub const OSS_MEM_MGR_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093026_u32 as _);
pub const OSS_MORE_BUF: windows_core::HRESULT = windows_core::HRESULT(0x80093001_u32 as _);
pub const OSS_MORE_INPUT: windows_core::HRESULT = windows_core::HRESULT(0x80093004_u32 as _);
pub const OSS_MUTEX_NOT_CREATED: windows_core::HRESULT = windows_core::HRESULT(0x8009302D_u32 as _);
pub const OSS_NEGATIVE_UINTEGER: windows_core::HRESULT = windows_core::HRESULT(0x80093002_u32 as _);
pub const OSS_NULL_FCN: windows_core::HRESULT = windows_core::HRESULT(0x80093015_u32 as _);
pub const OSS_NULL_TBL: windows_core::HRESULT = windows_core::HRESULT(0x80093014_u32 as _);
pub const OSS_OID_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x8009301A_u32 as _);
pub const OSS_OPEN_TYPE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8009302C_u32 as _);
pub const OSS_OUT_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80093008_u32 as _);
pub const OSS_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80093021_u32 as _);
pub const OSS_PDU_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80093009_u32 as _);
pub const OSS_PDU_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80093003_u32 as _);
pub const OSS_PDV_CODE_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093028_u32 as _);
pub const OSS_PDV_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093027_u32 as _);
pub const OSS_PER_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x8009302B_u32 as _);
pub const OSS_REAL_CODE_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x80093020_u32 as _);
pub const OSS_REAL_DLL_NOT_LINKED: windows_core::HRESULT = windows_core::HRESULT(0x8009301F_u32 as _);
pub const OSS_TABLE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8009301D_u32 as _);
pub const OSS_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80093010_u32 as _);
pub const OSS_TRACE_FILE_ALREADY_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x8009301C_u32 as _);
pub const OSS_TYPE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8009301E_u32 as _);
pub const OSS_UNAVAIL_ENCRULES: windows_core::HRESULT = windows_core::HRESULT(0x80093017_u32 as _);
pub const OSS_UNIMPLEMENTED: windows_core::HRESULT = windows_core::HRESULT(0x80093019_u32 as _);
pub const PEERDIST_ERROR_ALREADY_COMPLETED: i32 = 4060i32;
pub const PEERDIST_ERROR_ALREADY_EXISTS: i32 = 4058i32;
pub const PEERDIST_ERROR_ALREADY_INITIALIZED: i32 = 4055i32;
pub const PEERDIST_ERROR_CANNOT_PARSE_CONTENTINFO: i32 = 4051i32;
pub const PEERDIST_ERROR_CONTENTINFO_VERSION_UNSUPPORTED: i32 = 4050i32;
pub const PEERDIST_ERROR_INVALIDATED: i32 = 4057i32;
pub const PEERDIST_ERROR_INVALID_CONFIGURATION: i32 = 4063i32;
pub const PEERDIST_ERROR_MISSING_DATA: i32 = 4052i32;
pub const PEERDIST_ERROR_NOT_INITIALIZED: i32 = 4054i32;
pub const PEERDIST_ERROR_NOT_LICENSED: i32 = 4064i32;
pub const PEERDIST_ERROR_NO_MORE: i32 = 4053i32;
pub const PEERDIST_ERROR_OPERATION_NOTFOUND: i32 = 4059i32;
pub const PEERDIST_ERROR_OUT_OF_BOUNDS: i32 = 4061i32;
pub const PEERDIST_ERROR_SERVICE_UNAVAILABLE: i32 = 4065i32;
pub const PEERDIST_ERROR_SHUTDOWN_IN_PROGRESS: i32 = 4056i32;
pub const PEERDIST_ERROR_TRUST_FAILURE: i32 = 4066i32;
pub const PEERDIST_ERROR_VERSION_UNSUPPORTED: i32 = 4062i32;
pub const PEER_E_ALREADY_LISTENING: windows_core::HRESULT = windows_core::HRESULT(0x80630107_u32 as _);
pub const PEER_E_CANNOT_CONVERT_PEER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80634001_u32 as _);
pub const PEER_E_CANNOT_START_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80630003_u32 as _);
pub const PEER_E_CERT_STORE_CORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x80630801_u32 as _);
pub const PEER_E_CHAIN_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80630703_u32 as _);
pub const PEER_E_CIRCULAR_CHAIN_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x80630706_u32 as _);
pub const PEER_E_CLASSIFIER_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80630201_u32 as _);
pub const PEER_E_CLOUD_NAME_AMBIGUOUS: windows_core::HRESULT = windows_core::HRESULT(0x80631005_u32 as _);
pub const PEER_E_CONNECTION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80630109_u32 as _);
pub const PEER_E_CONNECTION_NOT_AUTHENTICATED: windows_core::HRESULT = windows_core::HRESULT(0x8063010A_u32 as _);
pub const PEER_E_CONNECTION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80630103_u32 as _);
pub const PEER_E_CONNECTION_REFUSED: windows_core::HRESULT = windows_core::HRESULT(0x8063010B_u32 as _);
pub const PEER_E_CONNECT_SELF: windows_core::HRESULT = windows_core::HRESULT(0x80630106_u32 as _);
pub const PEER_E_CONTACT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80636001_u32 as _);
pub const PEER_E_DATABASE_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80630302_u32 as _);
pub const PEER_E_DATABASE_ALREADY_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80630305_u32 as _);
pub const PEER_E_DATABASE_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80630306_u32 as _);
pub const PEER_E_DBINITIALIZATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80630303_u32 as _);
pub const PEER_E_DBNAME_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x80630011_u32 as _);
pub const PEER_E_DEFERRED_VALIDATION: windows_core::HRESULT = windows_core::HRESULT(0x80632030_u32 as _);
pub const PEER_E_DUPLICATE_GRAPH: windows_core::HRESULT = windows_core::HRESULT(0x80630012_u32 as _);
pub const PEER_E_EVENT_HANDLE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80630501_u32 as _);
pub const PEER_E_FW_BLOCKED_BY_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x80637009_u32 as _);
pub const PEER_E_FW_BLOCKED_BY_SHIELDS_UP: windows_core::HRESULT = windows_core::HRESULT(0x8063700A_u32 as _);
pub const PEER_E_FW_DECLINED: windows_core::HRESULT = windows_core::HRESULT(0x8063700B_u32 as _);
pub const PEER_E_FW_EXCEPTION_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80637008_u32 as _);
pub const PEER_E_GRAPH_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80630015_u32 as _);
pub const PEER_E_GRAPH_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80630013_u32 as _);
pub const PEER_E_GRAPH_SHUTTING_DOWN: windows_core::HRESULT = windows_core::HRESULT(0x80630014_u32 as _);
pub const PEER_E_GROUPS_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80630204_u32 as _);
pub const PEER_E_GROUP_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80632092_u32 as _);
pub const PEER_E_GROUP_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80632091_u32 as _);
pub const PEER_E_IDENTITY_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x806320A0_u32 as _);
pub const PEER_E_IDENTITY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80630401_u32 as _);
pub const PEER_E_INVALID_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80637007_u32 as _);
pub const PEER_E_INVALID_ATTRIBUTES: windows_core::HRESULT = windows_core::HRESULT(0x80630602_u32 as _);
pub const PEER_E_INVALID_CLASSIFIER: windows_core::HRESULT = windows_core::HRESULT(0x80632060_u32 as _);
pub const PEER_E_INVALID_CLASSIFIER_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80632072_u32 as _);
pub const PEER_E_INVALID_CREDENTIAL: windows_core::HRESULT = windows_core::HRESULT(0x80632082_u32 as _);
pub const PEER_E_INVALID_CREDENTIAL_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80632081_u32 as _);
pub const PEER_E_INVALID_DATABASE: windows_core::HRESULT = windows_core::HRESULT(0x80630016_u32 as _);
pub const PEER_E_INVALID_FRIENDLY_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80632070_u32 as _);
pub const PEER_E_INVALID_GRAPH: windows_core::HRESULT = windows_core::HRESULT(0x80630010_u32 as _);
pub const PEER_E_INVALID_GROUP: windows_core::HRESULT = windows_core::HRESULT(0x80632093_u32 as _);
pub const PEER_E_INVALID_GROUP_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0x80632040_u32 as _);
pub const PEER_E_INVALID_PEER_HOST_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80634002_u32 as _);
pub const PEER_E_INVALID_PEER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80632050_u32 as _);
pub const PEER_E_INVALID_RECORD: windows_core::HRESULT = windows_core::HRESULT(0x80632010_u32 as _);
pub const PEER_E_INVALID_RECORD_EXPIRATION: windows_core::HRESULT = windows_core::HRESULT(0x80632080_u32 as _);
pub const PEER_E_INVALID_RECORD_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80632083_u32 as _);
pub const PEER_E_INVALID_ROLE_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80632071_u32 as _);
pub const PEER_E_INVALID_SEARCH: windows_core::HRESULT = windows_core::HRESULT(0x80630601_u32 as _);
pub const PEER_E_INVALID_TIME_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x80630705_u32 as _);
pub const PEER_E_INVITATION_NOT_TRUSTED: windows_core::HRESULT = windows_core::HRESULT(0x80630701_u32 as _);
pub const PEER_E_INVITE_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80637000_u32 as _);
pub const PEER_E_INVITE_RESPONSE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80637001_u32 as _);
pub const PEER_E_IPV6_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80630001_u32 as _);
pub const PEER_E_MAX_RECORD_SIZE_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80630304_u32 as _);
pub const PEER_E_NODE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80630108_u32 as _);
pub const PEER_E_NOT_AUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0x80632020_u32 as _);
pub const PEER_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80630002_u32 as _);
pub const PEER_E_NOT_LICENSED: windows_core::HRESULT = windows_core::HRESULT(0x80630004_u32 as _);
pub const PEER_E_NOT_SIGNED_IN: windows_core::HRESULT = windows_core::HRESULT(0x80637003_u32 as _);
pub const PEER_E_NO_CLOUD: windows_core::HRESULT = windows_core::HRESULT(0x80631001_u32 as _);
pub const PEER_E_NO_KEY_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x80630203_u32 as _);
pub const PEER_E_NO_MEMBERS_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80632094_u32 as _);
pub const PEER_E_NO_MEMBER_CONNECTIONS: windows_core::HRESULT = windows_core::HRESULT(0x80632095_u32 as _);
pub const PEER_E_NO_MORE: windows_core::HRESULT = windows_core::HRESULT(0x80634003_u32 as _);
pub const PEER_E_PASSWORD_DOES_NOT_MEET_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x80632021_u32 as _);
pub const PEER_E_PNRP_DUPLICATE_PEER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80634005_u32 as _);
pub const PEER_E_PRIVACY_DECLINED: windows_core::HRESULT = windows_core::HRESULT(0x80637004_u32 as _);
pub const PEER_E_RECORD_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80630301_u32 as _);
pub const PEER_E_SERVICE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x806320A1_u32 as _);
pub const PEER_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80637005_u32 as _);
pub const PEER_E_TOO_MANY_ATTRIBUTES: windows_core::HRESULT = windows_core::HRESULT(0x80630017_u32 as _);
pub const PEER_E_TOO_MANY_IDENTITIES: windows_core::HRESULT = windows_core::HRESULT(0x80630202_u32 as _);
pub const PEER_E_UNABLE_TO_LISTEN: windows_core::HRESULT = windows_core::HRESULT(0x80632096_u32 as _);
pub const PEER_E_UNSUPPORTED_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80632090_u32 as _);
pub const PEER_S_ALREADY_A_MEMBER: windows_core::HRESULT = windows_core::HRESULT(0x630006_u32 as _);
pub const PEER_S_ALREADY_CONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x632000_u32 as _);
pub const PEER_S_GRAPH_DATA_CREATED: windows_core::HRESULT = windows_core::HRESULT(0x630001_u32 as _);
pub const PEER_S_NO_CONNECTIVITY: windows_core::HRESULT = windows_core::HRESULT(0x630005_u32 as _);
pub const PEER_S_NO_EVENT_DATA: windows_core::HRESULT = windows_core::HRESULT(0x630002_u32 as _);
pub const PEER_S_SUBSCRIPTION_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x636000_u32 as _);
pub const PERSIST_E_NOTSELFSIZING: windows_core::HRESULT = windows_core::HRESULT(0x800B000B_u32 as _);
pub const PERSIST_E_SIZEDEFINITE: windows_core::HRESULT = windows_core::HRESULT(0x800B0009_u32 as _);
pub const PERSIST_E_SIZEINDEFINITE: windows_core::HRESULT = windows_core::HRESULT(0x800B000A_u32 as _);
pub const PLA_E_CABAPI_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80300113_u32 as _);
pub const PLA_E_CONFLICT_INCL_EXCL_API: windows_core::HRESULT = windows_core::HRESULT(0x80300105_u32 as _);
pub const PLA_E_CREDENTIALS_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80300103_u32 as _);
pub const PLA_E_DCS_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x803000B7_u32 as _);
pub const PLA_E_DCS_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x803000AA_u32 as _);
pub const PLA_E_DCS_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80300002_u32 as _);
pub const PLA_E_DCS_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80300104_u32 as _);
pub const PLA_E_DCS_SINGLETON_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80300102_u32 as _);
pub const PLA_E_DCS_START_WAIT_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8030010A_u32 as _);
pub const PLA_E_DC_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80300109_u32 as _);
pub const PLA_E_DC_START_WAIT_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8030010B_u32 as _);
pub const PLA_E_EXE_ALREADY_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x80300107_u32 as _);
pub const PLA_E_EXE_FULL_PATH_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8030010E_u32 as _);
pub const PLA_E_EXE_PATH_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0x80300108_u32 as _);
pub const PLA_E_INVALID_SESSION_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8030010F_u32 as _);
pub const PLA_E_NETWORK_EXE_NOT_VALID: windows_core::HRESULT = windows_core::HRESULT(0x80300106_u32 as _);
pub const PLA_E_NO_DUPLICATES: windows_core::HRESULT = windows_core::HRESULT(0x8030010D_u32 as _);
pub const PLA_E_NO_MIN_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80300070_u32 as _);
pub const PLA_E_PLA_CHANNEL_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80300110_u32 as _);
pub const PLA_E_PROPERTY_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x80300101_u32 as _);
pub const PLA_E_REPORT_WAIT_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8030010C_u32 as _);
pub const PLA_E_RULES_MANAGER_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80300112_u32 as _);
pub const PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80300111_u32 as _);
pub const PLA_E_TOO_MANY_FOLDERS: windows_core::HRESULT = windows_core::HRESULT(0x80300045_u32 as _);
pub const PLA_S_PROPERTY_IGNORED: windows_core::HRESULT = windows_core::HRESULT(0x300100_u32 as _);
pub const PRESENTATION_ERROR_LOST: windows_core::HRESULT = windows_core::HRESULT(0x88810001_u32 as _);
pub const PSINK_E_INDEX_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80041791_u32 as _);
pub const PSINK_E_LARGE_ATTACHMENT: windows_core::HRESULT = windows_core::HRESULT(0x80041792_u32 as _);
pub const PSINK_E_QUERY_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80041790_u32 as _);
pub const PSINK_S_LARGE_WORD: windows_core::HRESULT = windows_core::HRESULT(0x41793_u32 as _);
pub const QPARSE_E_EXPECTING_BRACE: windows_core::HRESULT = windows_core::HRESULT(0x80041666_u32 as _);
pub const QPARSE_E_EXPECTING_COMMA: windows_core::HRESULT = windows_core::HRESULT(0x80041671_u32 as _);
pub const QPARSE_E_EXPECTING_CURRENCY: windows_core::HRESULT = windows_core::HRESULT(0x80041664_u32 as _);
pub const QPARSE_E_EXPECTING_DATE: windows_core::HRESULT = windows_core::HRESULT(0x80041663_u32 as _);
pub const QPARSE_E_EXPECTING_EOS: windows_core::HRESULT = windows_core::HRESULT(0x80041670_u32 as _);
pub const QPARSE_E_EXPECTING_GUID: windows_core::HRESULT = windows_core::HRESULT(0x80041665_u32 as _);
pub const QPARSE_E_EXPECTING_INTEGER: windows_core::HRESULT = windows_core::HRESULT(0x80041661_u32 as _);
pub const QPARSE_E_EXPECTING_PAREN: windows_core::HRESULT = windows_core::HRESULT(0x80041667_u32 as _);
pub const QPARSE_E_EXPECTING_PHRASE: windows_core::HRESULT = windows_core::HRESULT(0x8004166A_u32 as _);
pub const QPARSE_E_EXPECTING_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80041668_u32 as _);
pub const QPARSE_E_EXPECTING_REAL: windows_core::HRESULT = windows_core::HRESULT(0x80041662_u32 as _);
pub const QPARSE_E_EXPECTING_REGEX: windows_core::HRESULT = windows_core::HRESULT(0x8004166C_u32 as _);
pub const QPARSE_E_EXPECTING_REGEX_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004166D_u32 as _);
pub const QPARSE_E_INVALID_GROUPING: windows_core::HRESULT = windows_core::HRESULT(0x80041677_u32 as _);
pub const QPARSE_E_INVALID_LITERAL: windows_core::HRESULT = windows_core::HRESULT(0x8004166E_u32 as _);
pub const QPARSE_E_INVALID_QUERY: windows_core::HRESULT = windows_core::HRESULT(0x8004167A_u32 as _);
pub const QPARSE_E_INVALID_RANKMETHOD: windows_core::HRESULT = windows_core::HRESULT(0x8004167B_u32 as _);
pub const QPARSE_E_INVALID_SORT_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x80041675_u32 as _);
pub const QPARSE_E_NOT_YET_IMPLEMENTED: windows_core::HRESULT = windows_core::HRESULT(0x80041669_u32 as _);
pub const QPARSE_E_NO_SUCH_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004166F_u32 as _);
pub const QPARSE_E_NO_SUCH_SORT_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80041674_u32 as _);
pub const QPARSE_E_UNEXPECTED_EOS: windows_core::HRESULT = windows_core::HRESULT(0x80041672_u32 as _);
pub const QPARSE_E_UNEXPECTED_NOT: windows_core::HRESULT = windows_core::HRESULT(0x80041660_u32 as _);
pub const QPARSE_E_UNSUPPORTED_PROPERTY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8004166B_u32 as _);
pub const QPARSE_E_WEIGHT_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80041673_u32 as _);
pub const QPLIST_E_BAD_GUID: windows_core::HRESULT = windows_core::HRESULT(0x80041659_u32 as _);
pub const QPLIST_E_BYREF_USED_WITHOUT_PTRTYPE: windows_core::HRESULT = windows_core::HRESULT(0x8004165E_u32 as _);
pub const QPLIST_E_CANT_OPEN_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80041651_u32 as _);
pub const QPLIST_E_CANT_SET_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004165B_u32 as _);
pub const QPLIST_E_DUPLICATE: windows_core::HRESULT = windows_core::HRESULT(0x8004165C_u32 as _);
pub const QPLIST_E_EXPECTING_CLOSE_PAREN: windows_core::HRESULT = windows_core::HRESULT(0x80041657_u32 as _);
pub const QPLIST_E_EXPECTING_GUID: windows_core::HRESULT = windows_core::HRESULT(0x80041658_u32 as _);
pub const QPLIST_E_EXPECTING_INTEGER: windows_core::HRESULT = windows_core::HRESULT(0x80041656_u32 as _);
pub const QPLIST_E_EXPECTING_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80041653_u32 as _);
pub const QPLIST_E_EXPECTING_PROP_SPEC: windows_core::HRESULT = windows_core::HRESULT(0x8004165A_u32 as _);
pub const QPLIST_E_EXPECTING_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80041654_u32 as _);
pub const QPLIST_E_READ_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80041652_u32 as _);
pub const QPLIST_E_UNRECOGNIZED_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80041655_u32 as _);
pub const QPLIST_E_VECTORBYREF_USED_ALONE: windows_core::HRESULT = windows_core::HRESULT(0x8004165D_u32 as _);
pub const QPLIST_S_DUPLICATE: windows_core::HRESULT = windows_core::HRESULT(0x41679_u32 as _);
pub const QUERY_E_ALLNOISE: windows_core::HRESULT = windows_core::HRESULT(0x80041605_u32 as _);
pub const QUERY_E_DIR_ON_REMOVABLE_DRIVE: windows_core::HRESULT = windows_core::HRESULT(0x8004160B_u32 as _);
pub const QUERY_E_DUPLICATE_OUTPUT_COLUMN: windows_core::HRESULT = windows_core::HRESULT(0x80041608_u32 as _);
pub const QUERY_E_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80041600_u32 as _);
pub const QUERY_E_INVALIDCATEGORIZE: windows_core::HRESULT = windows_core::HRESULT(0x80041604_u32 as _);
pub const QUERY_E_INVALIDQUERY: windows_core::HRESULT = windows_core::HRESULT(0x80041601_u32 as _);
pub const QUERY_E_INVALIDRESTRICTION: windows_core::HRESULT = windows_core::HRESULT(0x80041602_u32 as _);
pub const QUERY_E_INVALIDSORT: windows_core::HRESULT = windows_core::HRESULT(0x80041603_u32 as _);
pub const QUERY_E_INVALID_DIRECTORY: windows_core::HRESULT = windows_core::HRESULT(0x8004160A_u32 as _);
pub const QUERY_E_INVALID_OUTPUT_COLUMN: windows_core::HRESULT = windows_core::HRESULT(0x80041609_u32 as _);
pub const QUERY_E_TIMEDOUT: windows_core::HRESULT = windows_core::HRESULT(0x80041607_u32 as _);
pub const QUERY_E_TOOCOMPLEX: windows_core::HRESULT = windows_core::HRESULT(0x80041606_u32 as _);
pub const QUERY_S_NO_QUERY: windows_core::HRESULT = windows_core::HRESULT(0x8004160C_u32 as _);
pub const QUTIL_E_CANT_CONVERT_VROOT: windows_core::HRESULT = windows_core::HRESULT(0x80041676_u32 as _);
pub const QUTIL_E_INVALID_CODEPAGE: windows_core::HRESULT = windows_core::HRESULT(0xC0041678_u32 as _);
pub const REGDB_E_BADTHREADINGMODEL: windows_core::HRESULT = windows_core::HRESULT(0x80040156_u32 as _);
pub const REGDB_E_CLASSNOTREG: windows_core::HRESULT = windows_core::HRESULT(0x80040154_u32 as _);
pub const REGDB_E_FIRST: i32 = -2147221168i32;
pub const REGDB_E_IIDNOTREG: windows_core::HRESULT = windows_core::HRESULT(0x80040155_u32 as _);
pub const REGDB_E_INVALIDVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80040153_u32 as _);
pub const REGDB_E_KEYMISSING: windows_core::HRESULT = windows_core::HRESULT(0x80040152_u32 as _);
pub const REGDB_E_LAST: i32 = -2147221153i32;
pub const REGDB_E_PACKAGEPOLICYVIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80040157_u32 as _);
pub const REGDB_E_READREGDB: windows_core::HRESULT = windows_core::HRESULT(0x80040150_u32 as _);
pub const REGDB_E_WRITEREGDB: windows_core::HRESULT = windows_core::HRESULT(0x80040151_u32 as _);
pub const REGDB_S_FIRST: i32 = 262480i32;
pub const REGDB_S_LAST: i32 = 262495i32;
pub const ROUTEBASE: u32 = 900u32;
pub const ROUTEBASEEND: u32 = 957u32;
pub const RO_E_BLOCKED_CROSS_ASTA_CALL: windows_core::HRESULT = windows_core::HRESULT(0x8000001F_u32 as _);
pub const RO_E_CANNOT_ACTIVATE_FULL_TRUST_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80000020_u32 as _);
pub const RO_E_CANNOT_ACTIVATE_UNIVERSAL_APPLICATION_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80000021_u32 as _);
pub const RO_E_CHANGE_NOTIFICATION_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80000015_u32 as _);
pub const RO_E_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0x80000013_u32 as _);
pub const RO_E_COMMITTED: windows_core::HRESULT = windows_core::HRESULT(0x8000001E_u32 as _);
pub const RO_E_ERROR_STRING_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80000016_u32 as _);
pub const RO_E_EXCLUSIVE_WRITE: windows_core::HRESULT = windows_core::HRESULT(0x80000014_u32 as _);
pub const RO_E_INVALID_METADATA_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80000012_u32 as _);
pub const RO_E_METADATA_INVALID_TYPE_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80000011_u32 as _);
pub const RO_E_METADATA_NAME_IS_NAMESPACE: windows_core::HRESULT = windows_core::HRESULT(0x80000010_u32 as _);
pub const RO_E_METADATA_NAME_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8000000F_u32 as _);
pub const RO_E_MUST_BE_AGILE: windows_core::HRESULT = windows_core::HRESULT(0x8000001C_u32 as _);
pub const RO_E_UNSUPPORTED_FROM_MTA: windows_core::HRESULT = windows_core::HRESULT(0x8000001D_u32 as _);
pub const RPC_E_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x8001011B_u32 as _);
pub const RPC_E_ATTEMPTED_MULTITHREAD: windows_core::HRESULT = windows_core::HRESULT(0x80010102_u32 as _);
pub const RPC_E_CALL_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80010002_u32 as _);
pub const RPC_E_CALL_COMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80010117_u32 as _);
pub const RPC_E_CALL_REJECTED: windows_core::HRESULT = windows_core::HRESULT(0x80010001_u32 as _);
pub const RPC_E_CANTCALLOUT_AGAIN: windows_core::HRESULT = windows_core::HRESULT(0x80010011_u32 as _);
pub const RPC_E_CANTCALLOUT_INASYNCCALL: windows_core::HRESULT = windows_core::HRESULT(0x80010004_u32 as _);
pub const RPC_E_CANTCALLOUT_INEXTERNALCALL: windows_core::HRESULT = windows_core::HRESULT(0x80010005_u32 as _);
pub const RPC_E_CANTCALLOUT_ININPUTSYNCCALL: windows_core::HRESULT = windows_core::HRESULT(0x8001010D_u32 as _);
pub const RPC_E_CANTPOST_INSENDCALL: windows_core::HRESULT = windows_core::HRESULT(0x80010003_u32 as _);
pub const RPC_E_CANTTRANSMIT_CALL: windows_core::HRESULT = windows_core::HRESULT(0x8001000A_u32 as _);
pub const RPC_E_CHANGED_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80010106_u32 as _);
pub const RPC_E_CLIENT_CANTMARSHAL_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8001000B_u32 as _);
pub const RPC_E_CLIENT_CANTUNMARSHAL_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8001000C_u32 as _);
pub const RPC_E_CLIENT_DIED: windows_core::HRESULT = windows_core::HRESULT(0x80010008_u32 as _);
pub const RPC_E_CONNECTION_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x80010006_u32 as _);
pub const RPC_E_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80010108_u32 as _);
pub const RPC_E_FAULT: windows_core::HRESULT = windows_core::HRESULT(0x80010104_u32 as _);
pub const RPC_E_FULLSIC_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80010121_u32 as _);
pub const RPC_E_INVALIDMETHOD: windows_core::HRESULT = windows_core::HRESULT(0x80010107_u32 as _);
pub const RPC_E_INVALID_CALLDATA: windows_core::HRESULT = windows_core::HRESULT(0x8001010C_u32 as _);
pub const RPC_E_INVALID_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8001000F_u32 as _);
pub const RPC_E_INVALID_DATAPACKET: windows_core::HRESULT = windows_core::HRESULT(0x80010009_u32 as _);
pub const RPC_E_INVALID_EXTENSION: windows_core::HRESULT = windows_core::HRESULT(0x80010112_u32 as _);
pub const RPC_E_INVALID_HEADER: windows_core::HRESULT = windows_core::HRESULT(0x80010111_u32 as _);
pub const RPC_E_INVALID_IPID: windows_core::HRESULT = windows_core::HRESULT(0x80010113_u32 as _);
pub const RPC_E_INVALID_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80010114_u32 as _);
pub const RPC_E_INVALID_OBJREF: windows_core::HRESULT = windows_core::HRESULT(0x8001011D_u32 as _);
pub const RPC_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80010010_u32 as _);
pub const RPC_E_INVALID_STD_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80010122_u32 as _);
pub const RPC_E_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80010103_u32 as _);
pub const RPC_E_NO_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x8001011E_u32 as _);
pub const RPC_E_NO_GOOD_SECURITY_PACKAGES: windows_core::HRESULT = windows_core::HRESULT(0x8001011A_u32 as _);
pub const RPC_E_NO_SYNC: windows_core::HRESULT = windows_core::HRESULT(0x80010120_u32 as _);
pub const RPC_E_OUT_OF_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x80010101_u32 as _);
pub const RPC_E_REMOTE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8001011C_u32 as _);
pub const RPC_E_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80010109_u32 as _);
pub const RPC_E_SERVERCALL_REJECTED: windows_core::HRESULT = windows_core::HRESULT(0x8001010B_u32 as _);
pub const RPC_E_SERVERCALL_RETRYLATER: windows_core::HRESULT = windows_core::HRESULT(0x8001010A_u32 as _);
pub const RPC_E_SERVERFAULT: windows_core::HRESULT = windows_core::HRESULT(0x80010105_u32 as _);
pub const RPC_E_SERVER_CANTMARSHAL_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8001000D_u32 as _);
pub const RPC_E_SERVER_CANTUNMARSHAL_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8001000E_u32 as _);
pub const RPC_E_SERVER_DIED: windows_core::HRESULT = windows_core::HRESULT(0x80010007_u32 as _);
pub const RPC_E_SERVER_DIED_DNE: windows_core::HRESULT = windows_core::HRESULT(0x80010012_u32 as _);
pub const RPC_E_SYS_CALL_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80010100_u32 as _);
pub const RPC_E_THREAD_NOT_INIT: windows_core::HRESULT = windows_core::HRESULT(0x8001010F_u32 as _);
pub const RPC_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8001011F_u32 as _);
pub const RPC_E_TOO_LATE: windows_core::HRESULT = windows_core::HRESULT(0x80010119_u32 as _);
pub const RPC_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8001FFFF_u32 as _);
pub const RPC_E_UNSECURE_CALL: windows_core::HRESULT = windows_core::HRESULT(0x80010118_u32 as _);
pub const RPC_E_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80010110_u32 as _);
pub const RPC_E_WRONG_THREAD: windows_core::HRESULT = windows_core::HRESULT(0x8001010E_u32 as _);
pub const RPC_NT_ADDRESS_ERROR: NTSTATUS = NTSTATUS(0xC0020045_u32 as _);
pub const RPC_NT_ALREADY_LISTENING: NTSTATUS = NTSTATUS(0xC002000E_u32 as _);
pub const RPC_NT_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(0xC002000C_u32 as _);
pub const RPC_NT_BAD_STUB_DATA: NTSTATUS = NTSTATUS(0xC003000C_u32 as _);
pub const RPC_NT_BINDING_HAS_NO_AUTH: NTSTATUS = NTSTATUS(0xC002002F_u32 as _);
pub const RPC_NT_BINDING_INCOMPLETE: NTSTATUS = NTSTATUS(0xC0020051_u32 as _);
pub const RPC_NT_BYTE_COUNT_TOO_SMALL: NTSTATUS = NTSTATUS(0xC003000B_u32 as _);
pub const RPC_NT_CALL_CANCELLED: NTSTATUS = NTSTATUS(0xC0020050_u32 as _);
pub const RPC_NT_CALL_FAILED: NTSTATUS = NTSTATUS(0xC002001B_u32 as _);
pub const RPC_NT_CALL_FAILED_DNE: NTSTATUS = NTSTATUS(0xC002001C_u32 as _);
pub const RPC_NT_CALL_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0020049_u32 as _);
pub const RPC_NT_CANNOT_SUPPORT: NTSTATUS = NTSTATUS(0xC0020041_u32 as _);
pub const RPC_NT_CANT_CREATE_ENDPOINT: NTSTATUS = NTSTATUS(0xC0020015_u32 as _);
pub const RPC_NT_COMM_FAILURE: NTSTATUS = NTSTATUS(0xC0020052_u32 as _);
pub const RPC_NT_COOKIE_AUTH_FAILED: NTSTATUS = NTSTATUS(0xC0020065_u32 as _);
pub const RPC_NT_DUPLICATE_ENDPOINT: NTSTATUS = NTSTATUS(0xC0020029_u32 as _);
pub const RPC_NT_ENTRY_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC002003D_u32 as _);
pub const RPC_NT_ENTRY_NOT_FOUND: NTSTATUS = NTSTATUS(0xC002003E_u32 as _);
pub const RPC_NT_ENUM_VALUE_OUT_OF_RANGE: NTSTATUS = NTSTATUS(0xC003000A_u32 as _);
pub const RPC_NT_FP_DIV_ZERO: NTSTATUS = NTSTATUS(0xC0020046_u32 as _);
pub const RPC_NT_FP_OVERFLOW: NTSTATUS = NTSTATUS(0xC0020048_u32 as _);
pub const RPC_NT_FP_UNDERFLOW: NTSTATUS = NTSTATUS(0xC0020047_u32 as _);
pub const RPC_NT_GROUP_MEMBER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC002004B_u32 as _);
pub const RPC_NT_INCOMPLETE_NAME: NTSTATUS = NTSTATUS(0xC0020038_u32 as _);
pub const RPC_NT_INTERFACE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC002003C_u32 as _);
pub const RPC_NT_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC0020043_u32 as _);
pub const RPC_NT_INVALID_ASYNC_CALL: NTSTATUS = NTSTATUS(0xC0020063_u32 as _);
pub const RPC_NT_INVALID_ASYNC_HANDLE: NTSTATUS = NTSTATUS(0xC0020062_u32 as _);
pub const RPC_NT_INVALID_AUTH_IDENTITY: NTSTATUS = NTSTATUS(0xC0020032_u32 as _);
pub const RPC_NT_INVALID_BINDING: NTSTATUS = NTSTATUS(0xC0020003_u32 as _);
pub const RPC_NT_INVALID_BOUND: NTSTATUS = NTSTATUS(0xC0020023_u32 as _);
pub const RPC_NT_INVALID_ENDPOINT_FORMAT: NTSTATUS = NTSTATUS(0xC0020007_u32 as _);
pub const RPC_NT_INVALID_ES_ACTION: NTSTATUS = NTSTATUS(0xC0030059_u32 as _);
pub const RPC_NT_INVALID_NAF_ID: NTSTATUS = NTSTATUS(0xC0020040_u32 as _);
pub const RPC_NT_INVALID_NAME_SYNTAX: NTSTATUS = NTSTATUS(0xC0020025_u32 as _);
pub const RPC_NT_INVALID_NETWORK_OPTIONS: NTSTATUS = NTSTATUS(0xC0020019_u32 as _);
pub const RPC_NT_INVALID_NET_ADDR: NTSTATUS = NTSTATUS(0xC0020008_u32 as _);
pub const RPC_NT_INVALID_OBJECT: NTSTATUS = NTSTATUS(0xC002004D_u32 as _);
pub const RPC_NT_INVALID_PIPE_OBJECT: NTSTATUS = NTSTATUS(0xC003005C_u32 as _);
pub const RPC_NT_INVALID_PIPE_OPERATION: NTSTATUS = NTSTATUS(0xC003005D_u32 as _);
pub const RPC_NT_INVALID_RPC_PROTSEQ: NTSTATUS = NTSTATUS(0xC0020005_u32 as _);
pub const RPC_NT_INVALID_STRING_BINDING: NTSTATUS = NTSTATUS(0xC0020001_u32 as _);
pub const RPC_NT_INVALID_STRING_UUID: NTSTATUS = NTSTATUS(0xC0020006_u32 as _);
pub const RPC_NT_INVALID_TAG: NTSTATUS = NTSTATUS(0xC0020022_u32 as _);
pub const RPC_NT_INVALID_TIMEOUT: NTSTATUS = NTSTATUS(0xC002000A_u32 as _);
pub const RPC_NT_INVALID_VERS_OPTION: NTSTATUS = NTSTATUS(0xC0020039_u32 as _);
pub const RPC_NT_MAX_CALLS_TOO_SMALL: NTSTATUS = NTSTATUS(0xC002002B_u32 as _);
pub const RPC_NT_NAME_SERVICE_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC002003F_u32 as _);
pub const RPC_NT_NOTHING_TO_EXPORT: NTSTATUS = NTSTATUS(0xC0020037_u32 as _);
pub const RPC_NT_NOT_ALL_OBJS_UNEXPORTED: NTSTATUS = NTSTATUS(0xC002003B_u32 as _);
pub const RPC_NT_NOT_CANCELLED: NTSTATUS = NTSTATUS(0xC0020058_u32 as _);
pub const RPC_NT_NOT_LISTENING: NTSTATUS = NTSTATUS(0xC0020010_u32 as _);
pub const RPC_NT_NOT_RPC_ERROR: NTSTATUS = NTSTATUS(0xC0020055_u32 as _);
pub const RPC_NT_NO_BINDINGS: NTSTATUS = NTSTATUS(0xC0020013_u32 as _);
pub const RPC_NT_NO_CALL_ACTIVE: NTSTATUS = NTSTATUS(0xC002001A_u32 as _);
pub const RPC_NT_NO_CONTEXT_AVAILABLE: NTSTATUS = NTSTATUS(0xC0020042_u32 as _);
pub const RPC_NT_NO_ENDPOINT_FOUND: NTSTATUS = NTSTATUS(0xC0020009_u32 as _);
pub const RPC_NT_NO_ENTRY_NAME: NTSTATUS = NTSTATUS(0xC0020024_u32 as _);
pub const RPC_NT_NO_INTERFACES: NTSTATUS = NTSTATUS(0xC002004F_u32 as _);
pub const RPC_NT_NO_MORE_BINDINGS: NTSTATUS = NTSTATUS(0xC002004A_u32 as _);
pub const RPC_NT_NO_MORE_ENTRIES: NTSTATUS = NTSTATUS(0xC0030001_u32 as _);
pub const RPC_NT_NO_MORE_MEMBERS: NTSTATUS = NTSTATUS(0xC002003A_u32 as _);
pub const RPC_NT_NO_PRINC_NAME: NTSTATUS = NTSTATUS(0xC0020054_u32 as _);
pub const RPC_NT_NO_PROTSEQS: NTSTATUS = NTSTATUS(0xC0020014_u32 as _);
pub const RPC_NT_NO_PROTSEQS_REGISTERED: NTSTATUS = NTSTATUS(0xC002000F_u32 as _);
pub const RPC_NT_NULL_REF_POINTER: NTSTATUS = NTSTATUS(0xC0030009_u32 as _);
pub const RPC_NT_OBJECT_NOT_FOUND: NTSTATUS = NTSTATUS(0xC002000B_u32 as _);
pub const RPC_NT_OUT_OF_RESOURCES: NTSTATUS = NTSTATUS(0xC0020016_u32 as _);
pub const RPC_NT_PIPE_CLOSED: NTSTATUS = NTSTATUS(0xC003005F_u32 as _);
pub const RPC_NT_PIPE_DISCIPLINE_ERROR: NTSTATUS = NTSTATUS(0xC0030060_u32 as _);
pub const RPC_NT_PIPE_EMPTY: NTSTATUS = NTSTATUS(0xC0030061_u32 as _);
pub const RPC_NT_PROCNUM_OUT_OF_RANGE: NTSTATUS = NTSTATUS(0xC002002E_u32 as _);
pub const RPC_NT_PROTOCOL_ERROR: NTSTATUS = NTSTATUS(0xC002001D_u32 as _);
pub const RPC_NT_PROTSEQ_NOT_FOUND: NTSTATUS = NTSTATUS(0xC002002D_u32 as _);
pub const RPC_NT_PROTSEQ_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0020004_u32 as _);
pub const RPC_NT_PROXY_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC0020064_u32 as _);
pub const RPC_NT_SEC_PKG_ERROR: NTSTATUS = NTSTATUS(0xC0020057_u32 as _);
pub const RPC_NT_SEND_INCOMPLETE: NTSTATUS = NTSTATUS(0x400200AF_u32 as _);
pub const RPC_NT_SERVER_TOO_BUSY: NTSTATUS = NTSTATUS(0xC0020018_u32 as _);
pub const RPC_NT_SERVER_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC0020017_u32 as _);
pub const RPC_NT_SS_CANNOT_GET_CALL_HANDLE: NTSTATUS = NTSTATUS(0xC0030008_u32 as _);
pub const RPC_NT_SS_CHAR_TRANS_OPEN_FAIL: NTSTATUS = NTSTATUS(0xC0030002_u32 as _);
pub const RPC_NT_SS_CHAR_TRANS_SHORT_FILE: NTSTATUS = NTSTATUS(0xC0030003_u32 as _);
pub const RPC_NT_SS_CONTEXT_DAMAGED: NTSTATUS = NTSTATUS(0xC0030006_u32 as _);
pub const RPC_NT_SS_CONTEXT_MISMATCH: NTSTATUS = NTSTATUS(0xC0030005_u32 as _);
pub const RPC_NT_SS_HANDLES_MISMATCH: NTSTATUS = NTSTATUS(0xC0030007_u32 as _);
pub const RPC_NT_SS_IN_NULL_CONTEXT: NTSTATUS = NTSTATUS(0xC0030004_u32 as _);
pub const RPC_NT_STRING_TOO_LONG: NTSTATUS = NTSTATUS(0xC002002C_u32 as _);
pub const RPC_NT_TYPE_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(0xC002000D_u32 as _);
pub const RPC_NT_UNKNOWN_AUTHN_LEVEL: NTSTATUS = NTSTATUS(0xC0020031_u32 as _);
pub const RPC_NT_UNKNOWN_AUTHN_SERVICE: NTSTATUS = NTSTATUS(0xC0020030_u32 as _);
pub const RPC_NT_UNKNOWN_AUTHN_TYPE: NTSTATUS = NTSTATUS(0xC002002A_u32 as _);
pub const RPC_NT_UNKNOWN_AUTHZ_SERVICE: NTSTATUS = NTSTATUS(0xC0020033_u32 as _);
pub const RPC_NT_UNKNOWN_IF: NTSTATUS = NTSTATUS(0xC0020012_u32 as _);
pub const RPC_NT_UNKNOWN_MGR_TYPE: NTSTATUS = NTSTATUS(0xC0020011_u32 as _);
pub const RPC_NT_UNSUPPORTED_AUTHN_LEVEL: NTSTATUS = NTSTATUS(0xC0020053_u32 as _);
pub const RPC_NT_UNSUPPORTED_NAME_SYNTAX: NTSTATUS = NTSTATUS(0xC0020026_u32 as _);
pub const RPC_NT_UNSUPPORTED_TRANS_SYN: NTSTATUS = NTSTATUS(0xC002001F_u32 as _);
pub const RPC_NT_UNSUPPORTED_TYPE: NTSTATUS = NTSTATUS(0xC0020021_u32 as _);
pub const RPC_NT_UUID_LOCAL_ONLY: NTSTATUS = NTSTATUS(0x40020056_u32 as _);
pub const RPC_NT_UUID_NO_ADDRESS: NTSTATUS = NTSTATUS(0xC0020028_u32 as _);
pub const RPC_NT_WRONG_ES_VERSION: NTSTATUS = NTSTATUS(0xC003005A_u32 as _);
pub const RPC_NT_WRONG_KIND_OF_BINDING: NTSTATUS = NTSTATUS(0xC0020002_u32 as _);
pub const RPC_NT_WRONG_PIPE_VERSION: NTSTATUS = NTSTATUS(0xC003005E_u32 as _);
pub const RPC_NT_WRONG_STUB_VERSION: NTSTATUS = NTSTATUS(0xC003005B_u32 as _);
pub const RPC_NT_ZERO_DIVIDE: NTSTATUS = NTSTATUS(0xC0020044_u32 as _);
pub const RPC_S_ACCESS_DENIED: i32 = -1073741790i32;
pub const RPC_S_ASYNC_CALL_PENDING: i32 = 259i32;
pub const RPC_S_BUFFER_TOO_SMALL: i32 = -1073741789i32;
pub const RPC_S_CALLPENDING: windows_core::HRESULT = windows_core::HRESULT(0x80010115_u32 as _);
pub const RPC_S_INVALID_ARG: i32 = -1073741811i32;
pub const RPC_S_INVALID_LEVEL: i32 = -1073741811i32;
pub const RPC_S_INVALID_SECURITY_DESC: i32 = -1073741703i32;
pub const RPC_S_NOT_ENOUGH_QUOTA: i32 = -1073741756i32;
pub const RPC_S_OUT_OF_MEMORY: i32 = -1073741801i32;
pub const RPC_S_OUT_OF_THREADS: i32 = -1073741801i32;
pub const RPC_S_SERVER_OUT_OF_MEMORY: i32 = -1073741307i32;
pub const RPC_S_UNKNOWN_PRINCIPAL: i32 = -1073741709i32;
pub const RPC_S_WAITONTIMER: windows_core::HRESULT = windows_core::HRESULT(0x80010116_u32 as _);
pub const RPC_X_BAD_STUB_DATA: i32 = 1783i32;
pub const RPC_X_BYTE_COUNT_TOO_SMALL: i32 = 1782i32;
pub const RPC_X_ENUM_VALUE_OUT_OF_RANGE: i32 = 1781i32;
pub const RPC_X_ENUM_VALUE_TOO_LARGE: i32 = 1781i32;
pub const RPC_X_INVALID_BOUND: i32 = -1073610717i32;
pub const RPC_X_INVALID_BUFFER: i32 = -1073741306i32;
pub const RPC_X_INVALID_ES_ACTION: i32 = 1827i32;
pub const RPC_X_INVALID_PIPE_OBJECT: i32 = 1830i32;
pub const RPC_X_INVALID_PIPE_OPERATION: i32 = 1831i32;
pub const RPC_X_INVALID_TAG: i32 = -1073610718i32;
pub const RPC_X_NO_MEMORY: i32 = -1073741801i32;
pub const RPC_X_NO_MORE_ENTRIES: i32 = 1772i32;
pub const RPC_X_NULL_REF_POINTER: i32 = 1780i32;
pub const RPC_X_PIPE_APP_MEMORY: i32 = -1073741801i32;
pub const RPC_X_PIPE_CLOSED: i32 = 1916i32;
pub const RPC_X_PIPE_DISCIPLINE_ERROR: i32 = 1917i32;
pub const RPC_X_PIPE_EMPTY: i32 = 1918i32;
pub const RPC_X_SS_CANNOT_GET_CALL_HANDLE: i32 = 1779i32;
pub const RPC_X_SS_CHAR_TRANS_OPEN_FAIL: i32 = 1773i32;
pub const RPC_X_SS_CHAR_TRANS_SHORT_FILE: i32 = 1774i32;
pub const RPC_X_SS_CONTEXT_DAMAGED: i32 = 1777i32;
pub const RPC_X_SS_CONTEXT_MISMATCH: i32 = -1073545211i32;
pub const RPC_X_SS_HANDLES_MISMATCH: i32 = 1778i32;
pub const RPC_X_SS_IN_NULL_CONTEXT: i32 = 1775i32;
pub const RPC_X_WRONG_ES_VERSION: i32 = 1828i32;
pub const RPC_X_WRONG_PIPE_ORDER: i32 = 1831i32;
pub const RPC_X_WRONG_PIPE_VERSION: i32 = 1832i32;
pub const RPC_X_WRONG_STUB_VERSION: i32 = 1829i32;
pub const SCARD_E_BAD_SEEK: windows_core::HRESULT = windows_core::HRESULT(0x80100029_u32 as _);
pub const SCARD_E_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80100002_u32 as _);
pub const SCARD_E_CANT_DISPOSE: windows_core::HRESULT = windows_core::HRESULT(0x8010000E_u32 as _);
pub const SCARD_E_CARD_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8010001C_u32 as _);
pub const SCARD_E_CERTIFICATE_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8010002D_u32 as _);
pub const SCARD_E_COMM_DATA_LOST: windows_core::HRESULT = windows_core::HRESULT(0x8010002F_u32 as _);
pub const SCARD_E_DIR_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80100023_u32 as _);
pub const SCARD_E_DUPLICATE_READER: windows_core::HRESULT = windows_core::HRESULT(0x8010001B_u32 as _);
pub const SCARD_E_FILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80100024_u32 as _);
pub const SCARD_E_ICC_CREATEORDER: windows_core::HRESULT = windows_core::HRESULT(0x80100021_u32 as _);
pub const SCARD_E_ICC_INSTALLATION: windows_core::HRESULT = windows_core::HRESULT(0x80100020_u32 as _);
pub const SCARD_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x80100008_u32 as _);
pub const SCARD_E_INVALID_ATR: windows_core::HRESULT = windows_core::HRESULT(0x80100015_u32 as _);
pub const SCARD_E_INVALID_CHV: windows_core::HRESULT = windows_core::HRESULT(0x8010002A_u32 as _);
pub const SCARD_E_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80100003_u32 as _);
pub const SCARD_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80100004_u32 as _);
pub const SCARD_E_INVALID_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x80100005_u32 as _);
pub const SCARD_E_INVALID_VALUE: windows_core::HRESULT = windows_core::HRESULT(0x80100011_u32 as _);
pub const SCARD_E_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80100010_u32 as _);
pub const SCARD_E_NOT_TRANSACTED: windows_core::HRESULT = windows_core::HRESULT(0x80100016_u32 as _);
pub const SCARD_E_NO_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x80100027_u32 as _);
pub const SCARD_E_NO_DIR: windows_core::HRESULT = windows_core::HRESULT(0x80100025_u32 as _);
pub const SCARD_E_NO_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80100026_u32 as _);
pub const SCARD_E_NO_KEY_CONTAINER: windows_core::HRESULT = windows_core::HRESULT(0x80100030_u32 as _);
pub const SCARD_E_NO_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80100006_u32 as _);
pub const SCARD_E_NO_PIN_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x80100033_u32 as _);
pub const SCARD_E_NO_READERS_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8010002E_u32 as _);
pub const SCARD_E_NO_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x8010001D_u32 as _);
pub const SCARD_E_NO_SMARTCARD: windows_core::HRESULT = windows_core::HRESULT(0x8010000C_u32 as _);
pub const SCARD_E_NO_SUCH_CERTIFICATE: windows_core::HRESULT = windows_core::HRESULT(0x8010002C_u32 as _);
pub const SCARD_E_PCI_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80100019_u32 as _);
pub const SCARD_E_PIN_CACHE_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x80100032_u32 as _);
pub const SCARD_E_PROTO_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8010000F_u32 as _);
pub const SCARD_E_READER_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80100017_u32 as _);
pub const SCARD_E_READER_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8010001A_u32 as _);
pub const SCARD_E_READ_ONLY_CARD: windows_core::HRESULT = windows_core::HRESULT(0x80100034_u32 as _);
pub const SCARD_E_SERVER_TOO_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x80100031_u32 as _);
pub const SCARD_E_SERVICE_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0x8010001E_u32 as _);
pub const SCARD_E_SHARING_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x8010000B_u32 as _);
pub const SCARD_E_SYSTEM_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80100012_u32 as _);
pub const SCARD_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8010000A_u32 as _);
pub const SCARD_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8010001F_u32 as _);
pub const SCARD_E_UNKNOWN_CARD: windows_core::HRESULT = windows_core::HRESULT(0x8010000D_u32 as _);
pub const SCARD_E_UNKNOWN_READER: windows_core::HRESULT = windows_core::HRESULT(0x80100009_u32 as _);
pub const SCARD_E_UNKNOWN_RES_MNG: windows_core::HRESULT = windows_core::HRESULT(0x8010002B_u32 as _);
pub const SCARD_E_UNSUPPORTED_FEATURE: windows_core::HRESULT = windows_core::HRESULT(0x80100022_u32 as _);
pub const SCARD_E_WRITE_TOO_MANY: windows_core::HRESULT = windows_core::HRESULT(0x80100028_u32 as _);
pub const SCARD_F_COMM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80100013_u32 as _);
pub const SCARD_F_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80100001_u32 as _);
pub const SCARD_F_UNKNOWN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80100014_u32 as _);
pub const SCARD_F_WAITED_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80100007_u32 as _);
pub const SCARD_P_SHUTDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80100018_u32 as _);
pub const SCARD_W_CACHE_ITEM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80100070_u32 as _);
pub const SCARD_W_CACHE_ITEM_STALE: windows_core::HRESULT = windows_core::HRESULT(0x80100071_u32 as _);
pub const SCARD_W_CACHE_ITEM_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x80100072_u32 as _);
pub const SCARD_W_CANCELLED_BY_USER: windows_core::HRESULT = windows_core::HRESULT(0x8010006E_u32 as _);
pub const SCARD_W_CARD_NOT_AUTHENTICATED: windows_core::HRESULT = windows_core::HRESULT(0x8010006F_u32 as _);
pub const SCARD_W_CHV_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8010006C_u32 as _);
pub const SCARD_W_EOF: windows_core::HRESULT = windows_core::HRESULT(0x8010006D_u32 as _);
pub const SCARD_W_REMOVED_CARD: windows_core::HRESULT = windows_core::HRESULT(0x80100069_u32 as _);
pub const SCARD_W_RESET_CARD: windows_core::HRESULT = windows_core::HRESULT(0x80100068_u32 as _);
pub const SCARD_W_SECURITY_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x8010006A_u32 as _);
pub const SCARD_W_UNPOWERED_CARD: windows_core::HRESULT = windows_core::HRESULT(0x80100067_u32 as _);
pub const SCARD_W_UNRESPONSIVE_CARD: windows_core::HRESULT = windows_core::HRESULT(0x80100066_u32 as _);
pub const SCARD_W_UNSUPPORTED_CARD: windows_core::HRESULT = windows_core::HRESULT(0x80100065_u32 as _);
pub const SCARD_W_WRONG_CHV: windows_core::HRESULT = windows_core::HRESULT(0x8010006B_u32 as _);
pub const SCHED_E_ACCOUNT_DBASE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80041311_u32 as _);
pub const SCHED_E_ACCOUNT_INFORMATION_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x8004130F_u32 as _);
pub const SCHED_E_ACCOUNT_NAME_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80041310_u32 as _);
pub const SCHED_E_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004131F_u32 as _);
pub const SCHED_E_CANNOT_OPEN_TASK: windows_core::HRESULT = windows_core::HRESULT(0x8004130D_u32 as _);
pub const SCHED_E_DEPRECATED_FEATURE_USED: windows_core::HRESULT = windows_core::HRESULT(0x80041330_u32 as _);
pub const SCHED_E_INVALIDVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80041318_u32 as _);
pub const SCHED_E_INVALID_TASK: windows_core::HRESULT = windows_core::HRESULT(0x8004130E_u32 as _);
pub const SCHED_E_INVALID_TASK_HASH: windows_core::HRESULT = windows_core::HRESULT(0x80041321_u32 as _);
pub const SCHED_E_MALFORMEDXML: windows_core::HRESULT = windows_core::HRESULT(0x8004131A_u32 as _);
pub const SCHED_E_MISSINGNODE: windows_core::HRESULT = windows_core::HRESULT(0x80041319_u32 as _);
pub const SCHED_E_NAMESPACE: windows_core::HRESULT = windows_core::HRESULT(0x80041317_u32 as _);
pub const SCHED_E_NO_SECURITY_SERVICES: windows_core::HRESULT = windows_core::HRESULT(0x80041312_u32 as _);
pub const SCHED_E_PAST_END_BOUNDARY: windows_core::HRESULT = windows_core::HRESULT(0x8004131E_u32 as _);
pub const SCHED_E_SERVICE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80041322_u32 as _);
pub const SCHED_E_SERVICE_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x8004130C_u32 as _);
pub const SCHED_E_SERVICE_NOT_LOCALSYSTEM: i32 = 6200i32;
pub const SCHED_E_SERVICE_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80041315_u32 as _);
pub const SCHED_E_SERVICE_TOO_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x80041323_u32 as _);
pub const SCHED_E_START_ON_DEMAND: windows_core::HRESULT = windows_core::HRESULT(0x80041328_u32 as _);
pub const SCHED_E_TASK_ATTEMPTED: windows_core::HRESULT = windows_core::HRESULT(0x80041324_u32 as _);
pub const SCHED_E_TASK_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80041326_u32 as _);
pub const SCHED_E_TASK_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x8004130A_u32 as _);
pub const SCHED_E_TASK_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004130B_u32 as _);
pub const SCHED_E_TASK_NOT_UBPM_COMPAT: windows_core::HRESULT = windows_core::HRESULT(0x80041329_u32 as _);
pub const SCHED_E_TASK_NOT_V1_COMPAT: windows_core::HRESULT = windows_core::HRESULT(0x80041327_u32 as _);
pub const SCHED_E_TOO_MANY_NODES: windows_core::HRESULT = windows_core::HRESULT(0x8004131D_u32 as _);
pub const SCHED_E_TRIGGER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80041309_u32 as _);
pub const SCHED_E_UNEXPECTEDNODE: windows_core::HRESULT = windows_core::HRESULT(0x80041316_u32 as _);
pub const SCHED_E_UNKNOWN_OBJECT_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80041313_u32 as _);
pub const SCHED_E_UNSUPPORTED_ACCOUNT_OPTION: windows_core::HRESULT = windows_core::HRESULT(0x80041314_u32 as _);
pub const SCHED_E_USER_NOT_LOGGED_ON: windows_core::HRESULT = windows_core::HRESULT(0x80041320_u32 as _);
pub const SCHED_S_BATCH_LOGON_PROBLEM: windows_core::HRESULT = windows_core::HRESULT(0x4131C_u32 as _);
pub const SCHED_S_EVENT_TRIGGER: windows_core::HRESULT = windows_core::HRESULT(0x41308_u32 as _);
pub const SCHED_S_SOME_TRIGGERS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x4131B_u32 as _);
pub const SCHED_S_TASK_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x41302_u32 as _);
pub const SCHED_S_TASK_HAS_NOT_RUN: windows_core::HRESULT = windows_core::HRESULT(0x41303_u32 as _);
pub const SCHED_S_TASK_NOT_SCHEDULED: windows_core::HRESULT = windows_core::HRESULT(0x41305_u32 as _);
pub const SCHED_S_TASK_NO_MORE_RUNS: windows_core::HRESULT = windows_core::HRESULT(0x41304_u32 as _);
pub const SCHED_S_TASK_NO_VALID_TRIGGERS: windows_core::HRESULT = windows_core::HRESULT(0x41307_u32 as _);
pub const SCHED_S_TASK_QUEUED: windows_core::HRESULT = windows_core::HRESULT(0x41325_u32 as _);
pub const SCHED_S_TASK_READY: windows_core::HRESULT = windows_core::HRESULT(0x41300_u32 as _);
pub const SCHED_S_TASK_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x41301_u32 as _);
pub const SCHED_S_TASK_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x41306_u32 as _);
pub const SDIAG_E_CANCELLED: i32 = -2143551232i32;
pub const SDIAG_E_CANNOTRUN: i32 = -2143551224i32;
pub const SDIAG_E_DISABLED: i32 = -2143551226i32;
pub const SDIAG_E_MANAGEDHOST: i32 = -2143551229i32;
pub const SDIAG_E_NOVERIFIER: i32 = -2143551228i32;
pub const SDIAG_E_POWERSHELL: i32 = -2143551230i32;
pub const SDIAG_E_RESOURCE: i32 = -2143551222i32;
pub const SDIAG_E_ROOTCAUSE: i32 = -2143551221i32;
pub const SDIAG_E_SCRIPT: i32 = -2143551231i32;
pub const SDIAG_E_TRUST: i32 = -2143551225i32;
pub const SDIAG_E_VERSION: i32 = -2143551223i32;
pub const SDIAG_S_CANNOTRUN: i32 = 3932421i32;
pub const SEARCH_E_NOMONIKER: windows_core::HRESULT = windows_core::HRESULT(0x800416A1_u32 as _);
pub const SEARCH_E_NOREGION: windows_core::HRESULT = windows_core::HRESULT(0x800416A2_u32 as _);
pub const SEARCH_S_NOMOREHITS: windows_core::HRESULT = windows_core::HRESULT(0x416A0_u32 as _);
pub const SEC_E_ALGORITHM_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80090331_u32 as _);
pub const SEC_E_APPLICATION_PROTOCOL_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80090367_u32 as _);
pub const SEC_E_BAD_BINDINGS: windows_core::HRESULT = windows_core::HRESULT(0x80090346_u32 as _);
pub const SEC_E_BAD_PKGID: windows_core::HRESULT = windows_core::HRESULT(0x80090316_u32 as _);
pub const SEC_E_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80090321_u32 as _);
pub const SEC_E_CANNOT_INSTALL: windows_core::HRESULT = windows_core::HRESULT(0x80090307_u32 as _);
pub const SEC_E_CANNOT_PACK: windows_core::HRESULT = windows_core::HRESULT(0x80090309_u32 as _);
pub const SEC_E_CERT_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x80090328_u32 as _);
pub const SEC_E_CERT_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x80090327_u32 as _);
pub const SEC_E_CERT_WRONG_USAGE: windows_core::HRESULT = windows_core::HRESULT(0x80090349_u32 as _);
pub const SEC_E_CONTEXT_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x80090317_u32 as _);
pub const SEC_E_CROSSREALM_DELEGATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80090357_u32 as _);
pub const SEC_E_CRYPTO_SYSTEM_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80090337_u32 as _);
pub const SEC_E_DECRYPT_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80090330_u32 as _);
pub const SEC_E_DELEGATION_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x8009035E_u32 as _);
pub const SEC_E_DELEGATION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80090345_u32 as _);
pub const SEC_E_DOWNGRADE_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x80090350_u32 as _);
pub const SEC_E_ENCRYPT_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80090329_u32 as _);
pub const SEC_E_EXT_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x8009036A_u32 as _);
pub const SEC_E_ILLEGAL_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x80090326_u32 as _);
pub const SEC_E_INCOMPLETE_CREDENTIALS: windows_core::HRESULT = windows_core::HRESULT(0x80090320_u32 as _);
pub const SEC_E_INCOMPLETE_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x80090318_u32 as _);
pub const SEC_E_INSUFFICIENT_BUFFERS: windows_core::HRESULT = windows_core::HRESULT(0x8009036B_u32 as _);
pub const SEC_E_INSUFFICIENT_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80090300_u32 as _);
pub const SEC_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80090304_u32 as _);
pub const SEC_E_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80090301_u32 as _);
pub const SEC_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x8009035D_u32 as _);
pub const SEC_E_INVALID_TOKEN: windows_core::HRESULT = windows_core::HRESULT(0x80090308_u32 as _);
pub const SEC_E_INVALID_UPN_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80090369_u32 as _);
pub const SEC_E_ISSUING_CA_UNTRUSTED: windows_core::HRESULT = windows_core::HRESULT(0x80090352_u32 as _);
pub const SEC_E_ISSUING_CA_UNTRUSTED_KDC: windows_core::HRESULT = windows_core::HRESULT(0x80090359_u32 as _);
pub const SEC_E_KDC_CERT_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x8009035A_u32 as _);
pub const SEC_E_KDC_CERT_REVOKED: windows_core::HRESULT = windows_core::HRESULT(0x8009035B_u32 as _);
pub const SEC_E_KDC_INVALID_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80090340_u32 as _);
pub const SEC_E_KDC_UNABLE_TO_REFER: windows_core::HRESULT = windows_core::HRESULT(0x80090341_u32 as _);
pub const SEC_E_KDC_UNKNOWN_ETYPE: windows_core::HRESULT = windows_core::HRESULT(0x80090342_u32 as _);
pub const SEC_E_LOGON_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x8009030C_u32 as _);
pub const SEC_E_MAX_REFERRALS_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80090338_u32 as _);
pub const SEC_E_MESSAGE_ALTERED: windows_core::HRESULT = windows_core::HRESULT(0x8009030F_u32 as _);
pub const SEC_E_MULTIPLE_ACCOUNTS: windows_core::HRESULT = windows_core::HRESULT(0x80090347_u32 as _);
pub const SEC_E_MUST_BE_KDC: windows_core::HRESULT = windows_core::HRESULT(0x80090339_u32 as _);
pub const SEC_E_MUTUAL_AUTH_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80090363_u32 as _);
pub const SEC_E_NOT_OWNER: windows_core::HRESULT = windows_core::HRESULT(0x80090306_u32 as _);
pub const SEC_E_NOT_SUPPORTED: i32 = -2146893054i32;
pub const SEC_E_NO_AUTHENTICATING_AUTHORITY: windows_core::HRESULT = windows_core::HRESULT(0x80090311_u32 as _);
pub const SEC_E_NO_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80090361_u32 as _);
pub const SEC_E_NO_CREDENTIALS: windows_core::HRESULT = windows_core::HRESULT(0x8009030E_u32 as _);
pub const SEC_E_NO_IMPERSONATION: windows_core::HRESULT = windows_core::HRESULT(0x8009030B_u32 as _);
pub const SEC_E_NO_IP_ADDRESSES: windows_core::HRESULT = windows_core::HRESULT(0x80090335_u32 as _);
pub const SEC_E_NO_KERB_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80090348_u32 as _);
pub const SEC_E_NO_PA_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8009033C_u32 as _);
pub const SEC_E_NO_S4U_PROT_SUPPORT: windows_core::HRESULT = windows_core::HRESULT(0x80090356_u32 as _);
pub const SEC_E_NO_SPM: i32 = -2146893052i32;
pub const SEC_E_NO_TGT_REPLY: windows_core::HRESULT = windows_core::HRESULT(0x80090334_u32 as _);
pub const SEC_E_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const SEC_E_ONLY_HTTPS_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80090365_u32 as _);
pub const SEC_E_OUT_OF_SEQUENCE: windows_core::HRESULT = windows_core::HRESULT(0x80090310_u32 as _);
pub const SEC_E_PKINIT_CLIENT_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80090354_u32 as _);
pub const SEC_E_PKINIT_NAME_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8009033D_u32 as _);
pub const SEC_E_PKU2U_CERT_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80090362_u32 as _);
pub const SEC_E_POLICY_NLTM_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x8009035F_u32 as _);
pub const SEC_E_QOP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8009030A_u32 as _);
pub const SEC_E_REVOCATION_OFFLINE_C: windows_core::HRESULT = windows_core::HRESULT(0x80090353_u32 as _);
pub const SEC_E_REVOCATION_OFFLINE_KDC: windows_core::HRESULT = windows_core::HRESULT(0x80090358_u32 as _);
pub const SEC_E_SECPKG_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80090305_u32 as _);
pub const SEC_E_SECURITY_QOS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80090332_u32 as _);
pub const SEC_E_SHUTDOWN_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8009033F_u32 as _);
pub const SEC_E_SMARTCARD_CERT_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x80090355_u32 as _);
pub const SEC_E_SMARTCARD_CERT_REVOKED: windows_core::HRESULT = windows_core::HRESULT(0x80090351_u32 as _);
pub const SEC_E_SMARTCARD_LOGON_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8009033E_u32 as _);
pub const SEC_E_STRONG_CRYPTO_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8009033A_u32 as _);
pub const SEC_E_TARGET_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x80090303_u32 as _);
pub const SEC_E_TIME_SKEW: windows_core::HRESULT = windows_core::HRESULT(0x80090324_u32 as _);
pub const SEC_E_TOO_MANY_PRINCIPALS: windows_core::HRESULT = windows_core::HRESULT(0x8009033B_u32 as _);
pub const SEC_E_UNFINISHED_CONTEXT_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x80090333_u32 as _);
pub const SEC_E_UNKNOWN_CREDENTIALS: windows_core::HRESULT = windows_core::HRESULT(0x8009030D_u32 as _);
pub const SEC_E_UNSUPPORTED_FUNCTION: windows_core::HRESULT = windows_core::HRESULT(0x80090302_u32 as _);
pub const SEC_E_UNSUPPORTED_PREAUTH: windows_core::HRESULT = windows_core::HRESULT(0x80090343_u32 as _);
pub const SEC_E_UNTRUSTED_ROOT: windows_core::HRESULT = windows_core::HRESULT(0x80090325_u32 as _);
pub const SEC_E_WRONG_CREDENTIAL_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80090336_u32 as _);
pub const SEC_E_WRONG_PRINCIPAL: windows_core::HRESULT = windows_core::HRESULT(0x80090322_u32 as _);
pub const SEC_I_ASYNC_CALL_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x90368_u32 as _);
pub const SEC_I_COMPLETE_AND_CONTINUE: windows_core::HRESULT = windows_core::HRESULT(0x90314_u32 as _);
pub const SEC_I_COMPLETE_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x90313_u32 as _);
pub const SEC_I_CONTEXT_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x90317_u32 as _);
pub const SEC_I_CONTINUE_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x90312_u32 as _);
pub const SEC_I_CONTINUE_NEEDED_MESSAGE_OK: windows_core::HRESULT = windows_core::HRESULT(0x90366_u32 as _);
pub const SEC_I_GENERIC_EXTENSION_RECEIVED: windows_core::HRESULT = windows_core::HRESULT(0x90316_u32 as _);
pub const SEC_I_INCOMPLETE_CREDENTIALS: windows_core::HRESULT = windows_core::HRESULT(0x90320_u32 as _);
pub const SEC_I_LOCAL_LOGON: windows_core::HRESULT = windows_core::HRESULT(0x90315_u32 as _);
pub const SEC_I_MESSAGE_FRAGMENT: windows_core::HRESULT = windows_core::HRESULT(0x90364_u32 as _);
pub const SEC_I_NO_LSA_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x90323_u32 as _);
pub const SEC_I_NO_RENEGOTIATION: windows_core::HRESULT = windows_core::HRESULT(0x90360_u32 as _);
pub const SEC_I_RENEGOTIATE: windows_core::HRESULT = windows_core::HRESULT(0x90321_u32 as _);
pub const SEC_I_SIGNATURE_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x9035C_u32 as _);
pub const SEVERITY_ERROR: u32 = 1u32;
pub const SEVERITY_SUCCESS: u32 = 0u32;
pub const SPAPI_E_AUTHENTICODE_DISALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x800F0240_u32 as _);
pub const SPAPI_E_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: windows_core::HRESULT = windows_core::HRESULT(0x800F0243_u32 as _);
pub const SPAPI_E_AUTHENTICODE_TRUSTED_PUBLISHER: windows_core::HRESULT = windows_core::HRESULT(0x800F0241_u32 as _);
pub const SPAPI_E_AUTHENTICODE_TRUST_NOT_ESTABLISHED: windows_core::HRESULT = windows_core::HRESULT(0x800F0242_u32 as _);
pub const SPAPI_E_BAD_INTERFACE_INSTALLSECT: windows_core::HRESULT = windows_core::HRESULT(0x800F021D_u32 as _);
pub const SPAPI_E_BAD_SECTION_NAME_LINE: windows_core::HRESULT = windows_core::HRESULT(0x800F0001_u32 as _);
pub const SPAPI_E_BAD_SERVICE_INSTALLSECT: windows_core::HRESULT = windows_core::HRESULT(0x800F0217_u32 as _);
pub const SPAPI_E_CANT_LOAD_CLASS_ICON: windows_core::HRESULT = windows_core::HRESULT(0x800F020C_u32 as _);
pub const SPAPI_E_CANT_REMOVE_DEVINST: windows_core::HRESULT = windows_core::HRESULT(0x800F0232_u32 as _);
pub const SPAPI_E_CLASS_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x800F0201_u32 as _);
pub const SPAPI_E_DEVICE_INSTALLER_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x800F0246_u32 as _);
pub const SPAPI_E_DEVICE_INSTALL_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x800F0248_u32 as _);
pub const SPAPI_E_DEVICE_INTERFACE_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x800F021B_u32 as _);
pub const SPAPI_E_DEVICE_INTERFACE_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x800F021C_u32 as _);
pub const SPAPI_E_DEVINFO_DATA_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x800F0213_u32 as _);
pub const SPAPI_E_DEVINFO_LIST_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x800F0212_u32 as _);
pub const SPAPI_E_DEVINFO_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x800F0208_u32 as _);
pub const SPAPI_E_DEVINSTALL_QUEUE_NONNATIVE: windows_core::HRESULT = windows_core::HRESULT(0x800F0230_u32 as _);
pub const SPAPI_E_DEVINST_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x800F0207_u32 as _);
pub const SPAPI_E_DI_BAD_PATH: windows_core::HRESULT = windows_core::HRESULT(0x800F0214_u32 as _);
pub const SPAPI_E_DI_DONT_INSTALL: windows_core::HRESULT = windows_core::HRESULT(0x800F022B_u32 as _);
pub const SPAPI_E_DI_DO_DEFAULT: windows_core::HRESULT = windows_core::HRESULT(0x800F020E_u32 as _);
pub const SPAPI_E_DI_FUNCTION_OBSOLETE: windows_core::HRESULT = windows_core::HRESULT(0x800F023E_u32 as _);
pub const SPAPI_E_DI_NOFILECOPY: windows_core::HRESULT = windows_core::HRESULT(0x800F020F_u32 as _);
pub const SPAPI_E_DI_POSTPROCESSING_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x800F0226_u32 as _);
pub const SPAPI_E_DRIVER_INSTALL_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x800F0249_u32 as _);
pub const SPAPI_E_DRIVER_NONNATIVE: windows_core::HRESULT = windows_core::HRESULT(0x800F0234_u32 as _);
pub const SPAPI_E_DRIVER_STORE_ADD_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800F0247_u32 as _);
pub const SPAPI_E_DRIVER_STORE_DELETE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800F024C_u32 as _);
pub const SPAPI_E_DUPLICATE_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x800F0202_u32 as _);
pub const SPAPI_E_ERROR_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x800F1000_u32 as _);
pub const SPAPI_E_EXPECTED_SECTION_NAME: windows_core::HRESULT = windows_core::HRESULT(0x800F0000_u32 as _);
pub const SPAPI_E_FILEQUEUE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x800F0216_u32 as _);
pub const SPAPI_E_FILE_HASH_NOT_IN_CATALOG: windows_core::HRESULT = windows_core::HRESULT(0x800F024B_u32 as _);
pub const SPAPI_E_GENERAL_SYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x800F0003_u32 as _);
pub const SPAPI_E_INCORRECTLY_COPIED_INF: windows_core::HRESULT = windows_core::HRESULT(0x800F0237_u32 as _);
pub const SPAPI_E_INF_IN_USE_BY_DEVICES: windows_core::HRESULT = windows_core::HRESULT(0x800F023D_u32 as _);
pub const SPAPI_E_INVALID_CLASS: windows_core::HRESULT = windows_core::HRESULT(0x800F0206_u32 as _);
pub const SPAPI_E_INVALID_CLASS_INSTALLER: windows_core::HRESULT = windows_core::HRESULT(0x800F020D_u32 as _);
pub const SPAPI_E_INVALID_COINSTALLER: windows_core::HRESULT = windows_core::HRESULT(0x800F0227_u32 as _);
pub const SPAPI_E_INVALID_DEVINST_NAME: windows_core::HRESULT = windows_core::HRESULT(0x800F0205_u32 as _);
pub const SPAPI_E_INVALID_FILTER_DRIVER: windows_core::HRESULT = windows_core::HRESULT(0x800F022C_u32 as _);
pub const SPAPI_E_INVALID_HWPROFILE: windows_core::HRESULT = windows_core::HRESULT(0x800F0210_u32 as _);
pub const SPAPI_E_INVALID_INF_LOGCONFIG: windows_core::HRESULT = windows_core::HRESULT(0x800F022A_u32 as _);
pub const SPAPI_E_INVALID_MACHINENAME: windows_core::HRESULT = windows_core::HRESULT(0x800F0220_u32 as _);
pub const SPAPI_E_INVALID_PROPPAGE_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x800F0224_u32 as _);
pub const SPAPI_E_INVALID_REFERENCE_STRING: windows_core::HRESULT = windows_core::HRESULT(0x800F021F_u32 as _);
pub const SPAPI_E_INVALID_REG_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x800F0209_u32 as _);
pub const SPAPI_E_INVALID_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x800F0233_u32 as _);
pub const SPAPI_E_IN_WOW64: windows_core::HRESULT = windows_core::HRESULT(0x800F0235_u32 as _);
pub const SPAPI_E_KEY_DOES_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x800F0204_u32 as _);
pub const SPAPI_E_LINE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x800F0102_u32 as _);
pub const SPAPI_E_MACHINE_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x800F0222_u32 as _);
pub const SPAPI_E_NON_WINDOWS_DRIVER: windows_core::HRESULT = windows_core::HRESULT(0x800F022E_u32 as _);
pub const SPAPI_E_NON_WINDOWS_NT_DRIVER: windows_core::HRESULT = windows_core::HRESULT(0x800F022D_u32 as _);
pub const SPAPI_E_NOT_AN_INSTALLED_OEM_INF: windows_core::HRESULT = windows_core::HRESULT(0x800F023C_u32 as _);
pub const SPAPI_E_NOT_DISABLEABLE: windows_core::HRESULT = windows_core::HRESULT(0x800F0231_u32 as _);
pub const SPAPI_E_NO_ASSOCIATED_CLASS: windows_core::HRESULT = windows_core::HRESULT(0x800F0200_u32 as _);
pub const SPAPI_E_NO_ASSOCIATED_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x800F0219_u32 as _);
pub const SPAPI_E_NO_AUTHENTICODE_CATALOG: windows_core::HRESULT = windows_core::HRESULT(0x800F023F_u32 as _);
pub const SPAPI_E_NO_BACKUP: windows_core::HRESULT = windows_core::HRESULT(0x800F0103_u32 as _);
pub const SPAPI_E_NO_CATALOG_FOR_OEM_INF: windows_core::HRESULT = windows_core::HRESULT(0x800F022F_u32 as _);
pub const SPAPI_E_NO_CLASSINSTALL_PARAMS: windows_core::HRESULT = windows_core::HRESULT(0x800F0215_u32 as _);
pub const SPAPI_E_NO_CLASS_DRIVER_LIST: windows_core::HRESULT = windows_core::HRESULT(0x800F0218_u32 as _);
pub const SPAPI_E_NO_COMPAT_DRIVERS: windows_core::HRESULT = windows_core::HRESULT(0x800F0228_u32 as _);
pub const SPAPI_E_NO_CONFIGMGR_SERVICES: windows_core::HRESULT = windows_core::HRESULT(0x800F0223_u32 as _);
pub const SPAPI_E_NO_DEFAULT_DEVICE_INTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x800F021A_u32 as _);
pub const SPAPI_E_NO_DEVICE_ICON: windows_core::HRESULT = windows_core::HRESULT(0x800F0229_u32 as _);
pub const SPAPI_E_NO_DEVICE_SELECTED: windows_core::HRESULT = windows_core::HRESULT(0x800F0211_u32 as _);
pub const SPAPI_E_NO_DRIVER_SELECTED: windows_core::HRESULT = windows_core::HRESULT(0x800F0203_u32 as _);
pub const SPAPI_E_NO_INF: windows_core::HRESULT = windows_core::HRESULT(0x800F020A_u32 as _);
pub const SPAPI_E_NO_SUCH_DEVICE_INTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x800F0225_u32 as _);
pub const SPAPI_E_NO_SUCH_DEVINST: windows_core::HRESULT = windows_core::HRESULT(0x800F020B_u32 as _);
pub const SPAPI_E_NO_SUCH_INTERFACE_CLASS: windows_core::HRESULT = windows_core::HRESULT(0x800F021E_u32 as _);
pub const SPAPI_E_ONLY_VALIDATE_VIA_AUTHENTICODE: windows_core::HRESULT = windows_core::HRESULT(0x800F0245_u32 as _);
pub const SPAPI_E_PNP_REGISTRY_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x800F023A_u32 as _);
pub const SPAPI_E_REMOTE_COMM_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x800F0221_u32 as _);
pub const SPAPI_E_REMOTE_REQUEST_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x800F023B_u32 as _);
pub const SPAPI_E_SCE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x800F0238_u32 as _);
pub const SPAPI_E_SECTION_NAME_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x800F0002_u32 as _);
pub const SPAPI_E_SECTION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x800F0101_u32 as _);
pub const SPAPI_E_SET_SYSTEM_RESTORE_POINT: windows_core::HRESULT = windows_core::HRESULT(0x800F0236_u32 as _);
pub const SPAPI_E_SIGNATURE_OSATTRIBUTE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x800F0244_u32 as _);
pub const SPAPI_E_UNKNOWN_EXCEPTION: windows_core::HRESULT = windows_core::HRESULT(0x800F0239_u32 as _);
pub const SPAPI_E_UNRECOVERABLE_STACK_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x800F0300_u32 as _);
pub const SPAPI_E_WRONG_INF_STYLE: windows_core::HRESULT = windows_core::HRESULT(0x800F0100_u32 as _);
pub const SPAPI_E_WRONG_INF_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x800F024A_u32 as _);
pub const SQLITE_E_ABORT: windows_core::HRESULT = windows_core::HRESULT(0x87AF0004_u32 as _);
pub const SQLITE_E_ABORT_ROLLBACK: windows_core::HRESULT = windows_core::HRESULT(0x87AF0204_u32 as _);
pub const SQLITE_E_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x87AF0017_u32 as _);
pub const SQLITE_E_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x87AF0005_u32 as _);
pub const SQLITE_E_BUSY_RECOVERY: windows_core::HRESULT = windows_core::HRESULT(0x87AF0105_u32 as _);
pub const SQLITE_E_BUSY_SNAPSHOT: windows_core::HRESULT = windows_core::HRESULT(0x87AF0205_u32 as _);
pub const SQLITE_E_CANTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x87AF000E_u32 as _);
pub const SQLITE_E_CANTOPEN_CONVPATH: windows_core::HRESULT = windows_core::HRESULT(0x87AF040E_u32 as _);
pub const SQLITE_E_CANTOPEN_FULLPATH: windows_core::HRESULT = windows_core::HRESULT(0x87AF030E_u32 as _);
pub const SQLITE_E_CANTOPEN_ISDIR: windows_core::HRESULT = windows_core::HRESULT(0x87AF020E_u32 as _);
pub const SQLITE_E_CANTOPEN_NOTEMPDIR: windows_core::HRESULT = windows_core::HRESULT(0x87AF010E_u32 as _);
pub const SQLITE_E_CONSTRAINT: windows_core::HRESULT = windows_core::HRESULT(0x87AF0013_u32 as _);
pub const SQLITE_E_CONSTRAINT_CHECK: windows_core::HRESULT = windows_core::HRESULT(0x87AF0113_u32 as _);
pub const SQLITE_E_CONSTRAINT_COMMITHOOK: windows_core::HRESULT = windows_core::HRESULT(0x87AF0213_u32 as _);
pub const SQLITE_E_CONSTRAINT_FOREIGNKEY: windows_core::HRESULT = windows_core::HRESULT(0x87AF0313_u32 as _);
pub const SQLITE_E_CONSTRAINT_FUNCTION: windows_core::HRESULT = windows_core::HRESULT(0x87AF0413_u32 as _);
pub const SQLITE_E_CONSTRAINT_NOTNULL: windows_core::HRESULT = windows_core::HRESULT(0x87AF0513_u32 as _);
pub const SQLITE_E_CONSTRAINT_PRIMARYKEY: windows_core::HRESULT = windows_core::HRESULT(0x87AF0613_u32 as _);
pub const SQLITE_E_CONSTRAINT_ROWID: windows_core::HRESULT = windows_core::HRESULT(0x87AF0A13_u32 as _);
pub const SQLITE_E_CONSTRAINT_TRIGGER: windows_core::HRESULT = windows_core::HRESULT(0x87AF0713_u32 as _);
pub const SQLITE_E_CONSTRAINT_UNIQUE: windows_core::HRESULT = windows_core::HRESULT(0x87AF0813_u32 as _);
pub const SQLITE_E_CONSTRAINT_VTAB: windows_core::HRESULT = windows_core::HRESULT(0x87AF0913_u32 as _);
pub const SQLITE_E_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x87AF000B_u32 as _);
pub const SQLITE_E_CORRUPT_VTAB: windows_core::HRESULT = windows_core::HRESULT(0x87AF010B_u32 as _);
pub const SQLITE_E_DONE: windows_core::HRESULT = windows_core::HRESULT(0x87AF0065_u32 as _);
pub const SQLITE_E_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x87AF0010_u32 as _);
pub const SQLITE_E_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x87AF0001_u32 as _);
pub const SQLITE_E_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x87AF0018_u32 as _);
pub const SQLITE_E_FULL: windows_core::HRESULT = windows_core::HRESULT(0x87AF000D_u32 as _);
pub const SQLITE_E_INTERNAL: windows_core::HRESULT = windows_core::HRESULT(0x87AF0002_u32 as _);
pub const SQLITE_E_INTERRUPT: windows_core::HRESULT = windows_core::HRESULT(0x87AF0009_u32 as _);
pub const SQLITE_E_IOERR: windows_core::HRESULT = windows_core::HRESULT(0x87AF000A_u32 as _);
pub const SQLITE_E_IOERR_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x87AF0D0A_u32 as _);
pub const SQLITE_E_IOERR_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x87AF1A03_u32 as _);
pub const SQLITE_E_IOERR_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x87AF0B0A_u32 as _);
pub const SQLITE_E_IOERR_CHECKRESERVEDLOCK: windows_core::HRESULT = windows_core::HRESULT(0x87AF0E0A_u32 as _);
pub const SQLITE_E_IOERR_CLOSE: windows_core::HRESULT = windows_core::HRESULT(0x87AF100A_u32 as _);
pub const SQLITE_E_IOERR_CONVPATH: windows_core::HRESULT = windows_core::HRESULT(0x87AF1A0A_u32 as _);
pub const SQLITE_E_IOERR_DELETE: windows_core::HRESULT = windows_core::HRESULT(0x87AF0A0A_u32 as _);
pub const SQLITE_E_IOERR_DELETE_NOENT: windows_core::HRESULT = windows_core::HRESULT(0x87AF170A_u32 as _);
pub const SQLITE_E_IOERR_DIR_CLOSE: windows_core::HRESULT = windows_core::HRESULT(0x87AF110A_u32 as _);
pub const SQLITE_E_IOERR_DIR_FSYNC: windows_core::HRESULT = windows_core::HRESULT(0x87AF050A_u32 as _);
pub const SQLITE_E_IOERR_FSTAT: windows_core::HRESULT = windows_core::HRESULT(0x87AF070A_u32 as _);
pub const SQLITE_E_IOERR_FSYNC: windows_core::HRESULT = windows_core::HRESULT(0x87AF040A_u32 as _);
pub const SQLITE_E_IOERR_GETTEMPPATH: windows_core::HRESULT = windows_core::HRESULT(0x87AF190A_u32 as _);
pub const SQLITE_E_IOERR_LOCK: windows_core::HRESULT = windows_core::HRESULT(0x87AF0F0A_u32 as _);
pub const SQLITE_E_IOERR_MMAP: windows_core::HRESULT = windows_core::HRESULT(0x87AF180A_u32 as _);
pub const SQLITE_E_IOERR_NOMEM: windows_core::HRESULT = windows_core::HRESULT(0x87AF0C0A_u32 as _);
pub const SQLITE_E_IOERR_RDLOCK: windows_core::HRESULT = windows_core::HRESULT(0x87AF090A_u32 as _);
pub const SQLITE_E_IOERR_READ: windows_core::HRESULT = windows_core::HRESULT(0x87AF010A_u32 as _);
pub const SQLITE_E_IOERR_SEEK: windows_core::HRESULT = windows_core::HRESULT(0x87AF160A_u32 as _);
pub const SQLITE_E_IOERR_SHMLOCK: windows_core::HRESULT = windows_core::HRESULT(0x87AF140A_u32 as _);
pub const SQLITE_E_IOERR_SHMMAP: windows_core::HRESULT = windows_core::HRESULT(0x87AF150A_u32 as _);
pub const SQLITE_E_IOERR_SHMOPEN: windows_core::HRESULT = windows_core::HRESULT(0x87AF120A_u32 as _);
pub const SQLITE_E_IOERR_SHMSIZE: windows_core::HRESULT = windows_core::HRESULT(0x87AF130A_u32 as _);
pub const SQLITE_E_IOERR_SHORT_READ: windows_core::HRESULT = windows_core::HRESULT(0x87AF020A_u32 as _);
pub const SQLITE_E_IOERR_TRUNCATE: windows_core::HRESULT = windows_core::HRESULT(0x87AF060A_u32 as _);
pub const SQLITE_E_IOERR_UNLOCK: windows_core::HRESULT = windows_core::HRESULT(0x87AF080A_u32 as _);
pub const SQLITE_E_IOERR_VNODE: windows_core::HRESULT = windows_core::HRESULT(0x87AF1A02_u32 as _);
pub const SQLITE_E_IOERR_WRITE: windows_core::HRESULT = windows_core::HRESULT(0x87AF030A_u32 as _);
pub const SQLITE_E_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x87AF0006_u32 as _);
pub const SQLITE_E_LOCKED_SHAREDCACHE: windows_core::HRESULT = windows_core::HRESULT(0x87AF0106_u32 as _);
pub const SQLITE_E_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x87AF0014_u32 as _);
pub const SQLITE_E_MISUSE: windows_core::HRESULT = windows_core::HRESULT(0x87AF0015_u32 as _);
pub const SQLITE_E_NOLFS: windows_core::HRESULT = windows_core::HRESULT(0x87AF0016_u32 as _);
pub const SQLITE_E_NOMEM: windows_core::HRESULT = windows_core::HRESULT(0x87AF0007_u32 as _);
pub const SQLITE_E_NOTADB: windows_core::HRESULT = windows_core::HRESULT(0x87AF001A_u32 as _);
pub const SQLITE_E_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x87AF000C_u32 as _);
pub const SQLITE_E_NOTICE: windows_core::HRESULT = windows_core::HRESULT(0x87AF001B_u32 as _);
pub const SQLITE_E_NOTICE_RECOVER_ROLLBACK: windows_core::HRESULT = windows_core::HRESULT(0x87AF021B_u32 as _);
pub const SQLITE_E_NOTICE_RECOVER_WAL: windows_core::HRESULT = windows_core::HRESULT(0x87AF011B_u32 as _);
pub const SQLITE_E_PERM: windows_core::HRESULT = windows_core::HRESULT(0x87AF0003_u32 as _);
pub const SQLITE_E_PROTOCOL: windows_core::HRESULT = windows_core::HRESULT(0x87AF000F_u32 as _);
pub const SQLITE_E_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x87AF0019_u32 as _);
pub const SQLITE_E_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x87AF0008_u32 as _);
pub const SQLITE_E_READONLY_CANTLOCK: windows_core::HRESULT = windows_core::HRESULT(0x87AF0208_u32 as _);
pub const SQLITE_E_READONLY_DBMOVED: windows_core::HRESULT = windows_core::HRESULT(0x87AF0408_u32 as _);
pub const SQLITE_E_READONLY_RECOVERY: windows_core::HRESULT = windows_core::HRESULT(0x87AF0108_u32 as _);
pub const SQLITE_E_READONLY_ROLLBACK: windows_core::HRESULT = windows_core::HRESULT(0x87AF0308_u32 as _);
pub const SQLITE_E_ROW: windows_core::HRESULT = windows_core::HRESULT(0x87AF0064_u32 as _);
pub const SQLITE_E_SCHEMA: windows_core::HRESULT = windows_core::HRESULT(0x87AF0011_u32 as _);
pub const SQLITE_E_TOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x87AF0012_u32 as _);
pub const SQLITE_E_WARNING: windows_core::HRESULT = windows_core::HRESULT(0x87AF001C_u32 as _);
pub const SQLITE_E_WARNING_AUTOINDEX: windows_core::HRESULT = windows_core::HRESULT(0x87AF011C_u32 as _);
pub const STATEREPOSITORY_ERROR_CACHE_CORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x80670012_u32 as _);
pub const STATEREPOSITORY_ERROR_DICTIONARY_CORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x80670005_u32 as _);
pub const STATEREPOSITORY_E_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80670006_u32 as _);
pub const STATEREPOSITORY_E_BUSY_RECOVERY_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80670008_u32 as _);
pub const STATEREPOSITORY_E_BUSY_RECOVERY_TIMEOUT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8067000D_u32 as _);
pub const STATEREPOSITORY_E_BUSY_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80670007_u32 as _);
pub const STATEREPOSITORY_E_BUSY_TIMEOUT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8067000C_u32 as _);
pub const STATEREPOSITORY_E_CACHE_NOT_INIITALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80670015_u32 as _);
pub const STATEREPOSITORY_E_CONCURRENCY_LOCKING_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80670001_u32 as _);
pub const STATEREPOSITORY_E_CONFIGURATION_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80670003_u32 as _);
pub const STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED: windows_core::HRESULT = windows_core::HRESULT(0x80670016_u32 as _);
pub const STATEREPOSITORY_E_LOCKED_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80670009_u32 as _);
pub const STATEREPOSITORY_E_LOCKED_SHAREDCACHE_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x8067000A_u32 as _);
pub const STATEREPOSITORY_E_LOCKED_SHAREDCACHE_TIMEOUT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8067000F_u32 as _);
pub const STATEREPOSITORY_E_LOCKED_TIMEOUT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x8067000E_u32 as _);
pub const STATEREPOSITORY_E_SERVICE_STOP_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80670010_u32 as _);
pub const STATEREPOSITORY_E_STATEMENT_INPROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80670002_u32 as _);
pub const STATEREPOSITORY_E_TRANSACTION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8067000B_u32 as _);
pub const STATEREPOSITORY_E_UNKNOWN_SCHEMA_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80670004_u32 as _);
pub const STATEREPOSITORY_TRANSACTION_CALLER_ID_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x670013_u32 as _);
pub const STATEREPOSITORY_TRANSACTION_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80670014_u32 as _);
pub const STATEREPOSTORY_E_NESTED_TRANSACTION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80670011_u32 as _);
pub const STATUS_ABANDONED: NTSTATUS = NTSTATUS(0x80_u32 as _);
pub const STATUS_ABANDONED_WAIT_0: NTSTATUS = NTSTATUS(0x80_u32 as _);
pub const STATUS_ABANDONED_WAIT_63: NTSTATUS = NTSTATUS(0xBF_u32 as _);
pub const STATUS_ABANDON_HIBERFILE: NTSTATUS = NTSTATUS(0x40000033_u32 as _);
pub const STATUS_ABIOS_INVALID_COMMAND: NTSTATUS = NTSTATUS(0xC0000113_u32 as _);
pub const STATUS_ABIOS_INVALID_LID: NTSTATUS = NTSTATUS(0xC0000114_u32 as _);
pub const STATUS_ABIOS_INVALID_SELECTOR: NTSTATUS = NTSTATUS(0xC0000116_u32 as _);
pub const STATUS_ABIOS_LID_ALREADY_OWNED: NTSTATUS = NTSTATUS(0xC0000111_u32 as _);
pub const STATUS_ABIOS_LID_NOT_EXIST: NTSTATUS = NTSTATUS(0xC0000110_u32 as _);
pub const STATUS_ABIOS_NOT_LID_OWNER: NTSTATUS = NTSTATUS(0xC0000112_u32 as _);
pub const STATUS_ABIOS_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC000010F_u32 as _);
pub const STATUS_ABIOS_SELECTOR_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC0000115_u32 as _);
pub const STATUS_ACCESS_AUDIT_BY_POLICY: NTSTATUS = NTSTATUS(0x40000032_u32 as _);
pub const STATUS_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC0000022_u32 as _);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_DEFAULT: NTSTATUS = NTSTATUS(0xC0000361_u32 as _);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_OTHER: NTSTATUS = NTSTATUS(0xC0000364_u32 as _);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_PATH: NTSTATUS = NTSTATUS(0xC0000362_u32 as _);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_PUBLISHER: NTSTATUS = NTSTATUS(0xC0000363_u32 as _);
pub const STATUS_ACCESS_DISABLED_NO_SAFER_UI_BY_POLICY: NTSTATUS = NTSTATUS(0xC0000372_u32 as _);
pub const STATUS_ACCESS_VIOLATION: NTSTATUS = NTSTATUS(0xC0000005_u32 as _);
pub const STATUS_ACPI_ACQUIRE_GLOBAL_LOCK: NTSTATUS = NTSTATUS(0xC0140012_u32 as _);
pub const STATUS_ACPI_ADDRESS_NOT_MAPPED: NTSTATUS = NTSTATUS(0xC014000C_u32 as _);
pub const STATUS_ACPI_ALREADY_INITIALIZED: NTSTATUS = NTSTATUS(0xC0140013_u32 as _);
pub const STATUS_ACPI_ASSERT_FAILED: NTSTATUS = NTSTATUS(0xC0140003_u32 as _);
pub const STATUS_ACPI_FATAL: NTSTATUS = NTSTATUS(0xC0140006_u32 as _);
pub const STATUS_ACPI_HANDLER_COLLISION: NTSTATUS = NTSTATUS(0xC014000E_u32 as _);
pub const STATUS_ACPI_INCORRECT_ARGUMENT_COUNT: NTSTATUS = NTSTATUS(0xC014000B_u32 as _);
pub const STATUS_ACPI_INVALID_ACCESS_SIZE: NTSTATUS = NTSTATUS(0xC0140011_u32 as _);
pub const STATUS_ACPI_INVALID_ARGTYPE: NTSTATUS = NTSTATUS(0xC0140008_u32 as _);
pub const STATUS_ACPI_INVALID_ARGUMENT: NTSTATUS = NTSTATUS(0xC0140005_u32 as _);
pub const STATUS_ACPI_INVALID_DATA: NTSTATUS = NTSTATUS(0xC014000F_u32 as _);
pub const STATUS_ACPI_INVALID_EVENTTYPE: NTSTATUS = NTSTATUS(0xC014000D_u32 as _);
pub const STATUS_ACPI_INVALID_INDEX: NTSTATUS = NTSTATUS(0xC0140004_u32 as _);
pub const STATUS_ACPI_INVALID_MUTEX_LEVEL: NTSTATUS = NTSTATUS(0xC0140015_u32 as _);
pub const STATUS_ACPI_INVALID_OBJTYPE: NTSTATUS = NTSTATUS(0xC0140009_u32 as _);
pub const STATUS_ACPI_INVALID_OPCODE: NTSTATUS = NTSTATUS(0xC0140001_u32 as _);
pub const STATUS_ACPI_INVALID_REGION: NTSTATUS = NTSTATUS(0xC0140010_u32 as _);
pub const STATUS_ACPI_INVALID_SUPERNAME: NTSTATUS = NTSTATUS(0xC0140007_u32 as _);
pub const STATUS_ACPI_INVALID_TABLE: NTSTATUS = NTSTATUS(0xC0140019_u32 as _);
pub const STATUS_ACPI_INVALID_TARGETTYPE: NTSTATUS = NTSTATUS(0xC014000A_u32 as _);
pub const STATUS_ACPI_MUTEX_NOT_OWNED: NTSTATUS = NTSTATUS(0xC0140016_u32 as _);
pub const STATUS_ACPI_MUTEX_NOT_OWNER: NTSTATUS = NTSTATUS(0xC0140017_u32 as _);
pub const STATUS_ACPI_NOT_INITIALIZED: NTSTATUS = NTSTATUS(0xC0140014_u32 as _);
pub const STATUS_ACPI_POWER_REQUEST_FAILED: NTSTATUS = NTSTATUS(0xC0140021_u32 as _);
pub const STATUS_ACPI_REG_HANDLER_FAILED: NTSTATUS = NTSTATUS(0xC0140020_u32 as _);
pub const STATUS_ACPI_RS_ACCESS: NTSTATUS = NTSTATUS(0xC0140018_u32 as _);
pub const STATUS_ACPI_STACK_OVERFLOW: NTSTATUS = NTSTATUS(0xC0140002_u32 as _);
pub const STATUS_ADAPTER_HARDWARE_ERROR: NTSTATUS = NTSTATUS(0xC00000C2_u32 as _);
pub const STATUS_ADDRESS_ALREADY_ASSOCIATED: NTSTATUS = NTSTATUS(0xC0000238_u32 as _);
pub const STATUS_ADDRESS_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC000020A_u32 as _);
pub const STATUS_ADDRESS_CLOSED: NTSTATUS = NTSTATUS(0xC000020B_u32 as _);
pub const STATUS_ADDRESS_NOT_ASSOCIATED: NTSTATUS = NTSTATUS(0xC0000239_u32 as _);
pub const STATUS_ADMINLESS_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC000A204_u32 as _);
pub const STATUS_ADVANCED_INSTALLER_FAILED: NTSTATUS = NTSTATUS(0xC0150020_u32 as _);
pub const STATUS_AGENTS_EXHAUSTED: NTSTATUS = NTSTATUS(0xC0000085_u32 as _);
pub const STATUS_ALERTED: NTSTATUS = NTSTATUS(0x101_u32 as _);
pub const STATUS_ALIAS_EXISTS: NTSTATUS = NTSTATUS(0xC0000154_u32 as _);
pub const STATUS_ALLOCATE_BUCKET: NTSTATUS = NTSTATUS(0xC000022F_u32 as _);
pub const STATUS_ALLOTTED_SPACE_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000099_u32 as _);
pub const STATUS_ALL_SIDS_FILTERED: NTSTATUS = NTSTATUS(0xC000035E_u32 as _);
pub const STATUS_ALL_USER_TRUST_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000402_u32 as _);
pub const STATUS_ALPC_CHECK_COMPLETION_LIST: NTSTATUS = NTSTATUS(0x40000030_u32 as _);
pub const STATUS_ALREADY_COMMITTED: NTSTATUS = NTSTATUS(0xC0000021_u32 as _);
pub const STATUS_ALREADY_COMPLETE: NTSTATUS = NTSTATUS(0xFF_u32 as _);
pub const STATUS_ALREADY_DISCONNECTED: NTSTATUS = NTSTATUS(0x80000025_u32 as _);
pub const STATUS_ALREADY_HAS_STREAM_ID: NTSTATUS = NTSTATUS(0xC000050E_u32 as _);
pub const STATUS_ALREADY_INITIALIZED: NTSTATUS = NTSTATUS(0xC0000510_u32 as _);
pub const STATUS_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(0xC0000718_u32 as _);
pub const STATUS_ALREADY_WIN32: NTSTATUS = NTSTATUS(0x4000001B_u32 as _);
pub const STATUS_AMBIGUOUS_SYSTEM_DEVICE: NTSTATUS = NTSTATUS(0xC0000451_u32 as _);
pub const STATUS_APC_RETURNED_WHILE_IMPERSONATING: NTSTATUS = NTSTATUS(0xC0000711_u32 as _);
pub const STATUS_APISET_NOT_HOSTED: NTSTATUS = NTSTATUS(0xC0000481_u32 as _);
pub const STATUS_APISET_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC0000482_u32 as _);
pub const STATUS_APPEXEC_APP_COMPAT_BLOCK: NTSTATUS = NTSTATUS(0xC0EC0008_u32 as _);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT: NTSTATUS = NTSTATUS(0xC0EC0009_u32 as _);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT_LICENSING: NTSTATUS = NTSTATUS(0xC0EC000B_u32 as _);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT_RESOURCES: NTSTATUS = NTSTATUS(0xC0EC000C_u32 as _);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT_TERMINATION: NTSTATUS = NTSTATUS(0xC0EC000A_u32 as _);
pub const STATUS_APPEXEC_CONDITION_NOT_SATISFIED: NTSTATUS = NTSTATUS(0xC0EC0000_u32 as _);
pub const STATUS_APPEXEC_HANDLE_INVALIDATED: NTSTATUS = NTSTATUS(0xC0EC0001_u32 as _);
pub const STATUS_APPEXEC_HOST_ID_MISMATCH: NTSTATUS = NTSTATUS(0xC0EC0006_u32 as _);
pub const STATUS_APPEXEC_INVALID_HOST_GENERATION: NTSTATUS = NTSTATUS(0xC0EC0002_u32 as _);
pub const STATUS_APPEXEC_INVALID_HOST_STATE: NTSTATUS = NTSTATUS(0xC0EC0004_u32 as _);
pub const STATUS_APPEXEC_NO_DONOR: NTSTATUS = NTSTATUS(0xC0EC0005_u32 as _);
pub const STATUS_APPEXEC_UNEXPECTED_PROCESS_REGISTRATION: NTSTATUS = NTSTATUS(0xC0EC0003_u32 as _);
pub const STATUS_APPEXEC_UNKNOWN_USER: NTSTATUS = NTSTATUS(0xC0EC0007_u32 as _);
pub const STATUS_APPHELP_BLOCK: NTSTATUS = NTSTATUS(0xC000035D_u32 as _);
pub const STATUS_APPX_FILE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(0xC00004A6_u32 as _);
pub const STATUS_APPX_INTEGRITY_FAILURE_CLR_NGEN: NTSTATUS = NTSTATUS(0xC000047F_u32 as _);
pub const STATUS_APP_DATA_CORRUPT: NTSTATUS = NTSTATUS(0xC000A283_u32 as _);
pub const STATUS_APP_DATA_EXPIRED: NTSTATUS = NTSTATUS(0xC000A282_u32 as _);
pub const STATUS_APP_DATA_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC000A284_u32 as _);
pub const STATUS_APP_DATA_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000A281_u32 as _);
pub const STATUS_APP_DATA_REBOOT_REQUIRED: NTSTATUS = NTSTATUS(0xC000A285_u32 as _);
pub const STATUS_APP_INIT_FAILURE: NTSTATUS = NTSTATUS(0xC0000145_u32 as _);
pub const STATUS_ARBITRATION_UNHANDLED: NTSTATUS = NTSTATUS(0x40000026_u32 as _);
pub const STATUS_ARRAY_BOUNDS_EXCEEDED: NTSTATUS = NTSTATUS(0xC000008C_u32 as _);
pub const STATUS_ASSERTION_FAILURE: NTSTATUS = NTSTATUS(0xC0000420_u32 as _);
pub const STATUS_ATTACHED_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(0xC0000725_u32 as _);
pub const STATUS_ATTRIBUTE_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC000050C_u32 as _);
pub const STATUS_AUDIO_ENGINE_NODE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0440001_u32 as _);
pub const STATUS_AUDITING_DISABLED: NTSTATUS = NTSTATUS(0xC0000356_u32 as _);
pub const STATUS_AUDIT_FAILED: NTSTATUS = NTSTATUS(0xC0000244_u32 as _);
pub const STATUS_AUTHIP_FAILURE: NTSTATUS = NTSTATUS(0xC000A086_u32 as _);
pub const STATUS_AUTH_TAG_MISMATCH: NTSTATUS = NTSTATUS(0xC000A002_u32 as _);
pub const STATUS_BACKUP_CONTROLLER: NTSTATUS = NTSTATUS(0xC0000187_u32 as _);
pub const STATUS_BAD_BINDINGS: NTSTATUS = NTSTATUS(0xC000035B_u32 as _);
pub const STATUS_BAD_CLUSTERS: NTSTATUS = NTSTATUS(0xC0000805_u32 as _);
pub const STATUS_BAD_COMPRESSION_BUFFER: NTSTATUS = NTSTATUS(0xC0000242_u32 as _);
pub const STATUS_BAD_CURRENT_DIRECTORY: NTSTATUS = NTSTATUS(0x40000007_u32 as _);
pub const STATUS_BAD_DATA: NTSTATUS = NTSTATUS(0xC000090B_u32 as _);
pub const STATUS_BAD_DESCRIPTOR_FORMAT: NTSTATUS = NTSTATUS(0xC00000E7_u32 as _);
pub const STATUS_BAD_DEVICE_TYPE: NTSTATUS = NTSTATUS(0xC00000CB_u32 as _);
pub const STATUS_BAD_DLL_ENTRYPOINT: NTSTATUS = NTSTATUS(0xC0000251_u32 as _);
pub const STATUS_BAD_FILE_TYPE: NTSTATUS = NTSTATUS(0xC0000903_u32 as _);
pub const STATUS_BAD_FUNCTION_TABLE: NTSTATUS = NTSTATUS(0xC00000FF_u32 as _);
pub const STATUS_BAD_IMPERSONATION_LEVEL: NTSTATUS = NTSTATUS(0xC00000A5_u32 as _);
pub const STATUS_BAD_INHERITANCE_ACL: NTSTATUS = NTSTATUS(0xC000007D_u32 as _);
pub const STATUS_BAD_INITIAL_PC: NTSTATUS = NTSTATUS(0xC000000A_u32 as _);
pub const STATUS_BAD_INITIAL_STACK: NTSTATUS = NTSTATUS(0xC0000009_u32 as _);
pub const STATUS_BAD_KEY: NTSTATUS = NTSTATUS(0xC000090A_u32 as _);
pub const STATUS_BAD_LOGON_SESSION_STATE: NTSTATUS = NTSTATUS(0xC0000104_u32 as _);
pub const STATUS_BAD_MASTER_BOOT_RECORD: NTSTATUS = NTSTATUS(0xC00000A9_u32 as _);
pub const STATUS_BAD_MCFG_TABLE: NTSTATUS = NTSTATUS(0xC0000908_u32 as _);
pub const STATUS_BAD_NETWORK_NAME: NTSTATUS = NTSTATUS(0xC00000CC_u32 as _);
pub const STATUS_BAD_NETWORK_PATH: NTSTATUS = NTSTATUS(0xC00000BE_u32 as _);
pub const STATUS_BAD_REMOTE_ADAPTER: NTSTATUS = NTSTATUS(0xC00000C5_u32 as _);
pub const STATUS_BAD_SERVICE_ENTRYPOINT: NTSTATUS = NTSTATUS(0xC0000252_u32 as _);
pub const STATUS_BAD_STACK: NTSTATUS = NTSTATUS(0xC0000028_u32 as _);
pub const STATUS_BAD_TOKEN_TYPE: NTSTATUS = NTSTATUS(0xC00000A8_u32 as _);
pub const STATUS_BAD_VALIDATION_CLASS: NTSTATUS = NTSTATUS(0xC00000A7_u32 as _);
pub const STATUS_BAD_WORKING_SET_LIMIT: NTSTATUS = NTSTATUS(0xC000004C_u32 as _);
pub const STATUS_BCD_NOT_ALL_ENTRIES_IMPORTED: NTSTATUS = NTSTATUS(0x80390001_u32 as _);
pub const STATUS_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED: NTSTATUS = NTSTATUS(0x80390003_u32 as _);
pub const STATUS_BCD_TOO_MANY_ELEMENTS: NTSTATUS = NTSTATUS(0xC0390002_u32 as _);
pub const STATUS_BEGINNING_OF_MEDIA: NTSTATUS = NTSTATUS(0x8000001F_u32 as _);
pub const STATUS_BEYOND_VDL: NTSTATUS = NTSTATUS(0xC0000432_u32 as _);
pub const STATUS_BIOS_FAILED_TO_CONNECT_INTERRUPT: NTSTATUS = NTSTATUS(0xC000016E_u32 as _);
pub const STATUS_BIZRULES_NOT_ENABLED: NTSTATUS = NTSTATUS(0x40000034_u32 as _);
pub const STATUS_BLOCKED_BY_PARENTAL_CONTROLS: NTSTATUS = NTSTATUS(0xC0000488_u32 as _);
pub const STATUS_BLOCK_SHARED: NTSTATUS = NTSTATUS(0xC0000915_u32 as _);
pub const STATUS_BLOCK_SOURCE_WEAK_REFERENCE_INVALID: NTSTATUS = NTSTATUS(0xC0000913_u32 as _);
pub const STATUS_BLOCK_TARGET_WEAK_REFERENCE_INVALID: NTSTATUS = NTSTATUS(0xC0000914_u32 as _);
pub const STATUS_BLOCK_TOO_MANY_REFERENCES: NTSTATUS = NTSTATUS(0xC000048C_u32 as _);
pub const STATUS_BLOCK_WEAK_REFERENCE_INVALID: NTSTATUS = NTSTATUS(0xC0000912_u32 as _);
pub const STATUS_BREAKPOINT: NTSTATUS = NTSTATUS(0x80000003_u32 as _);
pub const STATUS_BTH_ATT_ATTRIBUTE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC042000A_u32 as _);
pub const STATUS_BTH_ATT_ATTRIBUTE_NOT_LONG: NTSTATUS = NTSTATUS(0xC042000B_u32 as _);
pub const STATUS_BTH_ATT_INSUFFICIENT_AUTHENTICATION: NTSTATUS = NTSTATUS(0xC0420005_u32 as _);
pub const STATUS_BTH_ATT_INSUFFICIENT_AUTHORIZATION: NTSTATUS = NTSTATUS(0xC0420008_u32 as _);
pub const STATUS_BTH_ATT_INSUFFICIENT_ENCRYPTION: NTSTATUS = NTSTATUS(0xC042000F_u32 as _);
pub const STATUS_BTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE: NTSTATUS = NTSTATUS(0xC042000C_u32 as _);
pub const STATUS_BTH_ATT_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(0xC0420011_u32 as _);
pub const STATUS_BTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH: NTSTATUS = NTSTATUS(0xC042000D_u32 as _);
pub const STATUS_BTH_ATT_INVALID_HANDLE: NTSTATUS = NTSTATUS(0xC0420001_u32 as _);
pub const STATUS_BTH_ATT_INVALID_OFFSET: NTSTATUS = NTSTATUS(0xC0420007_u32 as _);
pub const STATUS_BTH_ATT_INVALID_PDU: NTSTATUS = NTSTATUS(0xC0420004_u32 as _);
pub const STATUS_BTH_ATT_PREPARE_QUEUE_FULL: NTSTATUS = NTSTATUS(0xC0420009_u32 as _);
pub const STATUS_BTH_ATT_READ_NOT_PERMITTED: NTSTATUS = NTSTATUS(0xC0420002_u32 as _);
pub const STATUS_BTH_ATT_REQUEST_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0420006_u32 as _);
pub const STATUS_BTH_ATT_UNKNOWN_ERROR: NTSTATUS = NTSTATUS(0xC0421000_u32 as _);
pub const STATUS_BTH_ATT_UNLIKELY: NTSTATUS = NTSTATUS(0xC042000E_u32 as _);
pub const STATUS_BTH_ATT_UNSUPPORTED_GROUP_TYPE: NTSTATUS = NTSTATUS(0xC0420010_u32 as _);
pub const STATUS_BTH_ATT_WRITE_NOT_PERMITTED: NTSTATUS = NTSTATUS(0xC0420003_u32 as _);
pub const STATUS_BUFFER_ALL_ZEROS: NTSTATUS = NTSTATUS(0x117_u32 as _);
pub const STATUS_BUFFER_OVERFLOW: NTSTATUS = NTSTATUS(0x80000005_u32 as _);
pub const STATUS_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(0xC0000023_u32 as _);
pub const STATUS_BUS_RESET: NTSTATUS = NTSTATUS(0x8000001D_u32 as _);
pub const STATUS_BYPASSIO_FLT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00004D2_u32 as _);
pub const STATUS_CACHE_PAGE_LOCKED: NTSTATUS = NTSTATUS(0x115_u32 as _);
pub const STATUS_CALLBACK_BYPASS: NTSTATUS = NTSTATUS(0xC0000503_u32 as _);
pub const STATUS_CALLBACK_INVOKE_INLINE: NTSTATUS = NTSTATUS(0xC000048B_u32 as _);
pub const STATUS_CALLBACK_POP_STACK: NTSTATUS = NTSTATUS(0xC0000423_u32 as _);
pub const STATUS_CALLBACK_RETURNED_LANG: NTSTATUS = NTSTATUS(0xC000071F_u32 as _);
pub const STATUS_CALLBACK_RETURNED_LDR_LOCK: NTSTATUS = NTSTATUS(0xC000071E_u32 as _);
pub const STATUS_CALLBACK_RETURNED_PRI_BACK: NTSTATUS = NTSTATUS(0xC0000720_u32 as _);
pub const STATUS_CALLBACK_RETURNED_THREAD_AFFINITY: NTSTATUS = NTSTATUS(0xC0000721_u32 as _);
pub const STATUS_CALLBACK_RETURNED_THREAD_PRIORITY: NTSTATUS = NTSTATUS(0xC000071B_u32 as _);
pub const STATUS_CALLBACK_RETURNED_TRANSACTION: NTSTATUS = NTSTATUS(0xC000071D_u32 as _);
pub const STATUS_CALLBACK_RETURNED_WHILE_IMPERSONATING: NTSTATUS = NTSTATUS(0xC0000710_u32 as _);
pub const STATUS_CANCELLED: NTSTATUS = NTSTATUS(0xC0000120_u32 as _);
pub const STATUS_CANNOT_ABORT_TRANSACTIONS: NTSTATUS = NTSTATUS(0xC019004D_u32 as _);
pub const STATUS_CANNOT_ACCEPT_TRANSACTED_WORK: NTSTATUS = NTSTATUS(0xC019004C_u32 as _);
pub const STATUS_CANNOT_BREAK_OPLOCK: NTSTATUS = NTSTATUS(0xC0000909_u32 as _);
pub const STATUS_CANNOT_DELETE: NTSTATUS = NTSTATUS(0xC0000121_u32 as _);
pub const STATUS_CANNOT_EXECUTE_FILE_IN_TRANSACTION: NTSTATUS = NTSTATUS(0xC0190044_u32 as _);
pub const STATUS_CANNOT_GRANT_REQUESTED_OPLOCK: NTSTATUS = NTSTATUS(0x8000002E_u32 as _);
pub const STATUS_CANNOT_IMPERSONATE: NTSTATUS = NTSTATUS(0xC000010D_u32 as _);
pub const STATUS_CANNOT_LOAD_REGISTRY_FILE: NTSTATUS = NTSTATUS(0xC0000218_u32 as _);
pub const STATUS_CANNOT_MAKE: NTSTATUS = NTSTATUS(0xC00002EA_u32 as _);
pub const STATUS_CANNOT_SWITCH_RUNLEVEL: NTSTATUS = NTSTATUS(0xC000A141_u32 as _);
pub const STATUS_CANT_ACCESS_DOMAIN_INFO: NTSTATUS = NTSTATUS(0xC00000DA_u32 as _);
pub const STATUS_CANT_ATTACH_TO_DEV_VOLUME: NTSTATUS = NTSTATUS(0xC00004DF_u32 as _);
pub const STATUS_CANT_BREAK_TRANSACTIONAL_DEPENDENCY: NTSTATUS = NTSTATUS(0xC0190037_u32 as _);
pub const STATUS_CANT_CLEAR_ENCRYPTION_FLAG: NTSTATUS = NTSTATUS(0xC00004B8_u32 as _);
pub const STATUS_CANT_CREATE_MORE_STREAM_MINIVERSIONS: NTSTATUS = NTSTATUS(0xC0190026_u32 as _);
pub const STATUS_CANT_CROSS_RM_BOUNDARY: NTSTATUS = NTSTATUS(0xC0190038_u32 as _);
pub const STATUS_CANT_DISABLE_MANDATORY: NTSTATUS = NTSTATUS(0xC000005D_u32 as _);
pub const STATUS_CANT_ENABLE_DENY_ONLY: NTSTATUS = NTSTATUS(0xC00002B3_u32 as _);
pub const STATUS_CANT_OPEN_ANONYMOUS: NTSTATUS = NTSTATUS(0xC00000A6_u32 as _);
pub const STATUS_CANT_OPEN_MINIVERSION_WITH_MODIFY_INTENT: NTSTATUS = NTSTATUS(0xC0190025_u32 as _);
pub const STATUS_CANT_RECOVER_WITH_HANDLE_OPEN: NTSTATUS = NTSTATUS(0x80190031_u32 as _);
pub const STATUS_CANT_TERMINATE_SELF: NTSTATUS = NTSTATUS(0xC00000DB_u32 as _);
pub const STATUS_CANT_WAIT: NTSTATUS = NTSTATUS(0xC00000D8_u32 as _);
pub const STATUS_CARDBUS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0x40000027_u32 as _);
pub const STATUS_CASE_DIFFERING_NAMES_IN_DIR: NTSTATUS = NTSTATUS(0xC00004B3_u32 as _);
pub const STATUS_CASE_SENSITIVE_PATH: NTSTATUS = NTSTATUS(0xC00004BA_u32 as _);
pub const STATUS_CC_NEEDS_CALLBACK_SECTION_DRAIN: NTSTATUS = NTSTATUS(0xC000A008_u32 as _);
pub const STATUS_CERTIFICATE_MAPPING_NOT_UNIQUE: NTSTATUS = NTSTATUS(0xC0000714_u32 as _);
pub const STATUS_CERTIFICATE_VALIDATION_PREFERENCE_CONFLICT: NTSTATUS = NTSTATUS(0xC00001B5_u32 as _);
pub const STATUS_CHECKING_FILE_SYSTEM: NTSTATUS = NTSTATUS(0x40000014_u32 as _);
pub const STATUS_CHECKOUT_REQUIRED: NTSTATUS = NTSTATUS(0xC0000902_u32 as _);
pub const STATUS_CHILD_MUST_BE_VOLATILE: NTSTATUS = NTSTATUS(0xC0000181_u32 as _);
pub const STATUS_CHILD_PROCESS_BLOCKED: NTSTATUS = NTSTATUS(0xC000049D_u32 as _);
pub const STATUS_CIMFS_IMAGE_CORRUPT: NTSTATUS = NTSTATUS(0xC000C001_u32 as _);
pub const STATUS_CIMFS_IMAGE_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000C002_u32 as _);
pub const STATUS_CLEANER_CARTRIDGE_INSTALLED: NTSTATUS = NTSTATUS(0x80000027_u32 as _);
pub const STATUS_CLIENT_SERVER_PARAMETERS_INVALID: NTSTATUS = NTSTATUS(0xC0000223_u32 as _);
pub const STATUS_CLIP_DEVICE_LICENSE_MISSING: NTSTATUS = NTSTATUS(0xC0EA0003_u32 as _);
pub const STATUS_CLIP_KEYHOLDER_LICENSE_MISSING_OR_INVALID: NTSTATUS = NTSTATUS(0xC0EA0005_u32 as _);
pub const STATUS_CLIP_LICENSE_DEVICE_ID_MISMATCH: NTSTATUS = NTSTATUS(0xC0EA000A_u32 as _);
pub const STATUS_CLIP_LICENSE_EXPIRED: NTSTATUS = NTSTATUS(0xC0EA0006_u32 as _);
pub const STATUS_CLIP_LICENSE_HARDWARE_ID_OUT_OF_TOLERANCE: NTSTATUS = NTSTATUS(0xC0EA0009_u32 as _);
pub const STATUS_CLIP_LICENSE_INVALID_SIGNATURE: NTSTATUS = NTSTATUS(0xC0EA0004_u32 as _);
pub const STATUS_CLIP_LICENSE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0EA0002_u32 as _);
pub const STATUS_CLIP_LICENSE_NOT_SIGNED: NTSTATUS = NTSTATUS(0xC0EA0008_u32 as _);
pub const STATUS_CLIP_LICENSE_SIGNED_BY_UNKNOWN_SOURCE: NTSTATUS = NTSTATUS(0xC0EA0007_u32 as _);
pub const STATUS_CLOUD_FILE_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC000CF18_u32 as _);
pub const STATUS_CLOUD_FILE_ALREADY_CONNECTED: NTSTATUS = NTSTATUS(0xC000CF09_u32 as _);
pub const STATUS_CLOUD_FILE_AUTHENTICATION_FAILED: NTSTATUS = NTSTATUS(0xC000CF0F_u32 as _);
pub const STATUS_CLOUD_FILE_CONNECTED_PROVIDER_ONLY: NTSTATUS = NTSTATUS(0xC000CF0D_u32 as _);
pub const STATUS_CLOUD_FILE_DEHYDRATION_DISALLOWED: NTSTATUS = NTSTATUS(0xC000CF20_u32 as _);
pub const STATUS_CLOUD_FILE_INCOMPATIBLE_HARDLINKS: NTSTATUS = NTSTATUS(0xC000CF19_u32 as _);
pub const STATUS_CLOUD_FILE_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(0xC000CF10_u32 as _);
pub const STATUS_CLOUD_FILE_INVALID_REQUEST: NTSTATUS = NTSTATUS(0xC000CF0B_u32 as _);
pub const STATUS_CLOUD_FILE_IN_USE: NTSTATUS = NTSTATUS(0xC000CF14_u32 as _);
pub const STATUS_CLOUD_FILE_METADATA_CORRUPT: NTSTATUS = NTSTATUS(0xC000CF02_u32 as _);
pub const STATUS_CLOUD_FILE_METADATA_TOO_LARGE: NTSTATUS = NTSTATUS(0xC000CF03_u32 as _);
pub const STATUS_CLOUD_FILE_NETWORK_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC000CF11_u32 as _);
pub const STATUS_CLOUD_FILE_NOT_IN_SYNC: NTSTATUS = NTSTATUS(0xC000CF08_u32 as _);
pub const STATUS_CLOUD_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000CF0A_u32 as _);
pub const STATUS_CLOUD_FILE_NOT_UNDER_SYNC_ROOT: NTSTATUS = NTSTATUS(0xC000CF13_u32 as _);
pub const STATUS_CLOUD_FILE_PINNED: NTSTATUS = NTSTATUS(0xC000CF15_u32 as _);
pub const STATUS_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(0x8000CF00_u32 as _);
pub const STATUS_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE: NTSTATUS = NTSTATUS(0x8000CF04_u32 as _);
pub const STATUS_CLOUD_FILE_PROPERTY_CORRUPT: NTSTATUS = NTSTATUS(0xC000CF17_u32 as _);
pub const STATUS_CLOUD_FILE_PROPERTY_LOCK_CONFLICT: NTSTATUS = NTSTATUS(0xC000CF1A_u32 as _);
pub const STATUS_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000CF06_u32 as _);
pub const STATUS_CLOUD_FILE_PROVIDER_NOT_RUNNING: NTSTATUS = NTSTATUS(0xC000CF01_u32 as _);
pub const STATUS_CLOUD_FILE_PROVIDER_TERMINATED: NTSTATUS = NTSTATUS(0xC000CF1D_u32 as _);
pub const STATUS_CLOUD_FILE_READ_ONLY_VOLUME: NTSTATUS = NTSTATUS(0xC000CF0C_u32 as _);
pub const STATUS_CLOUD_FILE_REQUEST_ABORTED: NTSTATUS = NTSTATUS(0xC000CF16_u32 as _);
pub const STATUS_CLOUD_FILE_REQUEST_CANCELED: NTSTATUS = NTSTATUS(0xC000CF1B_u32 as _);
pub const STATUS_CLOUD_FILE_REQUEST_TIMEOUT: NTSTATUS = NTSTATUS(0xC000CF1F_u32 as _);
pub const STATUS_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT: NTSTATUS = NTSTATUS(0xC000CF00_u32 as _);
pub const STATUS_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS: NTSTATUS = NTSTATUS(0x8000CF05_u32 as _);
pub const STATUS_CLOUD_FILE_UNSUCCESSFUL: NTSTATUS = NTSTATUS(0xC000CF12_u32 as _);
pub const STATUS_CLOUD_FILE_US_MESSAGE_TIMEOUT: NTSTATUS = NTSTATUS(0xC000CF21_u32 as _);
pub const STATUS_CLOUD_FILE_VALIDATION_FAILED: NTSTATUS = NTSTATUS(0xC000CF0E_u32 as _);
pub const STATUS_CLUSTER_CAM_TICKET_REPLAY_DETECTED: NTSTATUS = NTSTATUS(0xC0130031_u32 as _);
pub const STATUS_CLUSTER_CSV_AUTO_PAUSE_ERROR: NTSTATUS = NTSTATUS(0xC0130021_u32 as _);
pub const STATUS_CLUSTER_CSV_INVALID_HANDLE: NTSTATUS = NTSTATUS(0xC0130029_u32 as _);
pub const STATUS_CLUSTER_CSV_NOT_REDIRECTED: NTSTATUS = NTSTATUS(0xC0130023_u32 as _);
pub const STATUS_CLUSTER_CSV_NO_SNAPSHOTS: NTSTATUS = NTSTATUS(0xC0130027_u32 as _);
pub const STATUS_CLUSTER_CSV_READ_OPLOCK_BREAK_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0130020_u32 as _);
pub const STATUS_CLUSTER_CSV_REDIRECTED: NTSTATUS = NTSTATUS(0xC0130022_u32 as _);
pub const STATUS_CLUSTER_CSV_SNAPSHOT_CREATION_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0130025_u32 as _);
pub const STATUS_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR: NTSTATUS = NTSTATUS(0xC0130030_u32 as _);
pub const STATUS_CLUSTER_CSV_VOLUME_DRAINING: NTSTATUS = NTSTATUS(0xC0130024_u32 as _);
pub const STATUS_CLUSTER_CSV_VOLUME_DRAINING_SUCCEEDED_DOWNLEVEL: NTSTATUS = NTSTATUS(0xC0130026_u32 as _);
pub const STATUS_CLUSTER_CSV_VOLUME_NOT_LOCAL: NTSTATUS = NTSTATUS(0xC0130019_u32 as _);
pub const STATUS_CLUSTER_INVALID_NETWORK: NTSTATUS = NTSTATUS(0xC0130010_u32 as _);
pub const STATUS_CLUSTER_INVALID_NETWORK_PROVIDER: NTSTATUS = NTSTATUS(0xC013000B_u32 as _);
pub const STATUS_CLUSTER_INVALID_NODE: NTSTATUS = NTSTATUS(0xC0130001_u32 as _);
pub const STATUS_CLUSTER_INVALID_REQUEST: NTSTATUS = NTSTATUS(0xC013000A_u32 as _);
pub const STATUS_CLUSTER_JOIN_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0130003_u32 as _);
pub const STATUS_CLUSTER_JOIN_NOT_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC013000F_u32 as _);
pub const STATUS_CLUSTER_LOCAL_NODE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0130005_u32 as _);
pub const STATUS_CLUSTER_NETINTERFACE_EXISTS: NTSTATUS = NTSTATUS(0xC0130008_u32 as _);
pub const STATUS_CLUSTER_NETINTERFACE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0130009_u32 as _);
pub const STATUS_CLUSTER_NETWORK_ALREADY_OFFLINE: NTSTATUS = NTSTATUS(0x80130004_u32 as _);
pub const STATUS_CLUSTER_NETWORK_ALREADY_ONLINE: NTSTATUS = NTSTATUS(0x80130003_u32 as _);
pub const STATUS_CLUSTER_NETWORK_EXISTS: NTSTATUS = NTSTATUS(0xC0130006_u32 as _);
pub const STATUS_CLUSTER_NETWORK_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0130007_u32 as _);
pub const STATUS_CLUSTER_NETWORK_NOT_INTERNAL: NTSTATUS = NTSTATUS(0xC0130016_u32 as _);
pub const STATUS_CLUSTER_NODE_ALREADY_DOWN: NTSTATUS = NTSTATUS(0x80130002_u32 as _);
pub const STATUS_CLUSTER_NODE_ALREADY_MEMBER: NTSTATUS = NTSTATUS(0x80130005_u32 as _);
pub const STATUS_CLUSTER_NODE_ALREADY_UP: NTSTATUS = NTSTATUS(0x80130001_u32 as _);
pub const STATUS_CLUSTER_NODE_DOWN: NTSTATUS = NTSTATUS(0xC013000C_u32 as _);
pub const STATUS_CLUSTER_NODE_EXISTS: NTSTATUS = NTSTATUS(0xC0130002_u32 as _);
pub const STATUS_CLUSTER_NODE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0130004_u32 as _);
pub const STATUS_CLUSTER_NODE_NOT_MEMBER: NTSTATUS = NTSTATUS(0xC013000E_u32 as _);
pub const STATUS_CLUSTER_NODE_NOT_PAUSED: NTSTATUS = NTSTATUS(0xC0130014_u32 as _);
pub const STATUS_CLUSTER_NODE_PAUSED: NTSTATUS = NTSTATUS(0xC0130013_u32 as _);
pub const STATUS_CLUSTER_NODE_UNREACHABLE: NTSTATUS = NTSTATUS(0xC013000D_u32 as _);
pub const STATUS_CLUSTER_NODE_UP: NTSTATUS = NTSTATUS(0xC0130012_u32 as _);
pub const STATUS_CLUSTER_NON_CSV_PATH: NTSTATUS = NTSTATUS(0xC0130018_u32 as _);
pub const STATUS_CLUSTER_NO_NET_ADAPTERS: NTSTATUS = NTSTATUS(0xC0130011_u32 as _);
pub const STATUS_CLUSTER_NO_SECURITY_CONTEXT: NTSTATUS = NTSTATUS(0xC0130015_u32 as _);
pub const STATUS_CLUSTER_POISONED: NTSTATUS = NTSTATUS(0xC0130017_u32 as _);
pub const STATUS_COMMITMENT_LIMIT: NTSTATUS = NTSTATUS(0xC000012D_u32 as _);
pub const STATUS_COMMITMENT_MINIMUM: NTSTATUS = NTSTATUS(0xC00002C8_u32 as _);
pub const STATUS_COMPRESSED_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000047B_u32 as _);
pub const STATUS_COMPRESSION_DISABLED: NTSTATUS = NTSTATUS(0xC0000426_u32 as _);
pub const STATUS_COMPRESSION_NOT_ALLOWED_IN_TRANSACTION: NTSTATUS = NTSTATUS(0xC0190056_u32 as _);
pub const STATUS_COMPRESSION_NOT_BENEFICIAL: NTSTATUS = NTSTATUS(0xC000046F_u32 as _);
pub const STATUS_CONFLICTING_ADDRESSES: NTSTATUS = NTSTATUS(0xC0000018_u32 as _);
pub const STATUS_CONNECTION_ABORTED: NTSTATUS = NTSTATUS(0xC0000241_u32 as _);
pub const STATUS_CONNECTION_ACTIVE: NTSTATUS = NTSTATUS(0xC000023B_u32 as _);
pub const STATUS_CONNECTION_COUNT_LIMIT: NTSTATUS = NTSTATUS(0xC0000246_u32 as _);
pub const STATUS_CONNECTION_DISCONNECTED: NTSTATUS = NTSTATUS(0xC000020C_u32 as _);
pub const STATUS_CONNECTION_INVALID: NTSTATUS = NTSTATUS(0xC000023A_u32 as _);
pub const STATUS_CONNECTION_IN_USE: NTSTATUS = NTSTATUS(0xC0000108_u32 as _);
pub const STATUS_CONNECTION_REFUSED: NTSTATUS = NTSTATUS(0xC0000236_u32 as _);
pub const STATUS_CONNECTION_RESET: NTSTATUS = NTSTATUS(0xC000020D_u32 as _);
pub const STATUS_CONTAINER_ASSIGNED: NTSTATUS = NTSTATUS(0xC0000508_u32 as _);
pub const STATUS_CONTENT_BLOCKED: NTSTATUS = NTSTATUS(0xC0000804_u32 as _);
pub const STATUS_CONTEXT_MISMATCH: NTSTATUS = NTSTATUS(0xC0000719_u32 as _);
pub const STATUS_CONTEXT_STOWED_EXCEPTION: NTSTATUS = NTSTATUS(0xC000027C_u32 as _);
pub const STATUS_CONTROL_C_EXIT: NTSTATUS = NTSTATUS(0xC000013A_u32 as _);
pub const STATUS_CONTROL_STACK_VIOLATION: NTSTATUS = NTSTATUS(0xC00001B2_u32 as _);
pub const STATUS_CONVERT_TO_LARGE: NTSTATUS = NTSTATUS(0xC000022C_u32 as _);
pub const STATUS_COPY_PROTECTION_FAILURE: NTSTATUS = NTSTATUS(0xC0000305_u32 as _);
pub const STATUS_CORRUPT_LOG_CLEARED: NTSTATUS = NTSTATUS(0xC000080D_u32 as _);
pub const STATUS_CORRUPT_LOG_CORRUPTED: NTSTATUS = NTSTATUS(0xC000080A_u32 as _);
pub const STATUS_CORRUPT_LOG_DELETED_FULL: NTSTATUS = NTSTATUS(0xC000080C_u32 as _);
pub const STATUS_CORRUPT_LOG_OVERFULL: NTSTATUS = NTSTATUS(0xC0000809_u32 as _);
pub const STATUS_CORRUPT_LOG_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC000080B_u32 as _);
pub const STATUS_CORRUPT_LOG_UPLEVEL_RECORDS: NTSTATUS = NTSTATUS(0xC0000811_u32 as _);
pub const STATUS_CORRUPT_SYSTEM_FILE: NTSTATUS = NTSTATUS(0xC00002C4_u32 as _);
pub const STATUS_COULD_NOT_INTERPRET: NTSTATUS = NTSTATUS(0xC00000B9_u32 as _);
pub const STATUS_COULD_NOT_RESIZE_LOG: NTSTATUS = NTSTATUS(0x80190009_u32 as _);
pub const STATUS_CPU_SET_INVALID: NTSTATUS = NTSTATUS(0xC00001AF_u32 as _);
pub const STATUS_CRASH_DUMP: NTSTATUS = NTSTATUS(0x116_u32 as _);
pub const STATUS_CRC_ERROR: NTSTATUS = NTSTATUS(0xC000003F_u32 as _);
pub const STATUS_CRED_REQUIRES_CONFIRMATION: NTSTATUS = NTSTATUS(0xC0000440_u32 as _);
pub const STATUS_CRM_PROTOCOL_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC019000F_u32 as _);
pub const STATUS_CRM_PROTOCOL_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0190011_u32 as _);
pub const STATUS_CROSSREALM_DELEGATION_FAILURE: NTSTATUS = NTSTATUS(0xC000040B_u32 as _);
pub const STATUS_CROSS_PARTITION_VIOLATION: NTSTATUS = NTSTATUS(0xC000060B_u32 as _);
pub const STATUS_CRYPTO_SYSTEM_INVALID: NTSTATUS = NTSTATUS(0xC00002F3_u32 as _);
pub const STATUS_CSS_AUTHENTICATION_FAILURE: NTSTATUS = NTSTATUS(0xC0000306_u32 as _);
pub const STATUS_CSS_KEY_NOT_ESTABLISHED: NTSTATUS = NTSTATUS(0xC0000308_u32 as _);
pub const STATUS_CSS_KEY_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC0000307_u32 as _);
pub const STATUS_CSS_REGION_MISMATCH: NTSTATUS = NTSTATUS(0xC000030A_u32 as _);
pub const STATUS_CSS_RESETS_EXHAUSTED: NTSTATUS = NTSTATUS(0xC000030B_u32 as _);
pub const STATUS_CSS_SCRAMBLED_SECTOR: NTSTATUS = NTSTATUS(0xC0000309_u32 as _);
pub const STATUS_CSV_IO_PAUSE_TIMEOUT: NTSTATUS = NTSTATUS(0xC0130028_u32 as _);
pub const STATUS_CS_ENCRYPTION_EXISTING_ENCRYPTED_FILE: NTSTATUS = NTSTATUS(0xC0000443_u32 as _);
pub const STATUS_CS_ENCRYPTION_FILE_NOT_CSE: NTSTATUS = NTSTATUS(0xC0000445_u32 as _);
pub const STATUS_CS_ENCRYPTION_INVALID_SERVER_RESPONSE: NTSTATUS = NTSTATUS(0xC0000441_u32 as _);
pub const STATUS_CS_ENCRYPTION_NEW_ENCRYPTED_FILE: NTSTATUS = NTSTATUS(0xC0000444_u32 as _);
pub const STATUS_CS_ENCRYPTION_UNSUPPORTED_SERVER: NTSTATUS = NTSTATUS(0xC0000442_u32 as _);
pub const STATUS_CTLOG_INCONSISTENT_TRACKING_FILE: NTSTATUS = NTSTATUS(0xC03A0024_u32 as _);
pub const STATUS_CTLOG_INVALID_TRACKING_STATE: NTSTATUS = NTSTATUS(0xC03A0023_u32 as _);
pub const STATUS_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE: NTSTATUS = NTSTATUS(0xC03A0021_u32 as _);
pub const STATUS_CTLOG_TRACKING_NOT_INITIALIZED: NTSTATUS = NTSTATUS(0xC03A0020_u32 as _);
pub const STATUS_CTLOG_VHD_CHANGED_OFFLINE: NTSTATUS = NTSTATUS(0xC03A0022_u32 as _);
pub const STATUS_CTL_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0000057_u32 as _);
pub const STATUS_CTX_BAD_VIDEO_MODE: NTSTATUS = NTSTATUS(0xC00A0018_u32 as _);
pub const STATUS_CTX_CDM_CONNECT: NTSTATUS = NTSTATUS(0x400A0004_u32 as _);
pub const STATUS_CTX_CDM_DISCONNECT: NTSTATUS = NTSTATUS(0x400A0005_u32 as _);
pub const STATUS_CTX_CLIENT_LICENSE_IN_USE: NTSTATUS = NTSTATUS(0xC00A0034_u32 as _);
pub const STATUS_CTX_CLIENT_LICENSE_NOT_SET: NTSTATUS = NTSTATUS(0xC00A0033_u32 as _);
pub const STATUS_CTX_CLIENT_QUERY_TIMEOUT: NTSTATUS = NTSTATUS(0xC00A0026_u32 as _);
pub const STATUS_CTX_CLOSE_PENDING: NTSTATUS = NTSTATUS(0xC00A0006_u32 as _);
pub const STATUS_CTX_CONSOLE_CONNECT: NTSTATUS = NTSTATUS(0xC00A0028_u32 as _);
pub const STATUS_CTX_CONSOLE_DISCONNECT: NTSTATUS = NTSTATUS(0xC00A0027_u32 as _);
pub const STATUS_CTX_GRAPHICS_INVALID: NTSTATUS = NTSTATUS(0xC00A0022_u32 as _);
pub const STATUS_CTX_INVALID_MODEMNAME: NTSTATUS = NTSTATUS(0xC00A0009_u32 as _);
pub const STATUS_CTX_INVALID_PD: NTSTATUS = NTSTATUS(0xC00A0002_u32 as _);
pub const STATUS_CTX_INVALID_WD: NTSTATUS = NTSTATUS(0xC00A002E_u32 as _);
pub const STATUS_CTX_LICENSE_CLIENT_INVALID: NTSTATUS = NTSTATUS(0xC00A0012_u32 as _);
pub const STATUS_CTX_LICENSE_EXPIRED: NTSTATUS = NTSTATUS(0xC00A0014_u32 as _);
pub const STATUS_CTX_LICENSE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC00A0013_u32 as _);
pub const STATUS_CTX_LOGON_DISABLED: NTSTATUS = NTSTATUS(0xC00A0037_u32 as _);
pub const STATUS_CTX_MODEM_INF_NOT_FOUND: NTSTATUS = NTSTATUS(0xC00A0008_u32 as _);
pub const STATUS_CTX_MODEM_RESPONSE_BUSY: NTSTATUS = NTSTATUS(0xC00A000E_u32 as _);
pub const STATUS_CTX_MODEM_RESPONSE_NO_CARRIER: NTSTATUS = NTSTATUS(0xC00A000C_u32 as _);
pub const STATUS_CTX_MODEM_RESPONSE_NO_DIALTONE: NTSTATUS = NTSTATUS(0xC00A000D_u32 as _);
pub const STATUS_CTX_MODEM_RESPONSE_TIMEOUT: NTSTATUS = NTSTATUS(0xC00A000B_u32 as _);
pub const STATUS_CTX_MODEM_RESPONSE_VOICE: NTSTATUS = NTSTATUS(0xC00A000F_u32 as _);
pub const STATUS_CTX_NOT_CONSOLE: NTSTATUS = NTSTATUS(0xC00A0024_u32 as _);
pub const STATUS_CTX_NO_OUTBUF: NTSTATUS = NTSTATUS(0xC00A0007_u32 as _);
pub const STATUS_CTX_PD_NOT_FOUND: NTSTATUS = NTSTATUS(0xC00A0003_u32 as _);
pub const STATUS_CTX_RESPONSE_ERROR: NTSTATUS = NTSTATUS(0xC00A000A_u32 as _);
pub const STATUS_CTX_SECURITY_LAYER_ERROR: NTSTATUS = NTSTATUS(0xC00A0038_u32 as _);
pub const STATUS_CTX_SHADOW_DENIED: NTSTATUS = NTSTATUS(0xC00A002A_u32 as _);
pub const STATUS_CTX_SHADOW_DISABLED: NTSTATUS = NTSTATUS(0xC00A0031_u32 as _);
pub const STATUS_CTX_SHADOW_ENDED_BY_MODE_CHANGE: NTSTATUS = NTSTATUS(0xC00A0035_u32 as _);
pub const STATUS_CTX_SHADOW_INVALID: NTSTATUS = NTSTATUS(0xC00A0030_u32 as _);
pub const STATUS_CTX_SHADOW_NOT_RUNNING: NTSTATUS = NTSTATUS(0xC00A0036_u32 as _);
pub const STATUS_CTX_TD_ERROR: NTSTATUS = NTSTATUS(0xC00A0010_u32 as _);
pub const STATUS_CTX_WD_NOT_FOUND: NTSTATUS = NTSTATUS(0xC00A002F_u32 as _);
pub const STATUS_CTX_WINSTATION_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC00A002B_u32 as _);
pub const STATUS_CTX_WINSTATION_BUSY: NTSTATUS = NTSTATUS(0xC00A0017_u32 as _);
pub const STATUS_CTX_WINSTATION_NAME_COLLISION: NTSTATUS = NTSTATUS(0xC00A0016_u32 as _);
pub const STATUS_CTX_WINSTATION_NAME_INVALID: NTSTATUS = NTSTATUS(0xC00A0001_u32 as _);
pub const STATUS_CTX_WINSTATION_NOT_FOUND: NTSTATUS = NTSTATUS(0xC00A0015_u32 as _);
pub const STATUS_CURRENT_DOMAIN_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC00002E9_u32 as _);
pub const STATUS_CURRENT_TRANSACTION_NOT_VALID: NTSTATUS = NTSTATUS(0xC0190018_u32 as _);
pub const STATUS_DATATYPE_MISALIGNMENT: NTSTATUS = NTSTATUS(0x80000002_u32 as _);
pub const STATUS_DATATYPE_MISALIGNMENT_ERROR: NTSTATUS = NTSTATUS(0xC00002C5_u32 as _);
pub const STATUS_DATA_CHECKSUM_ERROR: NTSTATUS = NTSTATUS(0xC0000470_u32 as _);
pub const STATUS_DATA_ERROR: NTSTATUS = NTSTATUS(0xC000003E_u32 as _);
pub const STATUS_DATA_LATE_ERROR: NTSTATUS = NTSTATUS(0xC000003D_u32 as _);
pub const STATUS_DATA_LOST_REPAIR: NTSTATUS = NTSTATUS(0x80000803_u32 as _);
pub const STATUS_DATA_NOT_ACCEPTED: NTSTATUS = NTSTATUS(0xC000021B_u32 as _);
pub const STATUS_DATA_OVERRUN: NTSTATUS = NTSTATUS(0xC000003C_u32 as _);
pub const STATUS_DATA_OVERWRITTEN: NTSTATUS = NTSTATUS(0x130_u32 as _);
pub const STATUS_DAX_MAPPING_EXISTS: NTSTATUS = NTSTATUS(0xC000049C_u32 as _);
pub const STATUS_DEBUGGER_INACTIVE: NTSTATUS = NTSTATUS(0xC0000354_u32 as _);
pub const STATUS_DEBUG_ATTACH_FAILED: NTSTATUS = NTSTATUS(0xC0000219_u32 as _);
pub const STATUS_DECRYPTION_FAILED: NTSTATUS = NTSTATUS(0xC000028B_u32 as _);
pub const STATUS_DELAY_LOAD_FAILED: NTSTATUS = NTSTATUS(0xC0000412_u32 as _);
pub const STATUS_DELETE_PENDING: NTSTATUS = NTSTATUS(0xC0000056_u32 as _);
pub const STATUS_DESTINATION_ELEMENT_FULL: NTSTATUS = NTSTATUS(0xC0000284_u32 as _);
pub const STATUS_DEVICE_ALREADY_ATTACHED: NTSTATUS = NTSTATUS(0xC0000038_u32 as _);
pub const STATUS_DEVICE_BUSY: NTSTATUS = NTSTATUS(0x80000011_u32 as _);
pub const STATUS_DEVICE_CONFIGURATION_ERROR: NTSTATUS = NTSTATUS(0xC0000182_u32 as _);
pub const STATUS_DEVICE_DATA_ERROR: NTSTATUS = NTSTATUS(0xC000009C_u32 as _);
pub const STATUS_DEVICE_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(0xC00000C0_u32 as _);
pub const STATUS_DEVICE_DOOR_OPEN: NTSTATUS = NTSTATUS(0x80000289_u32 as _);
pub const STATUS_DEVICE_ENUMERATION_ERROR: NTSTATUS = NTSTATUS(0xC0000366_u32 as _);
pub const STATUS_DEVICE_FEATURE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0000463_u32 as _);
pub const STATUS_DEVICE_HARDWARE_ERROR: NTSTATUS = NTSTATUS(0xC0000483_u32 as _);
pub const STATUS_DEVICE_HINT_NAME_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(0xC0000496_u32 as _);
pub const STATUS_DEVICE_HUNG: NTSTATUS = NTSTATUS(0xC0000507_u32 as _);
pub const STATUS_DEVICE_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(0xC0000468_u32 as _);
pub const STATUS_DEVICE_IN_MAINTENANCE: NTSTATUS = NTSTATUS(0xC0000499_u32 as _);
pub const STATUS_DEVICE_NOT_CONNECTED: NTSTATUS = NTSTATUS(0xC000009D_u32 as _);
pub const STATUS_DEVICE_NOT_PARTITIONED: NTSTATUS = NTSTATUS(0xC0000174_u32 as _);
pub const STATUS_DEVICE_NOT_READY: NTSTATUS = NTSTATUS(0xC00000A3_u32 as _);
pub const STATUS_DEVICE_OFF_LINE: NTSTATUS = NTSTATUS(0x80000010_u32 as _);
pub const STATUS_DEVICE_PAPER_EMPTY: NTSTATUS = NTSTATUS(0x8000000E_u32 as _);
pub const STATUS_DEVICE_POWERED_OFF: NTSTATUS = NTSTATUS(0x8000000F_u32 as _);
pub const STATUS_DEVICE_POWER_CYCLE_REQUIRED: NTSTATUS = NTSTATUS(0x80000031_u32 as _);
pub const STATUS_DEVICE_POWER_FAILURE: NTSTATUS = NTSTATUS(0xC000009E_u32 as _);
pub const STATUS_DEVICE_PROTOCOL_ERROR: NTSTATUS = NTSTATUS(0xC0000186_u32 as _);
pub const STATUS_DEVICE_REMOVED: NTSTATUS = NTSTATUS(0xC00002B6_u32 as _);
pub const STATUS_DEVICE_REQUIRES_CLEANING: NTSTATUS = NTSTATUS(0x80000288_u32 as _);
pub const STATUS_DEVICE_RESET_REQUIRED: NTSTATUS = NTSTATUS(0x800001B6_u32 as _);
pub const STATUS_DEVICE_SUPPORT_IN_PROGRESS: NTSTATUS = NTSTATUS(0x80000030_u32 as _);
pub const STATUS_DEVICE_UNREACHABLE: NTSTATUS = NTSTATUS(0xC0000464_u32 as _);
pub const STATUS_DEVICE_UNRESPONSIVE: NTSTATUS = NTSTATUS(0xC000050A_u32 as _);
pub const STATUS_DFS_EXIT_PATH_FOUND: NTSTATUS = NTSTATUS(0xC000009B_u32 as _);
pub const STATUS_DFS_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC000026D_u32 as _);
pub const STATUS_DIF_BINDING_API_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000C7F_u32 as _);
pub const STATUS_DIF_IOCALLBACK_NOT_REPLACED: NTSTATUS = NTSTATUS(0xC0000C76_u32 as _);
pub const STATUS_DIF_LIVEDUMP_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000C77_u32 as _);
pub const STATUS_DIF_VOLATILE_DRIVER_HOTPATCHED: NTSTATUS = NTSTATUS(0xC0000C79_u32 as _);
pub const STATUS_DIF_VOLATILE_DRIVER_IS_NOT_RUNNING: NTSTATUS = NTSTATUS(0xC0000C7B_u32 as _);
pub const STATUS_DIF_VOLATILE_INVALID_INFO: NTSTATUS = NTSTATUS(0xC0000C7A_u32 as _);
pub const STATUS_DIF_VOLATILE_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0000C7E_u32 as _);
pub const STATUS_DIF_VOLATILE_PLUGIN_CHANGE_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0000C7D_u32 as _);
pub const STATUS_DIF_VOLATILE_PLUGIN_IS_NOT_RUNNING: NTSTATUS = NTSTATUS(0xC0000C7C_u32 as _);
pub const STATUS_DIF_VOLATILE_SECTION_NOT_LOCKED: NTSTATUS = NTSTATUS(0xC0000C78_u32 as _);
pub const STATUS_DIRECTORY_IS_A_REPARSE_POINT: NTSTATUS = NTSTATUS(0xC0000281_u32 as _);
pub const STATUS_DIRECTORY_NOT_EMPTY: NTSTATUS = NTSTATUS(0xC0000101_u32 as _);
pub const STATUS_DIRECTORY_NOT_RM: NTSTATUS = NTSTATUS(0xC0190008_u32 as _);
pub const STATUS_DIRECTORY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000047C_u32 as _);
pub const STATUS_DIRECTORY_SERVICE_REQUIRED: NTSTATUS = NTSTATUS(0xC00002B1_u32 as _);
pub const STATUS_DISK_CORRUPT_ERROR: NTSTATUS = NTSTATUS(0xC0000032_u32 as _);
pub const STATUS_DISK_FULL: NTSTATUS = NTSTATUS(0xC000007F_u32 as _);
pub const STATUS_DISK_OPERATION_FAILED: NTSTATUS = NTSTATUS(0xC000016A_u32 as _);
pub const STATUS_DISK_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000802_u32 as _);
pub const STATUS_DISK_RECALIBRATE_FAILED: NTSTATUS = NTSTATUS(0xC0000169_u32 as _);
pub const STATUS_DISK_REPAIR_DISABLED: NTSTATUS = NTSTATUS(0xC0000800_u32 as _);
pub const STATUS_DISK_REPAIR_REDIRECTED: NTSTATUS = NTSTATUS(0x40000807_u32 as _);
pub const STATUS_DISK_REPAIR_UNSUCCESSFUL: NTSTATUS = NTSTATUS(0xC0000808_u32 as _);
pub const STATUS_DISK_RESET_FAILED: NTSTATUS = NTSTATUS(0xC000016B_u32 as _);
pub const STATUS_DISK_RESOURCES_EXHAUSTED: NTSTATUS = NTSTATUS(0xC0000461_u32 as _);
pub const STATUS_DLL_INIT_FAILED: NTSTATUS = NTSTATUS(0xC0000142_u32 as _);
pub const STATUS_DLL_INIT_FAILED_LOGOFF: NTSTATUS = NTSTATUS(0xC000026B_u32 as _);
pub const STATUS_DLL_MIGHT_BE_INCOMPATIBLE: NTSTATUS = NTSTATUS(0x8000002C_u32 as _);
pub const STATUS_DLL_MIGHT_BE_INSECURE: NTSTATUS = NTSTATUS(0x8000002B_u32 as _);
pub const STATUS_DLL_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000135_u32 as _);
pub const STATUS_DM_OPERATION_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC0370600_u32 as _);
pub const STATUS_DOMAIN_CONTROLLER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000233_u32 as _);
pub const STATUS_DOMAIN_CTRLR_CONFIG_ERROR: NTSTATUS = NTSTATUS(0xC000015E_u32 as _);
pub const STATUS_DOMAIN_EXISTS: NTSTATUS = NTSTATUS(0xC00000E0_u32 as _);
pub const STATUS_DOMAIN_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC00000E1_u32 as _);
pub const STATUS_DOMAIN_TRUST_INCONSISTENT: NTSTATUS = NTSTATUS(0xC000019B_u32 as _);
pub const STATUS_DRIVERS_LEAKING_LOCKED_PAGES: NTSTATUS = NTSTATUS(0x4000002D_u32 as _);
pub const STATUS_DRIVER_BLOCKED: NTSTATUS = NTSTATUS(0xC000036C_u32 as _);
pub const STATUS_DRIVER_BLOCKED_CRITICAL: NTSTATUS = NTSTATUS(0xC000036B_u32 as _);
pub const STATUS_DRIVER_CANCEL_TIMEOUT: NTSTATUS = NTSTATUS(0xC000021E_u32 as _);
pub const STATUS_DRIVER_DATABASE_ERROR: NTSTATUS = NTSTATUS(0xC000036D_u32 as _);
pub const STATUS_DRIVER_ENTRYPOINT_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000263_u32 as _);
pub const STATUS_DRIVER_FAILED_PRIOR_UNLOAD: NTSTATUS = NTSTATUS(0xC000038E_u32 as _);
pub const STATUS_DRIVER_FAILED_SLEEP: NTSTATUS = NTSTATUS(0xC00002C2_u32 as _);
pub const STATUS_DRIVER_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC0000183_u32 as _);
pub const STATUS_DRIVER_ORDINAL_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000262_u32 as _);
pub const STATUS_DRIVER_PROCESS_TERMINATED: NTSTATUS = NTSTATUS(0xC0000450_u32 as _);
pub const STATUS_DRIVER_UNABLE_TO_LOAD: NTSTATUS = NTSTATUS(0xC000026C_u32 as _);
pub const STATUS_DS_ADMIN_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC00002C1_u32 as _);
pub const STATUS_DS_AG_CANT_HAVE_UNIVERSAL_MEMBER: NTSTATUS = NTSTATUS(0xC0000358_u32 as _);
pub const STATUS_DS_ATTRIBUTE_OR_VALUE_EXISTS: NTSTATUS = NTSTATUS(0xC00002A4_u32 as _);
pub const STATUS_DS_ATTRIBUTE_TYPE_UNDEFINED: NTSTATUS = NTSTATUS(0xC00002A3_u32 as _);
pub const STATUS_DS_BUSY: NTSTATUS = NTSTATUS(0xC00002A5_u32 as _);
pub const STATUS_DS_CANT_MOD_OBJ_CLASS: NTSTATUS = NTSTATUS(0xC00002AE_u32 as _);
pub const STATUS_DS_CANT_MOD_PRIMARYGROUPID: NTSTATUS = NTSTATUS(0xC00002D0_u32 as _);
pub const STATUS_DS_CANT_ON_NON_LEAF: NTSTATUS = NTSTATUS(0xC00002AC_u32 as _);
pub const STATUS_DS_CANT_ON_RDN: NTSTATUS = NTSTATUS(0xC00002AD_u32 as _);
pub const STATUS_DS_CANT_START: NTSTATUS = NTSTATUS(0xC00002E1_u32 as _);
pub const STATUS_DS_CROSS_DOM_MOVE_FAILED: NTSTATUS = NTSTATUS(0xC00002AF_u32 as _);
pub const STATUS_DS_DOMAIN_NAME_EXISTS_IN_FOREST: NTSTATUS = NTSTATUS(0xC000041A_u32 as _);
pub const STATUS_DS_DOMAIN_RENAME_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0000801_u32 as _);
pub const STATUS_DS_DUPLICATE_ID_FOUND: NTSTATUS = NTSTATUS(0xC0000405_u32 as _);
pub const STATUS_DS_FLAT_NAME_EXISTS_IN_FOREST: NTSTATUS = NTSTATUS(0xC000041B_u32 as _);
pub const STATUS_DS_GC_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC00002B0_u32 as _);
pub const STATUS_DS_GC_REQUIRED: NTSTATUS = NTSTATUS(0xC00002E4_u32 as _);
pub const STATUS_DS_GLOBAL_CANT_HAVE_CROSSDOMAIN_MEMBER: NTSTATUS = NTSTATUS(0xC00002DA_u32 as _);
pub const STATUS_DS_GLOBAL_CANT_HAVE_LOCAL_MEMBER: NTSTATUS = NTSTATUS(0xC00002D7_u32 as _);
pub const STATUS_DS_GLOBAL_CANT_HAVE_UNIVERSAL_MEMBER: NTSTATUS = NTSTATUS(0xC00002D8_u32 as _);
pub const STATUS_DS_GROUP_CONVERSION_ERROR: NTSTATUS = NTSTATUS(0xC0000406_u32 as _);
pub const STATUS_DS_HAVE_PRIMARY_MEMBERS: NTSTATUS = NTSTATUS(0xC00002DC_u32 as _);
pub const STATUS_DS_INCORRECT_ROLE_OWNER: NTSTATUS = NTSTATUS(0xC00002A9_u32 as _);
pub const STATUS_DS_INIT_FAILURE: NTSTATUS = NTSTATUS(0xC00002E2_u32 as _);
pub const STATUS_DS_INIT_FAILURE_CONSOLE: NTSTATUS = NTSTATUS(0xC00002EC_u32 as _);
pub const STATUS_DS_INVALID_ATTRIBUTE_SYNTAX: NTSTATUS = NTSTATUS(0xC00002A2_u32 as _);
pub const STATUS_DS_INVALID_GROUP_TYPE: NTSTATUS = NTSTATUS(0xC00002D4_u32 as _);
pub const STATUS_DS_LOCAL_CANT_HAVE_CROSSDOMAIN_LOCAL_MEMBER: NTSTATUS = NTSTATUS(0xC00002DB_u32 as _);
pub const STATUS_DS_LOCAL_MEMBER_OF_LOCAL_ONLY: NTSTATUS = NTSTATUS(0xC00002E5_u32 as _);
pub const STATUS_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC00002E7_u32 as _);
pub const STATUS_DS_MEMBERSHIP_EVALUATED_LOCALLY: NTSTATUS = NTSTATUS(0x121_u32 as _);
pub const STATUS_DS_NAME_NOT_UNIQUE: NTSTATUS = NTSTATUS(0xC0000404_u32 as _);
pub const STATUS_DS_NO_ATTRIBUTE_OR_VALUE: NTSTATUS = NTSTATUS(0xC00002A1_u32 as _);
pub const STATUS_DS_NO_FPO_IN_UNIVERSAL_GROUPS: NTSTATUS = NTSTATUS(0xC00002E6_u32 as _);
pub const STATUS_DS_NO_MORE_RIDS: NTSTATUS = NTSTATUS(0xC00002A8_u32 as _);
pub const STATUS_DS_NO_NEST_GLOBALGROUP_IN_MIXEDDOMAIN: NTSTATUS = NTSTATUS(0xC00002D5_u32 as _);
pub const STATUS_DS_NO_NEST_LOCALGROUP_IN_MIXEDDOMAIN: NTSTATUS = NTSTATUS(0xC00002D6_u32 as _);
pub const STATUS_DS_NO_RIDS_ALLOCATED: NTSTATUS = NTSTATUS(0xC00002A7_u32 as _);
pub const STATUS_DS_OBJ_CLASS_VIOLATION: NTSTATUS = NTSTATUS(0xC00002AB_u32 as _);
pub const STATUS_DS_OID_MAPPED_GROUP_CANT_HAVE_MEMBERS: NTSTATUS = NTSTATUS(0xC000A087_u32 as _);
pub const STATUS_DS_OID_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000A088_u32 as _);
pub const STATUS_DS_RIDMGR_DISABLED: NTSTATUS = NTSTATUS(0xC00002BA_u32 as _);
pub const STATUS_DS_RIDMGR_INIT_ERROR: NTSTATUS = NTSTATUS(0xC00002AA_u32 as _);
pub const STATUS_DS_SAM_INIT_FAILURE: NTSTATUS = NTSTATUS(0xC00002CB_u32 as _);
pub const STATUS_DS_SAM_INIT_FAILURE_CONSOLE: NTSTATUS = NTSTATUS(0xC00002ED_u32 as _);
pub const STATUS_DS_SENSITIVE_GROUP_VIOLATION: NTSTATUS = NTSTATUS(0xC00002CD_u32 as _);
pub const STATUS_DS_SHUTTING_DOWN: NTSTATUS = NTSTATUS(0x40000370_u32 as _);
pub const STATUS_DS_SRC_SID_EXISTS_IN_FOREST: NTSTATUS = NTSTATUS(0xC0000419_u32 as _);
pub const STATUS_DS_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC00002A6_u32 as _);
pub const STATUS_DS_UNIVERSAL_CANT_HAVE_LOCAL_MEMBER: NTSTATUS = NTSTATUS(0xC00002D9_u32 as _);
pub const STATUS_DS_VERSION_CHECK_FAILURE: NTSTATUS = NTSTATUS(0xC0000355_u32 as _);
pub const STATUS_DUPLICATE_NAME: NTSTATUS = NTSTATUS(0xC00000BD_u32 as _);
pub const STATUS_DUPLICATE_OBJECTID: NTSTATUS = NTSTATUS(0xC000022A_u32 as _);
pub const STATUS_DUPLICATE_PRIVILEGES: NTSTATUS = NTSTATUS(0xC00001A6_u32 as _);
pub const STATUS_DYNAMIC_CODE_BLOCKED: NTSTATUS = NTSTATUS(0xC0000604_u32 as _);
pub const STATUS_EAS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000004F_u32 as _);
pub const STATUS_EA_CORRUPT_ERROR: NTSTATUS = NTSTATUS(0xC0000053_u32 as _);
pub const STATUS_EA_LIST_INCONSISTENT: NTSTATUS = NTSTATUS(0x80000014_u32 as _);
pub const STATUS_EA_TOO_LARGE: NTSTATUS = NTSTATUS(0xC0000050_u32 as _);
pub const STATUS_EFS_ALG_BLOB_TOO_BIG: NTSTATUS = NTSTATUS(0xC0000352_u32 as _);
pub const STATUS_EFS_NOT_ALLOWED_IN_TRANSACTION: NTSTATUS = NTSTATUS(0xC019003E_u32 as _);
pub const STATUS_ELEVATION_REQUIRED: NTSTATUS = NTSTATUS(0xC000042C_u32 as _);
pub const STATUS_EMULATION_BREAKPOINT: NTSTATUS = NTSTATUS(0x40000038_u32 as _);
pub const STATUS_EMULATION_SYSCALL: NTSTATUS = NTSTATUS(0x40000039_u32 as _);
pub const STATUS_ENCLAVE_FAILURE: NTSTATUS = NTSTATUS(0xC000048F_u32 as _);
pub const STATUS_ENCLAVE_IS_TERMINATING: NTSTATUS = NTSTATUS(0xC0000512_u32 as _);
pub const STATUS_ENCLAVE_NOT_TERMINATED: NTSTATUS = NTSTATUS(0xC0000511_u32 as _);
pub const STATUS_ENCLAVE_VIOLATION: NTSTATUS = NTSTATUS(0xC00004A2_u32 as _);
pub const STATUS_ENCOUNTERED_WRITE_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0000433_u32 as _);
pub const STATUS_ENCRYPTED_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00004C3_u32 as _);
pub const STATUS_ENCRYPTED_IO_NOT_POSSIBLE: NTSTATUS = NTSTATUS(0xC0000810_u32 as _);
pub const STATUS_ENCRYPTING_METADATA_DISALLOWED: NTSTATUS = NTSTATUS(0xC00004B7_u32 as _);
pub const STATUS_ENCRYPTION_DISABLED: NTSTATUS = NTSTATUS(0xC00004B6_u32 as _);
pub const STATUS_ENCRYPTION_FAILED: NTSTATUS = NTSTATUS(0xC000028A_u32 as _);
pub const STATUS_END_OF_FILE: NTSTATUS = NTSTATUS(0xC0000011_u32 as _);
pub const STATUS_END_OF_MEDIA: NTSTATUS = NTSTATUS(0x8000001E_u32 as _);
pub const STATUS_ENLISTMENT_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0190050_u32 as _);
pub const STATUS_ENLISTMENT_NOT_SUPERIOR: NTSTATUS = NTSTATUS(0xC0190033_u32 as _);
pub const STATUS_ENTRYPOINT_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000139_u32 as _);
pub const STATUS_EOF_ON_GHOSTED_RANGE: NTSTATUS = NTSTATUS(0xC000A007_u32 as _);
pub const STATUS_EOM_OVERFLOW: NTSTATUS = NTSTATUS(0xC0000177_u32 as _);
pub const STATUS_ERROR_PROCESS_NOT_IN_JOB: NTSTATUS = NTSTATUS(0xC00001AE_u32 as _);
pub const STATUS_EVALUATION_EXPIRATION: NTSTATUS = NTSTATUS(0xC0000268_u32 as _);
pub const STATUS_EVENTLOG_CANT_START: NTSTATUS = NTSTATUS(0xC000018F_u32 as _);
pub const STATUS_EVENTLOG_FILE_CHANGED: NTSTATUS = NTSTATUS(0xC0000197_u32 as _);
pub const STATUS_EVENTLOG_FILE_CORRUPT: NTSTATUS = NTSTATUS(0xC000018E_u32 as _);
pub const STATUS_EVENT_DONE: NTSTATUS = NTSTATUS(0x40000012_u32 as _);
pub const STATUS_EVENT_PENDING: NTSTATUS = NTSTATUS(0x40000013_u32 as _);
pub const STATUS_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(0xC0000723_u32 as _);
pub const STATUS_EXPIRED_HANDLE: NTSTATUS = NTSTATUS(0xC0190060_u32 as _);
pub const STATUS_EXTERNAL_BACKING_PROVIDER_UNKNOWN: NTSTATUS = NTSTATUS(0xC000046E_u32 as _);
pub const STATUS_EXTERNAL_SYSKEY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00004A1_u32 as _);
pub const STATUS_EXTRANEOUS_INFORMATION: NTSTATUS = NTSTATUS(0x80000017_u32 as _);
pub const STATUS_FAILED_DRIVER_ENTRY: NTSTATUS = NTSTATUS(0xC0000365_u32 as _);
pub const STATUS_FAILED_STACK_SWITCH: NTSTATUS = NTSTATUS(0xC0000373_u32 as _);
pub const STATUS_FAIL_CHECK: NTSTATUS = NTSTATUS(0xC0000229_u32 as _);
pub const STATUS_FAIL_FAST_EXCEPTION: NTSTATUS = NTSTATUS(0xC0000602_u32 as _);
pub const STATUS_FASTPATH_REJECTED: NTSTATUS = NTSTATUS(0xC000A014_u32 as _);
pub const STATUS_FATAL_APP_EXIT: NTSTATUS = NTSTATUS(0x40000015_u32 as _);
pub const STATUS_FATAL_MEMORY_EXHAUSTION: NTSTATUS = NTSTATUS(0xC00001AD_u32 as _);
pub const STATUS_FATAL_USER_CALLBACK_EXCEPTION: NTSTATUS = NTSTATUS(0xC000041D_u32 as _);
pub const STATUS_FILEMARK_DETECTED: NTSTATUS = NTSTATUS(0x8000001B_u32 as _);
pub const STATUS_FILES_OPEN: NTSTATUS = NTSTATUS(0xC0000107_u32 as _);
pub const STATUS_FILE_CHECKED_OUT: NTSTATUS = NTSTATUS(0xC0000901_u32 as _);
pub const STATUS_FILE_CLOSED: NTSTATUS = NTSTATUS(0xC0000128_u32 as _);
pub const STATUS_FILE_CORRUPT_ERROR: NTSTATUS = NTSTATUS(0xC0000102_u32 as _);
pub const STATUS_FILE_DELETED: NTSTATUS = NTSTATUS(0xC0000123_u32 as _);
pub const STATUS_FILE_ENCRYPTED: NTSTATUS = NTSTATUS(0xC0000293_u32 as _);
pub const STATUS_FILE_FORCED_CLOSED: NTSTATUS = NTSTATUS(0xC00000B6_u32 as _);
pub const STATUS_FILE_HANDLE_REVOKED: NTSTATUS = NTSTATUS(0xC0000910_u32 as _);
pub const STATUS_FILE_IDENTITY_NOT_PERSISTENT: NTSTATUS = NTSTATUS(0xC0190036_u32 as _);
pub const STATUS_FILE_INVALID: NTSTATUS = NTSTATUS(0xC0000098_u32 as _);
pub const STATUS_FILE_IS_A_DIRECTORY: NTSTATUS = NTSTATUS(0xC00000BA_u32 as _);
pub const STATUS_FILE_IS_OFFLINE: NTSTATUS = NTSTATUS(0xC0000267_u32 as _);
pub const STATUS_FILE_LOCKED_WITH_ONLY_READERS: NTSTATUS = NTSTATUS(0x12A_u32 as _);
pub const STATUS_FILE_LOCKED_WITH_WRITERS: NTSTATUS = NTSTATUS(0x12B_u32 as _);
pub const STATUS_FILE_LOCK_CONFLICT: NTSTATUS = NTSTATUS(0xC0000054_u32 as _);
pub const STATUS_FILE_METADATA_OPTIMIZATION_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC00001AB_u32 as _);
pub const STATUS_FILE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC0000467_u32 as _);
pub const STATUS_FILE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(0xC0000291_u32 as _);
pub const STATUS_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00004B4_u32 as _);
pub const STATUS_FILE_PROTECTED_UNDER_DPL: NTSTATUS = NTSTATUS(0xC00004A3_u32 as _);
pub const STATUS_FILE_RENAMED: NTSTATUS = NTSTATUS(0xC00000D5_u32 as _);
pub const STATUS_FILE_SNAP_INVALID_PARAMETER: NTSTATUS = NTSTATUS(0xC000F505_u32 as _);
pub const STATUS_FILE_SNAP_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC000F500_u32 as _);
pub const STATUS_FILE_SNAP_IO_NOT_COORDINATED: NTSTATUS = NTSTATUS(0xC000F503_u32 as _);
pub const STATUS_FILE_SNAP_MODIFY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000F502_u32 as _);
pub const STATUS_FILE_SNAP_UNEXPECTED_ERROR: NTSTATUS = NTSTATUS(0xC000F504_u32 as _);
pub const STATUS_FILE_SNAP_USER_SECTION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000F501_u32 as _);
pub const STATUS_FILE_SYSTEM_LIMITATION: NTSTATUS = NTSTATUS(0xC0000427_u32 as _);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_BUSY: NTSTATUS = NTSTATUS(0xC000CE03_u32 as _);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_INVALID_OPERATION: NTSTATUS = NTSTATUS(0xC000CE05_u32 as _);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_METADATA_CORRUPT: NTSTATUS = NTSTATUS(0xC000CE02_u32 as _);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_PROVIDER_UNKNOWN: NTSTATUS = NTSTATUS(0xC000CE04_u32 as _);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC000CE01_u32 as _);
pub const STATUS_FILE_TOO_LARGE: NTSTATUS = NTSTATUS(0xC0000904_u32 as _);
pub const STATUS_FIRMWARE_IMAGE_INVALID: NTSTATUS = NTSTATUS(0xC0000485_u32 as _);
pub const STATUS_FIRMWARE_SLOT_INVALID: NTSTATUS = NTSTATUS(0xC0000484_u32 as _);
pub const STATUS_FIRMWARE_UPDATED: NTSTATUS = NTSTATUS(0x4000002C_u32 as _);
pub const STATUS_FLOATED_SECTION: NTSTATUS = NTSTATUS(0xC019004B_u32 as _);
pub const STATUS_FLOAT_DENORMAL_OPERAND: NTSTATUS = NTSTATUS(0xC000008D_u32 as _);
pub const STATUS_FLOAT_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(0xC000008E_u32 as _);
pub const STATUS_FLOAT_INEXACT_RESULT: NTSTATUS = NTSTATUS(0xC000008F_u32 as _);
pub const STATUS_FLOAT_INVALID_OPERATION: NTSTATUS = NTSTATUS(0xC0000090_u32 as _);
pub const STATUS_FLOAT_MULTIPLE_FAULTS: NTSTATUS = NTSTATUS(0xC00002B4_u32 as _);
pub const STATUS_FLOAT_MULTIPLE_TRAPS: NTSTATUS = NTSTATUS(0xC00002B5_u32 as _);
pub const STATUS_FLOAT_OVERFLOW: NTSTATUS = NTSTATUS(0xC0000091_u32 as _);
pub const STATUS_FLOAT_STACK_CHECK: NTSTATUS = NTSTATUS(0xC0000092_u32 as _);
pub const STATUS_FLOAT_UNDERFLOW: NTSTATUS = NTSTATUS(0xC0000093_u32 as _);
pub const STATUS_FLOPPY_BAD_REGISTERS: NTSTATUS = NTSTATUS(0xC0000168_u32 as _);
pub const STATUS_FLOPPY_ID_MARK_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000165_u32 as _);
pub const STATUS_FLOPPY_UNKNOWN_ERROR: NTSTATUS = NTSTATUS(0xC0000167_u32 as _);
pub const STATUS_FLOPPY_VOLUME: NTSTATUS = NTSTATUS(0xC0000164_u32 as _);
pub const STATUS_FLOPPY_WRONG_CYLINDER: NTSTATUS = NTSTATUS(0xC0000166_u32 as _);
pub const STATUS_FLT_ALREADY_ENLISTED: NTSTATUS = NTSTATUS(0xC01C001B_u32 as _);
pub const STATUS_FLT_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(0x801C0001_u32 as _);
pub const STATUS_FLT_CBDQ_DISABLED: NTSTATUS = NTSTATUS(0xC01C000E_u32 as _);
pub const STATUS_FLT_CONTEXT_ALLOCATION_NOT_FOUND: NTSTATUS = NTSTATUS(0xC01C0016_u32 as _);
pub const STATUS_FLT_CONTEXT_ALREADY_DEFINED: NTSTATUS = NTSTATUS(0xC01C0002_u32 as _);
pub const STATUS_FLT_CONTEXT_ALREADY_LINKED: NTSTATUS = NTSTATUS(0xC01C001C_u32 as _);
pub const STATUS_FLT_DELETING_OBJECT: NTSTATUS = NTSTATUS(0xC01C000B_u32 as _);
pub const STATUS_FLT_DISALLOW_FAST_IO: NTSTATUS = NTSTATUS(0xC01C0004_u32 as _);
pub const STATUS_FLT_DISALLOW_FSFILTER_IO: i32 = -1071906812i32;
pub const STATUS_FLT_DO_NOT_ATTACH: NTSTATUS = NTSTATUS(0xC01C000F_u32 as _);
pub const STATUS_FLT_DO_NOT_DETACH: NTSTATUS = NTSTATUS(0xC01C0010_u32 as _);
pub const STATUS_FLT_DUPLICATE_ENTRY: NTSTATUS = NTSTATUS(0xC01C000D_u32 as _);
pub const STATUS_FLT_FILTER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC01C0013_u32 as _);
pub const STATUS_FLT_FILTER_NOT_READY: NTSTATUS = NTSTATUS(0xC01C0008_u32 as _);
pub const STATUS_FLT_INSTANCE_ALTITUDE_COLLISION: NTSTATUS = NTSTATUS(0xC01C0011_u32 as _);
pub const STATUS_FLT_INSTANCE_NAME_COLLISION: NTSTATUS = NTSTATUS(0xC01C0012_u32 as _);
pub const STATUS_FLT_INSTANCE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC01C0015_u32 as _);
pub const STATUS_FLT_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC01C000A_u32 as _);
pub const STATUS_FLT_INVALID_ASYNCHRONOUS_REQUEST: NTSTATUS = NTSTATUS(0xC01C0003_u32 as _);
pub const STATUS_FLT_INVALID_CONTEXT_REGISTRATION: NTSTATUS = NTSTATUS(0xC01C0017_u32 as _);
pub const STATUS_FLT_INVALID_NAME_REQUEST: NTSTATUS = NTSTATUS(0xC01C0005_u32 as _);
pub const STATUS_FLT_IO_COMPLETE: NTSTATUS = NTSTATUS(0x1C0001_u32 as _);
pub const STATUS_FLT_MUST_BE_NONPAGED_POOL: NTSTATUS = NTSTATUS(0xC01C000C_u32 as _);
pub const STATUS_FLT_NAME_CACHE_MISS: NTSTATUS = NTSTATUS(0xC01C0018_u32 as _);
pub const STATUS_FLT_NOT_INITIALIZED: NTSTATUS = NTSTATUS(0xC01C0007_u32 as _);
pub const STATUS_FLT_NOT_SAFE_TO_POST_OPERATION: NTSTATUS = NTSTATUS(0xC01C0006_u32 as _);
pub const STATUS_FLT_NO_DEVICE_OBJECT: NTSTATUS = NTSTATUS(0xC01C0019_u32 as _);
pub const STATUS_FLT_NO_HANDLER_DEFINED: NTSTATUS = NTSTATUS(0xC01C0001_u32 as _);
pub const STATUS_FLT_NO_WAITER_FOR_REPLY: NTSTATUS = NTSTATUS(0xC01C0020_u32 as _);
pub const STATUS_FLT_POST_OPERATION_CLEANUP: NTSTATUS = NTSTATUS(0xC01C0009_u32 as _);
pub const STATUS_FLT_REGISTRATION_BUSY: NTSTATUS = NTSTATUS(0xC01C0023_u32 as _);
pub const STATUS_FLT_VOLUME_ALREADY_MOUNTED: NTSTATUS = NTSTATUS(0xC01C001A_u32 as _);
pub const STATUS_FLT_VOLUME_NOT_FOUND: NTSTATUS = NTSTATUS(0xC01C0014_u32 as _);
pub const STATUS_FLT_WCOS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01C0024_u32 as _);
pub const STATUS_FORMS_AUTH_REQUIRED: NTSTATUS = NTSTATUS(0xC0000905_u32 as _);
pub const STATUS_FOUND_OUT_OF_SCOPE: NTSTATUS = NTSTATUS(0xC000022E_u32 as _);
pub const STATUS_FREE_SPACE_TOO_FRAGMENTED: NTSTATUS = NTSTATUS(0xC000049B_u32 as _);
pub const STATUS_FREE_VM_NOT_AT_BASE: NTSTATUS = NTSTATUS(0xC000009F_u32 as _);
pub const STATUS_FSFILTER_OP_COMPLETED_SUCCESSFULLY: NTSTATUS = NTSTATUS(0x126_u32 as _);
pub const STATUS_FS_DRIVER_REQUIRED: NTSTATUS = NTSTATUS(0xC000019C_u32 as _);
pub const STATUS_FS_GUID_MISMATCH: NTSTATUS = NTSTATUS(0xC00004DE_u32 as _);
pub const STATUS_FS_METADATA_INCONSISTENT: NTSTATUS = NTSTATUS(0xC0000518_u32 as _);
pub const STATUS_FT_DI_SCAN_REQUIRED: NTSTATUS = NTSTATUS(0xC000046C_u32 as _);
pub const STATUS_FT_MISSING_MEMBER: NTSTATUS = NTSTATUS(0xC000015F_u32 as _);
pub const STATUS_FT_ORPHANING: NTSTATUS = NTSTATUS(0xC000016D_u32 as _);
pub const STATUS_FT_READ_FAILURE: NTSTATUS = NTSTATUS(0xC00004AB_u32 as _);
pub const STATUS_FT_READ_FROM_COPY: NTSTATUS = NTSTATUS(0x40000035_u32 as _);
pub const STATUS_FT_READ_FROM_COPY_FAILURE: NTSTATUS = NTSTATUS(0xC00004BF_u32 as _);
pub const STATUS_FT_READ_RECOVERY_FROM_BACKUP: NTSTATUS = NTSTATUS(0x4000000A_u32 as _);
pub const STATUS_FT_WRITE_FAILURE: NTSTATUS = NTSTATUS(0xC000046B_u32 as _);
pub const STATUS_FT_WRITE_RECOVERY: NTSTATUS = NTSTATUS(0x4000000B_u32 as _);
pub const STATUS_FULLSCREEN_MODE: NTSTATUS = NTSTATUS(0xC0000159_u32 as _);
pub const STATUS_FVE_ACTION_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0210009_u32 as _);
pub const STATUS_FVE_AUTH_INVALID_APPLICATION: NTSTATUS = NTSTATUS(0xC021001B_u32 as _);
pub const STATUS_FVE_AUTH_INVALID_CONFIG: NTSTATUS = NTSTATUS(0xC021001C_u32 as _);
pub const STATUS_FVE_BAD_DATA: NTSTATUS = NTSTATUS(0xC021000A_u32 as _);
pub const STATUS_FVE_BAD_INFORMATION: NTSTATUS = NTSTATUS(0xC0210002_u32 as _);
pub const STATUS_FVE_BAD_METADATA_POINTER: NTSTATUS = NTSTATUS(0xC021001F_u32 as _);
pub const STATUS_FVE_BAD_PARTITION_SIZE: NTSTATUS = NTSTATUS(0xC0210005_u32 as _);
pub const STATUS_FVE_CONV_READ_ERROR: NTSTATUS = NTSTATUS(0xC021000D_u32 as _);
pub const STATUS_FVE_CONV_RECOVERY_FAILED: NTSTATUS = NTSTATUS(0xC0210028_u32 as _);
pub const STATUS_FVE_CONV_WRITE_ERROR: NTSTATUS = NTSTATUS(0xC021000E_u32 as _);
pub const STATUS_FVE_DATASET_FULL: NTSTATUS = NTSTATUS(0xC0210043_u32 as _);
pub const STATUS_FVE_DEBUGGER_ENABLED: NTSTATUS = NTSTATUS(0xC021001D_u32 as _);
pub const STATUS_FVE_DEVICE_LOCKEDOUT: NTSTATUS = NTSTATUS(0xC021003B_u32 as _);
pub const STATUS_FVE_DRY_RUN_FAILED: NTSTATUS = NTSTATUS(0xC021001E_u32 as _);
pub const STATUS_FVE_EDRIVE_BAND_ENUMERATION_FAILED: NTSTATUS = NTSTATUS(0xC0210041_u32 as _);
pub const STATUS_FVE_EDRIVE_DRY_RUN_FAILED: NTSTATUS = NTSTATUS(0xC0210038_u32 as _);
pub const STATUS_FVE_ENH_PIN_INVALID: NTSTATUS = NTSTATUS(0xC0210031_u32 as _);
pub const STATUS_FVE_FAILED_AUTHENTICATION: NTSTATUS = NTSTATUS(0xC0210011_u32 as _);
pub const STATUS_FVE_FAILED_SECTOR_SIZE: NTSTATUS = NTSTATUS(0xC0210010_u32 as _);
pub const STATUS_FVE_FAILED_WRONG_FS: NTSTATUS = NTSTATUS(0xC0210004_u32 as _);
pub const STATUS_FVE_FS_MOUNTED: NTSTATUS = NTSTATUS(0xC0210007_u32 as _);
pub const STATUS_FVE_FS_NOT_EXTENDED: NTSTATUS = NTSTATUS(0xC0210006_u32 as _);
pub const STATUS_FVE_FULL_ENCRYPTION_NOT_ALLOWED_ON_TP_STORAGE: NTSTATUS = NTSTATUS(0xC0210032_u32 as _);
pub const STATUS_FVE_INVALID_DATUM_TYPE: NTSTATUS = NTSTATUS(0xC021002A_u32 as _);
pub const STATUS_FVE_KEYFILE_INVALID: NTSTATUS = NTSTATUS(0xC0210014_u32 as _);
pub const STATUS_FVE_KEYFILE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0210013_u32 as _);
pub const STATUS_FVE_KEYFILE_NO_VMK: NTSTATUS = NTSTATUS(0xC0210015_u32 as _);
pub const STATUS_FVE_LOCKED_VOLUME: NTSTATUS = NTSTATUS(0xC0210000_u32 as _);
pub const STATUS_FVE_METADATA_FULL: NTSTATUS = NTSTATUS(0xC0210044_u32 as _);
pub const STATUS_FVE_MOR_FAILED: NTSTATUS = NTSTATUS(0xC0210025_u32 as _);
pub const STATUS_FVE_NOT_ALLOWED_ON_CLUSTER: NTSTATUS = NTSTATUS(0xC0210035_u32 as _);
pub const STATUS_FVE_NOT_ALLOWED_ON_CSV_STACK: NTSTATUS = NTSTATUS(0xC0210034_u32 as _);
pub const STATUS_FVE_NOT_ALLOWED_TO_UPGRADE_WHILE_CONVERTING: NTSTATUS = NTSTATUS(0xC0210036_u32 as _);
pub const STATUS_FVE_NOT_DATA_VOLUME: NTSTATUS = NTSTATUS(0xC021000C_u32 as _);
pub const STATUS_FVE_NOT_DE_VOLUME: NTSTATUS = NTSTATUS(0xC021003D_u32 as _);
pub const STATUS_FVE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(0xC0210001_u32 as _);
pub const STATUS_FVE_NOT_OS_VOLUME: NTSTATUS = NTSTATUS(0xC0210012_u32 as _);
pub const STATUS_FVE_NO_AUTOUNLOCK_MASTER_KEY: NTSTATUS = NTSTATUS(0xC0210024_u32 as _);
pub const STATUS_FVE_NO_FEATURE_LICENSE: NTSTATUS = NTSTATUS(0xC0210026_u32 as _);
pub const STATUS_FVE_NO_LICENSE: NTSTATUS = NTSTATUS(0xC0210008_u32 as _);
pub const STATUS_FVE_OLD_METADATA_COPY: NTSTATUS = NTSTATUS(0xC0210020_u32 as _);
pub const STATUS_FVE_OSV_KSR_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0210040_u32 as _);
pub const STATUS_FVE_OVERLAPPED_UPDATE: NTSTATUS = NTSTATUS(0xC021000F_u32 as _);
pub const STATUS_FVE_PARTIAL_METADATA: NTSTATUS = NTSTATUS(0x80210001_u32 as _);
pub const STATUS_FVE_PIN_INVALID: NTSTATUS = NTSTATUS(0xC021001A_u32 as _);
pub const STATUS_FVE_POLICY_ON_RDV_EXCLUSION_LIST: NTSTATUS = NTSTATUS(0xC0210042_u32 as _);
pub const STATUS_FVE_POLICY_USER_DISABLE_RDV_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0210027_u32 as _);
pub const STATUS_FVE_PROTECTION_CANNOT_BE_DISABLED: NTSTATUS = NTSTATUS(0xC021003F_u32 as _);
pub const STATUS_FVE_PROTECTION_DISABLED: NTSTATUS = NTSTATUS(0xC021003E_u32 as _);
pub const STATUS_FVE_RAW_ACCESS: NTSTATUS = NTSTATUS(0xC0210022_u32 as _);
pub const STATUS_FVE_RAW_BLOCKED: NTSTATUS = NTSTATUS(0xC0210023_u32 as _);
pub const STATUS_FVE_REBOOT_REQUIRED: NTSTATUS = NTSTATUS(0xC0210021_u32 as _);
pub const STATUS_FVE_SECUREBOOT_CONFIG_CHANGE: NTSTATUS = NTSTATUS(0xC021003A_u32 as _);
pub const STATUS_FVE_SECUREBOOT_DISABLED: NTSTATUS = NTSTATUS(0xC0210039_u32 as _);
pub const STATUS_FVE_TOO_SMALL: NTSTATUS = NTSTATUS(0xC0210003_u32 as _);
pub const STATUS_FVE_TPM_DISABLED: NTSTATUS = NTSTATUS(0xC0210016_u32 as _);
pub const STATUS_FVE_TPM_INVALID_PCR: NTSTATUS = NTSTATUS(0xC0210018_u32 as _);
pub const STATUS_FVE_TPM_NO_VMK: NTSTATUS = NTSTATUS(0xC0210019_u32 as _);
pub const STATUS_FVE_TPM_SRK_AUTH_NOT_ZERO: NTSTATUS = NTSTATUS(0xC0210017_u32 as _);
pub const STATUS_FVE_TRANSIENT_STATE: NTSTATUS = NTSTATUS(0x80210002_u32 as _);
pub const STATUS_FVE_VIRTUALIZED_SPACE_TOO_BIG: NTSTATUS = NTSTATUS(0xC0210029_u32 as _);
pub const STATUS_FVE_VOLUME_EXTEND_PREVENTS_EOW_DECRYPT: NTSTATUS = NTSTATUS(0xC021003C_u32 as _);
pub const STATUS_FVE_VOLUME_NOT_BOUND: NTSTATUS = NTSTATUS(0xC021000B_u32 as _);
pub const STATUS_FVE_VOLUME_TOO_SMALL: NTSTATUS = NTSTATUS(0xC0210030_u32 as _);
pub const STATUS_FVE_WIPE_CANCEL_NOT_APPLICABLE: NTSTATUS = NTSTATUS(0xC0210037_u32 as _);
pub const STATUS_FVE_WIPE_NOT_ALLOWED_ON_TP_STORAGE: NTSTATUS = NTSTATUS(0xC0210033_u32 as _);
pub const STATUS_FWP_ACTION_INCOMPATIBLE_WITH_LAYER: NTSTATUS = NTSTATUS(0xC022002C_u32 as _);
pub const STATUS_FWP_ACTION_INCOMPATIBLE_WITH_SUBLAYER: NTSTATUS = NTSTATUS(0xC022002D_u32 as _);
pub const STATUS_FWP_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC0220009_u32 as _);
pub const STATUS_FWP_BUILTIN_OBJECT: NTSTATUS = NTSTATUS(0xC0220017_u32 as _);
pub const STATUS_FWP_CALLOUT_NOTIFICATION_FAILED: NTSTATUS = NTSTATUS(0xC0220037_u32 as _);
pub const STATUS_FWP_CALLOUT_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220001_u32 as _);
pub const STATUS_FWP_CANNOT_PEND: NTSTATUS = NTSTATUS(0xC0220103_u32 as _);
pub const STATUS_FWP_CONDITION_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220002_u32 as _);
pub const STATUS_FWP_CONNECTIONS_DISABLED: NTSTATUS = NTSTATUS(0xC0220041_u32 as _);
pub const STATUS_FWP_CONTEXT_INCOMPATIBLE_WITH_CALLOUT: NTSTATUS = NTSTATUS(0xC022002F_u32 as _);
pub const STATUS_FWP_CONTEXT_INCOMPATIBLE_WITH_LAYER: NTSTATUS = NTSTATUS(0xC022002E_u32 as _);
pub const STATUS_FWP_DROP_NOICMP: NTSTATUS = NTSTATUS(0xC0220104_u32 as _);
pub const STATUS_FWP_DUPLICATE_AUTH_METHOD: NTSTATUS = NTSTATUS(0xC022003C_u32 as _);
pub const STATUS_FWP_DUPLICATE_CONDITION: NTSTATUS = NTSTATUS(0xC022002A_u32 as _);
pub const STATUS_FWP_DUPLICATE_KEYMOD: NTSTATUS = NTSTATUS(0xC022002B_u32 as _);
pub const STATUS_FWP_DYNAMIC_SESSION_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC022000B_u32 as _);
pub const STATUS_FWP_EM_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0220032_u32 as _);
pub const STATUS_FWP_FILTER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220003_u32 as _);
pub const STATUS_FWP_IKEEXT_NOT_RUNNING: NTSTATUS = NTSTATUS(0xC0220044_u32 as _);
pub const STATUS_FWP_INCOMPATIBLE_AUTH_METHOD: NTSTATUS = NTSTATUS(0xC0220030_u32 as _);
pub const STATUS_FWP_INCOMPATIBLE_CIPHER_TRANSFORM: NTSTATUS = NTSTATUS(0xC022003A_u32 as _);
pub const STATUS_FWP_INCOMPATIBLE_DH_GROUP: NTSTATUS = NTSTATUS(0xC0220031_u32 as _);
pub const STATUS_FWP_INCOMPATIBLE_LAYER: NTSTATUS = NTSTATUS(0xC0220014_u32 as _);
pub const STATUS_FWP_INCOMPATIBLE_SA_STATE: NTSTATUS = NTSTATUS(0xC022001B_u32 as _);
pub const STATUS_FWP_INCOMPATIBLE_TXN: NTSTATUS = NTSTATUS(0xC0220011_u32 as _);
pub const STATUS_FWP_INJECT_HANDLE_CLOSING: NTSTATUS = NTSTATUS(0xC0220101_u32 as _);
pub const STATUS_FWP_INJECT_HANDLE_STALE: NTSTATUS = NTSTATUS(0xC0220102_u32 as _);
pub const STATUS_FWP_INVALID_ACTION_TYPE: NTSTATUS = NTSTATUS(0xC0220024_u32 as _);
pub const STATUS_FWP_INVALID_AUTH_TRANSFORM: NTSTATUS = NTSTATUS(0xC0220038_u32 as _);
pub const STATUS_FWP_INVALID_CIPHER_TRANSFORM: NTSTATUS = NTSTATUS(0xC0220039_u32 as _);
pub const STATUS_FWP_INVALID_DNS_NAME: NTSTATUS = NTSTATUS(0xC0220042_u32 as _);
pub const STATUS_FWP_INVALID_ENUMERATOR: NTSTATUS = NTSTATUS(0xC022001D_u32 as _);
pub const STATUS_FWP_INVALID_FLAGS: NTSTATUS = NTSTATUS(0xC022001E_u32 as _);
pub const STATUS_FWP_INVALID_INTERVAL: NTSTATUS = NTSTATUS(0xC0220021_u32 as _);
pub const STATUS_FWP_INVALID_NET_MASK: NTSTATUS = NTSTATUS(0xC022001F_u32 as _);
pub const STATUS_FWP_INVALID_PARAMETER: NTSTATUS = NTSTATUS(0xC0220035_u32 as _);
pub const STATUS_FWP_INVALID_RANGE: NTSTATUS = NTSTATUS(0xC0220020_u32 as _);
pub const STATUS_FWP_INVALID_TRANSFORM_COMBINATION: NTSTATUS = NTSTATUS(0xC022003B_u32 as _);
pub const STATUS_FWP_INVALID_TUNNEL_ENDPOINT: NTSTATUS = NTSTATUS(0xC022003D_u32 as _);
pub const STATUS_FWP_INVALID_WEIGHT: NTSTATUS = NTSTATUS(0xC0220025_u32 as _);
pub const STATUS_FWP_IN_USE: NTSTATUS = NTSTATUS(0xC022000A_u32 as _);
pub const STATUS_FWP_KEY_DICTATION_INVALID_KEYING_MATERIAL: NTSTATUS = NTSTATUS(0xC0220040_u32 as _);
pub const STATUS_FWP_KEY_DICTATOR_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(0xC022003F_u32 as _);
pub const STATUS_FWP_KM_CLIENTS_ONLY: NTSTATUS = NTSTATUS(0xC0220015_u32 as _);
pub const STATUS_FWP_L2_DRIVER_NOT_READY: NTSTATUS = NTSTATUS(0xC022003E_u32 as _);
pub const STATUS_FWP_LAYER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220004_u32 as _);
pub const STATUS_FWP_LIFETIME_MISMATCH: NTSTATUS = NTSTATUS(0xC0220016_u32 as _);
pub const STATUS_FWP_MATCH_TYPE_MISMATCH: NTSTATUS = NTSTATUS(0xC0220026_u32 as _);
pub const STATUS_FWP_NET_EVENTS_DISABLED: NTSTATUS = NTSTATUS(0xC0220013_u32 as _);
pub const STATUS_FWP_NEVER_MATCH: NTSTATUS = NTSTATUS(0xC0220033_u32 as _);
pub const STATUS_FWP_NOTIFICATION_DROPPED: NTSTATUS = NTSTATUS(0xC0220019_u32 as _);
pub const STATUS_FWP_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220008_u32 as _);
pub const STATUS_FWP_NO_TXN_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC022000D_u32 as _);
pub const STATUS_FWP_NULL_DISPLAY_NAME: NTSTATUS = NTSTATUS(0xC0220023_u32 as _);
pub const STATUS_FWP_NULL_POINTER: NTSTATUS = NTSTATUS(0xC022001C_u32 as _);
pub const STATUS_FWP_OUT_OF_BOUNDS: NTSTATUS = NTSTATUS(0xC0220028_u32 as _);
pub const STATUS_FWP_PROVIDER_CONTEXT_MISMATCH: NTSTATUS = NTSTATUS(0xC0220034_u32 as _);
pub const STATUS_FWP_PROVIDER_CONTEXT_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220006_u32 as _);
pub const STATUS_FWP_PROVIDER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220005_u32 as _);
pub const STATUS_FWP_RESERVED: NTSTATUS = NTSTATUS(0xC0220029_u32 as _);
pub const STATUS_FWP_SESSION_ABORTED: NTSTATUS = NTSTATUS(0xC0220010_u32 as _);
pub const STATUS_FWP_STILL_ON: NTSTATUS = NTSTATUS(0xC0220043_u32 as _);
pub const STATUS_FWP_SUBLAYER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0220007_u32 as _);
pub const STATUS_FWP_TCPIP_NOT_READY: NTSTATUS = NTSTATUS(0xC0220100_u32 as _);
pub const STATUS_FWP_TIMEOUT: NTSTATUS = NTSTATUS(0xC0220012_u32 as _);
pub const STATUS_FWP_TOO_MANY_CALLOUTS: NTSTATUS = NTSTATUS(0xC0220018_u32 as _);
pub const STATUS_FWP_TOO_MANY_SUBLAYERS: NTSTATUS = NTSTATUS(0xC0220036_u32 as _);
pub const STATUS_FWP_TRAFFIC_MISMATCH: NTSTATUS = NTSTATUS(0xC022001A_u32 as _);
pub const STATUS_FWP_TXN_ABORTED: NTSTATUS = NTSTATUS(0xC022000F_u32 as _);
pub const STATUS_FWP_TXN_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC022000E_u32 as _);
pub const STATUS_FWP_TYPE_MISMATCH: NTSTATUS = NTSTATUS(0xC0220027_u32 as _);
pub const STATUS_FWP_WRONG_SESSION: NTSTATUS = NTSTATUS(0xC022000C_u32 as _);
pub const STATUS_FWP_ZERO_LENGTH_ARRAY: NTSTATUS = NTSTATUS(0xC0220022_u32 as _);
pub const STATUS_GDI_HANDLE_LEAK: NTSTATUS = NTSTATUS(0x803F0001_u32 as _);
pub const STATUS_GENERIC_COMMAND_FAILED: NTSTATUS = NTSTATUS(0xC0150026_u32 as _);
pub const STATUS_GENERIC_NOT_MAPPED: NTSTATUS = NTSTATUS(0xC00000E6_u32 as _);
pub const STATUS_GHOSTED: NTSTATUS = NTSTATUS(0x12F_u32 as _);
pub const STATUS_GPIO_CLIENT_INFORMATION_INVALID: NTSTATUS = NTSTATUS(0xC000A122_u32 as _);
pub const STATUS_GPIO_INCOMPATIBLE_CONNECT_MODE: NTSTATUS = NTSTATUS(0xC000A126_u32 as _);
pub const STATUS_GPIO_INTERRUPT_ALREADY_UNMASKED: NTSTATUS = NTSTATUS(0x8000A127_u32 as _);
pub const STATUS_GPIO_INVALID_REGISTRATION_PACKET: NTSTATUS = NTSTATUS(0xC000A124_u32 as _);
pub const STATUS_GPIO_OPERATION_DENIED: NTSTATUS = NTSTATUS(0xC000A125_u32 as _);
pub const STATUS_GPIO_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A123_u32 as _);
pub const STATUS_GRACEFUL_DISCONNECT: NTSTATUS = NTSTATUS(0xC0000237_u32 as _);
pub const STATUS_GRAPHICS_ADAPTER_ACCESS_NOT_EXCLUDED: NTSTATUS = NTSTATUS(0xC01E043B_u32 as _);
pub const STATUS_GRAPHICS_ADAPTER_CHAIN_NOT_READY: NTSTATUS = NTSTATUS(0xC01E0433_u32 as _);
pub const STATUS_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_SOURCE: NTSTATUS = NTSTATUS(0xC01E0328_u32 as _);
pub const STATUS_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_TARGET: NTSTATUS = NTSTATUS(0xC01E0329_u32 as _);
pub const STATUS_GRAPHICS_ADAPTER_WAS_RESET: NTSTATUS = NTSTATUS(0xC01E0003_u32 as _);
pub const STATUS_GRAPHICS_ALLOCATION_BUSY: NTSTATUS = NTSTATUS(0xC01E0102_u32 as _);
pub const STATUS_GRAPHICS_ALLOCATION_CLOSED: NTSTATUS = NTSTATUS(0xC01E0112_u32 as _);
pub const STATUS_GRAPHICS_ALLOCATION_CONTENT_LOST: NTSTATUS = NTSTATUS(0xC01E0116_u32 as _);
pub const STATUS_GRAPHICS_ALLOCATION_INVALID: NTSTATUS = NTSTATUS(0xC01E0106_u32 as _);
pub const STATUS_GRAPHICS_CANCEL_VIDPN_TOPOLOGY_AUGMENTATION: NTSTATUS = NTSTATUS(0xC01E035A_u32 as _);
pub const STATUS_GRAPHICS_CANNOTCOLORCONVERT: NTSTATUS = NTSTATUS(0xC01E0008_u32 as _);
pub const STATUS_GRAPHICS_CANT_ACCESS_ACTIVE_VIDPN: NTSTATUS = NTSTATUS(0xC01E0343_u32 as _);
pub const STATUS_GRAPHICS_CANT_EVICT_PINNED_ALLOCATION: NTSTATUS = NTSTATUS(0xC01E0109_u32 as _);
pub const STATUS_GRAPHICS_CANT_LOCK_MEMORY: NTSTATUS = NTSTATUS(0xC01E0101_u32 as _);
pub const STATUS_GRAPHICS_CANT_RENDER_LOCKED_ALLOCATION: NTSTATUS = NTSTATUS(0xC01E0111_u32 as _);
pub const STATUS_GRAPHICS_CHAINLINKS_NOT_ENUMERATED: NTSTATUS = NTSTATUS(0xC01E0432_u32 as _);
pub const STATUS_GRAPHICS_CHAINLINKS_NOT_POWERED_ON: NTSTATUS = NTSTATUS(0xC01E0435_u32 as _);
pub const STATUS_GRAPHICS_CHAINLINKS_NOT_STARTED: NTSTATUS = NTSTATUS(0xC01E0434_u32 as _);
pub const STATUS_GRAPHICS_CHILD_DESCRIPTOR_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0401_u32 as _);
pub const STATUS_GRAPHICS_CLIENTVIDPN_NOT_SET: NTSTATUS = NTSTATUS(0xC01E035C_u32 as _);
pub const STATUS_GRAPHICS_COPP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0501_u32 as _);
pub const STATUS_GRAPHICS_DATASET_IS_EMPTY: NTSTATUS = NTSTATUS(0x401E034B_u32 as _);
pub const STATUS_GRAPHICS_DDCCI_INVALID_CAPABILITIES_STRING: NTSTATUS = NTSTATUS(0xC01E0587_u32 as _);
pub const STATUS_GRAPHICS_DDCCI_INVALID_DATA: NTSTATUS = NTSTATUS(0xC01E0585_u32 as _);
pub const STATUS_GRAPHICS_DDCCI_INVALID_MESSAGE_CHECKSUM: NTSTATUS = NTSTATUS(0xC01E058B_u32 as _);
pub const STATUS_GRAPHICS_DDCCI_INVALID_MESSAGE_COMMAND: NTSTATUS = NTSTATUS(0xC01E0589_u32 as _);
pub const STATUS_GRAPHICS_DDCCI_INVALID_MESSAGE_LENGTH: NTSTATUS = NTSTATUS(0xC01E058A_u32 as _);
pub const STATUS_GRAPHICS_DDCCI_MONITOR_RETURNED_INVALID_TIMING_STATUS_BYTE: NTSTATUS = NTSTATUS(0xC01E0586_u32 as _);
pub const STATUS_GRAPHICS_DDCCI_VCP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0584_u32 as _);
pub const STATUS_GRAPHICS_DEPENDABLE_CHILD_STATUS: NTSTATUS = NTSTATUS(0x401E043C_u32 as _);
pub const STATUS_GRAPHICS_DISPLAY_DEVICE_NOT_ATTACHED_TO_DESKTOP: NTSTATUS = NTSTATUS(0xC01E05E2_u32 as _);
pub const STATUS_GRAPHICS_DRIVER_MISMATCH: NTSTATUS = NTSTATUS(0xC01E0009_u32 as _);
pub const STATUS_GRAPHICS_EMPTY_ADAPTER_MONITOR_MODE_SUPPORT_INTERSECTION: NTSTATUS = NTSTATUS(0xC01E0325_u32 as _);
pub const STATUS_GRAPHICS_FREQUENCYRANGE_ALREADY_IN_SET: NTSTATUS = NTSTATUS(0xC01E031F_u32 as _);
pub const STATUS_GRAPHICS_FREQUENCYRANGE_NOT_IN_SET: NTSTATUS = NTSTATUS(0xC01E031D_u32 as _);
pub const STATUS_GRAPHICS_GAMMA_RAMP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0348_u32 as _);
pub const STATUS_GRAPHICS_GPU_EXCEPTION_ON_DEVICE: NTSTATUS = NTSTATUS(0xC01E0200_u32 as _);
pub const STATUS_GRAPHICS_I2C_DEVICE_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(0xC01E0581_u32 as _);
pub const STATUS_GRAPHICS_I2C_ERROR_RECEIVING_DATA: NTSTATUS = NTSTATUS(0xC01E0583_u32 as _);
pub const STATUS_GRAPHICS_I2C_ERROR_TRANSMITTING_DATA: NTSTATUS = NTSTATUS(0xC01E0582_u32 as _);
pub const STATUS_GRAPHICS_I2C_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0580_u32 as _);
pub const STATUS_GRAPHICS_INCOMPATIBLE_PRIVATE_FORMAT: NTSTATUS = NTSTATUS(0xC01E0355_u32 as _);
pub const STATUS_GRAPHICS_INCONSISTENT_DEVICE_LINK_STATE: NTSTATUS = NTSTATUS(0xC01E0436_u32 as _);
pub const STATUS_GRAPHICS_INDIRECT_DISPLAY_ABANDON_SWAPCHAIN: NTSTATUS = NTSTATUS(0xC01E0012_u32 as _);
pub const STATUS_GRAPHICS_INDIRECT_DISPLAY_DEVICE_STOPPED: NTSTATUS = NTSTATUS(0xC01E0013_u32 as _);
pub const STATUS_GRAPHICS_INSUFFICIENT_DMA_BUFFER: NTSTATUS = NTSTATUS(0xC01E0001_u32 as _);
pub const STATUS_GRAPHICS_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC01E05E7_u32 as _);
pub const STATUS_GRAPHICS_INVALID_ACTIVE_REGION: NTSTATUS = NTSTATUS(0xC01E030B_u32 as _);
pub const STATUS_GRAPHICS_INVALID_ALLOCATION_HANDLE: NTSTATUS = NTSTATUS(0xC01E0114_u32 as _);
pub const STATUS_GRAPHICS_INVALID_ALLOCATION_INSTANCE: NTSTATUS = NTSTATUS(0xC01E0113_u32 as _);
pub const STATUS_GRAPHICS_INVALID_ALLOCATION_USAGE: NTSTATUS = NTSTATUS(0xC01E0110_u32 as _);
pub const STATUS_GRAPHICS_INVALID_CLIENT_TYPE: NTSTATUS = NTSTATUS(0xC01E035B_u32 as _);
pub const STATUS_GRAPHICS_INVALID_COLORBASIS: NTSTATUS = NTSTATUS(0xC01E033E_u32 as _);
pub const STATUS_GRAPHICS_INVALID_COPYPROTECTION_TYPE: NTSTATUS = NTSTATUS(0xC01E034F_u32 as _);
pub const STATUS_GRAPHICS_INVALID_DISPLAY_ADAPTER: NTSTATUS = NTSTATUS(0xC01E0002_u32 as _);
pub const STATUS_GRAPHICS_INVALID_DRIVER_MODEL: NTSTATUS = NTSTATUS(0xC01E0004_u32 as _);
pub const STATUS_GRAPHICS_INVALID_FREQUENCY: NTSTATUS = NTSTATUS(0xC01E030A_u32 as _);
pub const STATUS_GRAPHICS_INVALID_GAMMA_RAMP: NTSTATUS = NTSTATUS(0xC01E0347_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MODE_PRUNING_ALGORITHM: NTSTATUS = NTSTATUS(0xC01E0356_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITORDESCRIPTOR: NTSTATUS = NTSTATUS(0xC01E032B_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITORDESCRIPTORSET: NTSTATUS = NTSTATUS(0xC01E032A_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITOR_CAPABILITY_ORIGIN: NTSTATUS = NTSTATUS(0xC01E0357_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE: NTSTATUS = NTSTATUS(0xC01E031C_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGESET: NTSTATUS = NTSTATUS(0xC01E031B_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE_CONSTRAINT: NTSTATUS = NTSTATUS(0xC01E0358_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITOR_SOURCEMODESET: NTSTATUS = NTSTATUS(0xC01E0321_u32 as _);
pub const STATUS_GRAPHICS_INVALID_MONITOR_SOURCE_MODE: NTSTATUS = NTSTATUS(0xC01E0322_u32 as _);
pub const STATUS_GRAPHICS_INVALID_PATH_CONTENT_GEOMETRY_TRANSFORMATION: NTSTATUS = NTSTATUS(0xC01E0345_u32 as _);
pub const STATUS_GRAPHICS_INVALID_PATH_CONTENT_TYPE: NTSTATUS = NTSTATUS(0xC01E034E_u32 as _);
pub const STATUS_GRAPHICS_INVALID_PATH_IMPORTANCE_ORDINAL: NTSTATUS = NTSTATUS(0xC01E0344_u32 as _);
pub const STATUS_GRAPHICS_INVALID_PHYSICAL_MONITOR_HANDLE: NTSTATUS = NTSTATUS(0xC01E058C_u32 as _);
pub const STATUS_GRAPHICS_INVALID_PIXELFORMAT: NTSTATUS = NTSTATUS(0xC01E033D_u32 as _);
pub const STATUS_GRAPHICS_INVALID_PIXELVALUEACCESSMODE: NTSTATUS = NTSTATUS(0xC01E033F_u32 as _);
pub const STATUS_GRAPHICS_INVALID_POINTER: NTSTATUS = NTSTATUS(0xC01E05E4_u32 as _);
pub const STATUS_GRAPHICS_INVALID_PRIMARYSURFACE_SIZE: NTSTATUS = NTSTATUS(0xC01E033A_u32 as _);
pub const STATUS_GRAPHICS_INVALID_SCANLINE_ORDERING: NTSTATUS = NTSTATUS(0xC01E0352_u32 as _);
pub const STATUS_GRAPHICS_INVALID_STRIDE: NTSTATUS = NTSTATUS(0xC01E033C_u32 as _);
pub const STATUS_GRAPHICS_INVALID_TOTAL_REGION: NTSTATUS = NTSTATUS(0xC01E030C_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDEOPRESENTSOURCESET: NTSTATUS = NTSTATUS(0xC01E0315_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDEOPRESENTTARGETSET: NTSTATUS = NTSTATUS(0xC01E0316_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE: NTSTATUS = NTSTATUS(0xC01E0304_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE_MODE: NTSTATUS = NTSTATUS(0xC01E0310_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET: NTSTATUS = NTSTATUS(0xC01E0305_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET_MODE: NTSTATUS = NTSTATUS(0xC01E0311_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDPN: NTSTATUS = NTSTATUS(0xC01E0303_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDPN_PRESENT_PATH: NTSTATUS = NTSTATUS(0xC01E0319_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDPN_SOURCEMODESET: NTSTATUS = NTSTATUS(0xC01E0308_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TARGETMODESET: NTSTATUS = NTSTATUS(0xC01E0309_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TARGET_SUBSET_TYPE: NTSTATUS = NTSTATUS(0xC01E032F_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TOPOLOGY: NTSTATUS = NTSTATUS(0xC01E0300_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TOPOLOGY_RECOMMENDATION_REASON: NTSTATUS = NTSTATUS(0xC01E034D_u32 as _);
pub const STATUS_GRAPHICS_INVALID_VISIBLEREGION_SIZE: NTSTATUS = NTSTATUS(0xC01E033B_u32 as _);
pub const STATUS_GRAPHICS_LEADLINK_NOT_ENUMERATED: NTSTATUS = NTSTATUS(0xC01E0431_u32 as _);
pub const STATUS_GRAPHICS_LEADLINK_START_DEFERRED: NTSTATUS = NTSTATUS(0x401E0437_u32 as _);
pub const STATUS_GRAPHICS_LINK_CONFIGURATION_IN_PROGRESS: NTSTATUS = NTSTATUS(0x801E0000_u32 as _);
pub const STATUS_GRAPHICS_MAX_NUM_PATHS_REACHED: NTSTATUS = NTSTATUS(0xC01E0359_u32 as _);
pub const STATUS_GRAPHICS_MCA_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC01E0588_u32 as _);
pub const STATUS_GRAPHICS_MIRRORING_DEVICES_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E05E3_u32 as _);
pub const STATUS_GRAPHICS_MODE_ALREADY_IN_MODESET: NTSTATUS = NTSTATUS(0xC01E0314_u32 as _);
pub const STATUS_GRAPHICS_MODE_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(0xC01E0324_u32 as _);
pub const STATUS_GRAPHICS_MODE_NOT_IN_MODESET: NTSTATUS = NTSTATUS(0xC01E034A_u32 as _);
pub const STATUS_GRAPHICS_MODE_NOT_PINNED: NTSTATUS = NTSTATUS(0x401E0307_u32 as _);
pub const STATUS_GRAPHICS_MONITORDESCRIPTOR_ALREADY_IN_SET: NTSTATUS = NTSTATUS(0xC01E032D_u32 as _);
pub const STATUS_GRAPHICS_MONITORDESCRIPTOR_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(0xC01E032E_u32 as _);
pub const STATUS_GRAPHICS_MONITORDESCRIPTOR_NOT_IN_SET: NTSTATUS = NTSTATUS(0xC01E032C_u32 as _);
pub const STATUS_GRAPHICS_MONITOR_COULD_NOT_BE_ASSOCIATED_WITH_ADAPTER: NTSTATUS = NTSTATUS(0xC01E0334_u32 as _);
pub const STATUS_GRAPHICS_MONITOR_NOT_CONNECTED: NTSTATUS = NTSTATUS(0xC01E0338_u32 as _);
pub const STATUS_GRAPHICS_MONITOR_NO_LONGER_EXISTS: NTSTATUS = NTSTATUS(0xC01E058D_u32 as _);
pub const STATUS_GRAPHICS_MPO_ALLOCATION_UNPINNED: NTSTATUS = NTSTATUS(0xC01E0018_u32 as _);
pub const STATUS_GRAPHICS_MULTISAMPLING_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0349_u32 as _);
pub const STATUS_GRAPHICS_NOT_A_LINKED_ADAPTER: NTSTATUS = NTSTATUS(0xC01E0430_u32 as _);
pub const STATUS_GRAPHICS_NOT_EXCLUSIVE_MODE_OWNER: NTSTATUS = NTSTATUS(0xC01E0000_u32 as _);
pub const STATUS_GRAPHICS_NOT_POST_DEVICE_DRIVER: NTSTATUS = NTSTATUS(0xC01E0438_u32 as _);
pub const STATUS_GRAPHICS_NO_ACTIVE_VIDPN: NTSTATUS = NTSTATUS(0xC01E0336_u32 as _);
pub const STATUS_GRAPHICS_NO_AVAILABLE_IMPORTANCE_ORDINALS: NTSTATUS = NTSTATUS(0xC01E0354_u32 as _);
pub const STATUS_GRAPHICS_NO_AVAILABLE_VIDPN_TARGET: NTSTATUS = NTSTATUS(0xC01E0333_u32 as _);
pub const STATUS_GRAPHICS_NO_DISPLAY_DEVICE_CORRESPONDS_TO_NAME: NTSTATUS = NTSTATUS(0xC01E05E1_u32 as _);
pub const STATUS_GRAPHICS_NO_DISPLAY_MODE_MANAGEMENT_SUPPORT: NTSTATUS = NTSTATUS(0xC01E0341_u32 as _);
pub const STATUS_GRAPHICS_NO_MONITORS_CORRESPOND_TO_DISPLAY_DEVICE: NTSTATUS = NTSTATUS(0xC01E05E5_u32 as _);
pub const STATUS_GRAPHICS_NO_MORE_ELEMENTS_IN_DATASET: NTSTATUS = NTSTATUS(0x401E034C_u32 as _);
pub const STATUS_GRAPHICS_NO_PREFERRED_MODE: NTSTATUS = NTSTATUS(0x401E031E_u32 as _);
pub const STATUS_GRAPHICS_NO_RECOMMENDED_FUNCTIONAL_VIDPN: NTSTATUS = NTSTATUS(0xC01E0323_u32 as _);
pub const STATUS_GRAPHICS_NO_RECOMMENDED_VIDPN_TOPOLOGY: NTSTATUS = NTSTATUS(0xC01E031A_u32 as _);
pub const STATUS_GRAPHICS_NO_VIDEO_MEMORY: NTSTATUS = NTSTATUS(0xC01E0100_u32 as _);
pub const STATUS_GRAPHICS_NO_VIDPNMGR: NTSTATUS = NTSTATUS(0xC01E0335_u32 as _);
pub const STATUS_GRAPHICS_ONLY_CONSOLE_SESSION_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E05E0_u32 as _);
pub const STATUS_GRAPHICS_OPM_ALL_HDCP_HARDWARE_ALREADY_IN_USE: NTSTATUS = NTSTATUS(0xC01E0518_u32 as _);
pub const STATUS_GRAPHICS_OPM_DRIVER_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC01E051E_u32 as _);
pub const STATUS_GRAPHICS_OPM_HDCP_SRM_NEVER_SET: NTSTATUS = NTSTATUS(0xC01E0516_u32 as _);
pub const STATUS_GRAPHICS_OPM_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC01E050B_u32 as _);
pub const STATUS_GRAPHICS_OPM_INVALID_CONFIGURATION_REQUEST: NTSTATUS = NTSTATUS(0xC01E0521_u32 as _);
pub const STATUS_GRAPHICS_OPM_INVALID_ENCRYPTED_PARAMETERS: NTSTATUS = NTSTATUS(0xC01E0503_u32 as _);
pub const STATUS_GRAPHICS_OPM_INVALID_HANDLE: NTSTATUS = NTSTATUS(0xC01E050C_u32 as _);
pub const STATUS_GRAPHICS_OPM_INVALID_INFORMATION_REQUEST: NTSTATUS = NTSTATUS(0xC01E051D_u32 as _);
pub const STATUS_GRAPHICS_OPM_INVALID_SRM: NTSTATUS = NTSTATUS(0xC01E0512_u32 as _);
pub const STATUS_GRAPHICS_OPM_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0500_u32 as _);
pub const STATUS_GRAPHICS_OPM_NO_PROTECTED_OUTPUTS_EXIST: NTSTATUS = NTSTATUS(0xC01E0505_u32 as _);
pub const STATUS_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_ACP: NTSTATUS = NTSTATUS(0xC01E0514_u32 as _);
pub const STATUS_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_CGMSA: NTSTATUS = NTSTATUS(0xC01E0515_u32 as _);
pub const STATUS_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_HDCP: NTSTATUS = NTSTATUS(0xC01E0513_u32 as _);
pub const STATUS_GRAPHICS_OPM_PROTECTED_OUTPUT_DOES_NOT_HAVE_COPP_SEMANTICS: NTSTATUS = NTSTATUS(0xC01E051C_u32 as _);
pub const STATUS_GRAPHICS_OPM_PROTECTED_OUTPUT_DOES_NOT_HAVE_OPM_SEMANTICS: NTSTATUS = NTSTATUS(0xC01E051F_u32 as _);
pub const STATUS_GRAPHICS_OPM_PROTECTED_OUTPUT_NO_LONGER_EXISTS: NTSTATUS = NTSTATUS(0xC01E051A_u32 as _);
pub const STATUS_GRAPHICS_OPM_RESOLUTION_TOO_HIGH: NTSTATUS = NTSTATUS(0xC01E0517_u32 as _);
pub const STATUS_GRAPHICS_OPM_SIGNALING_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0520_u32 as _);
pub const STATUS_GRAPHICS_OPM_SPANNING_MODE_ENABLED: NTSTATUS = NTSTATUS(0xC01E050F_u32 as _);
pub const STATUS_GRAPHICS_OPM_THEATER_MODE_ENABLED: NTSTATUS = NTSTATUS(0xC01E0510_u32 as _);
pub const STATUS_GRAPHICS_PARAMETER_ARRAY_TOO_SMALL: NTSTATUS = NTSTATUS(0xC01E05E6_u32 as _);
pub const STATUS_GRAPHICS_PARTIAL_DATA_POPULATED: NTSTATUS = NTSTATUS(0x401E000A_u32 as _);
pub const STATUS_GRAPHICS_PATH_ALREADY_IN_TOPOLOGY: NTSTATUS = NTSTATUS(0xC01E0313_u32 as _);
pub const STATUS_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_PINNED: NTSTATUS = NTSTATUS(0x401E0351_u32 as _);
pub const STATUS_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0346_u32 as _);
pub const STATUS_GRAPHICS_PATH_NOT_IN_TOPOLOGY: NTSTATUS = NTSTATUS(0xC01E0327_u32 as _);
pub const STATUS_GRAPHICS_PINNED_MODE_MUST_REMAIN_IN_SET: NTSTATUS = NTSTATUS(0xC01E0312_u32 as _);
pub const STATUS_GRAPHICS_POLLING_TOO_FREQUENTLY: NTSTATUS = NTSTATUS(0x401E0439_u32 as _);
pub const STATUS_GRAPHICS_PRESENT_BUFFER_NOT_BOUND: NTSTATUS = NTSTATUS(0xC01E0010_u32 as _);
pub const STATUS_GRAPHICS_PRESENT_DENIED: NTSTATUS = NTSTATUS(0xC01E0007_u32 as _);
pub const STATUS_GRAPHICS_PRESENT_INVALID_WINDOW: NTSTATUS = NTSTATUS(0xC01E000F_u32 as _);
pub const STATUS_GRAPHICS_PRESENT_MODE_CHANGED: NTSTATUS = NTSTATUS(0xC01E0005_u32 as _);
pub const STATUS_GRAPHICS_PRESENT_OCCLUDED: NTSTATUS = NTSTATUS(0xC01E0006_u32 as _);
pub const STATUS_GRAPHICS_PRESENT_REDIRECTION_DISABLED: NTSTATUS = NTSTATUS(0xC01E000B_u32 as _);
pub const STATUS_GRAPHICS_PRESENT_UNOCCLUDED: NTSTATUS = NTSTATUS(0xC01E000C_u32 as _);
pub const STATUS_GRAPHICS_PVP_HFS_FAILED: NTSTATUS = NTSTATUS(0xC01E0511_u32 as _);
pub const STATUS_GRAPHICS_PVP_INVALID_CERTIFICATE_LENGTH: NTSTATUS = NTSTATUS(0xC01E050E_u32 as _);
pub const STATUS_GRAPHICS_RESOURCES_NOT_RELATED: NTSTATUS = NTSTATUS(0xC01E0330_u32 as _);
pub const STATUS_GRAPHICS_SESSION_TYPE_CHANGE_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC01E05E8_u32 as _);
pub const STATUS_GRAPHICS_SKIP_ALLOCATION_PREPARATION: NTSTATUS = NTSTATUS(0x401E0201_u32 as _);
pub const STATUS_GRAPHICS_SOURCE_ALREADY_IN_SET: NTSTATUS = NTSTATUS(0xC01E0317_u32 as _);
pub const STATUS_GRAPHICS_SOURCE_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(0xC01E0331_u32 as _);
pub const STATUS_GRAPHICS_SOURCE_NOT_IN_TOPOLOGY: NTSTATUS = NTSTATUS(0xC01E0339_u32 as _);
pub const STATUS_GRAPHICS_SPECIFIED_CHILD_ALREADY_CONNECTED: NTSTATUS = NTSTATUS(0xC01E0400_u32 as _);
pub const STATUS_GRAPHICS_STALE_MODESET: NTSTATUS = NTSTATUS(0xC01E0320_u32 as _);
pub const STATUS_GRAPHICS_STALE_VIDPN_TOPOLOGY: NTSTATUS = NTSTATUS(0xC01E0337_u32 as _);
pub const STATUS_GRAPHICS_START_DEFERRED: NTSTATUS = NTSTATUS(0x401E043A_u32 as _);
pub const STATUS_GRAPHICS_TARGET_ALREADY_IN_SET: NTSTATUS = NTSTATUS(0xC01E0318_u32 as _);
pub const STATUS_GRAPHICS_TARGET_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(0xC01E0332_u32 as _);
pub const STATUS_GRAPHICS_TARGET_NOT_IN_TOPOLOGY: NTSTATUS = NTSTATUS(0xC01E0340_u32 as _);
pub const STATUS_GRAPHICS_TOO_MANY_REFERENCES: NTSTATUS = NTSTATUS(0xC01E0103_u32 as _);
pub const STATUS_GRAPHICS_TOPOLOGY_CHANGES_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC01E0353_u32 as _);
pub const STATUS_GRAPHICS_TRY_AGAIN_LATER: NTSTATUS = NTSTATUS(0xC01E0104_u32 as _);
pub const STATUS_GRAPHICS_TRY_AGAIN_NOW: NTSTATUS = NTSTATUS(0xC01E0105_u32 as _);
pub const STATUS_GRAPHICS_UAB_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0502_u32 as _);
pub const STATUS_GRAPHICS_UNASSIGNED_MODESET_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC01E0350_u32 as _);
pub const STATUS_GRAPHICS_UNKNOWN_CHILD_STATUS: NTSTATUS = NTSTATUS(0x401E042F_u32 as _);
pub const STATUS_GRAPHICS_UNSWIZZLING_APERTURE_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC01E0107_u32 as _);
pub const STATUS_GRAPHICS_UNSWIZZLING_APERTURE_UNSUPPORTED: NTSTATUS = NTSTATUS(0xC01E0108_u32 as _);
pub const STATUS_GRAPHICS_VAIL_STATE_CHANGED: NTSTATUS = NTSTATUS(0xC01E0011_u32 as _);
pub const STATUS_GRAPHICS_VIDEO_PRESENT_TARGETS_LESS_THAN_SOURCES: NTSTATUS = NTSTATUS(0xC01E0326_u32 as _);
pub const STATUS_GRAPHICS_VIDPN_MODALITY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0306_u32 as _);
pub const STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE: NTSTATUS = NTSTATUS(0xC01E0342_u32 as _);
pub const STATUS_GRAPHICS_VIDPN_TOPOLOGY_CURRENTLY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0302_u32 as _);
pub const STATUS_GRAPHICS_VIDPN_TOPOLOGY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC01E0301_u32 as _);
pub const STATUS_GRAPHICS_WINDOWDC_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC01E000D_u32 as _);
pub const STATUS_GRAPHICS_WINDOWLESS_PRESENT_DISABLED: NTSTATUS = NTSTATUS(0xC01E000E_u32 as _);
pub const STATUS_GRAPHICS_WRONG_ALLOCATION_DEVICE: NTSTATUS = NTSTATUS(0xC01E0115_u32 as _);
pub const STATUS_GROUP_EXISTS: NTSTATUS = NTSTATUS(0xC0000065_u32 as _);
pub const STATUS_GUARD_PAGE_VIOLATION: NTSTATUS = NTSTATUS(0x80000001_u32 as _);
pub const STATUS_GUIDS_EXHAUSTED: NTSTATUS = NTSTATUS(0xC0000083_u32 as _);
pub const STATUS_GUID_SUBSTITUTION_MADE: NTSTATUS = NTSTATUS(0x8000000C_u32 as _);
pub const STATUS_HANDLES_CLOSED: NTSTATUS = NTSTATUS(0x8000000A_u32 as _);
pub const STATUS_HANDLE_NOT_CLOSABLE: NTSTATUS = NTSTATUS(0xC0000235_u32 as _);
pub const STATUS_HANDLE_NO_LONGER_VALID: NTSTATUS = NTSTATUS(0xC0190028_u32 as _);
pub const STATUS_HANDLE_REVOKED: NTSTATUS = NTSTATUS(0xC000A006_u32 as _);
pub const STATUS_HARDWARE_MEMORY_ERROR: NTSTATUS = NTSTATUS(0xC0000709_u32 as _);
pub const STATUS_HASH_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC000A101_u32 as _);
pub const STATUS_HASH_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A100_u32 as _);
pub const STATUS_HAS_SYSTEM_CRITICAL_FILES: NTSTATUS = NTSTATUS(0xC00004BD_u32 as _);
pub const STATUS_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0440003_u32 as _);
pub const STATUS_HDAUDIO_EMPTY_CONNECTION_LIST: NTSTATUS = NTSTATUS(0xC0440002_u32 as _);
pub const STATUS_HDAUDIO_NO_LOGICAL_DEVICES_CREATED: NTSTATUS = NTSTATUS(0xC0440004_u32 as _);
pub const STATUS_HDAUDIO_NULL_LINKED_LIST_ENTRY: NTSTATUS = NTSTATUS(0xC0440005_u32 as _);
pub const STATUS_HEAP_CORRUPTION: NTSTATUS = NTSTATUS(0xC0000374_u32 as _);
pub const STATUS_HEURISTIC_DAMAGE_POSSIBLE: NTSTATUS = NTSTATUS(0x40190001_u32 as _);
pub const STATUS_HIBERNATED: NTSTATUS = NTSTATUS(0x4000002A_u32 as _);
pub const STATUS_HIBERNATION_FAILURE: NTSTATUS = NTSTATUS(0xC0000411_u32 as _);
pub const STATUS_HIVE_UNLOADED: NTSTATUS = NTSTATUS(0xC0000425_u32 as _);
pub const STATUS_HMAC_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A001_u32 as _);
pub const STATUS_HOPLIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC000A012_u32 as _);
pub const STATUS_HOST_DOWN: NTSTATUS = NTSTATUS(0xC0000350_u32 as _);
pub const STATUS_HOST_UNREACHABLE: NTSTATUS = NTSTATUS(0xC000023D_u32 as _);
pub const STATUS_HUNG_DISPLAY_DRIVER_THREAD: NTSTATUS = NTSTATUS(0xC0000415_u32 as _);
pub const STATUS_HV_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC0350006_u32 as _);
pub const STATUS_HV_ACKNOWLEDGED: NTSTATUS = NTSTATUS(0xC0350016_u32 as _);
pub const STATUS_HV_CALL_PENDING: NTSTATUS = NTSTATUS(0xC0350079_u32 as _);
pub const STATUS_HV_CPUID_FEATURE_VALIDATION_ERROR: NTSTATUS = NTSTATUS(0xC035003C_u32 as _);
pub const STATUS_HV_CPUID_XSAVE_FEATURE_VALIDATION_ERROR: NTSTATUS = NTSTATUS(0xC035003D_u32 as _);
pub const STATUS_HV_DEVICE_NOT_IN_DOMAIN: NTSTATUS = NTSTATUS(0xC0350076_u32 as _);
pub const STATUS_HV_EVENT_BUFFER_ALREADY_FREED: NTSTATUS = NTSTATUS(0xC0350074_u32 as _);
pub const STATUS_HV_FEATURE_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC035001E_u32 as _);
pub const STATUS_HV_INACTIVE: NTSTATUS = NTSTATUS(0xC035001C_u32 as _);
pub const STATUS_HV_INSUFFICIENT_BUFFER: NTSTATUS = NTSTATUS(0xC0350033_u32 as _);
pub const STATUS_HV_INSUFFICIENT_BUFFERS: NTSTATUS = NTSTATUS(0xC0350013_u32 as _);
pub const STATUS_HV_INSUFFICIENT_CONTIGUOUS_MEMORY: NTSTATUS = NTSTATUS(0xC0350075_u32 as _);
pub const STATUS_HV_INSUFFICIENT_CONTIGUOUS_MEMORY_MIRRORING: NTSTATUS = NTSTATUS(0xC0350082_u32 as _);
pub const STATUS_HV_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY: NTSTATUS = NTSTATUS(0xC0350083_u32 as _);
pub const STATUS_HV_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY_MIRRORING: NTSTATUS = NTSTATUS(0xC0350085_u32 as _);
pub const STATUS_HV_INSUFFICIENT_DEVICE_DOMAINS: NTSTATUS = NTSTATUS(0xC0350038_u32 as _);
pub const STATUS_HV_INSUFFICIENT_MEMORY: NTSTATUS = NTSTATUS(0xC035000B_u32 as _);
pub const STATUS_HV_INSUFFICIENT_MEMORY_MIRRORING: NTSTATUS = NTSTATUS(0xC0350081_u32 as _);
pub const STATUS_HV_INSUFFICIENT_ROOT_MEMORY: NTSTATUS = NTSTATUS(0xC0350073_u32 as _);
pub const STATUS_HV_INSUFFICIENT_ROOT_MEMORY_MIRRORING: NTSTATUS = NTSTATUS(0xC0350084_u32 as _);
pub const STATUS_HV_INVALID_ALIGNMENT: NTSTATUS = NTSTATUS(0xC0350004_u32 as _);
pub const STATUS_HV_INVALID_CONNECTION_ID: NTSTATUS = NTSTATUS(0xC0350012_u32 as _);
pub const STATUS_HV_INVALID_CPU_GROUP_ID: NTSTATUS = NTSTATUS(0xC035006F_u32 as _);
pub const STATUS_HV_INVALID_CPU_GROUP_STATE: NTSTATUS = NTSTATUS(0xC0350070_u32 as _);
pub const STATUS_HV_INVALID_DEVICE_ID: NTSTATUS = NTSTATUS(0xC0350057_u32 as _);
pub const STATUS_HV_INVALID_DEVICE_STATE: NTSTATUS = NTSTATUS(0xC0350058_u32 as _);
pub const STATUS_HV_INVALID_HYPERCALL_CODE: NTSTATUS = NTSTATUS(0xC0350002_u32 as _);
pub const STATUS_HV_INVALID_HYPERCALL_INPUT: NTSTATUS = NTSTATUS(0xC0350003_u32 as _);
pub const STATUS_HV_INVALID_LP_INDEX: NTSTATUS = NTSTATUS(0xC0350041_u32 as _);
pub const STATUS_HV_INVALID_PARAMETER: NTSTATUS = NTSTATUS(0xC0350005_u32 as _);
pub const STATUS_HV_INVALID_PARTITION_ID: NTSTATUS = NTSTATUS(0xC035000D_u32 as _);
pub const STATUS_HV_INVALID_PARTITION_STATE: NTSTATUS = NTSTATUS(0xC0350007_u32 as _);
pub const STATUS_HV_INVALID_PORT_ID: NTSTATUS = NTSTATUS(0xC0350011_u32 as _);
pub const STATUS_HV_INVALID_PROXIMITY_DOMAIN_INFO: NTSTATUS = NTSTATUS(0xC035001A_u32 as _);
pub const STATUS_HV_INVALID_REGISTER_VALUE: NTSTATUS = NTSTATUS(0xC0350050_u32 as _);
pub const STATUS_HV_INVALID_SAVE_RESTORE_STATE: NTSTATUS = NTSTATUS(0xC0350017_u32 as _);
pub const STATUS_HV_INVALID_SYNIC_STATE: NTSTATUS = NTSTATUS(0xC0350018_u32 as _);
pub const STATUS_HV_INVALID_VP_INDEX: NTSTATUS = NTSTATUS(0xC035000E_u32 as _);
pub const STATUS_HV_INVALID_VP_STATE: NTSTATUS = NTSTATUS(0xC0350015_u32 as _);
pub const STATUS_HV_INVALID_VTL_STATE: NTSTATUS = NTSTATUS(0xC0350051_u32 as _);
pub const STATUS_HV_MSR_ACCESS_FAILED: NTSTATUS = NTSTATUS(0xC0350080_u32 as _);
pub const STATUS_HV_NESTED_VM_EXIT: NTSTATUS = NTSTATUS(0xC0350077_u32 as _);
pub const STATUS_HV_NOT_ACKNOWLEDGED: NTSTATUS = NTSTATUS(0xC0350014_u32 as _);
pub const STATUS_HV_NOT_ALLOWED_WITH_NESTED_VIRT_ACTIVE: NTSTATUS = NTSTATUS(0xC0350072_u32 as _);
pub const STATUS_HV_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC0351000_u32 as _);
pub const STATUS_HV_NO_DATA: NTSTATUS = NTSTATUS(0xC035001B_u32 as _);
pub const STATUS_HV_NO_RESOURCES: NTSTATUS = NTSTATUS(0xC035001D_u32 as _);
pub const STATUS_HV_NX_NOT_DETECTED: NTSTATUS = NTSTATUS(0xC0350055_u32 as _);
pub const STATUS_HV_OBJECT_IN_USE: NTSTATUS = NTSTATUS(0xC0350019_u32 as _);
pub const STATUS_HV_OPERATION_DENIED: NTSTATUS = NTSTATUS(0xC0350008_u32 as _);
pub const STATUS_HV_OPERATION_FAILED: NTSTATUS = NTSTATUS(0xC0350071_u32 as _);
pub const STATUS_HV_PAGE_REQUEST_INVALID: NTSTATUS = NTSTATUS(0xC0350060_u32 as _);
pub const STATUS_HV_PARTITION_TOO_DEEP: NTSTATUS = NTSTATUS(0xC035000C_u32 as _);
pub const STATUS_HV_PENDING_PAGE_REQUESTS: NTSTATUS = NTSTATUS(0x350059_u32 as _);
pub const STATUS_HV_PROCESSOR_STARTUP_TIMEOUT: NTSTATUS = NTSTATUS(0xC035003E_u32 as _);
pub const STATUS_HV_PROPERTY_VALUE_OUT_OF_RANGE: NTSTATUS = NTSTATUS(0xC035000A_u32 as _);
pub const STATUS_HV_SMX_ENABLED: NTSTATUS = NTSTATUS(0xC035003F_u32 as _);
pub const STATUS_HV_UNKNOWN_PROPERTY: NTSTATUS = NTSTATUS(0xC0350009_u32 as _);
pub const STATUS_ILLEGAL_CHARACTER: NTSTATUS = NTSTATUS(0xC0000161_u32 as _);
pub const STATUS_ILLEGAL_DLL_RELOCATION: NTSTATUS = NTSTATUS(0xC0000269_u32 as _);
pub const STATUS_ILLEGAL_ELEMENT_ADDRESS: NTSTATUS = NTSTATUS(0xC0000285_u32 as _);
pub const STATUS_ILLEGAL_FLOAT_CONTEXT: NTSTATUS = NTSTATUS(0xC000014A_u32 as _);
pub const STATUS_ILLEGAL_FUNCTION: NTSTATUS = NTSTATUS(0xC00000AF_u32 as _);
pub const STATUS_ILLEGAL_INSTRUCTION: NTSTATUS = NTSTATUS(0xC000001D_u32 as _);
pub const STATUS_ILL_FORMED_PASSWORD: NTSTATUS = NTSTATUS(0xC000006B_u32 as _);
pub const STATUS_ILL_FORMED_SERVICE_ENTRY: NTSTATUS = NTSTATUS(0xC0000160_u32 as _);
pub const STATUS_IMAGE_ALREADY_LOADED: NTSTATUS = NTSTATUS(0xC000010E_u32 as _);
pub const STATUS_IMAGE_ALREADY_LOADED_AS_DLL: NTSTATUS = NTSTATUS(0xC000019D_u32 as _);
pub const STATUS_IMAGE_AT_DIFFERENT_BASE: NTSTATUS = NTSTATUS(0x40000036_u32 as _);
pub const STATUS_IMAGE_CERT_EXPIRED: NTSTATUS = NTSTATUS(0xC0000605_u32 as _);
pub const STATUS_IMAGE_CERT_REVOKED: NTSTATUS = NTSTATUS(0xC0000603_u32 as _);
pub const STATUS_IMAGE_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(0xC0000221_u32 as _);
pub const STATUS_IMAGE_LOADED_AS_PATCH_IMAGE: NTSTATUS = NTSTATUS(0xC00004C0_u32 as _);
pub const STATUS_IMAGE_MACHINE_TYPE_MISMATCH: NTSTATUS = NTSTATUS(0x4000000E_u32 as _);
pub const STATUS_IMAGE_MACHINE_TYPE_MISMATCH_EXE: NTSTATUS = NTSTATUS(0x40000023_u32 as _);
pub const STATUS_IMAGE_MP_UP_MISMATCH: NTSTATUS = NTSTATUS(0xC0000249_u32 as _);
pub const STATUS_IMAGE_NOT_AT_BASE: NTSTATUS = NTSTATUS(0x40000003_u32 as _);
pub const STATUS_IMAGE_SUBSYSTEM_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC00001A3_u32 as _);
pub const STATUS_IMPLEMENTATION_LIMIT: NTSTATUS = NTSTATUS(0xC000042B_u32 as _);
pub const STATUS_INCOMPATIBLE_DRIVER_BLOCKED: NTSTATUS = NTSTATUS(0xC0000424_u32 as _);
pub const STATUS_INCOMPATIBLE_FILE_MAP: NTSTATUS = NTSTATUS(0xC000004D_u32 as _);
pub const STATUS_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING: NTSTATUS = NTSTATUS(0xC000019E_u32 as _);
pub const STATUS_INCORRECT_ACCOUNT_TYPE: NTSTATUS = NTSTATUS(0xC000A089_u32 as _);
pub const STATUS_INDEX_OUT_OF_BOUNDS: NTSTATUS = NTSTATUS(0xC00004D1_u32 as _);
pub const STATUS_INDOUBT_TRANSACTIONS_EXIST: NTSTATUS = NTSTATUS(0xC019003A_u32 as _);
pub const STATUS_INFO_LENGTH_MISMATCH: NTSTATUS = NTSTATUS(0xC0000004_u32 as _);
pub const STATUS_INSTANCE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC00000AB_u32 as _);
pub const STATUS_INSTRUCTION_MISALIGNMENT: NTSTATUS = NTSTATUS(0xC00000AA_u32 as _);
pub const STATUS_INSUFFICIENT_LOGON_INFO: NTSTATUS = NTSTATUS(0xC0000250_u32 as _);
pub const STATUS_INSUFFICIENT_NVRAM_RESOURCES: NTSTATUS = NTSTATUS(0xC0000454_u32 as _);
pub const STATUS_INSUFFICIENT_POWER: NTSTATUS = NTSTATUS(0xC00002DE_u32 as _);
pub const STATUS_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(0xC000009A_u32 as _);
pub const STATUS_INSUFFICIENT_RESOURCE_FOR_SPECIFIED_SHARED_SECTION_SIZE: NTSTATUS = NTSTATUS(0xC0000416_u32 as _);
pub const STATUS_INSUFFICIENT_VIRTUAL_ADDR_RESOURCES: NTSTATUS = NTSTATUS(0xC00004C2_u32 as _);
pub const STATUS_INSUFF_SERVER_RESOURCES: NTSTATUS = NTSTATUS(0xC0000205_u32 as _);
pub const STATUS_INTEGER_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(0xC0000094_u32 as _);
pub const STATUS_INTEGER_OVERFLOW: NTSTATUS = NTSTATUS(0xC0000095_u32 as _);
pub const STATUS_INTERMIXED_KERNEL_EA_OPERATION: NTSTATUS = NTSTATUS(0xC0000471_u32 as _);
pub const STATUS_INTERNAL_DB_CORRUPTION: NTSTATUS = NTSTATUS(0xC00000E4_u32 as _);
pub const STATUS_INTERNAL_DB_ERROR: NTSTATUS = NTSTATUS(0xC0000158_u32 as _);
pub const STATUS_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC00000E5_u32 as _);
pub const STATUS_INTERRUPTED: NTSTATUS = NTSTATUS(0xC0000515_u32 as _);
pub const STATUS_INTERRUPT_STILL_CONNECTED: NTSTATUS = NTSTATUS(0x128_u32 as _);
pub const STATUS_INTERRUPT_VECTOR_ALREADY_CONNECTED: NTSTATUS = NTSTATUS(0x127_u32 as _);
pub const STATUS_INVALID_ACCOUNT_NAME: NTSTATUS = NTSTATUS(0xC0000062_u32 as _);
pub const STATUS_INVALID_ACE_CONDITION: NTSTATUS = NTSTATUS(0xC00001A2_u32 as _);
pub const STATUS_INVALID_ACL: NTSTATUS = NTSTATUS(0xC0000077_u32 as _);
pub const STATUS_INVALID_ADDRESS: NTSTATUS = NTSTATUS(0xC0000141_u32 as _);
pub const STATUS_INVALID_ADDRESS_COMPONENT: NTSTATUS = NTSTATUS(0xC0000207_u32 as _);
pub const STATUS_INVALID_ADDRESS_WILDCARD: NTSTATUS = NTSTATUS(0xC0000208_u32 as _);
pub const STATUS_INVALID_BLOCK_LENGTH: NTSTATUS = NTSTATUS(0xC0000173_u32 as _);
pub const STATUS_INVALID_BUFFER_SIZE: NTSTATUS = NTSTATUS(0xC0000206_u32 as _);
pub const STATUS_INVALID_CAP: NTSTATUS = NTSTATUS(0xC0000505_u32 as _);
pub const STATUS_INVALID_CID: NTSTATUS = NTSTATUS(0xC000000B_u32 as _);
pub const STATUS_INVALID_COMPUTER_NAME: NTSTATUS = NTSTATUS(0xC0000122_u32 as _);
pub const STATUS_INVALID_CONFIG_VALUE: NTSTATUS = NTSTATUS(0xC00004E0_u32 as _);
pub const STATUS_INVALID_CONNECTION: NTSTATUS = NTSTATUS(0xC0000140_u32 as _);
pub const STATUS_INVALID_CRUNTIME_PARAMETER: NTSTATUS = NTSTATUS(0xC0000417_u32 as _);
pub const STATUS_INVALID_DEVICE_OBJECT_PARAMETER: NTSTATUS = NTSTATUS(0xC0000369_u32 as _);
pub const STATUS_INVALID_DEVICE_REQUEST: NTSTATUS = NTSTATUS(0xC0000010_u32 as _);
pub const STATUS_INVALID_DEVICE_STATE: NTSTATUS = NTSTATUS(0xC0000184_u32 as _);
pub const STATUS_INVALID_DISPOSITION: NTSTATUS = NTSTATUS(0xC0000026_u32 as _);
pub const STATUS_INVALID_DOMAIN_ROLE: NTSTATUS = NTSTATUS(0xC00000DE_u32 as _);
pub const STATUS_INVALID_DOMAIN_STATE: NTSTATUS = NTSTATUS(0xC00000DD_u32 as _);
pub const STATUS_INVALID_EA_FLAG: NTSTATUS = NTSTATUS(0x80000015_u32 as _);
pub const STATUS_INVALID_EA_NAME: NTSTATUS = NTSTATUS(0x80000013_u32 as _);
pub const STATUS_INVALID_EXCEPTION_HANDLER: NTSTATUS = NTSTATUS(0xC00001A5_u32 as _);
pub const STATUS_INVALID_FIELD_IN_PARAMETER_LIST: NTSTATUS = NTSTATUS(0xC0000475_u32 as _);
pub const STATUS_INVALID_FILE_FOR_SECTION: NTSTATUS = NTSTATUS(0xC0000020_u32 as _);
pub const STATUS_INVALID_GROUP_ATTRIBUTES: NTSTATUS = NTSTATUS(0xC00000A4_u32 as _);
pub const STATUS_INVALID_HANDLE: NTSTATUS = NTSTATUS(0xC0000008_u32 as _);
pub const STATUS_INVALID_HW_PROFILE: NTSTATUS = NTSTATUS(0xC0000260_u32 as _);
pub const STATUS_INVALID_IDN_NORMALIZATION: NTSTATUS = NTSTATUS(0xC0000716_u32 as _);
pub const STATUS_INVALID_ID_AUTHORITY: NTSTATUS = NTSTATUS(0xC0000084_u32 as _);
pub const STATUS_INVALID_IMAGE_FORMAT: NTSTATUS = NTSTATUS(0xC000007B_u32 as _);
pub const STATUS_INVALID_IMAGE_HASH: NTSTATUS = NTSTATUS(0xC0000428_u32 as _);
pub const STATUS_INVALID_IMAGE_LE_FORMAT: NTSTATUS = NTSTATUS(0xC000012E_u32 as _);
pub const STATUS_INVALID_IMAGE_NE_FORMAT: NTSTATUS = NTSTATUS(0xC000011B_u32 as _);
pub const STATUS_INVALID_IMAGE_NOT_MZ: NTSTATUS = NTSTATUS(0xC000012F_u32 as _);
pub const STATUS_INVALID_IMAGE_PROTECT: NTSTATUS = NTSTATUS(0xC0000130_u32 as _);
pub const STATUS_INVALID_IMAGE_WIN_16: NTSTATUS = NTSTATUS(0xC0000131_u32 as _);
pub const STATUS_INVALID_IMAGE_WIN_32: NTSTATUS = NTSTATUS(0xC0000359_u32 as _);
pub const STATUS_INVALID_IMAGE_WIN_64: NTSTATUS = NTSTATUS(0xC000035A_u32 as _);
pub const STATUS_INVALID_IMPORT_OF_NON_DLL: NTSTATUS = NTSTATUS(0xC000036F_u32 as _);
pub const STATUS_INVALID_INFO_CLASS: NTSTATUS = NTSTATUS(0xC0000003_u32 as _);
pub const STATUS_INVALID_INITIATOR_TARGET_PATH: NTSTATUS = NTSTATUS(0xC0000477_u32 as _);
pub const STATUS_INVALID_KERNEL_INFO_VERSION: NTSTATUS = NTSTATUS(0xC000A004_u32 as _);
pub const STATUS_INVALID_LABEL: NTSTATUS = NTSTATUS(0xC0000446_u32 as _);
pub const STATUS_INVALID_LDT_DESCRIPTOR: NTSTATUS = NTSTATUS(0xC000011A_u32 as _);
pub const STATUS_INVALID_LDT_OFFSET: NTSTATUS = NTSTATUS(0xC0000119_u32 as _);
pub const STATUS_INVALID_LDT_SIZE: NTSTATUS = NTSTATUS(0xC0000118_u32 as _);
pub const STATUS_INVALID_LEVEL: NTSTATUS = NTSTATUS(0xC0000148_u32 as _);
pub const STATUS_INVALID_LOCK_RANGE: NTSTATUS = NTSTATUS(0xC00001A1_u32 as _);
pub const STATUS_INVALID_LOCK_SEQUENCE: NTSTATUS = NTSTATUS(0xC000001E_u32 as _);
pub const STATUS_INVALID_LOGON_HOURS: NTSTATUS = NTSTATUS(0xC000006F_u32 as _);
pub const STATUS_INVALID_LOGON_TYPE: NTSTATUS = NTSTATUS(0xC000010B_u32 as _);
pub const STATUS_INVALID_MEMBER: NTSTATUS = NTSTATUS(0xC000017B_u32 as _);
pub const STATUS_INVALID_MESSAGE: NTSTATUS = NTSTATUS(0xC0000702_u32 as _);
pub const STATUS_INVALID_NETWORK_RESPONSE: NTSTATUS = NTSTATUS(0xC00000C3_u32 as _);
pub const STATUS_INVALID_OFFSET_ALIGNMENT: NTSTATUS = NTSTATUS(0xC0000474_u32 as _);
pub const STATUS_INVALID_OPLOCK_PROTOCOL: NTSTATUS = NTSTATUS(0xC00000E3_u32 as _);
pub const STATUS_INVALID_OWNER: NTSTATUS = NTSTATUS(0xC000005A_u32 as _);
pub const STATUS_INVALID_PACKAGE_SID_LENGTH: NTSTATUS = NTSTATUS(0xC000A202_u32 as _);
pub const STATUS_INVALID_PAGE_PROTECTION: NTSTATUS = NTSTATUS(0xC0000045_u32 as _);
pub const STATUS_INVALID_PARAMETER: NTSTATUS = NTSTATUS(0xC000000D_u32 as _);
pub const STATUS_INVALID_PARAMETER_1: NTSTATUS = NTSTATUS(0xC00000EF_u32 as _);
pub const STATUS_INVALID_PARAMETER_10: NTSTATUS = NTSTATUS(0xC00000F8_u32 as _);
pub const STATUS_INVALID_PARAMETER_11: NTSTATUS = NTSTATUS(0xC00000F9_u32 as _);
pub const STATUS_INVALID_PARAMETER_12: NTSTATUS = NTSTATUS(0xC00000FA_u32 as _);
pub const STATUS_INVALID_PARAMETER_2: NTSTATUS = NTSTATUS(0xC00000F0_u32 as _);
pub const STATUS_INVALID_PARAMETER_3: NTSTATUS = NTSTATUS(0xC00000F1_u32 as _);
pub const STATUS_INVALID_PARAMETER_4: NTSTATUS = NTSTATUS(0xC00000F2_u32 as _);
pub const STATUS_INVALID_PARAMETER_5: NTSTATUS = NTSTATUS(0xC00000F3_u32 as _);
pub const STATUS_INVALID_PARAMETER_6: NTSTATUS = NTSTATUS(0xC00000F4_u32 as _);
pub const STATUS_INVALID_PARAMETER_7: NTSTATUS = NTSTATUS(0xC00000F5_u32 as _);
pub const STATUS_INVALID_PARAMETER_8: NTSTATUS = NTSTATUS(0xC00000F6_u32 as _);
pub const STATUS_INVALID_PARAMETER_9: NTSTATUS = NTSTATUS(0xC00000F7_u32 as _);
pub const STATUS_INVALID_PARAMETER_MIX: NTSTATUS = NTSTATUS(0xC0000030_u32 as _);
pub const STATUS_INVALID_PEP_INFO_VERSION: NTSTATUS = NTSTATUS(0xC000A005_u32 as _);
pub const STATUS_INVALID_PIPE_STATE: NTSTATUS = NTSTATUS(0xC00000AD_u32 as _);
pub const STATUS_INVALID_PLUGPLAY_DEVICE_PATH: NTSTATUS = NTSTATUS(0xC0000261_u32 as _);
pub const STATUS_INVALID_PORT_ATTRIBUTES: NTSTATUS = NTSTATUS(0xC000002E_u32 as _);
pub const STATUS_INVALID_PORT_HANDLE: NTSTATUS = NTSTATUS(0xC0000042_u32 as _);
pub const STATUS_INVALID_PRIMARY_GROUP: NTSTATUS = NTSTATUS(0xC000005B_u32 as _);
pub const STATUS_INVALID_QUOTA_LOWER: NTSTATUS = NTSTATUS(0xC0000031_u32 as _);
pub const STATUS_INVALID_READ_MODE: NTSTATUS = NTSTATUS(0xC00000B4_u32 as _);
pub const STATUS_INVALID_RUNLEVEL_SETTING: NTSTATUS = NTSTATUS(0xC000A142_u32 as _);
pub const STATUS_INVALID_SECURITY_DESCR: NTSTATUS = NTSTATUS(0xC0000079_u32 as _);
pub const STATUS_INVALID_SERVER_STATE: NTSTATUS = NTSTATUS(0xC00000DC_u32 as _);
pub const STATUS_INVALID_SESSION: NTSTATUS = NTSTATUS(0xC0000455_u32 as _);
pub const STATUS_INVALID_SID: NTSTATUS = NTSTATUS(0xC0000078_u32 as _);
pub const STATUS_INVALID_SIGNATURE: NTSTATUS = NTSTATUS(0xC000A000_u32 as _);
pub const STATUS_INVALID_STATE_TRANSITION: NTSTATUS = NTSTATUS(0xC000A003_u32 as _);
pub const STATUS_INVALID_SUB_AUTHORITY: NTSTATUS = NTSTATUS(0xC0000076_u32 as _);
pub const STATUS_INVALID_SYSTEM_SERVICE: NTSTATUS = NTSTATUS(0xC000001C_u32 as _);
pub const STATUS_INVALID_TASK_INDEX: NTSTATUS = NTSTATUS(0xC0000501_u32 as _);
pub const STATUS_INVALID_TASK_NAME: NTSTATUS = NTSTATUS(0xC0000500_u32 as _);
pub const STATUS_INVALID_THREAD: NTSTATUS = NTSTATUS(0xC000071C_u32 as _);
pub const STATUS_INVALID_TOKEN: NTSTATUS = NTSTATUS(0xC0000465_u32 as _);
pub const STATUS_INVALID_TRANSACTION: NTSTATUS = NTSTATUS(0xC0190002_u32 as _);
pub const STATUS_INVALID_UNWIND_TARGET: NTSTATUS = NTSTATUS(0xC0000029_u32 as _);
pub const STATUS_INVALID_USER_BUFFER: NTSTATUS = NTSTATUS(0xC00000E8_u32 as _);
pub const STATUS_INVALID_USER_PRINCIPAL_NAME: NTSTATUS = NTSTATUS(0xC000041C_u32 as _);
pub const STATUS_INVALID_VARIANT: NTSTATUS = NTSTATUS(0xC0000232_u32 as _);
pub const STATUS_INVALID_VIEW_SIZE: NTSTATUS = NTSTATUS(0xC000001F_u32 as _);
pub const STATUS_INVALID_VOLUME_LABEL: NTSTATUS = NTSTATUS(0xC0000086_u32 as _);
pub const STATUS_INVALID_WEIGHT: NTSTATUS = NTSTATUS(0xC0000458_u32 as _);
pub const STATUS_INVALID_WORKSTATION: NTSTATUS = NTSTATUS(0xC0000070_u32 as _);
pub const STATUS_IN_PAGE_ERROR: NTSTATUS = NTSTATUS(0xC0000006_u32 as _);
pub const STATUS_IORING_COMPLETION_QUEUE_TOO_BIG: NTSTATUS = NTSTATUS(0xC0460005_u32 as _);
pub const STATUS_IORING_COMPLETION_QUEUE_TOO_FULL: NTSTATUS = NTSTATUS(0xC0460008_u32 as _);
pub const STATUS_IORING_CORRUPT: NTSTATUS = NTSTATUS(0xC0460007_u32 as _);
pub const STATUS_IORING_REQUIRED_FLAG_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0460001_u32 as _);
pub const STATUS_IORING_SUBMISSION_QUEUE_FULL: NTSTATUS = NTSTATUS(0xC0460002_u32 as _);
pub const STATUS_IORING_SUBMISSION_QUEUE_TOO_BIG: NTSTATUS = NTSTATUS(0xC0460004_u32 as _);
pub const STATUS_IORING_SUBMIT_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0460006_u32 as _);
pub const STATUS_IORING_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0460003_u32 as _);
pub const STATUS_IO_DEVICE_ERROR: NTSTATUS = NTSTATUS(0xC0000185_u32 as _);
pub const STATUS_IO_DEVICE_INVALID_DATA: NTSTATUS = NTSTATUS(0xC00001B0_u32 as _);
pub const STATUS_IO_OPERATION_TIMEOUT: NTSTATUS = NTSTATUS(0xC000047D_u32 as _);
pub const STATUS_IO_PREEMPTED: NTSTATUS = NTSTATUS(0xC0510001_u32 as _);
pub const STATUS_IO_PRIVILEGE_FAILED: NTSTATUS = NTSTATUS(0xC0000137_u32 as _);
pub const STATUS_IO_REISSUE_AS_CACHED: NTSTATUS = NTSTATUS(0xC0040039_u32 as _);
pub const STATUS_IO_REPARSE_DATA_INVALID: NTSTATUS = NTSTATUS(0xC0000278_u32 as _);
pub const STATUS_IO_REPARSE_TAG_INVALID: NTSTATUS = NTSTATUS(0xC0000276_u32 as _);
pub const STATUS_IO_REPARSE_TAG_MISMATCH: NTSTATUS = NTSTATUS(0xC0000277_u32 as _);
pub const STATUS_IO_REPARSE_TAG_NOT_HANDLED: NTSTATUS = NTSTATUS(0xC0000279_u32 as _);
pub const STATUS_IO_TIMEOUT: NTSTATUS = NTSTATUS(0xC00000B5_u32 as _);
pub const STATUS_IO_UNALIGNED_WRITE: NTSTATUS = NTSTATUS(0xC00001B1_u32 as _);
pub const STATUS_IPSEC_AUTH_FIREWALL_DROP: NTSTATUS = NTSTATUS(0xC0360008_u32 as _);
pub const STATUS_IPSEC_BAD_SPI: NTSTATUS = NTSTATUS(0xC0360001_u32 as _);
pub const STATUS_IPSEC_CLEAR_TEXT_DROP: NTSTATUS = NTSTATUS(0xC0360007_u32 as _);
pub const STATUS_IPSEC_DOSP_BLOCK: NTSTATUS = NTSTATUS(0xC0368000_u32 as _);
pub const STATUS_IPSEC_DOSP_INVALID_PACKET: NTSTATUS = NTSTATUS(0xC0368002_u32 as _);
pub const STATUS_IPSEC_DOSP_KEYMOD_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0368005_u32 as _);
pub const STATUS_IPSEC_DOSP_MAX_ENTRIES: NTSTATUS = NTSTATUS(0xC0368004_u32 as _);
pub const STATUS_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES: NTSTATUS = NTSTATUS(0xC0368006_u32 as _);
pub const STATUS_IPSEC_DOSP_RECEIVED_MULTICAST: NTSTATUS = NTSTATUS(0xC0368001_u32 as _);
pub const STATUS_IPSEC_DOSP_STATE_LOOKUP_FAILED: NTSTATUS = NTSTATUS(0xC0368003_u32 as _);
pub const STATUS_IPSEC_INTEGRITY_CHECK_FAILED: NTSTATUS = NTSTATUS(0xC0360006_u32 as _);
pub const STATUS_IPSEC_INVALID_PACKET: NTSTATUS = NTSTATUS(0xC0360005_u32 as _);
pub const STATUS_IPSEC_QUEUE_OVERFLOW: NTSTATUS = NTSTATUS(0xC000A010_u32 as _);
pub const STATUS_IPSEC_REPLAY_CHECK_FAILED: NTSTATUS = NTSTATUS(0xC0360004_u32 as _);
pub const STATUS_IPSEC_SA_LIFETIME_EXPIRED: NTSTATUS = NTSTATUS(0xC0360002_u32 as _);
pub const STATUS_IPSEC_THROTTLE_DROP: NTSTATUS = NTSTATUS(0xC0360009_u32 as _);
pub const STATUS_IPSEC_WRONG_SA: NTSTATUS = NTSTATUS(0xC0360003_u32 as _);
pub const STATUS_IP_ADDRESS_CONFLICT1: NTSTATUS = NTSTATUS(0xC0000254_u32 as _);
pub const STATUS_IP_ADDRESS_CONFLICT2: NTSTATUS = NTSTATUS(0xC0000255_u32 as _);
pub const STATUS_ISSUING_CA_UNTRUSTED: NTSTATUS = NTSTATUS(0xC000038A_u32 as _);
pub const STATUS_ISSUING_CA_UNTRUSTED_KDC: NTSTATUS = NTSTATUS(0xC000040D_u32 as _);
pub const STATUS_JOB_NOT_EMPTY: NTSTATUS = NTSTATUS(0xC000050F_u32 as _);
pub const STATUS_JOB_NO_CONTAINER: NTSTATUS = NTSTATUS(0xC0000509_u32 as _);
pub const STATUS_JOURNAL_DELETE_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC00002B7_u32 as _);
pub const STATUS_JOURNAL_ENTRY_DELETED: NTSTATUS = NTSTATUS(0xC00002CF_u32 as _);
pub const STATUS_JOURNAL_NOT_ACTIVE: NTSTATUS = NTSTATUS(0xC00002B8_u32 as _);
pub const STATUS_KDC_CERT_EXPIRED: NTSTATUS = NTSTATUS(0xC000040E_u32 as _);
pub const STATUS_KDC_CERT_REVOKED: NTSTATUS = NTSTATUS(0xC000040F_u32 as _);
pub const STATUS_KDC_INVALID_REQUEST: NTSTATUS = NTSTATUS(0xC00002FB_u32 as _);
pub const STATUS_KDC_UNABLE_TO_REFER: NTSTATUS = NTSTATUS(0xC00002FC_u32 as _);
pub const STATUS_KDC_UNKNOWN_ETYPE: NTSTATUS = NTSTATUS(0xC00002FD_u32 as _);
pub const STATUS_KERNEL_APC: NTSTATUS = NTSTATUS(0x100_u32 as _);
pub const STATUS_KERNEL_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(0xC0000724_u32 as _);
pub const STATUS_KEY_DELETED: NTSTATUS = NTSTATUS(0xC000017C_u32 as _);
pub const STATUS_KEY_HAS_CHILDREN: NTSTATUS = NTSTATUS(0xC0000180_u32 as _);
pub const STATUS_LAPS_ENCRYPTION_REQUIRES_2016_DFL: NTSTATUS = NTSTATUS(0xC000A08E_u32 as _);
pub const STATUS_LAPS_LEGACY_SCHEMA_MISSING: NTSTATUS = NTSTATUS(0xC000A08C_u32 as _);
pub const STATUS_LAPS_SCHEMA_MISSING: NTSTATUS = NTSTATUS(0xC000A08D_u32 as _);
pub const STATUS_LAST_ADMIN: NTSTATUS = NTSTATUS(0xC0000069_u32 as _);
pub const STATUS_LICENSE_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000259_u32 as _);
pub const STATUS_LICENSE_VIOLATION: NTSTATUS = NTSTATUS(0xC000026A_u32 as _);
pub const STATUS_LINK_FAILED: NTSTATUS = NTSTATUS(0xC000013E_u32 as _);
pub const STATUS_LINK_TIMEOUT: NTSTATUS = NTSTATUS(0xC000013F_u32 as _);
pub const STATUS_LM_CROSS_ENCRYPTION_REQUIRED: NTSTATUS = NTSTATUS(0xC000017F_u32 as _);
pub const STATUS_LOCAL_DISCONNECT: NTSTATUS = NTSTATUS(0xC000013B_u32 as _);
pub const STATUS_LOCAL_POLICY_MODIFICATION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A08A_u32 as _);
pub const STATUS_LOCAL_USER_SESSION_KEY: NTSTATUS = NTSTATUS(0x40000006_u32 as _);
pub const STATUS_LOCK_NOT_GRANTED: NTSTATUS = NTSTATUS(0xC0000055_u32 as _);
pub const STATUS_LOGIN_TIME_RESTRICTION: NTSTATUS = NTSTATUS(0xC0000247_u32 as _);
pub const STATUS_LOGIN_WKSTA_RESTRICTION: NTSTATUS = NTSTATUS(0xC0000248_u32 as _);
pub const STATUS_LOGON_NOT_GRANTED: NTSTATUS = NTSTATUS(0xC0000155_u32 as _);
pub const STATUS_LOGON_SERVER_CONFLICT: NTSTATUS = NTSTATUS(0xC0000132_u32 as _);
pub const STATUS_LOGON_SESSION_COLLISION: NTSTATUS = NTSTATUS(0xC0000105_u32 as _);
pub const STATUS_LOGON_SESSION_EXISTS: NTSTATUS = NTSTATUS(0xC00000EE_u32 as _);
pub const STATUS_LOG_APPENDED_FLUSH_FAILED: NTSTATUS = NTSTATUS(0xC01A002F_u32 as _);
pub const STATUS_LOG_ARCHIVE_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC01A0021_u32 as _);
pub const STATUS_LOG_ARCHIVE_NOT_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC01A0020_u32 as _);
pub const STATUS_LOG_BLOCKS_EXHAUSTED: NTSTATUS = NTSTATUS(0xC01A0006_u32 as _);
pub const STATUS_LOG_BLOCK_INCOMPLETE: NTSTATUS = NTSTATUS(0xC01A0004_u32 as _);
pub const STATUS_LOG_BLOCK_INVALID: NTSTATUS = NTSTATUS(0xC01A000A_u32 as _);
pub const STATUS_LOG_BLOCK_VERSION: NTSTATUS = NTSTATUS(0xC01A0009_u32 as _);
pub const STATUS_LOG_CANT_DELETE: NTSTATUS = NTSTATUS(0xC01A0011_u32 as _);
pub const STATUS_LOG_CLIENT_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(0xC01A0024_u32 as _);
pub const STATUS_LOG_CLIENT_NOT_REGISTERED: NTSTATUS = NTSTATUS(0xC01A0025_u32 as _);
pub const STATUS_LOG_CONTAINER_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC01A0012_u32 as _);
pub const STATUS_LOG_CONTAINER_OPEN_FAILED: NTSTATUS = NTSTATUS(0xC01A0029_u32 as _);
pub const STATUS_LOG_CONTAINER_READ_FAILED: NTSTATUS = NTSTATUS(0xC01A0027_u32 as _);
pub const STATUS_LOG_CONTAINER_STATE_INVALID: NTSTATUS = NTSTATUS(0xC01A002A_u32 as _);
pub const STATUS_LOG_CONTAINER_WRITE_FAILED: NTSTATUS = NTSTATUS(0xC01A0028_u32 as _);
pub const STATUS_LOG_CORRUPTION_DETECTED: NTSTATUS = NTSTATUS(0xC0190030_u32 as _);
pub const STATUS_LOG_DEDICATED: NTSTATUS = NTSTATUS(0xC01A001F_u32 as _);
pub const STATUS_LOG_EPHEMERAL: NTSTATUS = NTSTATUS(0xC01A0022_u32 as _);
pub const STATUS_LOG_FILE_FULL: NTSTATUS = NTSTATUS(0xC0000188_u32 as _);
pub const STATUS_LOG_FULL: NTSTATUS = NTSTATUS(0xC01A001D_u32 as _);
pub const STATUS_LOG_FULL_HANDLER_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC01A0026_u32 as _);
pub const STATUS_LOG_GROWTH_FAILED: NTSTATUS = NTSTATUS(0xC0190019_u32 as _);
pub const STATUS_LOG_HARD_ERROR: NTSTATUS = NTSTATUS(0x4000001A_u32 as _);
pub const STATUS_LOG_INCONSISTENT_SECURITY: NTSTATUS = NTSTATUS(0xC01A002E_u32 as _);
pub const STATUS_LOG_INVALID_RANGE: NTSTATUS = NTSTATUS(0xC01A0005_u32 as _);
pub const STATUS_LOG_METADATA_CORRUPT: NTSTATUS = NTSTATUS(0xC01A000D_u32 as _);
pub const STATUS_LOG_METADATA_FLUSH_FAILED: NTSTATUS = NTSTATUS(0xC01A002D_u32 as _);
pub const STATUS_LOG_METADATA_INCONSISTENT: NTSTATUS = NTSTATUS(0xC01A000F_u32 as _);
pub const STATUS_LOG_METADATA_INVALID: NTSTATUS = NTSTATUS(0xC01A000E_u32 as _);
pub const STATUS_LOG_MULTIPLEXED: NTSTATUS = NTSTATUS(0xC01A001E_u32 as _);
pub const STATUS_LOG_NOT_ENOUGH_CONTAINERS: NTSTATUS = NTSTATUS(0xC01A0023_u32 as _);
pub const STATUS_LOG_NO_RESTART: NTSTATUS = NTSTATUS(0x401A000C_u32 as _);
pub const STATUS_LOG_PINNED: NTSTATUS = NTSTATUS(0xC01A002C_u32 as _);
pub const STATUS_LOG_PINNED_ARCHIVE_TAIL: NTSTATUS = NTSTATUS(0xC01A0018_u32 as _);
pub const STATUS_LOG_PINNED_RESERVATION: NTSTATUS = NTSTATUS(0xC01A0030_u32 as _);
pub const STATUS_LOG_POLICY_ALREADY_INSTALLED: NTSTATUS = NTSTATUS(0xC01A0014_u32 as _);
pub const STATUS_LOG_POLICY_CONFLICT: NTSTATUS = NTSTATUS(0xC01A0017_u32 as _);
pub const STATUS_LOG_POLICY_INVALID: NTSTATUS = NTSTATUS(0xC01A0016_u32 as _);
pub const STATUS_LOG_POLICY_NOT_INSTALLED: NTSTATUS = NTSTATUS(0xC01A0015_u32 as _);
pub const STATUS_LOG_READ_CONTEXT_INVALID: NTSTATUS = NTSTATUS(0xC01A0007_u32 as _);
pub const STATUS_LOG_READ_MODE_INVALID: NTSTATUS = NTSTATUS(0xC01A000B_u32 as _);
pub const STATUS_LOG_RECORDS_RESERVED_INVALID: NTSTATUS = NTSTATUS(0xC01A001A_u32 as _);
pub const STATUS_LOG_RECORD_NONEXISTENT: NTSTATUS = NTSTATUS(0xC01A0019_u32 as _);
pub const STATUS_LOG_RESERVATION_INVALID: NTSTATUS = NTSTATUS(0xC01A0010_u32 as _);
pub const STATUS_LOG_RESIZE_INVALID_SIZE: NTSTATUS = NTSTATUS(0xC019000B_u32 as _);
pub const STATUS_LOG_RESTART_INVALID: NTSTATUS = NTSTATUS(0xC01A0008_u32 as _);
pub const STATUS_LOG_SECTOR_INVALID: NTSTATUS = NTSTATUS(0xC01A0001_u32 as _);
pub const STATUS_LOG_SECTOR_PARITY_INVALID: NTSTATUS = NTSTATUS(0xC01A0002_u32 as _);
pub const STATUS_LOG_SECTOR_REMAPPED: NTSTATUS = NTSTATUS(0xC01A0003_u32 as _);
pub const STATUS_LOG_SPACE_RESERVED_INVALID: NTSTATUS = NTSTATUS(0xC01A001B_u32 as _);
pub const STATUS_LOG_START_OF_LOG: NTSTATUS = NTSTATUS(0xC01A0013_u32 as _);
pub const STATUS_LOG_STATE_INVALID: NTSTATUS = NTSTATUS(0xC01A002B_u32 as _);
pub const STATUS_LOG_TAIL_INVALID: NTSTATUS = NTSTATUS(0xC01A001C_u32 as _);
pub const STATUS_LONGJUMP: NTSTATUS = NTSTATUS(0x80000026_u32 as _);
pub const STATUS_LOST_MODE_LOGON_RESTRICTION: NTSTATUS = NTSTATUS(0xC000030D_u32 as _);
pub const STATUS_LOST_WRITEBEHIND_DATA: NTSTATUS = NTSTATUS(0xC0000222_u32 as _);
pub const STATUS_LOST_WRITEBEHIND_DATA_LOCAL_DISK_ERROR: NTSTATUS = NTSTATUS(0xC000A082_u32 as _);
pub const STATUS_LOST_WRITEBEHIND_DATA_NETWORK_DISCONNECTED: NTSTATUS = NTSTATUS(0xC000A080_u32 as _);
pub const STATUS_LOST_WRITEBEHIND_DATA_NETWORK_SERVER_ERROR: NTSTATUS = NTSTATUS(0xC000A081_u32 as _);
pub const STATUS_LPAC_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC000A203_u32 as _);
pub const STATUS_LPC_HANDLE_COUNT_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000722_u32 as _);
pub const STATUS_LPC_INVALID_CONNECTION_USAGE: NTSTATUS = NTSTATUS(0xC0000706_u32 as _);
pub const STATUS_LPC_RECEIVE_BUFFER_EXPECTED: NTSTATUS = NTSTATUS(0xC0000705_u32 as _);
pub const STATUS_LPC_REPLY_LOST: NTSTATUS = NTSTATUS(0xC0000253_u32 as _);
pub const STATUS_LPC_REQUESTS_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0000707_u32 as _);
pub const STATUS_LUIDS_EXHAUSTED: NTSTATUS = NTSTATUS(0xC0000075_u32 as _);
pub const STATUS_MAGAZINE_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC0000286_u32 as _);
pub const STATUS_MAPPED_ALIGNMENT: NTSTATUS = NTSTATUS(0xC0000220_u32 as _);
pub const STATUS_MAPPED_FILE_SIZE_ZERO: NTSTATUS = NTSTATUS(0xC000011E_u32 as _);
pub const STATUS_MARKED_TO_DISALLOW_WRITES: NTSTATUS = NTSTATUS(0xC000048D_u32 as _);
pub const STATUS_MARSHALL_OVERFLOW: NTSTATUS = NTSTATUS(0xC0000231_u32 as _);
pub const STATUS_MAX_REFERRALS_EXCEEDED: NTSTATUS = NTSTATUS(0xC00002F4_u32 as _);
pub const STATUS_MCA_EXCEPTION: NTSTATUS = NTSTATUS(0xC0000713_u32 as _);
pub const STATUS_MCA_OCCURED: NTSTATUS = NTSTATUS(0xC000036A_u32 as _);
pub const STATUS_MEDIA_CHANGED: NTSTATUS = NTSTATUS(0x8000001C_u32 as _);
pub const STATUS_MEDIA_CHECK: NTSTATUS = NTSTATUS(0x80000020_u32 as _);
pub const STATUS_MEDIA_WRITE_PROTECTED: NTSTATUS = NTSTATUS(0xC00000A2_u32 as _);
pub const STATUS_MEMBERS_PRIMARY_GROUP: NTSTATUS = NTSTATUS(0xC0000127_u32 as _);
pub const STATUS_MEMBER_IN_ALIAS: NTSTATUS = NTSTATUS(0xC0000153_u32 as _);
pub const STATUS_MEMBER_IN_GROUP: NTSTATUS = NTSTATUS(0xC0000067_u32 as _);
pub const STATUS_MEMBER_NOT_IN_ALIAS: NTSTATUS = NTSTATUS(0xC0000152_u32 as _);
pub const STATUS_MEMBER_NOT_IN_GROUP: NTSTATUS = NTSTATUS(0xC0000068_u32 as _);
pub const STATUS_MEMORY_NOT_ALLOCATED: NTSTATUS = NTSTATUS(0xC00000A0_u32 as _);
pub const STATUS_MESSAGE_LOST: NTSTATUS = NTSTATUS(0xC0000701_u32 as _);
pub const STATUS_MESSAGE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000109_u32 as _);
pub const STATUS_MESSAGE_RETRIEVED: NTSTATUS = NTSTATUS(0x4000002E_u32 as _);
pub const STATUS_MFT_TOO_FRAGMENTED: NTSTATUS = NTSTATUS(0xC0000304_u32 as _);
pub const STATUS_MINIVERSION_INACCESSIBLE_FROM_SPECIFIED_TRANSACTION: NTSTATUS = NTSTATUS(0xC0190024_u32 as _);
pub const STATUS_MISSING_SYSTEMFILE: NTSTATUS = NTSTATUS(0xC0000143_u32 as _);
pub const STATUS_MONITOR_INVALID_DESCRIPTOR_CHECKSUM: NTSTATUS = NTSTATUS(0xC01D0003_u32 as _);
pub const STATUS_MONITOR_INVALID_DETAILED_TIMING_BLOCK: NTSTATUS = NTSTATUS(0xC01D0009_u32 as _);
pub const STATUS_MONITOR_INVALID_MANUFACTURE_DATE: NTSTATUS = NTSTATUS(0xC01D000A_u32 as _);
pub const STATUS_MONITOR_INVALID_SERIAL_NUMBER_MONDSC_BLOCK: NTSTATUS = NTSTATUS(0xC01D0006_u32 as _);
pub const STATUS_MONITOR_INVALID_STANDARD_TIMING_BLOCK: NTSTATUS = NTSTATUS(0xC01D0004_u32 as _);
pub const STATUS_MONITOR_INVALID_USER_FRIENDLY_MONDSC_BLOCK: NTSTATUS = NTSTATUS(0xC01D0007_u32 as _);
pub const STATUS_MONITOR_NO_DESCRIPTOR: NTSTATUS = NTSTATUS(0xC01D0001_u32 as _);
pub const STATUS_MONITOR_NO_MORE_DESCRIPTOR_DATA: NTSTATUS = NTSTATUS(0xC01D0008_u32 as _);
pub const STATUS_MONITOR_UNKNOWN_DESCRIPTOR_FORMAT: NTSTATUS = NTSTATUS(0xC01D0002_u32 as _);
pub const STATUS_MONITOR_WMI_DATABLOCK_REGISTRATION_FAILED: NTSTATUS = NTSTATUS(0xC01D0005_u32 as _);
pub const STATUS_MORE_ENTRIES: NTSTATUS = NTSTATUS(0x105_u32 as _);
pub const STATUS_MORE_PROCESSING_REQUIRED: NTSTATUS = NTSTATUS(0xC0000016_u32 as _);
pub const STATUS_MOUNT_POINT_NOT_RESOLVED: NTSTATUS = NTSTATUS(0xC0000368_u32 as _);
pub const STATUS_MP_PROCESSOR_MISMATCH: NTSTATUS = NTSTATUS(0x40000029_u32 as _);
pub const STATUS_MUI_FILE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC00B0001_u32 as _);
pub const STATUS_MUI_FILE_NOT_LOADED: NTSTATUS = NTSTATUS(0xC00B0006_u32 as _);
pub const STATUS_MUI_INVALID_FILE: NTSTATUS = NTSTATUS(0xC00B0002_u32 as _);
pub const STATUS_MUI_INVALID_LOCALE_NAME: NTSTATUS = NTSTATUS(0xC00B0004_u32 as _);
pub const STATUS_MUI_INVALID_RC_CONFIG: NTSTATUS = NTSTATUS(0xC00B0003_u32 as _);
pub const STATUS_MUI_INVALID_ULTIMATEFALLBACK_NAME: NTSTATUS = NTSTATUS(0xC00B0005_u32 as _);
pub const STATUS_MULTIPLE_FAULT_VIOLATION: NTSTATUS = NTSTATUS(0xC00002E8_u32 as _);
pub const STATUS_MUST_BE_KDC: NTSTATUS = NTSTATUS(0xC00002F5_u32 as _);
pub const STATUS_MUTANT_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000191_u32 as _);
pub const STATUS_MUTANT_NOT_OWNED: NTSTATUS = NTSTATUS(0xC0000046_u32 as _);
pub const STATUS_MUTUAL_AUTHENTICATION_FAILED: NTSTATUS = NTSTATUS(0xC00002C3_u32 as _);
pub const STATUS_NAME_TOO_LONG: NTSTATUS = NTSTATUS(0xC0000106_u32 as _);
pub const STATUS_NDIS_ADAPTER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0230006_u32 as _);
pub const STATUS_NDIS_ADAPTER_NOT_READY: NTSTATUS = NTSTATUS(0xC0230011_u32 as _);
pub const STATUS_NDIS_ADAPTER_REMOVED: NTSTATUS = NTSTATUS(0xC0230018_u32 as _);
pub const STATUS_NDIS_ALREADY_MAPPED: NTSTATUS = NTSTATUS(0xC023001D_u32 as _);
pub const STATUS_NDIS_BAD_CHARACTERISTICS: NTSTATUS = NTSTATUS(0xC0230005_u32 as _);
pub const STATUS_NDIS_BAD_VERSION: NTSTATUS = NTSTATUS(0xC0230004_u32 as _);
pub const STATUS_NDIS_BUFFER_TOO_SHORT: NTSTATUS = NTSTATUS(0xC0230016_u32 as _);
pub const STATUS_NDIS_CLOSING: NTSTATUS = NTSTATUS(0xC0230002_u32 as _);
pub const STATUS_NDIS_DEVICE_FAILED: NTSTATUS = NTSTATUS(0xC0230008_u32 as _);
pub const STATUS_NDIS_DOT11_AP_BAND_CURRENTLY_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC0232006_u32 as _);
pub const STATUS_NDIS_DOT11_AP_BAND_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0232008_u32 as _);
pub const STATUS_NDIS_DOT11_AP_CHANNEL_CURRENTLY_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC0232005_u32 as _);
pub const STATUS_NDIS_DOT11_AP_CHANNEL_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC0232007_u32 as _);
pub const STATUS_NDIS_DOT11_AUTO_CONFIG_ENABLED: NTSTATUS = NTSTATUS(0xC0232000_u32 as _);
pub const STATUS_NDIS_DOT11_MEDIA_IN_USE: NTSTATUS = NTSTATUS(0xC0232001_u32 as _);
pub const STATUS_NDIS_DOT11_POWER_STATE_INVALID: NTSTATUS = NTSTATUS(0xC0232002_u32 as _);
pub const STATUS_NDIS_ERROR_READING_FILE: NTSTATUS = NTSTATUS(0xC023001C_u32 as _);
pub const STATUS_NDIS_FILE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC023001B_u32 as _);
pub const STATUS_NDIS_GROUP_ADDRESS_IN_USE: NTSTATUS = NTSTATUS(0xC023001A_u32 as _);
pub const STATUS_NDIS_INDICATION_REQUIRED: NTSTATUS = NTSTATUS(0x40230001_u32 as _);
pub const STATUS_NDIS_INTERFACE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC023002B_u32 as _);
pub const STATUS_NDIS_INVALID_ADDRESS: NTSTATUS = NTSTATUS(0xC0230022_u32 as _);
pub const STATUS_NDIS_INVALID_DATA: NTSTATUS = NTSTATUS(0xC0230015_u32 as _);
pub const STATUS_NDIS_INVALID_DEVICE_REQUEST: NTSTATUS = NTSTATUS(0xC0230010_u32 as _);
pub const STATUS_NDIS_INVALID_LENGTH: NTSTATUS = NTSTATUS(0xC0230014_u32 as _);
pub const STATUS_NDIS_INVALID_OID: NTSTATUS = NTSTATUS(0xC0230017_u32 as _);
pub const STATUS_NDIS_INVALID_PACKET: NTSTATUS = NTSTATUS(0xC023000F_u32 as _);
pub const STATUS_NDIS_INVALID_PORT: NTSTATUS = NTSTATUS(0xC023002D_u32 as _);
pub const STATUS_NDIS_INVALID_PORT_STATE: NTSTATUS = NTSTATUS(0xC023002E_u32 as _);
pub const STATUS_NDIS_LOW_POWER_STATE: NTSTATUS = NTSTATUS(0xC023002F_u32 as _);
pub const STATUS_NDIS_MEDIA_DISCONNECTED: NTSTATUS = NTSTATUS(0xC023001F_u32 as _);
pub const STATUS_NDIS_MULTICAST_EXISTS: NTSTATUS = NTSTATUS(0xC023000A_u32 as _);
pub const STATUS_NDIS_MULTICAST_FULL: NTSTATUS = NTSTATUS(0xC0230009_u32 as _);
pub const STATUS_NDIS_MULTICAST_NOT_FOUND: NTSTATUS = NTSTATUS(0xC023000B_u32 as _);
pub const STATUS_NDIS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC02300BB_u32 as _);
pub const STATUS_NDIS_NO_QUEUES: NTSTATUS = NTSTATUS(0xC0230031_u32 as _);
pub const STATUS_NDIS_OFFLOAD_CONNECTION_REJECTED: NTSTATUS = NTSTATUS(0xC0231012_u32 as _);
pub const STATUS_NDIS_OFFLOAD_PATH_REJECTED: NTSTATUS = NTSTATUS(0xC0231013_u32 as _);
pub const STATUS_NDIS_OFFLOAD_POLICY: NTSTATUS = NTSTATUS(0xC023100F_u32 as _);
pub const STATUS_NDIS_OPEN_FAILED: NTSTATUS = NTSTATUS(0xC0230007_u32 as _);
pub const STATUS_NDIS_PAUSED: NTSTATUS = NTSTATUS(0xC023002A_u32 as _);
pub const STATUS_NDIS_PM_PROTOCOL_OFFLOAD_LIST_FULL: NTSTATUS = NTSTATUS(0xC0232004_u32 as _);
pub const STATUS_NDIS_PM_WOL_PATTERN_LIST_FULL: NTSTATUS = NTSTATUS(0xC0232003_u32 as _);
pub const STATUS_NDIS_REINIT_REQUIRED: NTSTATUS = NTSTATUS(0xC0230030_u32 as _);
pub const STATUS_NDIS_REQUEST_ABORTED: NTSTATUS = NTSTATUS(0xC023000C_u32 as _);
pub const STATUS_NDIS_RESET_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC023000D_u32 as _);
pub const STATUS_NDIS_RESOURCE_CONFLICT: NTSTATUS = NTSTATUS(0xC023001E_u32 as _);
pub const STATUS_NDIS_UNSUPPORTED_MEDIA: NTSTATUS = NTSTATUS(0xC0230019_u32 as _);
pub const STATUS_NDIS_UNSUPPORTED_REVISION: NTSTATUS = NTSTATUS(0xC023002C_u32 as _);
pub const STATUS_ND_QUEUE_OVERFLOW: NTSTATUS = NTSTATUS(0xC000A011_u32 as _);
pub const STATUS_NEEDS_REGISTRATION: NTSTATUS = NTSTATUS(0xC0000489_u32 as _);
pub const STATUS_NEEDS_REMEDIATION: NTSTATUS = NTSTATUS(0xC0000462_u32 as _);
pub const STATUS_NETLOGON_NOT_STARTED: NTSTATUS = NTSTATUS(0xC0000192_u32 as _);
pub const STATUS_NETWORK_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC00000CA_u32 as _);
pub const STATUS_NETWORK_ACCESS_DENIED_EDP: NTSTATUS = NTSTATUS(0xC000048E_u32 as _);
pub const STATUS_NETWORK_AUTHENTICATION_PROMPT_CANCELED: NTSTATUS = NTSTATUS(0xC05D0004_u32 as _);
pub const STATUS_NETWORK_BUSY: NTSTATUS = NTSTATUS(0xC00000BF_u32 as _);
pub const STATUS_NETWORK_CREDENTIAL_CONFLICT: NTSTATUS = NTSTATUS(0xC0000195_u32 as _);
pub const STATUS_NETWORK_NAME_DELETED: NTSTATUS = NTSTATUS(0xC00000C9_u32 as _);
pub const STATUS_NETWORK_OPEN_RESTRICTION: NTSTATUS = NTSTATUS(0xC0000201_u32 as _);
pub const STATUS_NETWORK_SESSION_EXPIRED: NTSTATUS = NTSTATUS(0xC000035C_u32 as _);
pub const STATUS_NETWORK_UNREACHABLE: NTSTATUS = NTSTATUS(0xC000023C_u32 as _);
pub const STATUS_NET_WRITE_FAULT: NTSTATUS = NTSTATUS(0xC00000D2_u32 as _);
pub const STATUS_NOINTERFACE: NTSTATUS = NTSTATUS(0xC00002B9_u32 as _);
pub const STATUS_NOLOGON_INTERDOMAIN_TRUST_ACCOUNT: NTSTATUS = NTSTATUS(0xC0000198_u32 as _);
pub const STATUS_NOLOGON_SERVER_TRUST_ACCOUNT: NTSTATUS = NTSTATUS(0xC000019A_u32 as _);
pub const STATUS_NOLOGON_WORKSTATION_TRUST_ACCOUNT: NTSTATUS = NTSTATUS(0xC0000199_u32 as _);
pub const STATUS_NONCONTINUABLE_EXCEPTION: NTSTATUS = NTSTATUS(0xC0000025_u32 as _);
pub const STATUS_NONEXISTENT_EA_ENTRY: NTSTATUS = NTSTATUS(0xC0000051_u32 as _);
pub const STATUS_NONEXISTENT_SECTOR: NTSTATUS = NTSTATUS(0xC0000015_u32 as _);
pub const STATUS_NONE_MAPPED: NTSTATUS = NTSTATUS(0xC0000073_u32 as _);
pub const STATUS_NOTHING_TO_TERMINATE: NTSTATUS = NTSTATUS(0x122_u32 as _);
pub const STATUS_NOTIFICATION_GUID_ALREADY_DEFINED: NTSTATUS = NTSTATUS(0xC00001A4_u32 as _);
pub const STATUS_NOTIFY_CLEANUP: NTSTATUS = NTSTATUS(0x10B_u32 as _);
pub const STATUS_NOTIFY_ENUM_DIR: NTSTATUS = NTSTATUS(0x10C_u32 as _);
pub const STATUS_NOT_ALLOWED_ON_SYSTEM_FILE: NTSTATUS = NTSTATUS(0xC00001A7_u32 as _);
pub const STATUS_NOT_ALL_ASSIGNED: NTSTATUS = NTSTATUS(0x106_u32 as _);
pub const STATUS_NOT_APPCONTAINER: NTSTATUS = NTSTATUS(0xC000A200_u32 as _);
pub const STATUS_NOT_A_CLOUD_FILE: NTSTATUS = NTSTATUS(0xC000CF07_u32 as _);
pub const STATUS_NOT_A_CLOUD_SYNC_ROOT: NTSTATUS = NTSTATUS(0xC000CF1E_u32 as _);
pub const STATUS_NOT_A_DAX_VOLUME: NTSTATUS = NTSTATUS(0xC00004B1_u32 as _);
pub const STATUS_NOT_A_DEV_VOLUME: NTSTATUS = NTSTATUS(0xC00004DD_u32 as _);
pub const STATUS_NOT_A_DIRECTORY: NTSTATUS = NTSTATUS(0xC0000103_u32 as _);
pub const STATUS_NOT_A_REPARSE_POINT: NTSTATUS = NTSTATUS(0xC0000275_u32 as _);
pub const STATUS_NOT_A_TIERED_VOLUME: NTSTATUS = NTSTATUS(0xC000050D_u32 as _);
pub const STATUS_NOT_CAPABLE: NTSTATUS = NTSTATUS(0xC0000429_u32 as _);
pub const STATUS_NOT_CLIENT_SESSION: NTSTATUS = NTSTATUS(0xC0000217_u32 as _);
pub const STATUS_NOT_COMMITTED: NTSTATUS = NTSTATUS(0xC000002D_u32 as _);
pub const STATUS_NOT_DAX_MAPPABLE: NTSTATUS = NTSTATUS(0xC00004B2_u32 as _);
pub const STATUS_NOT_EXPORT_FORMAT: NTSTATUS = NTSTATUS(0xC0000292_u32 as _);
pub const STATUS_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000225_u32 as _);
pub const STATUS_NOT_GUI_PROCESS: NTSTATUS = NTSTATUS(0xC0000506_u32 as _);
pub const STATUS_NOT_IMPLEMENTED: NTSTATUS = NTSTATUS(0xC0000002_u32 as _);
pub const STATUS_NOT_LOCKED: NTSTATUS = NTSTATUS(0xC000002A_u32 as _);
pub const STATUS_NOT_LOGON_PROCESS: NTSTATUS = NTSTATUS(0xC00000ED_u32 as _);
pub const STATUS_NOT_MAPPED_DATA: NTSTATUS = NTSTATUS(0xC0000088_u32 as _);
pub const STATUS_NOT_MAPPED_VIEW: NTSTATUS = NTSTATUS(0xC0000019_u32 as _);
pub const STATUS_NOT_READ_FROM_COPY: NTSTATUS = NTSTATUS(0xC000046A_u32 as _);
pub const STATUS_NOT_REDUNDANT_STORAGE: NTSTATUS = NTSTATUS(0xC0000479_u32 as _);
pub const STATUS_NOT_REGISTRY_FILE: NTSTATUS = NTSTATUS(0xC000015C_u32 as _);
pub const STATUS_NOT_SAFE_MODE_DRIVER: NTSTATUS = NTSTATUS(0xC000035F_u32 as _);
pub const STATUS_NOT_SAME_DEVICE: NTSTATUS = NTSTATUS(0xC00000D4_u32 as _);
pub const STATUS_NOT_SAME_OBJECT: NTSTATUS = NTSTATUS(0xC00001AC_u32 as _);
pub const STATUS_NOT_SERVER_SESSION: NTSTATUS = NTSTATUS(0xC0000216_u32 as _);
pub const STATUS_NOT_SNAPSHOT_VOLUME: NTSTATUS = NTSTATUS(0xC0190047_u32 as _);
pub const STATUS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00000BB_u32 as _);
pub const STATUS_NOT_SUPPORTED_IN_APPCONTAINER: NTSTATUS = NTSTATUS(0xC000A201_u32 as _);
pub const STATUS_NOT_SUPPORTED_ON_DAX: NTSTATUS = NTSTATUS(0xC000049A_u32 as _);
pub const STATUS_NOT_SUPPORTED_ON_SBS: NTSTATUS = NTSTATUS(0xC0000300_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_AUDITING: NTSTATUS = NTSTATUS(0xC00004CD_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_BTT: NTSTATUS = NTSTATUS(0xC00004B5_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_BYPASSIO: NTSTATUS = NTSTATUS(0xC00004C7_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_CACHED_HANDLE: NTSTATUS = NTSTATUS(0xC00004D5_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_COMPRESSION: NTSTATUS = NTSTATUS(0xC00004CA_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_DEDUPLICATION: NTSTATUS = NTSTATUS(0xC00004CC_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_ENCRYPTION: NTSTATUS = NTSTATUS(0xC00004C9_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_MONITORING: NTSTATUS = NTSTATUS(0xC00004CE_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_REPLICATION: NTSTATUS = NTSTATUS(0xC00004CB_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_SNAPSHOT: NTSTATUS = NTSTATUS(0xC00004CF_u32 as _);
pub const STATUS_NOT_SUPPORTED_WITH_VIRTUALIZATION: NTSTATUS = NTSTATUS(0xC00004D0_u32 as _);
pub const STATUS_NOT_TINY_STREAM: NTSTATUS = NTSTATUS(0xC0000226_u32 as _);
pub const STATUS_NO_ACE_CONDITION: NTSTATUS = NTSTATUS(0x8000002F_u32 as _);
pub const STATUS_NO_APPLICABLE_APP_LICENSES_FOUND: NTSTATUS = NTSTATUS(0xC0EA0001_u32 as _);
pub const STATUS_NO_APPLICATION_PACKAGE: NTSTATUS = NTSTATUS(0xC00001AA_u32 as _);
pub const STATUS_NO_BROWSER_SERVERS_FOUND: NTSTATUS = NTSTATUS(0xC000021C_u32 as _);
pub const STATUS_NO_BYPASSIO_DRIVER_SUPPORT: NTSTATUS = NTSTATUS(0xC00004C8_u32 as _);
pub const STATUS_NO_CALLBACK_ACTIVE: NTSTATUS = NTSTATUS(0xC0000258_u32 as _);
pub const STATUS_NO_DATA_DETECTED: NTSTATUS = NTSTATUS(0x80000022_u32 as _);
pub const STATUS_NO_EAS_ON_FILE: NTSTATUS = NTSTATUS(0xC0000052_u32 as _);
pub const STATUS_NO_EFS: NTSTATUS = NTSTATUS(0xC000028E_u32 as _);
pub const STATUS_NO_EVENT_PAIR: NTSTATUS = NTSTATUS(0xC000014E_u32 as _);
pub const STATUS_NO_GUID_TRANSLATION: NTSTATUS = NTSTATUS(0xC000010C_u32 as _);
pub const STATUS_NO_IMPERSONATION_TOKEN: NTSTATUS = NTSTATUS(0xC000005C_u32 as _);
pub const STATUS_NO_INHERITANCE: NTSTATUS = NTSTATUS(0x8000000B_u32 as _);
pub const STATUS_NO_IP_ADDRESSES: NTSTATUS = NTSTATUS(0xC00002F1_u32 as _);
pub const STATUS_NO_KERB_KEY: NTSTATUS = NTSTATUS(0xC0000322_u32 as _);
pub const STATUS_NO_KEY: NTSTATUS = NTSTATUS(0xC000090C_u32 as _);
pub const STATUS_NO_LDT: NTSTATUS = NTSTATUS(0xC0000117_u32 as _);
pub const STATUS_NO_LINK_TRACKING_IN_TRANSACTION: NTSTATUS = NTSTATUS(0xC0190059_u32 as _);
pub const STATUS_NO_LOGON_SERVERS: NTSTATUS = NTSTATUS(0xC000005E_u32 as _);
pub const STATUS_NO_LOG_SPACE: NTSTATUS = NTSTATUS(0xC000017D_u32 as _);
pub const STATUS_NO_MATCH: NTSTATUS = NTSTATUS(0xC0000272_u32 as _);
pub const STATUS_NO_MEDIA: NTSTATUS = NTSTATUS(0xC0000178_u32 as _);
pub const STATUS_NO_MEDIA_IN_DEVICE: NTSTATUS = NTSTATUS(0xC0000013_u32 as _);
pub const STATUS_NO_MEMORY: NTSTATUS = NTSTATUS(0xC0000017_u32 as _);
pub const STATUS_NO_MORE_EAS: NTSTATUS = NTSTATUS(0x80000012_u32 as _);
pub const STATUS_NO_MORE_ENTRIES: NTSTATUS = NTSTATUS(0x8000001A_u32 as _);
pub const STATUS_NO_MORE_FILES: NTSTATUS = NTSTATUS(0x80000006_u32 as _);
pub const STATUS_NO_MORE_MATCHES: NTSTATUS = NTSTATUS(0xC0000273_u32 as _);
pub const STATUS_NO_PAGEFILE: NTSTATUS = NTSTATUS(0xC0000147_u32 as _);
pub const STATUS_NO_PA_DATA: NTSTATUS = NTSTATUS(0xC00002F8_u32 as _);
pub const STATUS_NO_PHYSICALLY_ALIGNED_FREE_SPACE_FOUND: NTSTATUS = NTSTATUS(0xC00004A5_u32 as _);
pub const STATUS_NO_QUOTAS_FOR_ACCOUNT: NTSTATUS = NTSTATUS(0x10D_u32 as _);
pub const STATUS_NO_RANGES_PROCESSED: NTSTATUS = NTSTATUS(0xC0000460_u32 as _);
pub const STATUS_NO_RECOVERY_POLICY: NTSTATUS = NTSTATUS(0xC000028D_u32 as _);
pub const STATUS_NO_S4U_PROT_SUPPORT: NTSTATUS = NTSTATUS(0xC000040A_u32 as _);
pub const STATUS_NO_SAVEPOINT_WITH_OPEN_FILES: NTSTATUS = NTSTATUS(0xC0190048_u32 as _);
pub const STATUS_NO_SECRETS: NTSTATUS = NTSTATUS(0xC0000371_u32 as _);
pub const STATUS_NO_SECURITY_CONTEXT: NTSTATUS = NTSTATUS(0xC000042D_u32 as _);
pub const STATUS_NO_SECURITY_ON_OBJECT: NTSTATUS = NTSTATUS(0xC00000D7_u32 as _);
pub const STATUS_NO_SPOOL_SPACE: NTSTATUS = NTSTATUS(0xC00000C7_u32 as _);
pub const STATUS_NO_SUCH_ALIAS: NTSTATUS = NTSTATUS(0xC0000151_u32 as _);
pub const STATUS_NO_SUCH_DEVICE: NTSTATUS = NTSTATUS(0xC000000E_u32 as _);
pub const STATUS_NO_SUCH_DOMAIN: NTSTATUS = NTSTATUS(0xC00000DF_u32 as _);
pub const STATUS_NO_SUCH_FILE: NTSTATUS = NTSTATUS(0xC000000F_u32 as _);
pub const STATUS_NO_SUCH_GROUP: NTSTATUS = NTSTATUS(0xC0000066_u32 as _);
pub const STATUS_NO_SUCH_MEMBER: NTSTATUS = NTSTATUS(0xC000017A_u32 as _);
pub const STATUS_NO_SUCH_PACKAGE: NTSTATUS = NTSTATUS(0xC00000FE_u32 as _);
pub const STATUS_NO_SUCH_PRIVILEGE: NTSTATUS = NTSTATUS(0xC0000060_u32 as _);
pub const STATUS_NO_TGT_REPLY: NTSTATUS = NTSTATUS(0xC00002EF_u32 as _);
pub const STATUS_NO_TOKEN: NTSTATUS = NTSTATUS(0xC000007C_u32 as _);
pub const STATUS_NO_TRACKING_SERVICE: NTSTATUS = NTSTATUS(0xC000029F_u32 as _);
pub const STATUS_NO_TRUST_LSA_SECRET: NTSTATUS = NTSTATUS(0xC000018A_u32 as _);
pub const STATUS_NO_TRUST_SAM_ACCOUNT: NTSTATUS = NTSTATUS(0xC000018B_u32 as _);
pub const STATUS_NO_TXF_METADATA: NTSTATUS = NTSTATUS(0x80190029_u32 as _);
pub const STATUS_NO_UNICODE_TRANSLATION: NTSTATUS = NTSTATUS(0xC0000717_u32 as _);
pub const STATUS_NO_USER_KEYS: NTSTATUS = NTSTATUS(0xC0000290_u32 as _);
pub const STATUS_NO_USER_SESSION_KEY: NTSTATUS = NTSTATUS(0xC0000202_u32 as _);
pub const STATUS_NO_WORK_DONE: NTSTATUS = NTSTATUS(0x80000032_u32 as _);
pub const STATUS_NO_YIELD_PERFORMED: NTSTATUS = NTSTATUS(0x40000024_u32 as _);
pub const STATUS_NTLM_BLOCKED: NTSTATUS = NTSTATUS(0xC0000418_u32 as _);
pub const STATUS_NT_CROSS_ENCRYPTION_REQUIRED: NTSTATUS = NTSTATUS(0xC000015D_u32 as _);
pub const STATUS_NULL_LM_PASSWORD: NTSTATUS = NTSTATUS(0x4000000D_u32 as _);
pub const STATUS_OBJECTID_EXISTS: NTSTATUS = NTSTATUS(0xC000022B_u32 as _);
pub const STATUS_OBJECTID_NOT_FOUND: NTSTATUS = NTSTATUS(0xC00002F0_u32 as _);
pub const STATUS_OBJECT_IS_IMMUTABLE: NTSTATUS = NTSTATUS(0xC00004BE_u32 as _);
pub const STATUS_OBJECT_NAME_COLLISION: NTSTATUS = NTSTATUS(0xC0000035_u32 as _);
pub const STATUS_OBJECT_NAME_EXISTS: NTSTATUS = NTSTATUS(0x40000000_u32 as _);
pub const STATUS_OBJECT_NAME_INVALID: NTSTATUS = NTSTATUS(0xC0000033_u32 as _);
pub const STATUS_OBJECT_NAME_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000034_u32 as _);
pub const STATUS_OBJECT_NOT_EXTERNALLY_BACKED: NTSTATUS = NTSTATUS(0xC000046D_u32 as _);
pub const STATUS_OBJECT_NO_LONGER_EXISTS: NTSTATUS = NTSTATUS(0xC0190021_u32 as _);
pub const STATUS_OBJECT_PATH_INVALID: NTSTATUS = NTSTATUS(0xC0000039_u32 as _);
pub const STATUS_OBJECT_PATH_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000003A_u32 as _);
pub const STATUS_OBJECT_PATH_SYNTAX_BAD: NTSTATUS = NTSTATUS(0xC000003B_u32 as _);
pub const STATUS_OBJECT_TYPE_MISMATCH: NTSTATUS = NTSTATUS(0xC0000024_u32 as _);
pub const STATUS_OFFLOAD_READ_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A2A3_u32 as _);
pub const STATUS_OFFLOAD_READ_FLT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A2A1_u32 as _);
pub const STATUS_OFFLOAD_WRITE_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A2A4_u32 as _);
pub const STATUS_OFFLOAD_WRITE_FLT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A2A2_u32 as _);
pub const STATUS_ONLY_IF_CONNECTED: NTSTATUS = NTSTATUS(0xC00002CC_u32 as _);
pub const STATUS_OPEN_FAILED: NTSTATUS = NTSTATUS(0xC0000136_u32 as _);
pub const STATUS_OPERATION_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0000476_u32 as _);
pub const STATUS_OPERATION_NOT_SUPPORTED_IN_TRANSACTION: NTSTATUS = NTSTATUS(0xC019005A_u32 as _);
pub const STATUS_OPLOCK_BREAK_IN_PROGRESS: NTSTATUS = NTSTATUS(0x108_u32 as _);
pub const STATUS_OPLOCK_HANDLE_CLOSED: NTSTATUS = NTSTATUS(0x216_u32 as _);
pub const STATUS_OPLOCK_NOT_GRANTED: NTSTATUS = NTSTATUS(0xC00000E2_u32 as _);
pub const STATUS_OPLOCK_SWITCHED_TO_NEW_HANDLE: NTSTATUS = NTSTATUS(0x215_u32 as _);
pub const STATUS_ORDINAL_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000138_u32 as _);
pub const STATUS_ORPHAN_NAME_EXHAUSTED: NTSTATUS = NTSTATUS(0xC000080E_u32 as _);
pub const STATUS_PACKAGE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC0000497_u32 as _);
pub const STATUS_PACKAGE_UPDATING: NTSTATUS = NTSTATUS(0xC0000469_u32 as _);
pub const STATUS_PAGEFILE_CREATE_FAILED: NTSTATUS = NTSTATUS(0xC0000146_u32 as _);
pub const STATUS_PAGEFILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00004C5_u32 as _);
pub const STATUS_PAGEFILE_QUOTA: NTSTATUS = NTSTATUS(0xC0000007_u32 as _);
pub const STATUS_PAGEFILE_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC000012C_u32 as _);
pub const STATUS_PAGE_FAULT_COPY_ON_WRITE: NTSTATUS = NTSTATUS(0x112_u32 as _);
pub const STATUS_PAGE_FAULT_DEMAND_ZERO: NTSTATUS = NTSTATUS(0x111_u32 as _);
pub const STATUS_PAGE_FAULT_GUARD_PAGE: NTSTATUS = NTSTATUS(0x113_u32 as _);
pub const STATUS_PAGE_FAULT_PAGING_FILE: NTSTATUS = NTSTATUS(0x114_u32 as _);
pub const STATUS_PAGE_FAULT_RETRY: NTSTATUS = NTSTATUS(0x369_u32 as _);
pub const STATUS_PAGE_FAULT_TRANSITION: NTSTATUS = NTSTATUS(0x110_u32 as _);
pub const STATUS_PARAMETER_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000410_u32 as _);
pub const STATUS_PARITY_ERROR: NTSTATUS = NTSTATUS(0xC000002B_u32 as _);
pub const STATUS_PARTIAL_COPY: NTSTATUS = NTSTATUS(0x8000000D_u32 as _);
pub const STATUS_PARTITION_FAILURE: NTSTATUS = NTSTATUS(0xC0000172_u32 as _);
pub const STATUS_PARTITION_TERMINATING: NTSTATUS = NTSTATUS(0xC00004A0_u32 as _);
pub const STATUS_PASSWORD_CHANGE_REQUIRED: NTSTATUS = NTSTATUS(0xC000030C_u32 as _);
pub const STATUS_PASSWORD_RESTRICTION: NTSTATUS = NTSTATUS(0xC000006C_u32 as _);
pub const STATUS_PATCH_CONFLICT: NTSTATUS = NTSTATUS(0xC00004AC_u32 as _);
pub const STATUS_PATCH_DEFERRED: NTSTATUS = NTSTATUS(0x40000037_u32 as _);
pub const STATUS_PATCH_NOT_REGISTERED: NTSTATUS = NTSTATUS(0xC00004D4_u32 as _);
pub const STATUS_PATH_NOT_COVERED: NTSTATUS = NTSTATUS(0xC0000257_u32 as _);
pub const STATUS_PCP_ATTESTATION_CHALLENGE_NOT_SET: NTSTATUS = NTSTATUS(0xC0292012_u32 as _);
pub const STATUS_PCP_AUTHENTICATION_FAILED: NTSTATUS = NTSTATUS(0xC0292008_u32 as _);
pub const STATUS_PCP_AUTHENTICATION_IGNORED: NTSTATUS = NTSTATUS(0xC0292009_u32 as _);
pub const STATUS_PCP_BUFFER_LENGTH_MISMATCH: NTSTATUS = NTSTATUS(0xC029201E_u32 as _);
pub const STATUS_PCP_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(0xC0292006_u32 as _);
pub const STATUS_PCP_CLAIM_TYPE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC029201C_u32 as _);
pub const STATUS_PCP_DEVICE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC029200D_u32 as _);
pub const STATUS_PCP_DEVICE_NOT_READY: NTSTATUS = NTSTATUS(0xC0292001_u32 as _);
pub const STATUS_PCP_ERROR_MASK: NTSTATUS = NTSTATUS(0xC0292000_u32 as _);
pub const STATUS_PCP_FLAG_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0292004_u32 as _);
pub const STATUS_PCP_IFX_RSA_KEY_CREATION_BLOCKED: NTSTATUS = NTSTATUS(0xC029201F_u32 as _);
pub const STATUS_PCP_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC0292007_u32 as _);
pub const STATUS_PCP_INVALID_HANDLE: NTSTATUS = NTSTATUS(0xC0292002_u32 as _);
pub const STATUS_PCP_INVALID_PARAMETER: NTSTATUS = NTSTATUS(0xC0292003_u32 as _);
pub const STATUS_PCP_KEY_ALREADY_FINALIZED: NTSTATUS = NTSTATUS(0xC0292014_u32 as _);
pub const STATUS_PCP_KEY_HANDLE_INVALIDATED: NTSTATUS = NTSTATUS(0xC0292022_u32 as _);
pub const STATUS_PCP_KEY_NOT_AIK: NTSTATUS = NTSTATUS(0xC0292019_u32 as _);
pub const STATUS_PCP_KEY_NOT_AUTHENTICATED: NTSTATUS = NTSTATUS(0xC0292018_u32 as _);
pub const STATUS_PCP_KEY_NOT_FINALIZED: NTSTATUS = NTSTATUS(0xC0292011_u32 as _);
pub const STATUS_PCP_KEY_NOT_LOADED: NTSTATUS = NTSTATUS(0xC029200F_u32 as _);
pub const STATUS_PCP_KEY_NOT_SIGNING_KEY: NTSTATUS = NTSTATUS(0xC029201A_u32 as _);
pub const STATUS_PCP_KEY_USAGE_POLICY_INVALID: NTSTATUS = NTSTATUS(0xC0292016_u32 as _);
pub const STATUS_PCP_KEY_USAGE_POLICY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0292015_u32 as _);
pub const STATUS_PCP_LOCKED_OUT: NTSTATUS = NTSTATUS(0xC029201B_u32 as _);
pub const STATUS_PCP_NOT_PCR_BOUND: NTSTATUS = NTSTATUS(0xC0292013_u32 as _);
pub const STATUS_PCP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0292005_u32 as _);
pub const STATUS_PCP_NO_KEY_CERTIFICATION: NTSTATUS = NTSTATUS(0xC0292010_u32 as _);
pub const STATUS_PCP_POLICY_NOT_FOUND: NTSTATUS = NTSTATUS(0xC029200A_u32 as _);
pub const STATUS_PCP_PROFILE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC029200B_u32 as _);
pub const STATUS_PCP_RAW_POLICY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0292021_u32 as _);
pub const STATUS_PCP_SOFT_KEY_ERROR: NTSTATUS = NTSTATUS(0xC0292017_u32 as _);
pub const STATUS_PCP_TICKET_MISSING: NTSTATUS = NTSTATUS(0xC0292020_u32 as _);
pub const STATUS_PCP_TPM_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC029201D_u32 as _);
pub const STATUS_PCP_UNSUPPORTED_PSS_SALT: NTSTATUS = NTSTATUS(0x40292023_u32 as _);
pub const STATUS_PCP_VALIDATION_FAILED: NTSTATUS = NTSTATUS(0xC029200C_u32 as _);
pub const STATUS_PCP_WRONG_PARENT: NTSTATUS = NTSTATUS(0xC029200E_u32 as _);
pub const STATUS_PENDING: NTSTATUS = NTSTATUS(0x103_u32 as _);
pub const STATUS_PER_USER_TRUST_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000401_u32 as _);
pub const STATUS_PIPE_BROKEN: NTSTATUS = NTSTATUS(0xC000014B_u32 as _);
pub const STATUS_PIPE_BUSY: NTSTATUS = NTSTATUS(0xC00000AE_u32 as _);
pub const STATUS_PIPE_CLOSING: NTSTATUS = NTSTATUS(0xC00000B1_u32 as _);
pub const STATUS_PIPE_CONNECTED: NTSTATUS = NTSTATUS(0xC00000B2_u32 as _);
pub const STATUS_PIPE_DISCONNECTED: NTSTATUS = NTSTATUS(0xC00000B0_u32 as _);
pub const STATUS_PIPE_EMPTY: NTSTATUS = NTSTATUS(0xC00000D9_u32 as _);
pub const STATUS_PIPE_LISTENING: NTSTATUS = NTSTATUS(0xC00000B3_u32 as _);
pub const STATUS_PIPE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC00000AC_u32 as _);
pub const STATUS_PKINIT_CLIENT_FAILURE: NTSTATUS = NTSTATUS(0xC000038C_u32 as _);
pub const STATUS_PKINIT_FAILURE: NTSTATUS = NTSTATUS(0xC0000320_u32 as _);
pub const STATUS_PKINIT_NAME_MISMATCH: NTSTATUS = NTSTATUS(0xC00002F9_u32 as _);
pub const STATUS_PKU2U_CERT_FAILURE: NTSTATUS = NTSTATUS(0xC000042F_u32 as _);
pub const STATUS_PLATFORM_MANIFEST_BINARY_ID_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0EB0005_u32 as _);
pub const STATUS_PLATFORM_MANIFEST_CATALOG_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(0xC0EB0004_u32 as _);
pub const STATUS_PLATFORM_MANIFEST_FILE_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(0xC0EB0003_u32 as _);
pub const STATUS_PLATFORM_MANIFEST_INVALID: NTSTATUS = NTSTATUS(0xC0EB0002_u32 as _);
pub const STATUS_PLATFORM_MANIFEST_NOT_ACTIVE: NTSTATUS = NTSTATUS(0xC0EB0006_u32 as _);
pub const STATUS_PLATFORM_MANIFEST_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(0xC0EB0001_u32 as _);
pub const STATUS_PLATFORM_MANIFEST_NOT_SIGNED: NTSTATUS = NTSTATUS(0xC0EB0007_u32 as _);
pub const STATUS_PLUGPLAY_NO_DEVICE: NTSTATUS = NTSTATUS(0xC000025E_u32 as _);
pub const STATUS_PLUGPLAY_QUERY_VETOED: NTSTATUS = NTSTATUS(0x80000028_u32 as _);
pub const STATUS_PNP_BAD_MPS_TABLE: NTSTATUS = NTSTATUS(0xC0040035_u32 as _);
pub const STATUS_PNP_DEVICE_CONFIGURATION_PENDING: NTSTATUS = NTSTATUS(0xC0000495_u32 as _);
pub const STATUS_PNP_DRIVER_CONFIGURATION_INCOMPLETE: NTSTATUS = NTSTATUS(0xC0000493_u32 as _);
pub const STATUS_PNP_DRIVER_CONFIGURATION_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000492_u32 as _);
pub const STATUS_PNP_DRIVER_PACKAGE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000491_u32 as _);
pub const STATUS_PNP_FUNCTION_DRIVER_REQUIRED: NTSTATUS = NTSTATUS(0xC0000494_u32 as _);
pub const STATUS_PNP_INVALID_ID: NTSTATUS = NTSTATUS(0xC0040038_u32 as _);
pub const STATUS_PNP_IRQ_TRANSLATION_FAILED: NTSTATUS = NTSTATUS(0xC0040037_u32 as _);
pub const STATUS_PNP_NO_COMPAT_DRIVERS: NTSTATUS = NTSTATUS(0xC0000490_u32 as _);
pub const STATUS_PNP_REBOOT_REQUIRED: NTSTATUS = NTSTATUS(0xC00002D2_u32 as _);
pub const STATUS_PNP_RESTART_ENUMERATION: NTSTATUS = NTSTATUS(0xC00002CE_u32 as _);
pub const STATUS_PNP_TRANSLATION_FAILED: NTSTATUS = NTSTATUS(0xC0040036_u32 as _);
pub const STATUS_POLICY_CONTROLLED_ACCOUNT: NTSTATUS = NTSTATUS(0xC000A08B_u32 as _);
pub const STATUS_POLICY_OBJECT_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000029A_u32 as _);
pub const STATUS_POLICY_ONLY_IN_DS: NTSTATUS = NTSTATUS(0xC000029B_u32 as _);
pub const STATUS_PORT_ALREADY_HAS_COMPLETION_LIST: NTSTATUS = NTSTATUS(0xC000071A_u32 as _);
pub const STATUS_PORT_ALREADY_SET: NTSTATUS = NTSTATUS(0xC0000048_u32 as _);
pub const STATUS_PORT_CLOSED: NTSTATUS = NTSTATUS(0xC0000700_u32 as _);
pub const STATUS_PORT_CONNECTION_REFUSED: NTSTATUS = NTSTATUS(0xC0000041_u32 as _);
pub const STATUS_PORT_DISCONNECTED: NTSTATUS = NTSTATUS(0xC0000037_u32 as _);
pub const STATUS_PORT_DO_NOT_DISTURB: NTSTATUS = NTSTATUS(0xC0000036_u32 as _);
pub const STATUS_PORT_MESSAGE_TOO_LONG: NTSTATUS = NTSTATUS(0xC000002F_u32 as _);
pub const STATUS_PORT_NOT_SET: NTSTATUS = NTSTATUS(0xC0000353_u32 as _);
pub const STATUS_PORT_UNREACHABLE: NTSTATUS = NTSTATUS(0xC000023F_u32 as _);
pub const STATUS_POSSIBLE_DEADLOCK: NTSTATUS = NTSTATUS(0xC0000194_u32 as _);
pub const STATUS_POWER_STATE_INVALID: NTSTATUS = NTSTATUS(0xC00002D3_u32 as _);
pub const STATUS_PREDEFINED_HANDLE: NTSTATUS = NTSTATUS(0x40000016_u32 as _);
pub const STATUS_PRENT4_MACHINE_ACCOUNT: NTSTATUS = NTSTATUS(0xC0000357_u32 as _);
pub const STATUS_PRIMARY_TRANSPORT_CONNECT_FAILED: NTSTATUS = NTSTATUS(0x10E_u32 as _);
pub const STATUS_PRINT_CANCELLED: NTSTATUS = NTSTATUS(0xC00000C8_u32 as _);
pub const STATUS_PRINT_QUEUE_FULL: NTSTATUS = NTSTATUS(0xC00000C6_u32 as _);
pub const STATUS_PRIVILEGED_INSTRUCTION: NTSTATUS = NTSTATUS(0xC0000096_u32 as _);
pub const STATUS_PRIVILEGE_NOT_HELD: NTSTATUS = NTSTATUS(0xC0000061_u32 as _);
pub const STATUS_PROACTIVE_SCAN_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC000080F_u32 as _);
pub const STATUS_PROCEDURE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000007A_u32 as _);
pub const STATUS_PROCESS_CLONED: NTSTATUS = NTSTATUS(0x129_u32 as _);
pub const STATUS_PROCESS_IN_JOB: NTSTATUS = NTSTATUS(0x124_u32 as _);
pub const STATUS_PROCESS_IS_PROTECTED: NTSTATUS = NTSTATUS(0xC0000712_u32 as _);
pub const STATUS_PROCESS_IS_TERMINATING: NTSTATUS = NTSTATUS(0xC000010A_u32 as _);
pub const STATUS_PROCESS_NOT_IN_JOB: NTSTATUS = NTSTATUS(0x123_u32 as _);
pub const STATUS_PROFILING_AT_LIMIT: NTSTATUS = NTSTATUS(0xC00000D3_u32 as _);
pub const STATUS_PROFILING_NOT_STARTED: NTSTATUS = NTSTATUS(0xC00000B7_u32 as _);
pub const STATUS_PROFILING_NOT_STOPPED: NTSTATUS = NTSTATUS(0xC00000B8_u32 as _);
pub const STATUS_PROPSET_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000230_u32 as _);
pub const STATUS_PROTOCOL_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000A013_u32 as _);
pub const STATUS_PROTOCOL_UNREACHABLE: NTSTATUS = NTSTATUS(0xC000023E_u32 as _);
pub const STATUS_PTE_CHANGED: NTSTATUS = NTSTATUS(0xC0000434_u32 as _);
pub const STATUS_PURGE_FAILED: NTSTATUS = NTSTATUS(0xC0000435_u32 as _);
pub const STATUS_PWD_HISTORY_CONFLICT: NTSTATUS = NTSTATUS(0xC000025C_u32 as _);
pub const STATUS_PWD_TOO_LONG: NTSTATUS = NTSTATUS(0xC000027A_u32 as _);
pub const STATUS_PWD_TOO_RECENT: NTSTATUS = NTSTATUS(0xC000025B_u32 as _);
pub const STATUS_PWD_TOO_SHORT: NTSTATUS = NTSTATUS(0xC000025A_u32 as _);
pub const STATUS_QUERY_STORAGE_ERROR: NTSTATUS = NTSTATUS(0x803A0001_u32 as _);
pub const STATUS_QUIC_ALPN_NEG_FAILURE: NTSTATUS = NTSTATUS(0xC0240007_u32 as _);
pub const STATUS_QUIC_CONNECTION_IDLE: NTSTATUS = NTSTATUS(0xC0240005_u32 as _);
pub const STATUS_QUIC_CONNECTION_TIMEOUT: NTSTATUS = NTSTATUS(0xC0240006_u32 as _);
pub const STATUS_QUIC_HANDSHAKE_FAILURE: NTSTATUS = NTSTATUS(0xC0240000_u32 as _);
pub const STATUS_QUIC_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC0240003_u32 as _);
pub const STATUS_QUIC_PROTOCOL_VIOLATION: NTSTATUS = NTSTATUS(0xC0240004_u32 as _);
pub const STATUS_QUIC_USER_CANCELED: NTSTATUS = NTSTATUS(0xC0240002_u32 as _);
pub const STATUS_QUIC_VER_NEG_FAILURE: NTSTATUS = NTSTATUS(0xC0240001_u32 as _);
pub const STATUS_QUOTA_ACTIVITY: NTSTATUS = NTSTATUS(0xC000048A_u32 as _);
pub const STATUS_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000044_u32 as _);
pub const STATUS_QUOTA_LIST_INCONSISTENT: NTSTATUS = NTSTATUS(0xC0000266_u32 as _);
pub const STATUS_QUOTA_NOT_ENABLED: NTSTATUS = NTSTATUS(0xC00001A9_u32 as _);
pub const STATUS_RANGE_LIST_CONFLICT: NTSTATUS = NTSTATUS(0xC0000282_u32 as _);
pub const STATUS_RANGE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000028C_u32 as _);
pub const STATUS_RANGE_NOT_LOCKED: NTSTATUS = NTSTATUS(0xC000007E_u32 as _);
pub const STATUS_RDBSS_CONTINUE_OPERATION: NTSTATUS = NTSTATUS(0xC0410002_u32 as _);
pub const STATUS_RDBSS_POST_OPERATION: NTSTATUS = NTSTATUS(0xC0410003_u32 as _);
pub const STATUS_RDBSS_RESTART_OPERATION: NTSTATUS = NTSTATUS(0xC0410001_u32 as _);
pub const STATUS_RDBSS_RETRY_LOOKUP: NTSTATUS = NTSTATUS(0xC0410004_u32 as _);
pub const STATUS_RDP_PROTOCOL_ERROR: NTSTATUS = NTSTATUS(0xC00A0032_u32 as _);
pub const STATUS_RECEIVE_EXPEDITED: NTSTATUS = NTSTATUS(0x40000010_u32 as _);
pub const STATUS_RECEIVE_PARTIAL: NTSTATUS = NTSTATUS(0x4000000F_u32 as _);
pub const STATUS_RECEIVE_PARTIAL_EXPEDITED: NTSTATUS = NTSTATUS(0x40000011_u32 as _);
pub const STATUS_RECOVERABLE_BUGCHECK: NTSTATUS = NTSTATUS(0x80000034_u32 as _);
pub const STATUS_RECOVERY_FAILURE: NTSTATUS = NTSTATUS(0xC0000227_u32 as _);
pub const STATUS_RECOVERY_NOT_NEEDED: NTSTATUS = NTSTATUS(0x40190034_u32 as _);
pub const STATUS_RECURSIVE_DISPATCH: NTSTATUS = NTSTATUS(0xC0000704_u32 as _);
pub const STATUS_REDIRECTOR_HAS_OPEN_HANDLES: NTSTATUS = NTSTATUS(0x80000023_u32 as _);
pub const STATUS_REDIRECTOR_NOT_STARTED: NTSTATUS = NTSTATUS(0xC00000FB_u32 as _);
pub const STATUS_REDIRECTOR_PAUSED: NTSTATUS = NTSTATUS(0xC00000D1_u32 as _);
pub const STATUS_REDIRECTOR_STARTED: NTSTATUS = NTSTATUS(0xC00000FC_u32 as _);
pub const STATUS_REGISTRY_CORRUPT: NTSTATUS = NTSTATUS(0xC000014C_u32 as _);
pub const STATUS_REGISTRY_HIVE_RECOVERED: NTSTATUS = NTSTATUS(0x8000002A_u32 as _);
pub const STATUS_REGISTRY_IO_FAILED: NTSTATUS = NTSTATUS(0xC000014D_u32 as _);
pub const STATUS_REGISTRY_QUOTA_LIMIT: NTSTATUS = NTSTATUS(0xC0000256_u32 as _);
pub const STATUS_REGISTRY_RECOVERED: NTSTATUS = NTSTATUS(0x40000009_u32 as _);
pub const STATUS_REG_NAT_CONSUMPTION: NTSTATUS = NTSTATUS(0xC00002C9_u32 as _);
pub const STATUS_REINITIALIZATION_NEEDED: NTSTATUS = NTSTATUS(0xC0000287_u32 as _);
pub const STATUS_REMOTE_DISCONNECT: NTSTATUS = NTSTATUS(0xC000013C_u32 as _);
pub const STATUS_REMOTE_FILE_VERSION_MISMATCH: NTSTATUS = NTSTATUS(0xC019000C_u32 as _);
pub const STATUS_REMOTE_NOT_LISTENING: NTSTATUS = NTSTATUS(0xC00000BC_u32 as _);
pub const STATUS_REMOTE_RESOURCES: NTSTATUS = NTSTATUS(0xC000013D_u32 as _);
pub const STATUS_REMOTE_SESSION_LIMIT: NTSTATUS = NTSTATUS(0xC0000196_u32 as _);
pub const STATUS_REMOTE_STORAGE_MEDIA_ERROR: NTSTATUS = NTSTATUS(0xC000029E_u32 as _);
pub const STATUS_REMOTE_STORAGE_NOT_ACTIVE: NTSTATUS = NTSTATUS(0xC000029D_u32 as _);
pub const STATUS_REPAIR_NEEDED: NTSTATUS = NTSTATUS(0xC00001A8_u32 as _);
pub const STATUS_REPARSE: NTSTATUS = NTSTATUS(0x104_u32 as _);
pub const STATUS_REPARSE_ATTRIBUTE_CONFLICT: NTSTATUS = NTSTATUS(0xC00002B2_u32 as _);
pub const STATUS_REPARSE_GLOBAL: NTSTATUS = NTSTATUS(0x368_u32 as _);
pub const STATUS_REPARSE_OBJECT: NTSTATUS = NTSTATUS(0x118_u32 as _);
pub const STATUS_REPARSE_POINT_ENCOUNTERED: NTSTATUS = NTSTATUS(0xC000050B_u32 as _);
pub const STATUS_REPARSE_POINT_NOT_RESOLVED: NTSTATUS = NTSTATUS(0xC0000280_u32 as _);
pub const STATUS_REPLY_MESSAGE_MISMATCH: NTSTATUS = NTSTATUS(0xC000021F_u32 as _);
pub const STATUS_REQUEST_ABORTED: NTSTATUS = NTSTATUS(0xC0000240_u32 as _);
pub const STATUS_REQUEST_CANCELED: NTSTATUS = NTSTATUS(0xC0000703_u32 as _);
pub const STATUS_REQUEST_NOT_ACCEPTED: NTSTATUS = NTSTATUS(0xC00000D0_u32 as _);
pub const STATUS_REQUEST_OUT_OF_SEQUENCE: NTSTATUS = NTSTATUS(0xC000042A_u32 as _);
pub const STATUS_REQUEST_PAUSED: NTSTATUS = NTSTATUS(0xC0000459_u32 as _);
pub const STATUS_RESIDENT_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC000047A_u32 as _);
pub const STATUS_RESOURCEMANAGER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC019004F_u32 as _);
pub const STATUS_RESOURCEMANAGER_READ_ONLY: NTSTATUS = NTSTATUS(0x202_u32 as _);
pub const STATUS_RESOURCE_DATA_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000089_u32 as _);
pub const STATUS_RESOURCE_ENUM_USER_STOP: NTSTATUS = NTSTATUS(0xC00B0007_u32 as _);
pub const STATUS_RESOURCE_IN_USE: NTSTATUS = NTSTATUS(0xC0000708_u32 as _);
pub const STATUS_RESOURCE_LANG_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000204_u32 as _);
pub const STATUS_RESOURCE_NAME_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000008B_u32 as _);
pub const STATUS_RESOURCE_NOT_OWNED: NTSTATUS = NTSTATUS(0xC0000264_u32 as _);
pub const STATUS_RESOURCE_REQUIREMENTS_CHANGED: NTSTATUS = NTSTATUS(0x119_u32 as _);
pub const STATUS_RESOURCE_TYPE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC000008A_u32 as _);
pub const STATUS_RESTART_BOOT_APPLICATION: NTSTATUS = NTSTATUS(0xC0000453_u32 as _);
pub const STATUS_RESUME_HIBERNATION: NTSTATUS = NTSTATUS(0x4000002B_u32 as _);
pub const STATUS_RETRY: NTSTATUS = NTSTATUS(0xC000022D_u32 as _);
pub const STATUS_RETURN_ADDRESS_HIJACK_ATTEMPT: NTSTATUS = NTSTATUS(0x80000033_u32 as _);
pub const STATUS_REVISION_MISMATCH: NTSTATUS = NTSTATUS(0xC0000059_u32 as _);
pub const STATUS_REVOCATION_OFFLINE_C: NTSTATUS = NTSTATUS(0xC000038B_u32 as _);
pub const STATUS_REVOCATION_OFFLINE_KDC: NTSTATUS = NTSTATUS(0xC000040C_u32 as _);
pub const STATUS_RING_NEWLY_EMPTY: NTSTATUS = NTSTATUS(0x213_u32 as _);
pub const STATUS_RING_PREVIOUSLY_ABOVE_QUOTA: NTSTATUS = NTSTATUS(0x212_u32 as _);
pub const STATUS_RING_PREVIOUSLY_EMPTY: NTSTATUS = NTSTATUS(0x210_u32 as _);
pub const STATUS_RING_PREVIOUSLY_FULL: NTSTATUS = NTSTATUS(0x211_u32 as _);
pub const STATUS_RING_SIGNAL_OPPOSITE_ENDPOINT: NTSTATUS = NTSTATUS(0x214_u32 as _);
pub const STATUS_RKF_ACTIVE_KEY: NTSTATUS = NTSTATUS(0xC0400006_u32 as _);
pub const STATUS_RKF_BLOB_FULL: NTSTATUS = NTSTATUS(0xC0400003_u32 as _);
pub const STATUS_RKF_DUPLICATE_KEY: NTSTATUS = NTSTATUS(0xC0400002_u32 as _);
pub const STATUS_RKF_FILE_BLOCKED: NTSTATUS = NTSTATUS(0xC0400005_u32 as _);
pub const STATUS_RKF_KEY_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0400001_u32 as _);
pub const STATUS_RKF_STORE_FULL: NTSTATUS = NTSTATUS(0xC0400004_u32 as _);
pub const STATUS_RM_ALREADY_STARTED: NTSTATUS = NTSTATUS(0x40190035_u32 as _);
pub const STATUS_RM_CANNOT_BE_FROZEN_FOR_SNAPSHOT: NTSTATUS = NTSTATUS(0xC019005D_u32 as _);
pub const STATUS_RM_DISCONNECTED: NTSTATUS = NTSTATUS(0xC0190032_u32 as _);
pub const STATUS_RM_METADATA_CORRUPT: NTSTATUS = NTSTATUS(0xC0190006_u32 as _);
pub const STATUS_RM_NOT_ACTIVE: NTSTATUS = NTSTATUS(0xC0190005_u32 as _);
pub const STATUS_ROLLBACK_TIMER_EXPIRED: NTSTATUS = NTSTATUS(0xC019003C_u32 as _);
pub const STATUS_RTPM_CONTEXT_COMPLETE: NTSTATUS = NTSTATUS(0x293001_u32 as _);
pub const STATUS_RTPM_CONTEXT_CONTINUE: NTSTATUS = NTSTATUS(0x293000_u32 as _);
pub const STATUS_RTPM_INVALID_CONTEXT: NTSTATUS = NTSTATUS(0xC0293004_u32 as _);
pub const STATUS_RTPM_NO_RESULT: NTSTATUS = NTSTATUS(0xC0293002_u32 as _);
pub const STATUS_RTPM_PCR_READ_INCOMPLETE: NTSTATUS = NTSTATUS(0xC0293003_u32 as _);
pub const STATUS_RTPM_UNSUPPORTED_CMD: NTSTATUS = NTSTATUS(0xC0293005_u32 as _);
pub const STATUS_RUNLEVEL_SWITCH_AGENT_TIMEOUT: NTSTATUS = NTSTATUS(0xC000A145_u32 as _);
pub const STATUS_RUNLEVEL_SWITCH_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC000A146_u32 as _);
pub const STATUS_RUNLEVEL_SWITCH_TIMEOUT: NTSTATUS = NTSTATUS(0xC000A143_u32 as _);
pub const STATUS_RWRAW_ENCRYPTED_FILE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(0xC00004A7_u32 as _);
pub const STATUS_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILEOFFSET: NTSTATUS = NTSTATUS(0xC00004A8_u32 as _);
pub const STATUS_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILERANGE: NTSTATUS = NTSTATUS(0xC00004A9_u32 as _);
pub const STATUS_RWRAW_ENCRYPTED_INVALID_EDATAINFO_PARAMETER: NTSTATUS = NTSTATUS(0xC00004AA_u32 as _);
pub const STATUS_RXACT_COMMITTED: NTSTATUS = NTSTATUS(0x10A_u32 as _);
pub const STATUS_RXACT_COMMIT_FAILURE: NTSTATUS = NTSTATUS(0xC000011D_u32 as _);
pub const STATUS_RXACT_COMMIT_NECESSARY: NTSTATUS = NTSTATUS(0x80000018_u32 as _);
pub const STATUS_RXACT_INVALID_STATE: NTSTATUS = NTSTATUS(0xC000011C_u32 as _);
pub const STATUS_RXACT_STATE_CREATED: NTSTATUS = NTSTATUS(0x40000004_u32 as _);
pub const STATUS_SAM_INIT_FAILURE: NTSTATUS = NTSTATUS(0xC00002E3_u32 as _);
pub const STATUS_SAM_NEED_BOOTKEY_FLOPPY: NTSTATUS = NTSTATUS(0xC00002E0_u32 as _);
pub const STATUS_SAM_NEED_BOOTKEY_PASSWORD: NTSTATUS = NTSTATUS(0xC00002DF_u32 as _);
pub const STATUS_SCRUB_DATA_DISABLED: NTSTATUS = NTSTATUS(0xC0000478_u32 as _);
pub const STATUS_SECCORE_INVALID_COMMAND: NTSTATUS = NTSTATUS(0xC0E80000_u32 as _);
pub const STATUS_SECONDARY_IC_PROVIDER_NOT_REGISTERED: NTSTATUS = NTSTATUS(0xC000A121_u32 as _);
pub const STATUS_SECRET_TOO_LONG: NTSTATUS = NTSTATUS(0xC0000157_u32 as _);
pub const STATUS_SECTION_DIRECT_MAP_ONLY: NTSTATUS = NTSTATUS(0xC0000911_u32 as _);
pub const STATUS_SECTION_NOT_EXTENDED: NTSTATUS = NTSTATUS(0xC0000087_u32 as _);
pub const STATUS_SECTION_NOT_IMAGE: NTSTATUS = NTSTATUS(0xC0000049_u32 as _);
pub const STATUS_SECTION_PROTECTION: NTSTATUS = NTSTATUS(0xC000004E_u32 as _);
pub const STATUS_SECTION_TOO_BIG: NTSTATUS = NTSTATUS(0xC0000040_u32 as _);
pub const STATUS_SECUREBOOT_FILE_REPLACED: NTSTATUS = NTSTATUS(0xC0430007_u32 as _);
pub const STATUS_SECUREBOOT_INVALID_POLICY: NTSTATUS = NTSTATUS(0xC0430003_u32 as _);
pub const STATUS_SECUREBOOT_NOT_BASE_POLICY: NTSTATUS = NTSTATUS(0xC043000F_u32 as _);
pub const STATUS_SECUREBOOT_NOT_ENABLED: NTSTATUS = NTSTATUS(0x80430006_u32 as _);
pub const STATUS_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY: NTSTATUS = NTSTATUS(0xC0430010_u32 as _);
pub const STATUS_SECUREBOOT_PLATFORM_ID_MISMATCH: NTSTATUS = NTSTATUS(0xC043000B_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION: NTSTATUS = NTSTATUS(0xC043000A_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(0xC0430008_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_NOT_SIGNED: NTSTATUS = NTSTATUS(0xC0430005_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0430004_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_ROLLBACK_DETECTED: NTSTATUS = NTSTATUS(0xC043000C_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_UNKNOWN: NTSTATUS = NTSTATUS(0xC0430009_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_UPGRADE_MISMATCH: NTSTATUS = NTSTATUS(0xC043000D_u32 as _);
pub const STATUS_SECUREBOOT_POLICY_VIOLATION: NTSTATUS = NTSTATUS(0xC0430002_u32 as _);
pub const STATUS_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING: NTSTATUS = NTSTATUS(0xC043000E_u32 as _);
pub const STATUS_SECUREBOOT_ROLLBACK_DETECTED: NTSTATUS = NTSTATUS(0xC0430001_u32 as _);
pub const STATUS_SECURITY_STREAM_IS_INCONSISTENT: NTSTATUS = NTSTATUS(0xC00001A0_u32 as _);
pub const STATUS_SEGMENT_NOTIFICATION: NTSTATUS = NTSTATUS(0x40000005_u32 as _);
pub const STATUS_SEMAPHORE_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000047_u32 as _);
pub const STATUS_SERIAL_COUNTER_TIMEOUT: NTSTATUS = NTSTATUS(0x4000000C_u32 as _);
pub const STATUS_SERIAL_MORE_WRITES: NTSTATUS = NTSTATUS(0x40000008_u32 as _);
pub const STATUS_SERIAL_NO_DEVICE_INITED: NTSTATUS = NTSTATUS(0xC0000150_u32 as _);
pub const STATUS_SERVER_DISABLED: NTSTATUS = NTSTATUS(0xC0000080_u32 as _);
pub const STATUS_SERVER_HAS_OPEN_HANDLES: NTSTATUS = NTSTATUS(0x80000024_u32 as _);
pub const STATUS_SERVER_NOT_DISABLED: NTSTATUS = NTSTATUS(0xC0000081_u32 as _);
pub const STATUS_SERVER_SHUTDOWN_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC00002FF_u32 as _);
pub const STATUS_SERVER_SID_MISMATCH: NTSTATUS = NTSTATUS(0xC00002A0_u32 as _);
pub const STATUS_SERVER_TRANSPORT_CONFLICT: NTSTATUS = NTSTATUS(0xC00001B4_u32 as _);
pub const STATUS_SERVER_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC0000466_u32 as _);
pub const STATUS_SERVICES_FAILED_AUTOSTART: NTSTATUS = NTSTATUS(0x4000A144_u32 as _);
pub const STATUS_SERVICE_NOTIFICATION: NTSTATUS = NTSTATUS(0x40000018_u32 as _);
pub const STATUS_SESSION_KEY_TOO_SHORT: NTSTATUS = NTSTATUS(0xC0000517_u32 as _);
pub const STATUS_SETMARK_DETECTED: NTSTATUS = NTSTATUS(0x80000021_u32 as _);
pub const STATUS_SET_CONTEXT_DENIED: NTSTATUS = NTSTATUS(0xC000060A_u32 as _);
pub const STATUS_SEVERITY_COERROR: u32 = 2u32;
pub const STATUS_SEVERITY_COFAIL: u32 = 3u32;
pub const STATUS_SEVERITY_ERROR: NTSTATUS_SEVERITY_CODE = NTSTATUS_SEVERITY_CODE(3u32);
pub const STATUS_SEVERITY_INFORMATIONAL: NTSTATUS_SEVERITY_CODE = NTSTATUS_SEVERITY_CODE(1u32);
pub const STATUS_SEVERITY_SUCCESS: NTSTATUS_SEVERITY_CODE = NTSTATUS_SEVERITY_CODE(0u32);
pub const STATUS_SEVERITY_WARNING: NTSTATUS_SEVERITY_CODE = NTSTATUS_SEVERITY_CODE(2u32);
pub const STATUS_SHARED_IRQ_BUSY: NTSTATUS = NTSTATUS(0xC000016C_u32 as _);
pub const STATUS_SHARED_POLICY: NTSTATUS = NTSTATUS(0xC0000299_u32 as _);
pub const STATUS_SHARE_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC0000480_u32 as _);
pub const STATUS_SHARING_PAUSED: NTSTATUS = NTSTATUS(0xC00000CF_u32 as _);
pub const STATUS_SHARING_VIOLATION: NTSTATUS = NTSTATUS(0xC0000043_u32 as _);
pub const STATUS_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: NTSTATUS = NTSTATUS(0xC000019F_u32 as _);
pub const STATUS_SHUTDOWN_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC00002FE_u32 as _);
pub const STATUS_SINGLE_STEP: NTSTATUS = NTSTATUS(0x80000004_u32 as _);
pub const STATUS_SMARTCARD_CARD_BLOCKED: NTSTATUS = NTSTATUS(0xC0000381_u32 as _);
pub const STATUS_SMARTCARD_CARD_NOT_AUTHENTICATED: NTSTATUS = NTSTATUS(0xC0000382_u32 as _);
pub const STATUS_SMARTCARD_CERT_EXPIRED: NTSTATUS = NTSTATUS(0xC000038D_u32 as _);
pub const STATUS_SMARTCARD_CERT_REVOKED: NTSTATUS = NTSTATUS(0xC0000389_u32 as _);
pub const STATUS_SMARTCARD_IO_ERROR: NTSTATUS = NTSTATUS(0xC0000387_u32 as _);
pub const STATUS_SMARTCARD_LOGON_REQUIRED: NTSTATUS = NTSTATUS(0xC00002FA_u32 as _);
pub const STATUS_SMARTCARD_NO_CARD: NTSTATUS = NTSTATUS(0xC0000383_u32 as _);
pub const STATUS_SMARTCARD_NO_CERTIFICATE: NTSTATUS = NTSTATUS(0xC0000385_u32 as _);
pub const STATUS_SMARTCARD_NO_KEYSET: NTSTATUS = NTSTATUS(0xC0000386_u32 as _);
pub const STATUS_SMARTCARD_NO_KEY_CONTAINER: NTSTATUS = NTSTATUS(0xC0000384_u32 as _);
pub const STATUS_SMARTCARD_SILENT_CONTEXT: NTSTATUS = NTSTATUS(0xC000038F_u32 as _);
pub const STATUS_SMARTCARD_SUBSYSTEM_FAILURE: NTSTATUS = NTSTATUS(0xC0000321_u32 as _);
pub const STATUS_SMARTCARD_WRONG_PIN: NTSTATUS = NTSTATUS(0xC0000380_u32 as _);
pub const STATUS_SMB1_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC0000513_u32 as _);
pub const STATUS_SMB_BAD_CLUSTER_DIALECT: NTSTATUS = NTSTATUS(0xC05D0001_u32 as _);
pub const STATUS_SMB_GUEST_LOGON_BLOCKED: NTSTATUS = NTSTATUS(0xC05D0002_u32 as _);
pub const STATUS_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP: NTSTATUS = NTSTATUS(0xC05D0000_u32 as _);
pub const STATUS_SMB_NO_SIGNING_ALGORITHM_OVERLAP: NTSTATUS = NTSTATUS(0xC05D0003_u32 as _);
pub const STATUS_SMI_PRIMITIVE_INSTALLER_FAILED: NTSTATUS = NTSTATUS(0xC0150025_u32 as _);
pub const STATUS_SMR_GARBAGE_COLLECTION_REQUIRED: NTSTATUS = NTSTATUS(0xC0000514_u32 as _);
pub const STATUS_SOME_NOT_MAPPED: NTSTATUS = NTSTATUS(0x107_u32 as _);
pub const STATUS_SOURCE_ELEMENT_EMPTY: NTSTATUS = NTSTATUS(0xC0000283_u32 as _);
pub const STATUS_SPACES_ALLOCATION_SIZE_INVALID: NTSTATUS = NTSTATUS(0xC0E7000E_u32 as _);
pub const STATUS_SPACES_CACHE_FULL: NTSTATUS = NTSTATUS(0xC0E70026_u32 as _);
pub const STATUS_SPACES_COMPLETE: NTSTATUS = NTSTATUS(0xE70002_u32 as _);
pub const STATUS_SPACES_CORRUPT_METADATA: NTSTATUS = NTSTATUS(0xC0E70016_u32 as _);
pub const STATUS_SPACES_DRIVE_LOST_DATA: NTSTATUS = NTSTATUS(0xC0E7001D_u32 as _);
pub const STATUS_SPACES_DRIVE_NOT_READY: NTSTATUS = NTSTATUS(0xC0E7001B_u32 as _);
pub const STATUS_SPACES_DRIVE_OPERATIONAL_STATE_INVALID: NTSTATUS = NTSTATUS(0xC0E70012_u32 as _);
pub const STATUS_SPACES_DRIVE_REDUNDANCY_INVALID: NTSTATUS = NTSTATUS(0xC0E70006_u32 as _);
pub const STATUS_SPACES_DRIVE_SECTOR_SIZE_INVALID: NTSTATUS = NTSTATUS(0xC0E70004_u32 as _);
pub const STATUS_SPACES_DRIVE_SPLIT: NTSTATUS = NTSTATUS(0xC0E7001C_u32 as _);
pub const STATUS_SPACES_DRT_FULL: NTSTATUS = NTSTATUS(0xC0E70017_u32 as _);
pub const STATUS_SPACES_ENCLOSURE_AWARE_INVALID: NTSTATUS = NTSTATUS(0xC0E7000F_u32 as _);
pub const STATUS_SPACES_ENTRY_INCOMPLETE: NTSTATUS = NTSTATUS(0xC0E7001E_u32 as _);
pub const STATUS_SPACES_ENTRY_INVALID: NTSTATUS = NTSTATUS(0xC0E7001F_u32 as _);
pub const STATUS_SPACES_EXTENDED_ERROR: NTSTATUS = NTSTATUS(0xC0E7000C_u32 as _);
pub const STATUS_SPACES_FAULT_DOMAIN_TYPE_INVALID: NTSTATUS = NTSTATUS(0xC0E70001_u32 as _);
pub const STATUS_SPACES_FLUSH_METADATA: NTSTATUS = NTSTATUS(0xC0E70025_u32 as _);
pub const STATUS_SPACES_INCONSISTENCY: NTSTATUS = NTSTATUS(0xC0E70018_u32 as _);
pub const STATUS_SPACES_INTERLEAVE_LENGTH_INVALID: NTSTATUS = NTSTATUS(0xC0E70009_u32 as _);
pub const STATUS_SPACES_LOG_NOT_READY: NTSTATUS = NTSTATUS(0xC0E70019_u32 as _);
pub const STATUS_SPACES_MAP_REQUIRED: NTSTATUS = NTSTATUS(0xC0E70014_u32 as _);
pub const STATUS_SPACES_MARK_DIRTY: NTSTATUS = NTSTATUS(0xC0E70020_u32 as _);
pub const STATUS_SPACES_NOT_ENOUGH_DRIVES: NTSTATUS = NTSTATUS(0xC0E7000B_u32 as _);
pub const STATUS_SPACES_NO_REDUNDANCY: NTSTATUS = NTSTATUS(0xC0E7001A_u32 as _);
pub const STATUS_SPACES_NUMBER_OF_COLUMNS_INVALID: NTSTATUS = NTSTATUS(0xC0E7000A_u32 as _);
pub const STATUS_SPACES_NUMBER_OF_DATA_COPIES_INVALID: NTSTATUS = NTSTATUS(0xC0E70007_u32 as _);
pub const STATUS_SPACES_NUMBER_OF_GROUPS_INVALID: NTSTATUS = NTSTATUS(0xC0E70011_u32 as _);
pub const STATUS_SPACES_PAUSE: NTSTATUS = NTSTATUS(0xE70001_u32 as _);
pub const STATUS_SPACES_PD_INVALID_DATA: NTSTATUS = NTSTATUS(0xC0E70024_u32 as _);
pub const STATUS_SPACES_PD_LENGTH_MISMATCH: NTSTATUS = NTSTATUS(0xC0E70022_u32 as _);
pub const STATUS_SPACES_PD_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0E70021_u32 as _);
pub const STATUS_SPACES_PD_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(0xC0E70023_u32 as _);
pub const STATUS_SPACES_PROVISIONING_TYPE_INVALID: NTSTATUS = NTSTATUS(0xC0E7000D_u32 as _);
pub const STATUS_SPACES_REDIRECT: NTSTATUS = NTSTATUS(0xE70003_u32 as _);
pub const STATUS_SPACES_REPAIRED: NTSTATUS = NTSTATUS(0xE70000_u32 as _);
pub const STATUS_SPACES_REPAIR_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0E70027_u32 as _);
pub const STATUS_SPACES_RESILIENCY_TYPE_INVALID: NTSTATUS = NTSTATUS(0xC0E70003_u32 as _);
pub const STATUS_SPACES_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(0xC0E70015_u32 as _);
pub const STATUS_SPACES_UPDATE_COLUMN_STATE: NTSTATUS = NTSTATUS(0xC0E70013_u32 as _);
pub const STATUS_SPACES_WRITE_CACHE_SIZE_INVALID: NTSTATUS = NTSTATUS(0xC0E70010_u32 as _);
pub const STATUS_SPARSE_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00004C4_u32 as _);
pub const STATUS_SPARSE_NOT_ALLOWED_IN_TRANSACTION: NTSTATUS = NTSTATUS(0xC0190049_u32 as _);
pub const STATUS_SPECIAL_ACCOUNT: NTSTATUS = NTSTATUS(0xC0000124_u32 as _);
pub const STATUS_SPECIAL_GROUP: NTSTATUS = NTSTATUS(0xC0000125_u32 as _);
pub const STATUS_SPECIAL_USER: NTSTATUS = NTSTATUS(0xC0000126_u32 as _);
pub const STATUS_STACK_BUFFER_OVERRUN: NTSTATUS = NTSTATUS(0xC0000409_u32 as _);
pub const STATUS_STACK_OVERFLOW: NTSTATUS = NTSTATUS(0xC00000FD_u32 as _);
pub const STATUS_STACK_OVERFLOW_READ: NTSTATUS = NTSTATUS(0xC0000228_u32 as _);
pub const STATUS_STOPPED_ON_SYMLINK: NTSTATUS = NTSTATUS(0x8000002D_u32 as _);
pub const STATUS_STORAGE_LOST_DATA_PERSISTENCE: NTSTATUS = NTSTATUS(0xC000049E_u32 as _);
pub const STATUS_STORAGE_RESERVE_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC00004AF_u32 as _);
pub const STATUS_STORAGE_RESERVE_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(0xC00004AE_u32 as _);
pub const STATUS_STORAGE_RESERVE_ID_INVALID: NTSTATUS = NTSTATUS(0xC00004AD_u32 as _);
pub const STATUS_STORAGE_RESERVE_NOT_EMPTY: NTSTATUS = NTSTATUS(0xC00004B0_u32 as _);
pub const STATUS_STORAGE_STACK_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC00004C1_u32 as _);
pub const STATUS_STORAGE_TOPOLOGY_ID_MISMATCH: NTSTATUS = NTSTATUS(0xC0000486_u32 as _);
pub const STATUS_STOWED_EXCEPTION: NTSTATUS = NTSTATUS(0xC000027B_u32 as _);
pub const STATUS_STREAM_MINIVERSION_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0190022_u32 as _);
pub const STATUS_STREAM_MINIVERSION_NOT_VALID: NTSTATUS = NTSTATUS(0xC0190023_u32 as _);
pub const STATUS_STRICT_CFG_VIOLATION: NTSTATUS = NTSTATUS(0xC0000606_u32 as _);
pub const STATUS_STRONG_CRYPTO_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00002F6_u32 as _);
pub const STATUS_SUCCESS: NTSTATUS = NTSTATUS(0x0_u32 as _);
pub const STATUS_SUSPEND_COUNT_EXCEEDED: NTSTATUS = NTSTATUS(0xC000004A_u32 as _);
pub const STATUS_SVHDX_ERROR_NOT_AVAILABLE: NTSTATUS = NTSTATUS(0xC05CFF00_u32 as _);
pub const STATUS_SVHDX_ERROR_STORED: NTSTATUS = NTSTATUS(0xC05C0000_u32 as _);
pub const STATUS_SVHDX_NO_INITIATOR: NTSTATUS = NTSTATUS(0xC05CFF0B_u32 as _);
pub const STATUS_SVHDX_RESERVATION_CONFLICT: NTSTATUS = NTSTATUS(0xC05CFF07_u32 as _);
pub const STATUS_SVHDX_UNIT_ATTENTION_AVAILABLE: NTSTATUS = NTSTATUS(0xC05CFF01_u32 as _);
pub const STATUS_SVHDX_UNIT_ATTENTION_CAPACITY_DATA_CHANGED: NTSTATUS = NTSTATUS(0xC05CFF02_u32 as _);
pub const STATUS_SVHDX_UNIT_ATTENTION_OPERATING_DEFINITION_CHANGED: NTSTATUS = NTSTATUS(0xC05CFF06_u32 as _);
pub const STATUS_SVHDX_UNIT_ATTENTION_REGISTRATIONS_PREEMPTED: NTSTATUS = NTSTATUS(0xC05CFF05_u32 as _);
pub const STATUS_SVHDX_UNIT_ATTENTION_RESERVATIONS_PREEMPTED: NTSTATUS = NTSTATUS(0xC05CFF03_u32 as _);
pub const STATUS_SVHDX_UNIT_ATTENTION_RESERVATIONS_RELEASED: NTSTATUS = NTSTATUS(0xC05CFF04_u32 as _);
pub const STATUS_SVHDX_VERSION_MISMATCH: NTSTATUS = NTSTATUS(0xC05CFF09_u32 as _);
pub const STATUS_SVHDX_WRONG_FILE_TYPE: NTSTATUS = NTSTATUS(0xC05CFF08_u32 as _);
pub const STATUS_SXS_ACTIVATION_CONTEXT_DISABLED: NTSTATUS = NTSTATUS(0xC0150007_u32 as _);
pub const STATUS_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT: NTSTATUS = NTSTATUS(0xC015001E_u32 as _);
pub const STATUS_SXS_ASSEMBLY_MISSING: NTSTATUS = NTSTATUS(0xC015000C_u32 as _);
pub const STATUS_SXS_ASSEMBLY_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0150004_u32 as _);
pub const STATUS_SXS_CANT_GEN_ACTCTX: NTSTATUS = NTSTATUS(0xC0150002_u32 as _);
pub const STATUS_SXS_COMPONENT_STORE_CORRUPT: NTSTATUS = NTSTATUS(0xC015001A_u32 as _);
pub const STATUS_SXS_CORRUPTION: NTSTATUS = NTSTATUS(0xC0150015_u32 as _);
pub const STATUS_SXS_CORRUPT_ACTIVATION_STACK: NTSTATUS = NTSTATUS(0xC0150014_u32 as _);
pub const STATUS_SXS_EARLY_DEACTIVATION: NTSTATUS = NTSTATUS(0xC015000F_u32 as _);
pub const STATUS_SXS_FILE_HASH_MISMATCH: NTSTATUS = NTSTATUS(0xC015001B_u32 as _);
pub const STATUS_SXS_FILE_HASH_MISSING: NTSTATUS = NTSTATUS(0xC0150027_u32 as _);
pub const STATUS_SXS_FILE_NOT_PART_OF_ASSEMBLY: NTSTATUS = NTSTATUS(0xC015001F_u32 as _);
pub const STATUS_SXS_IDENTITIES_DIFFERENT: NTSTATUS = NTSTATUS(0xC015001D_u32 as _);
pub const STATUS_SXS_IDENTITY_DUPLICATE_ATTRIBUTE: NTSTATUS = NTSTATUS(0xC0150018_u32 as _);
pub const STATUS_SXS_IDENTITY_PARSE_ERROR: NTSTATUS = NTSTATUS(0xC0150019_u32 as _);
pub const STATUS_SXS_INVALID_ACTCTXDATA_FORMAT: NTSTATUS = NTSTATUS(0xC0150003_u32 as _);
pub const STATUS_SXS_INVALID_DEACTIVATION: NTSTATUS = NTSTATUS(0xC0150010_u32 as _);
pub const STATUS_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME: NTSTATUS = NTSTATUS(0xC0150017_u32 as _);
pub const STATUS_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE: NTSTATUS = NTSTATUS(0xC0150016_u32 as _);
pub const STATUS_SXS_KEY_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0150008_u32 as _);
pub const STATUS_SXS_MANIFEST_FORMAT_ERROR: NTSTATUS = NTSTATUS(0xC0150005_u32 as _);
pub const STATUS_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT: NTSTATUS = NTSTATUS(0xC015001C_u32 as _);
pub const STATUS_SXS_MANIFEST_PARSE_ERROR: NTSTATUS = NTSTATUS(0xC0150006_u32 as _);
pub const STATUS_SXS_MANIFEST_TOO_BIG: NTSTATUS = NTSTATUS(0xC0150022_u32 as _);
pub const STATUS_SXS_MULTIPLE_DEACTIVATION: NTSTATUS = NTSTATUS(0xC0150011_u32 as _);
pub const STATUS_SXS_PROCESS_DEFAULT_ALREADY_SET: NTSTATUS = NTSTATUS(0xC015000E_u32 as _);
pub const STATUS_SXS_PROCESS_TERMINATION_REQUESTED: NTSTATUS = NTSTATUS(0xC0150013_u32 as _);
pub const STATUS_SXS_RELEASE_ACTIVATION_CONTEXT: NTSTATUS = NTSTATUS(0x4015000D_u32 as _);
pub const STATUS_SXS_SECTION_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0150001_u32 as _);
pub const STATUS_SXS_SETTING_NOT_REGISTERED: NTSTATUS = NTSTATUS(0xC0150023_u32 as _);
pub const STATUS_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY: NTSTATUS = NTSTATUS(0xC0150012_u32 as _);
pub const STATUS_SXS_THREAD_QUERIES_DISABLED: NTSTATUS = NTSTATUS(0xC015000B_u32 as _);
pub const STATUS_SXS_TRANSACTION_CLOSURE_INCOMPLETE: NTSTATUS = NTSTATUS(0xC0150024_u32 as _);
pub const STATUS_SXS_VERSION_CONFLICT: NTSTATUS = NTSTATUS(0xC0150009_u32 as _);
pub const STATUS_SXS_WRONG_SECTION_TYPE: NTSTATUS = NTSTATUS(0xC015000A_u32 as _);
pub const STATUS_SYMLINK_CLASS_DISABLED: NTSTATUS = NTSTATUS(0xC0000715_u32 as _);
pub const STATUS_SYNCHRONIZATION_REQUIRED: NTSTATUS = NTSTATUS(0xC0000134_u32 as _);
pub const STATUS_SYSTEM_DEVICE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000452_u32 as _);
pub const STATUS_SYSTEM_HIVE_TOO_LARGE: NTSTATUS = NTSTATUS(0xC000036E_u32 as _);
pub const STATUS_SYSTEM_IMAGE_BAD_SIGNATURE: NTSTATUS = NTSTATUS(0xC00002D1_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_INVALID_POLICY: NTSTATUS = NTSTATUS(0xC0E90003_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_POLICY_NOT_SIGNED: NTSTATUS = NTSTATUS(0xC0E90004_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_POLICY_VIOLATION: NTSTATUS = NTSTATUS(0xC0E90002_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_DANGEROUS_EXT: NTSTATUS = NTSTATUS(0xC0E90009_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_EXPLICIT_DENY_FILE: NTSTATUS = NTSTATUS(0xC0E9000D_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_MALICIOUS: NTSTATUS = NTSTATUS(0xC0E90007_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_OFFLINE: NTSTATUS = NTSTATUS(0xC0E9000A_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_PUA: NTSTATUS = NTSTATUS(0xC0E90008_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_UNATTAINABLE: NTSTATUS = NTSTATUS(0xC0E9000C_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_UNFRIENDLY_FILE: NTSTATUS = NTSTATUS(0xC0E9000B_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_ROLLBACK_DETECTED: NTSTATUS = NTSTATUS(0xC0E90001_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_SUPPLEMENTAL_POLICY_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(0xC0E90006_u32 as _);
pub const STATUS_SYSTEM_INTEGRITY_TOO_MANY_POLICIES: NTSTATUS = NTSTATUS(0xC0E90005_u32 as _);
pub const STATUS_SYSTEM_NEEDS_REMEDIATION: NTSTATUS = NTSTATUS(0xC000047E_u32 as _);
pub const STATUS_SYSTEM_POWERSTATE_COMPLEX_TRANSITION: NTSTATUS = NTSTATUS(0x40000031_u32 as _);
pub const STATUS_SYSTEM_POWERSTATE_TRANSITION: NTSTATUS = NTSTATUS(0x4000002F_u32 as _);
pub const STATUS_SYSTEM_PROCESS_TERMINATED: NTSTATUS = NTSTATUS(0xC000021A_u32 as _);
pub const STATUS_SYSTEM_SHUTDOWN: NTSTATUS = NTSTATUS(0xC00002EB_u32 as _);
pub const STATUS_THREADPOOL_FREE_LIBRARY_ON_COMPLETION_FAILED: NTSTATUS = NTSTATUS(0xC000070E_u32 as _);
pub const STATUS_THREADPOOL_HANDLE_EXCEPTION: NTSTATUS = NTSTATUS(0xC000070A_u32 as _);
pub const STATUS_THREADPOOL_RELEASED_DURING_OPERATION: NTSTATUS = NTSTATUS(0xC000070F_u32 as _);
pub const STATUS_THREADPOOL_RELEASE_MUTEX_ON_COMPLETION_FAILED: NTSTATUS = NTSTATUS(0xC000070D_u32 as _);
pub const STATUS_THREADPOOL_RELEASE_SEMAPHORE_ON_COMPLETION_FAILED: NTSTATUS = NTSTATUS(0xC000070C_u32 as _);
pub const STATUS_THREADPOOL_SET_EVENT_ON_COMPLETION_FAILED: NTSTATUS = NTSTATUS(0xC000070B_u32 as _);
pub const STATUS_THREAD_ALREADY_IN_SESSION: NTSTATUS = NTSTATUS(0xC0000456_u32 as _);
pub const STATUS_THREAD_ALREADY_IN_TASK: NTSTATUS = NTSTATUS(0xC0000502_u32 as _);
pub const STATUS_THREAD_IS_TERMINATING: NTSTATUS = NTSTATUS(0xC000004B_u32 as _);
pub const STATUS_THREAD_NOT_IN_PROCESS: NTSTATUS = NTSTATUS(0xC000012A_u32 as _);
pub const STATUS_THREAD_NOT_IN_SESSION: NTSTATUS = NTSTATUS(0xC0000457_u32 as _);
pub const STATUS_THREAD_NOT_RUNNING: NTSTATUS = NTSTATUS(0xC0000516_u32 as _);
pub const STATUS_THREAD_WAS_SUSPENDED: NTSTATUS = NTSTATUS(0x40000001_u32 as _);
pub const STATUS_TIMEOUT: NTSTATUS = NTSTATUS(0x102_u32 as _);
pub const STATUS_TIMER_NOT_CANCELED: NTSTATUS = NTSTATUS(0xC000000C_u32 as _);
pub const STATUS_TIMER_RESOLUTION_NOT_SET: NTSTATUS = NTSTATUS(0xC0000245_u32 as _);
pub const STATUS_TIMER_RESUME_IGNORED: NTSTATUS = NTSTATUS(0x40000025_u32 as _);
pub const STATUS_TIME_DIFFERENCE_AT_DC: NTSTATUS = NTSTATUS(0xC0000133_u32 as _);
pub const STATUS_TM_IDENTITY_MISMATCH: NTSTATUS = NTSTATUS(0xC019004A_u32 as _);
pub const STATUS_TM_INITIALIZATION_FAILED: NTSTATUS = NTSTATUS(0xC0190004_u32 as _);
pub const STATUS_TM_VOLATILE: NTSTATUS = NTSTATUS(0xC019003B_u32 as _);
pub const STATUS_TOKEN_ALREADY_IN_USE: NTSTATUS = NTSTATUS(0xC000012B_u32 as _);
pub const STATUS_TOO_LATE: NTSTATUS = NTSTATUS(0xC0000189_u32 as _);
pub const STATUS_TOO_MANY_ADDRESSES: NTSTATUS = NTSTATUS(0xC0000209_u32 as _);
pub const STATUS_TOO_MANY_COMMANDS: NTSTATUS = NTSTATUS(0xC00000C1_u32 as _);
pub const STATUS_TOO_MANY_CONTEXT_IDS: NTSTATUS = NTSTATUS(0xC000015A_u32 as _);
pub const STATUS_TOO_MANY_GUIDS_REQUESTED: NTSTATUS = NTSTATUS(0xC0000082_u32 as _);
pub const STATUS_TOO_MANY_LINKS: NTSTATUS = NTSTATUS(0xC0000265_u32 as _);
pub const STATUS_TOO_MANY_LUIDS_REQUESTED: NTSTATUS = NTSTATUS(0xC0000074_u32 as _);
pub const STATUS_TOO_MANY_NAMES: NTSTATUS = NTSTATUS(0xC00000CD_u32 as _);
pub const STATUS_TOO_MANY_NODES: NTSTATUS = NTSTATUS(0xC000020E_u32 as _);
pub const STATUS_TOO_MANY_OPENED_FILES: NTSTATUS = NTSTATUS(0xC000011F_u32 as _);
pub const STATUS_TOO_MANY_PAGING_FILES: NTSTATUS = NTSTATUS(0xC0000097_u32 as _);
pub const STATUS_TOO_MANY_PRINCIPALS: NTSTATUS = NTSTATUS(0xC00002F7_u32 as _);
pub const STATUS_TOO_MANY_SECRETS: NTSTATUS = NTSTATUS(0xC0000156_u32 as _);
pub const STATUS_TOO_MANY_SEGMENT_DESCRIPTORS: NTSTATUS = NTSTATUS(0xC0000473_u32 as _);
pub const STATUS_TOO_MANY_SESSIONS: NTSTATUS = NTSTATUS(0xC00000CE_u32 as _);
pub const STATUS_TOO_MANY_SIDS: NTSTATUS = NTSTATUS(0xC000017E_u32 as _);
pub const STATUS_TOO_MANY_THREADS: NTSTATUS = NTSTATUS(0xC0000129_u32 as _);
pub const STATUS_TPM_20_E_ASYMMETRIC: NTSTATUS = NTSTATUS(0xC0290081_u32 as _);
pub const STATUS_TPM_20_E_ATTRIBUTES: NTSTATUS = NTSTATUS(0xC0290082_u32 as _);
pub const STATUS_TPM_20_E_AUTHSIZE: NTSTATUS = NTSTATUS(0xC0290144_u32 as _);
pub const STATUS_TPM_20_E_AUTH_CONTEXT: NTSTATUS = NTSTATUS(0xC0290145_u32 as _);
pub const STATUS_TPM_20_E_AUTH_FAIL: NTSTATUS = NTSTATUS(0xC029008E_u32 as _);
pub const STATUS_TPM_20_E_AUTH_MISSING: NTSTATUS = NTSTATUS(0xC0290125_u32 as _);
pub const STATUS_TPM_20_E_AUTH_TYPE: NTSTATUS = NTSTATUS(0xC0290124_u32 as _);
pub const STATUS_TPM_20_E_AUTH_UNAVAILABLE: NTSTATUS = NTSTATUS(0xC029012F_u32 as _);
pub const STATUS_TPM_20_E_BAD_AUTH: NTSTATUS = NTSTATUS(0xC02900A2_u32 as _);
pub const STATUS_TPM_20_E_BAD_CONTEXT: NTSTATUS = NTSTATUS(0xC0290150_u32 as _);
pub const STATUS_TPM_20_E_BINDING: NTSTATUS = NTSTATUS(0xC02900A5_u32 as _);
pub const STATUS_TPM_20_E_COMMAND_CODE: NTSTATUS = NTSTATUS(0xC0290143_u32 as _);
pub const STATUS_TPM_20_E_COMMAND_SIZE: NTSTATUS = NTSTATUS(0xC0290142_u32 as _);
pub const STATUS_TPM_20_E_CPHASH: NTSTATUS = NTSTATUS(0xC0290151_u32 as _);
pub const STATUS_TPM_20_E_CURVE: NTSTATUS = NTSTATUS(0xC02900A6_u32 as _);
pub const STATUS_TPM_20_E_DISABLED: NTSTATUS = NTSTATUS(0xC0290120_u32 as _);
pub const STATUS_TPM_20_E_ECC_CURVE: NTSTATUS = NTSTATUS(0xC0290123_u32 as _);
pub const STATUS_TPM_20_E_ECC_POINT: NTSTATUS = NTSTATUS(0xC02900A7_u32 as _);
pub const STATUS_TPM_20_E_EXCLUSIVE: NTSTATUS = NTSTATUS(0xC0290121_u32 as _);
pub const STATUS_TPM_20_E_EXPIRED: NTSTATUS = NTSTATUS(0xC02900A3_u32 as _);
pub const STATUS_TPM_20_E_FAILURE: NTSTATUS = NTSTATUS(0xC0290101_u32 as _);
pub const STATUS_TPM_20_E_HANDLE: NTSTATUS = NTSTATUS(0xC029008B_u32 as _);
pub const STATUS_TPM_20_E_HASH: NTSTATUS = NTSTATUS(0xC0290083_u32 as _);
pub const STATUS_TPM_20_E_HIERARCHY: NTSTATUS = NTSTATUS(0xC0290085_u32 as _);
pub const STATUS_TPM_20_E_HMAC: NTSTATUS = NTSTATUS(0xC0290119_u32 as _);
pub const STATUS_TPM_20_E_INITIALIZE: NTSTATUS = NTSTATUS(0xC0290100_u32 as _);
pub const STATUS_TPM_20_E_INSUFFICIENT: NTSTATUS = NTSTATUS(0xC029009A_u32 as _);
pub const STATUS_TPM_20_E_INTEGRITY: NTSTATUS = NTSTATUS(0xC029009F_u32 as _);
pub const STATUS_TPM_20_E_KDF: NTSTATUS = NTSTATUS(0xC029008C_u32 as _);
pub const STATUS_TPM_20_E_KEY: NTSTATUS = NTSTATUS(0xC029009C_u32 as _);
pub const STATUS_TPM_20_E_KEY_SIZE: NTSTATUS = NTSTATUS(0xC0290087_u32 as _);
pub const STATUS_TPM_20_E_MGF: NTSTATUS = NTSTATUS(0xC0290088_u32 as _);
pub const STATUS_TPM_20_E_MODE: NTSTATUS = NTSTATUS(0xC0290089_u32 as _);
pub const STATUS_TPM_20_E_NEEDS_TEST: NTSTATUS = NTSTATUS(0xC0290153_u32 as _);
pub const STATUS_TPM_20_E_NONCE: NTSTATUS = NTSTATUS(0xC029008F_u32 as _);
pub const STATUS_TPM_20_E_NO_RESULT: NTSTATUS = NTSTATUS(0xC0290154_u32 as _);
pub const STATUS_TPM_20_E_NV_AUTHORIZATION: NTSTATUS = NTSTATUS(0xC0290149_u32 as _);
pub const STATUS_TPM_20_E_NV_DEFINED: NTSTATUS = NTSTATUS(0xC029014C_u32 as _);
pub const STATUS_TPM_20_E_NV_LOCKED: NTSTATUS = NTSTATUS(0xC0290148_u32 as _);
pub const STATUS_TPM_20_E_NV_RANGE: NTSTATUS = NTSTATUS(0xC0290146_u32 as _);
pub const STATUS_TPM_20_E_NV_SIZE: NTSTATUS = NTSTATUS(0xC0290147_u32 as _);
pub const STATUS_TPM_20_E_NV_SPACE: NTSTATUS = NTSTATUS(0xC029014B_u32 as _);
pub const STATUS_TPM_20_E_NV_UNINITIALIZED: NTSTATUS = NTSTATUS(0xC029014A_u32 as _);
pub const STATUS_TPM_20_E_PARENT: NTSTATUS = NTSTATUS(0xC0290152_u32 as _);
pub const STATUS_TPM_20_E_PCR: NTSTATUS = NTSTATUS(0xC0290127_u32 as _);
pub const STATUS_TPM_20_E_PCR_CHANGED: NTSTATUS = NTSTATUS(0xC0290128_u32 as _);
pub const STATUS_TPM_20_E_POLICY: NTSTATUS = NTSTATUS(0xC0290126_u32 as _);
pub const STATUS_TPM_20_E_POLICY_CC: NTSTATUS = NTSTATUS(0xC02900A4_u32 as _);
pub const STATUS_TPM_20_E_POLICY_FAIL: NTSTATUS = NTSTATUS(0xC029009D_u32 as _);
pub const STATUS_TPM_20_E_PP: NTSTATUS = NTSTATUS(0xC0290090_u32 as _);
pub const STATUS_TPM_20_E_PRIVATE: NTSTATUS = NTSTATUS(0xC029010B_u32 as _);
pub const STATUS_TPM_20_E_RANGE: NTSTATUS = NTSTATUS(0xC029008D_u32 as _);
pub const STATUS_TPM_20_E_REBOOT: NTSTATUS = NTSTATUS(0xC0290130_u32 as _);
pub const STATUS_TPM_20_E_RESERVED_BITS: NTSTATUS = NTSTATUS(0xC02900A1_u32 as _);
pub const STATUS_TPM_20_E_SCHEME: NTSTATUS = NTSTATUS(0xC0290092_u32 as _);
pub const STATUS_TPM_20_E_SELECTOR: NTSTATUS = NTSTATUS(0xC0290098_u32 as _);
pub const STATUS_TPM_20_E_SENSITIVE: NTSTATUS = NTSTATUS(0xC0290155_u32 as _);
pub const STATUS_TPM_20_E_SEQUENCE: NTSTATUS = NTSTATUS(0xC0290103_u32 as _);
pub const STATUS_TPM_20_E_SIGNATURE: NTSTATUS = NTSTATUS(0xC029009B_u32 as _);
pub const STATUS_TPM_20_E_SIZE: NTSTATUS = NTSTATUS(0xC0290095_u32 as _);
pub const STATUS_TPM_20_E_SYMMETRIC: NTSTATUS = NTSTATUS(0xC0290096_u32 as _);
pub const STATUS_TPM_20_E_TAG: NTSTATUS = NTSTATUS(0xC0290097_u32 as _);
pub const STATUS_TPM_20_E_TICKET: NTSTATUS = NTSTATUS(0xC02900A0_u32 as _);
pub const STATUS_TPM_20_E_TOO_MANY_CONTEXTS: NTSTATUS = NTSTATUS(0xC029012E_u32 as _);
pub const STATUS_TPM_20_E_TYPE: NTSTATUS = NTSTATUS(0xC029008A_u32 as _);
pub const STATUS_TPM_20_E_UNBALANCED: NTSTATUS = NTSTATUS(0xC0290131_u32 as _);
pub const STATUS_TPM_20_E_UPGRADE: NTSTATUS = NTSTATUS(0xC029012D_u32 as _);
pub const STATUS_TPM_20_E_VALUE: NTSTATUS = NTSTATUS(0xC0290084_u32 as _);
pub const STATUS_TPM_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC0291004_u32 as _);
pub const STATUS_TPM_AREA_LOCKED: NTSTATUS = NTSTATUS(0xC029003C_u32 as _);
pub const STATUS_TPM_AUDITFAILURE: NTSTATUS = NTSTATUS(0xC0290004_u32 as _);
pub const STATUS_TPM_AUDITFAIL_SUCCESSFUL: NTSTATUS = NTSTATUS(0xC0290031_u32 as _);
pub const STATUS_TPM_AUDITFAIL_UNSUCCESSFUL: NTSTATUS = NTSTATUS(0xC0290030_u32 as _);
pub const STATUS_TPM_AUTH2FAIL: NTSTATUS = NTSTATUS(0xC029001D_u32 as _);
pub const STATUS_TPM_AUTHFAIL: NTSTATUS = NTSTATUS(0xC0290001_u32 as _);
pub const STATUS_TPM_AUTH_CONFLICT: NTSTATUS = NTSTATUS(0xC029003B_u32 as _);
pub const STATUS_TPM_BADCONTEXT: NTSTATUS = NTSTATUS(0xC029005A_u32 as _);
pub const STATUS_TPM_BADINDEX: NTSTATUS = NTSTATUS(0xC0290002_u32 as _);
pub const STATUS_TPM_BADTAG: NTSTATUS = NTSTATUS(0xC029001E_u32 as _);
pub const STATUS_TPM_BAD_ATTRIBUTES: NTSTATUS = NTSTATUS(0xC0290042_u32 as _);
pub const STATUS_TPM_BAD_COUNTER: NTSTATUS = NTSTATUS(0xC0290045_u32 as _);
pub const STATUS_TPM_BAD_DATASIZE: NTSTATUS = NTSTATUS(0xC029002B_u32 as _);
pub const STATUS_TPM_BAD_DELEGATE: NTSTATUS = NTSTATUS(0xC0290059_u32 as _);
pub const STATUS_TPM_BAD_HANDLE: NTSTATUS = NTSTATUS(0xC0290058_u32 as _);
pub const STATUS_TPM_BAD_KEY_PROPERTY: NTSTATUS = NTSTATUS(0xC0290028_u32 as _);
pub const STATUS_TPM_BAD_LOCALITY: NTSTATUS = NTSTATUS(0xC029003D_u32 as _);
pub const STATUS_TPM_BAD_MIGRATION: NTSTATUS = NTSTATUS(0xC0290029_u32 as _);
pub const STATUS_TPM_BAD_MODE: NTSTATUS = NTSTATUS(0xC029002C_u32 as _);
pub const STATUS_TPM_BAD_ORDINAL: NTSTATUS = NTSTATUS(0xC029000A_u32 as _);
pub const STATUS_TPM_BAD_PARAMETER: NTSTATUS = NTSTATUS(0xC0290003_u32 as _);
pub const STATUS_TPM_BAD_PARAM_SIZE: NTSTATUS = NTSTATUS(0xC0290019_u32 as _);
pub const STATUS_TPM_BAD_PRESENCE: NTSTATUS = NTSTATUS(0xC029002D_u32 as _);
pub const STATUS_TPM_BAD_SCHEME: NTSTATUS = NTSTATUS(0xC029002A_u32 as _);
pub const STATUS_TPM_BAD_SIGNATURE: NTSTATUS = NTSTATUS(0xC0290062_u32 as _);
pub const STATUS_TPM_BAD_TYPE: NTSTATUS = NTSTATUS(0xC0290034_u32 as _);
pub const STATUS_TPM_BAD_VERSION: NTSTATUS = NTSTATUS(0xC029002E_u32 as _);
pub const STATUS_TPM_CLEAR_DISABLED: NTSTATUS = NTSTATUS(0xC0290005_u32 as _);
pub const STATUS_TPM_COMMAND_BLOCKED: NTSTATUS = NTSTATUS(0xC0290400_u32 as _);
pub const STATUS_TPM_COMMAND_CANCELED: NTSTATUS = NTSTATUS(0xC0291001_u32 as _);
pub const STATUS_TPM_CONTEXT_GAP: NTSTATUS = NTSTATUS(0xC0290047_u32 as _);
pub const STATUS_TPM_DAA_INPUT_DATA0: NTSTATUS = NTSTATUS(0xC0290051_u32 as _);
pub const STATUS_TPM_DAA_INPUT_DATA1: NTSTATUS = NTSTATUS(0xC0290052_u32 as _);
pub const STATUS_TPM_DAA_ISSUER_SETTINGS: NTSTATUS = NTSTATUS(0xC0290053_u32 as _);
pub const STATUS_TPM_DAA_ISSUER_VALIDITY: NTSTATUS = NTSTATUS(0xC0290056_u32 as _);
pub const STATUS_TPM_DAA_RESOURCES: NTSTATUS = NTSTATUS(0xC0290050_u32 as _);
pub const STATUS_TPM_DAA_STAGE: NTSTATUS = NTSTATUS(0xC0290055_u32 as _);
pub const STATUS_TPM_DAA_TPM_SETTINGS: NTSTATUS = NTSTATUS(0xC0290054_u32 as _);
pub const STATUS_TPM_DAA_WRONG_W: NTSTATUS = NTSTATUS(0xC0290057_u32 as _);
pub const STATUS_TPM_DEACTIVATED: NTSTATUS = NTSTATUS(0xC0290006_u32 as _);
pub const STATUS_TPM_DECRYPT_ERROR: NTSTATUS = NTSTATUS(0xC0290021_u32 as _);
pub const STATUS_TPM_DEFEND_LOCK_RUNNING: NTSTATUS = NTSTATUS(0xC0290803_u32 as _);
pub const STATUS_TPM_DELEGATE_ADMIN: NTSTATUS = NTSTATUS(0xC029004D_u32 as _);
pub const STATUS_TPM_DELEGATE_FAMILY: NTSTATUS = NTSTATUS(0xC029004C_u32 as _);
pub const STATUS_TPM_DELEGATE_LOCK: NTSTATUS = NTSTATUS(0xC029004B_u32 as _);
pub const STATUS_TPM_DISABLED: NTSTATUS = NTSTATUS(0xC0290007_u32 as _);
pub const STATUS_TPM_DISABLED_CMD: NTSTATUS = NTSTATUS(0xC0290008_u32 as _);
pub const STATUS_TPM_DOING_SELFTEST: NTSTATUS = NTSTATUS(0xC0290802_u32 as _);
pub const STATUS_TPM_DUPLICATE_VHANDLE: NTSTATUS = NTSTATUS(0xC0290402_u32 as _);
pub const STATUS_TPM_EMBEDDED_COMMAND_BLOCKED: NTSTATUS = NTSTATUS(0xC0290403_u32 as _);
pub const STATUS_TPM_EMBEDDED_COMMAND_UNSUPPORTED: NTSTATUS = NTSTATUS(0xC0290404_u32 as _);
pub const STATUS_TPM_ENCRYPT_ERROR: NTSTATUS = NTSTATUS(0xC0290020_u32 as _);
pub const STATUS_TPM_ERROR_MASK: NTSTATUS = NTSTATUS(0xC0290000_u32 as _);
pub const STATUS_TPM_FAIL: NTSTATUS = NTSTATUS(0xC0290009_u32 as _);
pub const STATUS_TPM_FAILEDSELFTEST: NTSTATUS = NTSTATUS(0xC029001C_u32 as _);
pub const STATUS_TPM_FAMILYCOUNT: NTSTATUS = NTSTATUS(0xC0290040_u32 as _);
pub const STATUS_TPM_INAPPROPRIATE_ENC: NTSTATUS = NTSTATUS(0xC029000E_u32 as _);
pub const STATUS_TPM_INAPPROPRIATE_SIG: NTSTATUS = NTSTATUS(0xC0290027_u32 as _);
pub const STATUS_TPM_INSTALL_DISABLED: NTSTATUS = NTSTATUS(0xC029000B_u32 as _);
pub const STATUS_TPM_INSUFFICIENT_BUFFER: NTSTATUS = NTSTATUS(0xC0291005_u32 as _);
pub const STATUS_TPM_INVALID_AUTHHANDLE: NTSTATUS = NTSTATUS(0xC0290022_u32 as _);
pub const STATUS_TPM_INVALID_FAMILY: NTSTATUS = NTSTATUS(0xC0290037_u32 as _);
pub const STATUS_TPM_INVALID_HANDLE: NTSTATUS = NTSTATUS(0xC0290401_u32 as _);
pub const STATUS_TPM_INVALID_KEYHANDLE: NTSTATUS = NTSTATUS(0xC029000C_u32 as _);
pub const STATUS_TPM_INVALID_KEYUSAGE: NTSTATUS = NTSTATUS(0xC0290024_u32 as _);
pub const STATUS_TPM_INVALID_PCR_INFO: NTSTATUS = NTSTATUS(0xC0290010_u32 as _);
pub const STATUS_TPM_INVALID_POSTINIT: NTSTATUS = NTSTATUS(0xC0290026_u32 as _);
pub const STATUS_TPM_INVALID_RESOURCE: NTSTATUS = NTSTATUS(0xC0290035_u32 as _);
pub const STATUS_TPM_INVALID_STRUCTURE: NTSTATUS = NTSTATUS(0xC0290043_u32 as _);
pub const STATUS_TPM_IOERROR: NTSTATUS = NTSTATUS(0xC029001F_u32 as _);
pub const STATUS_TPM_KEYNOTFOUND: NTSTATUS = NTSTATUS(0xC029000D_u32 as _);
pub const STATUS_TPM_KEY_NOTSUPPORTED: NTSTATUS = NTSTATUS(0xC029003A_u32 as _);
pub const STATUS_TPM_KEY_OWNER_CONTROL: NTSTATUS = NTSTATUS(0xC0290044_u32 as _);
pub const STATUS_TPM_MAXNVWRITES: NTSTATUS = NTSTATUS(0xC0290048_u32 as _);
pub const STATUS_TPM_MA_AUTHORITY: NTSTATUS = NTSTATUS(0xC029005F_u32 as _);
pub const STATUS_TPM_MA_DESTINATION: NTSTATUS = NTSTATUS(0xC029005D_u32 as _);
pub const STATUS_TPM_MA_SOURCE: NTSTATUS = NTSTATUS(0xC029005E_u32 as _);
pub const STATUS_TPM_MA_TICKET_SIGNATURE: NTSTATUS = NTSTATUS(0xC029005C_u32 as _);
pub const STATUS_TPM_MIGRATEFAIL: NTSTATUS = NTSTATUS(0xC029000F_u32 as _);
pub const STATUS_TPM_NEEDS_SELFTEST: NTSTATUS = NTSTATUS(0xC0290801_u32 as _);
pub const STATUS_TPM_NOCONTEXTSPACE: NTSTATUS = NTSTATUS(0xC0290063_u32 as _);
pub const STATUS_TPM_NOOPERATOR: NTSTATUS = NTSTATUS(0xC0290049_u32 as _);
pub const STATUS_TPM_NOSPACE: NTSTATUS = NTSTATUS(0xC0290011_u32 as _);
pub const STATUS_TPM_NOSRK: NTSTATUS = NTSTATUS(0xC0290012_u32 as _);
pub const STATUS_TPM_NOTFIPS: NTSTATUS = NTSTATUS(0xC0290036_u32 as _);
pub const STATUS_TPM_NOTLOCAL: NTSTATUS = NTSTATUS(0xC0290033_u32 as _);
pub const STATUS_TPM_NOTRESETABLE: NTSTATUS = NTSTATUS(0xC0290032_u32 as _);
pub const STATUS_TPM_NOTSEALED_BLOB: NTSTATUS = NTSTATUS(0xC0290013_u32 as _);
pub const STATUS_TPM_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0291003_u32 as _);
pub const STATUS_TPM_NOT_FULLWRITE: NTSTATUS = NTSTATUS(0xC0290046_u32 as _);
pub const STATUS_TPM_NO_ENDORSEMENT: NTSTATUS = NTSTATUS(0xC0290023_u32 as _);
pub const STATUS_TPM_NO_NV_PERMISSION: NTSTATUS = NTSTATUS(0xC0290038_u32 as _);
pub const STATUS_TPM_NO_WRAP_TRANSPORT: NTSTATUS = NTSTATUS(0xC029002F_u32 as _);
pub const STATUS_TPM_OWNER_CONTROL: NTSTATUS = NTSTATUS(0xC029004F_u32 as _);
pub const STATUS_TPM_OWNER_SET: NTSTATUS = NTSTATUS(0xC0290014_u32 as _);
pub const STATUS_TPM_PERMANENTEK: NTSTATUS = NTSTATUS(0xC0290061_u32 as _);
pub const STATUS_TPM_PER_NOWRITE: NTSTATUS = NTSTATUS(0xC029003F_u32 as _);
pub const STATUS_TPM_PPI_FUNCTION_UNSUPPORTED: NTSTATUS = NTSTATUS(0xC0291006_u32 as _);
pub const STATUS_TPM_READ_ONLY: NTSTATUS = NTSTATUS(0xC029003E_u32 as _);
pub const STATUS_TPM_REQUIRES_SIGN: NTSTATUS = NTSTATUS(0xC0290039_u32 as _);
pub const STATUS_TPM_RESOURCEMISSING: NTSTATUS = NTSTATUS(0xC029004A_u32 as _);
pub const STATUS_TPM_RESOURCES: NTSTATUS = NTSTATUS(0xC0290015_u32 as _);
pub const STATUS_TPM_RETRY: NTSTATUS = NTSTATUS(0xC0290800_u32 as _);
pub const STATUS_TPM_SHA_ERROR: NTSTATUS = NTSTATUS(0xC029001B_u32 as _);
pub const STATUS_TPM_SHA_THREAD: NTSTATUS = NTSTATUS(0xC029001A_u32 as _);
pub const STATUS_TPM_SHORTRANDOM: NTSTATUS = NTSTATUS(0xC0290016_u32 as _);
pub const STATUS_TPM_SIZE: NTSTATUS = NTSTATUS(0xC0290017_u32 as _);
pub const STATUS_TPM_TOOMANYCONTEXTS: NTSTATUS = NTSTATUS(0xC029005B_u32 as _);
pub const STATUS_TPM_TOO_MANY_CONTEXTS: NTSTATUS = NTSTATUS(0xC0291002_u32 as _);
pub const STATUS_TPM_TRANSPORT_NOTEXCLUSIVE: NTSTATUS = NTSTATUS(0xC029004E_u32 as _);
pub const STATUS_TPM_WRITE_LOCKED: NTSTATUS = NTSTATUS(0xC0290041_u32 as _);
pub const STATUS_TPM_WRONGPCRVAL: NTSTATUS = NTSTATUS(0xC0290018_u32 as _);
pub const STATUS_TPM_WRONG_ENTITYTYPE: NTSTATUS = NTSTATUS(0xC0290025_u32 as _);
pub const STATUS_TPM_ZERO_EXHAUST_ENABLED: NTSTATUS = NTSTATUS(0xC0294000_u32 as _);
pub const STATUS_TRANSACTED_MAPPING_UNSUPPORTED_REMOTE: NTSTATUS = NTSTATUS(0xC0190040_u32 as _);
pub const STATUS_TRANSACTIONAL_CONFLICT: NTSTATUS = NTSTATUS(0xC0190001_u32 as _);
pub const STATUS_TRANSACTIONAL_OPEN_NOT_ALLOWED: NTSTATUS = NTSTATUS(0xC019003F_u32 as _);
pub const STATUS_TRANSACTIONMANAGER_IDENTITY_MISMATCH: NTSTATUS = NTSTATUS(0xC019005C_u32 as _);
pub const STATUS_TRANSACTIONMANAGER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0190051_u32 as _);
pub const STATUS_TRANSACTIONMANAGER_NOT_ONLINE: NTSTATUS = NTSTATUS(0xC0190052_u32 as _);
pub const STATUS_TRANSACTIONMANAGER_RECOVERY_NAME_COLLISION: NTSTATUS = NTSTATUS(0xC0190053_u32 as _);
pub const STATUS_TRANSACTIONS_NOT_FROZEN: NTSTATUS = NTSTATUS(0xC0190045_u32 as _);
pub const STATUS_TRANSACTIONS_UNSUPPORTED_REMOTE: NTSTATUS = NTSTATUS(0xC019000A_u32 as _);
pub const STATUS_TRANSACTION_ABORTED: NTSTATUS = NTSTATUS(0xC000020F_u32 as _);
pub const STATUS_TRANSACTION_ALREADY_ABORTED: NTSTATUS = NTSTATUS(0xC0190015_u32 as _);
pub const STATUS_TRANSACTION_ALREADY_COMMITTED: NTSTATUS = NTSTATUS(0xC0190016_u32 as _);
pub const STATUS_TRANSACTION_FREEZE_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0190046_u32 as _);
pub const STATUS_TRANSACTION_INTEGRITY_VIOLATED: NTSTATUS = NTSTATUS(0xC019005B_u32 as _);
pub const STATUS_TRANSACTION_INVALID_ID: NTSTATUS = NTSTATUS(0xC0000214_u32 as _);
pub const STATUS_TRANSACTION_INVALID_MARSHALL_BUFFER: NTSTATUS = NTSTATUS(0xC0190017_u32 as _);
pub const STATUS_TRANSACTION_INVALID_TYPE: NTSTATUS = NTSTATUS(0xC0000215_u32 as _);
pub const STATUS_TRANSACTION_MUST_WRITETHROUGH: NTSTATUS = NTSTATUS(0xC019005E_u32 as _);
pub const STATUS_TRANSACTION_NOT_ACTIVE: NTSTATUS = NTSTATUS(0xC0190003_u32 as _);
pub const STATUS_TRANSACTION_NOT_ENLISTED: NTSTATUS = NTSTATUS(0xC0190061_u32 as _);
pub const STATUS_TRANSACTION_NOT_FOUND: NTSTATUS = NTSTATUS(0xC019004E_u32 as _);
pub const STATUS_TRANSACTION_NOT_JOINED: NTSTATUS = NTSTATUS(0xC0190007_u32 as _);
pub const STATUS_TRANSACTION_NOT_REQUESTED: NTSTATUS = NTSTATUS(0xC0190014_u32 as _);
pub const STATUS_TRANSACTION_NOT_ROOT: NTSTATUS = NTSTATUS(0xC0190054_u32 as _);
pub const STATUS_TRANSACTION_NO_MATCH: NTSTATUS = NTSTATUS(0xC0000212_u32 as _);
pub const STATUS_TRANSACTION_NO_RELEASE: NTSTATUS = NTSTATUS(0xC0000211_u32 as _);
pub const STATUS_TRANSACTION_NO_SUPERIOR: NTSTATUS = NTSTATUS(0xC019005F_u32 as _);
pub const STATUS_TRANSACTION_OBJECT_EXPIRED: NTSTATUS = NTSTATUS(0xC0190055_u32 as _);
pub const STATUS_TRANSACTION_PROPAGATION_FAILED: NTSTATUS = NTSTATUS(0xC0190010_u32 as _);
pub const STATUS_TRANSACTION_RECORD_TOO_LONG: NTSTATUS = NTSTATUS(0xC0190058_u32 as _);
pub const STATUS_TRANSACTION_REQUEST_NOT_VALID: NTSTATUS = NTSTATUS(0xC0190013_u32 as _);
pub const STATUS_TRANSACTION_REQUIRED_PROMOTION: NTSTATUS = NTSTATUS(0xC0190043_u32 as _);
pub const STATUS_TRANSACTION_RESPONDED: NTSTATUS = NTSTATUS(0xC0000213_u32 as _);
pub const STATUS_TRANSACTION_RESPONSE_NOT_ENLISTED: NTSTATUS = NTSTATUS(0xC0190057_u32 as _);
pub const STATUS_TRANSACTION_SCOPE_CALLBACKS_NOT_SET: NTSTATUS = NTSTATUS(0x80190042_u32 as _);
pub const STATUS_TRANSACTION_SUPERIOR_EXISTS: NTSTATUS = NTSTATUS(0xC0190012_u32 as _);
pub const STATUS_TRANSACTION_TIMED_OUT: NTSTATUS = NTSTATUS(0xC0000210_u32 as _);
pub const STATUS_TRANSLATION_COMPLETE: NTSTATUS = NTSTATUS(0x120_u32 as _);
pub const STATUS_TRANSPORT_FULL: NTSTATUS = NTSTATUS(0xC00002CA_u32 as _);
pub const STATUS_TRIGGERED_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(0xC0000726_u32 as _);
pub const STATUS_TRIM_READ_ZERO_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0000472_u32 as _);
pub const STATUS_TRUSTED_DOMAIN_FAILURE: NTSTATUS = NTSTATUS(0xC000018C_u32 as _);
pub const STATUS_TRUSTED_RELATIONSHIP_FAILURE: NTSTATUS = NTSTATUS(0xC000018D_u32 as _);
pub const STATUS_TRUST_FAILURE: NTSTATUS = NTSTATUS(0xC0000190_u32 as _);
pub const STATUS_TS_INCOMPATIBLE_SESSIONS: NTSTATUS = NTSTATUS(0xC00A0039_u32 as _);
pub const STATUS_TS_VIDEO_SUBSYSTEM_ERROR: NTSTATUS = NTSTATUS(0xC00A003A_u32 as _);
pub const STATUS_TXF_ATTRIBUTE_CORRUPT: NTSTATUS = NTSTATUS(0xC019003D_u32 as _);
pub const STATUS_TXF_DIR_NOT_EMPTY: NTSTATUS = NTSTATUS(0xC0190039_u32 as _);
pub const STATUS_TXF_METADATA_ALREADY_PRESENT: NTSTATUS = NTSTATUS(0x80190041_u32 as _);
pub const STATUS_UNABLE_TO_DECOMMIT_VM: NTSTATUS = NTSTATUS(0xC000002C_u32 as _);
pub const STATUS_UNABLE_TO_DELETE_SECTION: NTSTATUS = NTSTATUS(0xC000001B_u32 as _);
pub const STATUS_UNABLE_TO_FREE_VM: NTSTATUS = NTSTATUS(0xC000001A_u32 as _);
pub const STATUS_UNABLE_TO_LOCK_MEDIA: NTSTATUS = NTSTATUS(0xC0000175_u32 as _);
pub const STATUS_UNABLE_TO_UNLOAD_MEDIA: NTSTATUS = NTSTATUS(0xC0000176_u32 as _);
pub const STATUS_UNDEFINED_CHARACTER: NTSTATUS = NTSTATUS(0xC0000163_u32 as _);
pub const STATUS_UNDEFINED_SCOPE: NTSTATUS = NTSTATUS(0xC0000504_u32 as _);
pub const STATUS_UNEXPECTED_IO_ERROR: NTSTATUS = NTSTATUS(0xC00000E9_u32 as _);
pub const STATUS_UNEXPECTED_MM_CREATE_ERR: NTSTATUS = NTSTATUS(0xC00000EA_u32 as _);
pub const STATUS_UNEXPECTED_MM_EXTEND_ERR: NTSTATUS = NTSTATUS(0xC00000EC_u32 as _);
pub const STATUS_UNEXPECTED_MM_MAP_ERROR: NTSTATUS = NTSTATUS(0xC00000EB_u32 as _);
pub const STATUS_UNEXPECTED_NETWORK_ERROR: NTSTATUS = NTSTATUS(0xC00000C4_u32 as _);
pub const STATUS_UNFINISHED_CONTEXT_DELETED: NTSTATUS = NTSTATUS(0xC00002EE_u32 as _);
pub const STATUS_UNHANDLED_EXCEPTION: NTSTATUS = NTSTATUS(0xC0000144_u32 as _);
pub const STATUS_UNKNOWN_REVISION: NTSTATUS = NTSTATUS(0xC0000058_u32 as _);
pub const STATUS_UNMAPPABLE_CHARACTER: NTSTATUS = NTSTATUS(0xC0000162_u32 as _);
pub const STATUS_UNRECOGNIZED_MEDIA: NTSTATUS = NTSTATUS(0xC0000014_u32 as _);
pub const STATUS_UNRECOGNIZED_VOLUME: NTSTATUS = NTSTATUS(0xC000014F_u32 as _);
pub const STATUS_UNSATISFIED_DEPENDENCIES: NTSTATUS = NTSTATUS(0xC00004B9_u32 as _);
pub const STATUS_UNSUCCESSFUL: NTSTATUS = NTSTATUS(0xC0000001_u32 as _);
pub const STATUS_UNSUPPORTED_COMPRESSION: NTSTATUS = NTSTATUS(0xC000025F_u32 as _);
pub const STATUS_UNSUPPORTED_PAGING_MODE: NTSTATUS = NTSTATUS(0xC00004BB_u32 as _);
pub const STATUS_UNSUPPORTED_PREAUTH: NTSTATUS = NTSTATUS(0xC0000351_u32 as _);
pub const STATUS_UNTRUSTED_MOUNT_POINT: NTSTATUS = NTSTATUS(0xC00004BC_u32 as _);
pub const STATUS_UNWIND: NTSTATUS = NTSTATUS(0xC0000027_u32 as _);
pub const STATUS_UNWIND_CONSOLIDATE: NTSTATUS = NTSTATUS(0x80000029_u32 as _);
pub const STATUS_USER2USER_REQUIRED: NTSTATUS = NTSTATUS(0xC0000408_u32 as _);
pub const STATUS_USER_APC: NTSTATUS = NTSTATUS(0xC0_u32 as _);
pub const STATUS_USER_DELETE_TRUST_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(0xC0000403_u32 as _);
pub const STATUS_USER_EXISTS: NTSTATUS = NTSTATUS(0xC0000063_u32 as _);
pub const STATUS_USER_MAPPED_FILE: NTSTATUS = NTSTATUS(0xC0000243_u32 as _);
pub const STATUS_USER_SESSION_DELETED: NTSTATUS = NTSTATUS(0xC0000203_u32 as _);
pub const STATUS_VALIDATE_CONTINUE: NTSTATUS = NTSTATUS(0xC0000271_u32 as _);
pub const STATUS_VALID_CATALOG_HASH: NTSTATUS = NTSTATUS(0x12D_u32 as _);
pub const STATUS_VALID_IMAGE_HASH: NTSTATUS = NTSTATUS(0x12C_u32 as _);
pub const STATUS_VALID_STRONG_CODE_HASH: NTSTATUS = NTSTATUS(0x12E_u32 as _);
pub const STATUS_VARIABLE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000100_u32 as _);
pub const STATUS_VDM_DISALLOWED: NTSTATUS = NTSTATUS(0xC0000414_u32 as _);
pub const STATUS_VDM_HARD_ERROR: NTSTATUS = NTSTATUS(0xC000021D_u32 as _);
pub const STATUS_VERIFIER_STOP: NTSTATUS = NTSTATUS(0xC0000421_u32 as _);
pub const STATUS_VERIFY_REQUIRED: NTSTATUS = NTSTATUS(0x80000016_u32 as _);
pub const STATUS_VHDSET_BACKING_STORAGE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC05CFF0C_u32 as _);
pub const STATUS_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE: NTSTATUS = NTSTATUS(0xC03A0033_u32 as _);
pub const STATUS_VHD_BITMAP_MISMATCH: NTSTATUS = NTSTATUS(0xC03A000C_u32 as _);
pub const STATUS_VHD_BLOCK_ALLOCATION_FAILURE: NTSTATUS = NTSTATUS(0xC03A0009_u32 as _);
pub const STATUS_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT: NTSTATUS = NTSTATUS(0xC03A000A_u32 as _);
pub const STATUS_VHD_CHANGE_TRACKING_DISABLED: NTSTATUS = NTSTATUS(0xC03A002A_u32 as _);
pub const STATUS_VHD_CHILD_PARENT_ID_MISMATCH: NTSTATUS = NTSTATUS(0xC03A000E_u32 as _);
pub const STATUS_VHD_CHILD_PARENT_SIZE_MISMATCH: NTSTATUS = NTSTATUS(0xC03A0017_u32 as _);
pub const STATUS_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH: NTSTATUS = NTSTATUS(0xC03A000F_u32 as _);
pub const STATUS_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE: NTSTATUS = NTSTATUS(0xC03A0032_u32 as _);
pub const STATUS_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED: NTSTATUS = NTSTATUS(0xC03A0018_u32 as _);
pub const STATUS_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT: NTSTATUS = NTSTATUS(0xC03A0019_u32 as _);
pub const STATUS_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(0xC03A0002_u32 as _);
pub const STATUS_VHD_DRIVE_FOOTER_CORRUPT: NTSTATUS = NTSTATUS(0xC03A0003_u32 as _);
pub const STATUS_VHD_DRIVE_FOOTER_MISSING: NTSTATUS = NTSTATUS(0xC03A0001_u32 as _);
pub const STATUS_VHD_FORMAT_UNKNOWN: NTSTATUS = NTSTATUS(0xC03A0004_u32 as _);
pub const STATUS_VHD_FORMAT_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(0xC03A0005_u32 as _);
pub const STATUS_VHD_INVALID_BLOCK_SIZE: NTSTATUS = NTSTATUS(0xC03A000B_u32 as _);
pub const STATUS_VHD_INVALID_CHANGE_TRACKING_ID: NTSTATUS = NTSTATUS(0xC03A0029_u32 as _);
pub const STATUS_VHD_INVALID_FILE_SIZE: NTSTATUS = NTSTATUS(0xC03A0013_u32 as _);
pub const STATUS_VHD_INVALID_SIZE: NTSTATUS = NTSTATUS(0xC03A0012_u32 as _);
pub const STATUS_VHD_INVALID_STATE: NTSTATUS = NTSTATUS(0xC03A001C_u32 as _);
pub const STATUS_VHD_INVALID_TYPE: NTSTATUS = NTSTATUS(0xC03A001B_u32 as _);
pub const STATUS_VHD_METADATA_FULL: NTSTATUS = NTSTATUS(0xC03A0028_u32 as _);
pub const STATUS_VHD_METADATA_READ_FAILURE: NTSTATUS = NTSTATUS(0xC03A0010_u32 as _);
pub const STATUS_VHD_METADATA_WRITE_FAILURE: NTSTATUS = NTSTATUS(0xC03A0011_u32 as _);
pub const STATUS_VHD_MISSING_CHANGE_TRACKING_INFORMATION: NTSTATUS = NTSTATUS(0xC03A0030_u32 as _);
pub const STATUS_VHD_PARENT_VHD_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC03A0016_u32 as _);
pub const STATUS_VHD_PARENT_VHD_NOT_FOUND: NTSTATUS = NTSTATUS(0xC03A000D_u32 as _);
pub const STATUS_VHD_RESIZE_WOULD_TRUNCATE_DATA: NTSTATUS = NTSTATUS(0xC03A0031_u32 as _);
pub const STATUS_VHD_SHARED: NTSTATUS = NTSTATUS(0xC05CFF0A_u32 as _);
pub const STATUS_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(0xC03A0006_u32 as _);
pub const STATUS_VHD_SPARSE_HEADER_CORRUPT: NTSTATUS = NTSTATUS(0xC03A0008_u32 as _);
pub const STATUS_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(0xC03A0007_u32 as _);
pub const STATUS_VHD_UNEXPECTED_ID: NTSTATUS = NTSTATUS(0xC03A0034_u32 as _);
pub const STATUS_VIDEO_DRIVER_DEBUG_REPORT_REQUEST: NTSTATUS = NTSTATUS(0x401B00EC_u32 as _);
pub const STATUS_VIDEO_HUNG_DISPLAY_DRIVER_THREAD: NTSTATUS = NTSTATUS(0xC01B00EA_u32 as _);
pub const STATUS_VIDEO_HUNG_DISPLAY_DRIVER_THREAD_RECOVERED: NTSTATUS = NTSTATUS(0x801B00EB_u32 as _);
pub const STATUS_VID_CHILD_GPA_PAGE_SET_CORRUPTED: NTSTATUS = NTSTATUS(0xC037000E_u32 as _);
pub const STATUS_VID_DUPLICATE_HANDLER: NTSTATUS = NTSTATUS(0xC0370001_u32 as _);
pub const STATUS_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT: NTSTATUS = NTSTATUS(0xC037001E_u32 as _);
pub const STATUS_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT: NTSTATUS = NTSTATUS(0xC037000C_u32 as _);
pub const STATUS_VID_HANDLER_NOT_PRESENT: NTSTATUS = NTSTATUS(0xC0370004_u32 as _);
pub const STATUS_VID_INSUFFICIENT_RESOURCES_HV_DEPOSIT: NTSTATUS = NTSTATUS(0xC037002D_u32 as _);
pub const STATUS_VID_INSUFFICIENT_RESOURCES_PHYSICAL_BUFFER: NTSTATUS = NTSTATUS(0xC037002C_u32 as _);
pub const STATUS_VID_INSUFFICIENT_RESOURCES_RESERVE: NTSTATUS = NTSTATUS(0xC037002B_u32 as _);
pub const STATUS_VID_INSUFFICIENT_RESOURCES_WITHDRAW: NTSTATUS = NTSTATUS(0xC037002F_u32 as _);
pub const STATUS_VID_INVALID_CHILD_GPA_PAGE_SET: NTSTATUS = NTSTATUS(0xC0370022_u32 as _);
pub const STATUS_VID_INVALID_GPA_RANGE_HANDLE: NTSTATUS = NTSTATUS(0xC0370015_u32 as _);
pub const STATUS_VID_INVALID_MEMORY_BLOCK_HANDLE: NTSTATUS = NTSTATUS(0xC0370012_u32 as _);
pub const STATUS_VID_INVALID_MESSAGE_QUEUE_HANDLE: NTSTATUS = NTSTATUS(0xC0370014_u32 as _);
pub const STATUS_VID_INVALID_NUMA_NODE_INDEX: NTSTATUS = NTSTATUS(0xC0370010_u32 as _);
pub const STATUS_VID_INVALID_NUMA_SETTINGS: NTSTATUS = NTSTATUS(0xC037000F_u32 as _);
pub const STATUS_VID_INVALID_OBJECT_NAME: NTSTATUS = NTSTATUS(0xC0370005_u32 as _);
pub const STATUS_VID_INVALID_PPM_HANDLE: NTSTATUS = NTSTATUS(0xC0370018_u32 as _);
pub const STATUS_VID_INVALID_PROCESSOR_STATE: NTSTATUS = NTSTATUS(0xC037001D_u32 as _);
pub const STATUS_VID_KM_INTERFACE_ALREADY_INITIALIZED: NTSTATUS = NTSTATUS(0xC037001F_u32 as _);
pub const STATUS_VID_MBPS_ARE_LOCKED: NTSTATUS = NTSTATUS(0xC0370019_u32 as _);
pub const STATUS_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE: NTSTATUS = NTSTATUS(0xC0370025_u32 as _);
pub const STATUS_VID_MBP_COUNT_EXCEEDED_LIMIT: NTSTATUS = NTSTATUS(0xC0370026_u32 as _);
pub const STATUS_VID_MB_PROPERTY_ALREADY_SET_RESET: NTSTATUS = NTSTATUS(0xC0370020_u32 as _);
pub const STATUS_VID_MB_STILL_REFERENCED: NTSTATUS = NTSTATUS(0xC037000D_u32 as _);
pub const STATUS_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED: NTSTATUS = NTSTATUS(0xC0370017_u32 as _);
pub const STATUS_VID_MEMORY_TYPE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC037002E_u32 as _);
pub const STATUS_VID_MESSAGE_QUEUE_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC037000B_u32 as _);
pub const STATUS_VID_MESSAGE_QUEUE_CLOSED: NTSTATUS = NTSTATUS(0xC037001A_u32 as _);
pub const STATUS_VID_MESSAGE_QUEUE_NAME_TOO_LONG: NTSTATUS = NTSTATUS(0xC0370007_u32 as _);
pub const STATUS_VID_MMIO_RANGE_DESTROYED: NTSTATUS = NTSTATUS(0xC0370021_u32 as _);
pub const STATUS_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED: NTSTATUS = NTSTATUS(0xC0370011_u32 as _);
pub const STATUS_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE: NTSTATUS = NTSTATUS(0xC0370016_u32 as _);
pub const STATUS_VID_PAGE_RANGE_OVERFLOW: NTSTATUS = NTSTATUS(0xC0370013_u32 as _);
pub const STATUS_VID_PARTITION_ALREADY_EXISTS: NTSTATUS = NTSTATUS(0xC0370008_u32 as _);
pub const STATUS_VID_PARTITION_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(0xC0370009_u32 as _);
pub const STATUS_VID_PARTITION_NAME_NOT_FOUND: NTSTATUS = NTSTATUS(0xC037000A_u32 as _);
pub const STATUS_VID_PARTITION_NAME_TOO_LONG: NTSTATUS = NTSTATUS(0xC0370006_u32 as _);
pub const STATUS_VID_PROCESS_ALREADY_SET: NTSTATUS = NTSTATUS(0xC0370030_u32 as _);
pub const STATUS_VID_QUEUE_FULL: NTSTATUS = NTSTATUS(0xC0370003_u32 as _);
pub const STATUS_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED: NTSTATUS = NTSTATUS(0x80370001_u32 as _);
pub const STATUS_VID_RESERVE_PAGE_SET_IS_BEING_USED: NTSTATUS = NTSTATUS(0xC0370023_u32 as _);
pub const STATUS_VID_RESERVE_PAGE_SET_TOO_SMALL: NTSTATUS = NTSTATUS(0xC0370024_u32 as _);
pub const STATUS_VID_SAVED_STATE_CORRUPT: NTSTATUS = NTSTATUS(0xC0370027_u32 as _);
pub const STATUS_VID_SAVED_STATE_INCOMPATIBLE: NTSTATUS = NTSTATUS(0xC0370029_u32 as _);
pub const STATUS_VID_SAVED_STATE_UNRECOGNIZED_ITEM: NTSTATUS = NTSTATUS(0xC0370028_u32 as _);
pub const STATUS_VID_STOP_PENDING: NTSTATUS = NTSTATUS(0xC037001C_u32 as _);
pub const STATUS_VID_TOO_MANY_HANDLERS: NTSTATUS = NTSTATUS(0xC0370002_u32 as _);
pub const STATUS_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(0xC037001B_u32 as _);
pub const STATUS_VID_VTL_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC037002A_u32 as _);
pub const STATUS_VIRTDISK_DISK_ALREADY_OWNED: NTSTATUS = NTSTATUS(0xC03A001E_u32 as _);
pub const STATUS_VIRTDISK_DISK_ONLINE_AND_WRITABLE: NTSTATUS = NTSTATUS(0xC03A001F_u32 as _);
pub const STATUS_VIRTDISK_NOT_VIRTUAL_DISK: NTSTATUS = NTSTATUS(0xC03A0015_u32 as _);
pub const STATUS_VIRTDISK_PROVIDER_NOT_FOUND: NTSTATUS = NTSTATUS(0xC03A0014_u32 as _);
pub const STATUS_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE: NTSTATUS = NTSTATUS(0xC03A001D_u32 as _);
pub const STATUS_VIRTUAL_CIRCUIT_CLOSED: NTSTATUS = NTSTATUS(0xC00000D6_u32 as _);
pub const STATUS_VIRTUAL_DISK_LIMITATION: NTSTATUS = NTSTATUS(0xC03A001A_u32 as _);
pub const STATUS_VIRUS_DELETED: NTSTATUS = NTSTATUS(0xC0000907_u32 as _);
pub const STATUS_VIRUS_INFECTED: NTSTATUS = NTSTATUS(0xC0000906_u32 as _);
pub const STATUS_VOLMGR_ALL_DISKS_FAILED: NTSTATUS = NTSTATUS(0xC0380029_u32 as _);
pub const STATUS_VOLMGR_BAD_BOOT_DISK: NTSTATUS = NTSTATUS(0xC038004F_u32 as _);
pub const STATUS_VOLMGR_DATABASE_FULL: NTSTATUS = NTSTATUS(0xC0380001_u32 as _);
pub const STATUS_VOLMGR_DIFFERENT_SECTOR_SIZE: NTSTATUS = NTSTATUS(0xC038004E_u32 as _);
pub const STATUS_VOLMGR_DISK_CONFIGURATION_CORRUPTED: NTSTATUS = NTSTATUS(0xC0380002_u32 as _);
pub const STATUS_VOLMGR_DISK_CONFIGURATION_NOT_IN_SYNC: NTSTATUS = NTSTATUS(0xC0380003_u32 as _);
pub const STATUS_VOLMGR_DISK_CONTAINS_NON_SIMPLE_VOLUME: NTSTATUS = NTSTATUS(0xC0380005_u32 as _);
pub const STATUS_VOLMGR_DISK_DUPLICATE: NTSTATUS = NTSTATUS(0xC0380006_u32 as _);
pub const STATUS_VOLMGR_DISK_DYNAMIC: NTSTATUS = NTSTATUS(0xC0380007_u32 as _);
pub const STATUS_VOLMGR_DISK_ID_INVALID: NTSTATUS = NTSTATUS(0xC0380008_u32 as _);
pub const STATUS_VOLMGR_DISK_INVALID: NTSTATUS = NTSTATUS(0xC0380009_u32 as _);
pub const STATUS_VOLMGR_DISK_LAST_VOTER: NTSTATUS = NTSTATUS(0xC038000A_u32 as _);
pub const STATUS_VOLMGR_DISK_LAYOUT_INVALID: NTSTATUS = NTSTATUS(0xC038000B_u32 as _);
pub const STATUS_VOLMGR_DISK_LAYOUT_NON_BASIC_BETWEEN_BASIC_PARTITIONS: NTSTATUS = NTSTATUS(0xC038000C_u32 as _);
pub const STATUS_VOLMGR_DISK_LAYOUT_NOT_CYLINDER_ALIGNED: NTSTATUS = NTSTATUS(0xC038000D_u32 as _);
pub const STATUS_VOLMGR_DISK_LAYOUT_PARTITIONS_TOO_SMALL: NTSTATUS = NTSTATUS(0xC038000E_u32 as _);
pub const STATUS_VOLMGR_DISK_LAYOUT_PRIMARY_BETWEEN_LOGICAL_PARTITIONS: NTSTATUS = NTSTATUS(0xC038000F_u32 as _);
pub const STATUS_VOLMGR_DISK_LAYOUT_TOO_MANY_PARTITIONS: NTSTATUS = NTSTATUS(0xC0380010_u32 as _);
pub const STATUS_VOLMGR_DISK_MISSING: NTSTATUS = NTSTATUS(0xC0380011_u32 as _);
pub const STATUS_VOLMGR_DISK_NOT_EMPTY: NTSTATUS = NTSTATUS(0xC0380012_u32 as _);
pub const STATUS_VOLMGR_DISK_NOT_ENOUGH_SPACE: NTSTATUS = NTSTATUS(0xC0380013_u32 as _);
pub const STATUS_VOLMGR_DISK_REVECTORING_FAILED: NTSTATUS = NTSTATUS(0xC0380014_u32 as _);
pub const STATUS_VOLMGR_DISK_SECTOR_SIZE_INVALID: NTSTATUS = NTSTATUS(0xC0380015_u32 as _);
pub const STATUS_VOLMGR_DISK_SET_NOT_CONTAINED: NTSTATUS = NTSTATUS(0xC0380016_u32 as _);
pub const STATUS_VOLMGR_DISK_USED_BY_MULTIPLE_MEMBERS: NTSTATUS = NTSTATUS(0xC0380017_u32 as _);
pub const STATUS_VOLMGR_DISK_USED_BY_MULTIPLE_PLEXES: NTSTATUS = NTSTATUS(0xC0380018_u32 as _);
pub const STATUS_VOLMGR_DYNAMIC_DISK_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0380019_u32 as _);
pub const STATUS_VOLMGR_EXTENT_ALREADY_USED: NTSTATUS = NTSTATUS(0xC038001A_u32 as _);
pub const STATUS_VOLMGR_EXTENT_NOT_CONTIGUOUS: NTSTATUS = NTSTATUS(0xC038001B_u32 as _);
pub const STATUS_VOLMGR_EXTENT_NOT_IN_PUBLIC_REGION: NTSTATUS = NTSTATUS(0xC038001C_u32 as _);
pub const STATUS_VOLMGR_EXTENT_NOT_SECTOR_ALIGNED: NTSTATUS = NTSTATUS(0xC038001D_u32 as _);
pub const STATUS_VOLMGR_EXTENT_OVERLAPS_EBR_PARTITION: NTSTATUS = NTSTATUS(0xC038001E_u32 as _);
pub const STATUS_VOLMGR_EXTENT_VOLUME_LENGTHS_DO_NOT_MATCH: NTSTATUS = NTSTATUS(0xC038001F_u32 as _);
pub const STATUS_VOLMGR_FAULT_TOLERANT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC0380020_u32 as _);
pub const STATUS_VOLMGR_INCOMPLETE_DISK_MIGRATION: NTSTATUS = NTSTATUS(0x80380002_u32 as _);
pub const STATUS_VOLMGR_INCOMPLETE_REGENERATION: NTSTATUS = NTSTATUS(0x80380001_u32 as _);
pub const STATUS_VOLMGR_INTERLEAVE_LENGTH_INVALID: NTSTATUS = NTSTATUS(0xC0380021_u32 as _);
pub const STATUS_VOLMGR_MAXIMUM_REGISTERED_USERS: NTSTATUS = NTSTATUS(0xC0380022_u32 as _);
pub const STATUS_VOLMGR_MEMBER_INDEX_DUPLICATE: NTSTATUS = NTSTATUS(0xC0380024_u32 as _);
pub const STATUS_VOLMGR_MEMBER_INDEX_INVALID: NTSTATUS = NTSTATUS(0xC0380025_u32 as _);
pub const STATUS_VOLMGR_MEMBER_IN_SYNC: NTSTATUS = NTSTATUS(0xC0380023_u32 as _);
pub const STATUS_VOLMGR_MEMBER_MISSING: NTSTATUS = NTSTATUS(0xC0380026_u32 as _);
pub const STATUS_VOLMGR_MEMBER_NOT_DETACHED: NTSTATUS = NTSTATUS(0xC0380027_u32 as _);
pub const STATUS_VOLMGR_MEMBER_REGENERATING: NTSTATUS = NTSTATUS(0xC0380028_u32 as _);
pub const STATUS_VOLMGR_MIRROR_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC038005B_u32 as _);
pub const STATUS_VOLMGR_NOTIFICATION_RESET: NTSTATUS = NTSTATUS(0xC038002C_u32 as _);
pub const STATUS_VOLMGR_NOT_PRIMARY_PACK: NTSTATUS = NTSTATUS(0xC0380052_u32 as _);
pub const STATUS_VOLMGR_NO_REGISTERED_USERS: NTSTATUS = NTSTATUS(0xC038002A_u32 as _);
pub const STATUS_VOLMGR_NO_SUCH_USER: NTSTATUS = NTSTATUS(0xC038002B_u32 as _);
pub const STATUS_VOLMGR_NO_VALID_LOG_COPIES: NTSTATUS = NTSTATUS(0xC0380058_u32 as _);
pub const STATUS_VOLMGR_NUMBER_OF_DISKS_INVALID: NTSTATUS = NTSTATUS(0xC038005A_u32 as _);
pub const STATUS_VOLMGR_NUMBER_OF_DISKS_IN_MEMBER_INVALID: NTSTATUS = NTSTATUS(0xC0380055_u32 as _);
pub const STATUS_VOLMGR_NUMBER_OF_DISKS_IN_PLEX_INVALID: NTSTATUS = NTSTATUS(0xC0380054_u32 as _);
pub const STATUS_VOLMGR_NUMBER_OF_EXTENTS_INVALID: NTSTATUS = NTSTATUS(0xC038004D_u32 as _);
pub const STATUS_VOLMGR_NUMBER_OF_MEMBERS_INVALID: NTSTATUS = NTSTATUS(0xC038002D_u32 as _);
pub const STATUS_VOLMGR_NUMBER_OF_PLEXES_INVALID: NTSTATUS = NTSTATUS(0xC038002E_u32 as _);
pub const STATUS_VOLMGR_PACK_CONFIG_OFFLINE: NTSTATUS = NTSTATUS(0xC0380050_u32 as _);
pub const STATUS_VOLMGR_PACK_CONFIG_ONLINE: NTSTATUS = NTSTATUS(0xC0380051_u32 as _);
pub const STATUS_VOLMGR_PACK_CONFIG_UPDATE_FAILED: NTSTATUS = NTSTATUS(0xC0380004_u32 as _);
pub const STATUS_VOLMGR_PACK_DUPLICATE: NTSTATUS = NTSTATUS(0xC038002F_u32 as _);
pub const STATUS_VOLMGR_PACK_HAS_QUORUM: NTSTATUS = NTSTATUS(0xC0380034_u32 as _);
pub const STATUS_VOLMGR_PACK_ID_INVALID: NTSTATUS = NTSTATUS(0xC0380030_u32 as _);
pub const STATUS_VOLMGR_PACK_INVALID: NTSTATUS = NTSTATUS(0xC0380031_u32 as _);
pub const STATUS_VOLMGR_PACK_LOG_UPDATE_FAILED: NTSTATUS = NTSTATUS(0xC0380053_u32 as _);
pub const STATUS_VOLMGR_PACK_NAME_INVALID: NTSTATUS = NTSTATUS(0xC0380032_u32 as _);
pub const STATUS_VOLMGR_PACK_OFFLINE: NTSTATUS = NTSTATUS(0xC0380033_u32 as _);
pub const STATUS_VOLMGR_PACK_WITHOUT_QUORUM: NTSTATUS = NTSTATUS(0xC0380035_u32 as _);
pub const STATUS_VOLMGR_PARTITION_STYLE_INVALID: NTSTATUS = NTSTATUS(0xC0380036_u32 as _);
pub const STATUS_VOLMGR_PARTITION_UPDATE_FAILED: NTSTATUS = NTSTATUS(0xC0380037_u32 as _);
pub const STATUS_VOLMGR_PLEX_INDEX_DUPLICATE: NTSTATUS = NTSTATUS(0xC0380039_u32 as _);
pub const STATUS_VOLMGR_PLEX_INDEX_INVALID: NTSTATUS = NTSTATUS(0xC038003A_u32 as _);
pub const STATUS_VOLMGR_PLEX_IN_SYNC: NTSTATUS = NTSTATUS(0xC0380038_u32 as _);
pub const STATUS_VOLMGR_PLEX_LAST_ACTIVE: NTSTATUS = NTSTATUS(0xC038003B_u32 as _);
pub const STATUS_VOLMGR_PLEX_MISSING: NTSTATUS = NTSTATUS(0xC038003C_u32 as _);
pub const STATUS_VOLMGR_PLEX_NOT_RAID5: NTSTATUS = NTSTATUS(0xC038003F_u32 as _);
pub const STATUS_VOLMGR_PLEX_NOT_SIMPLE: NTSTATUS = NTSTATUS(0xC0380040_u32 as _);
pub const STATUS_VOLMGR_PLEX_NOT_SIMPLE_SPANNED: NTSTATUS = NTSTATUS(0xC0380057_u32 as _);
pub const STATUS_VOLMGR_PLEX_REGENERATING: NTSTATUS = NTSTATUS(0xC038003D_u32 as _);
pub const STATUS_VOLMGR_PLEX_TYPE_INVALID: NTSTATUS = NTSTATUS(0xC038003E_u32 as _);
pub const STATUS_VOLMGR_PRIMARY_PACK_PRESENT: NTSTATUS = NTSTATUS(0xC0380059_u32 as _);
pub const STATUS_VOLMGR_RAID5_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC038005C_u32 as _);
pub const STATUS_VOLMGR_STRUCTURE_SIZE_INVALID: NTSTATUS = NTSTATUS(0xC0380041_u32 as _);
pub const STATUS_VOLMGR_TOO_MANY_NOTIFICATION_REQUESTS: NTSTATUS = NTSTATUS(0xC0380042_u32 as _);
pub const STATUS_VOLMGR_TRANSACTION_IN_PROGRESS: NTSTATUS = NTSTATUS(0xC0380043_u32 as _);
pub const STATUS_VOLMGR_UNEXPECTED_DISK_LAYOUT_CHANGE: NTSTATUS = NTSTATUS(0xC0380044_u32 as _);
pub const STATUS_VOLMGR_VOLUME_CONTAINS_MISSING_DISK: NTSTATUS = NTSTATUS(0xC0380045_u32 as _);
pub const STATUS_VOLMGR_VOLUME_ID_INVALID: NTSTATUS = NTSTATUS(0xC0380046_u32 as _);
pub const STATUS_VOLMGR_VOLUME_LENGTH_INVALID: NTSTATUS = NTSTATUS(0xC0380047_u32 as _);
pub const STATUS_VOLMGR_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: NTSTATUS = NTSTATUS(0xC0380048_u32 as _);
pub const STATUS_VOLMGR_VOLUME_MIRRORED: NTSTATUS = NTSTATUS(0xC0380056_u32 as _);
pub const STATUS_VOLMGR_VOLUME_NOT_MIRRORED: NTSTATUS = NTSTATUS(0xC0380049_u32 as _);
pub const STATUS_VOLMGR_VOLUME_NOT_RETAINED: NTSTATUS = NTSTATUS(0xC038004A_u32 as _);
pub const STATUS_VOLMGR_VOLUME_OFFLINE: NTSTATUS = NTSTATUS(0xC038004B_u32 as _);
pub const STATUS_VOLMGR_VOLUME_RETAINED: NTSTATUS = NTSTATUS(0xC038004C_u32 as _);
pub const STATUS_VOLSNAP_ACTIVATION_TIMEOUT: NTSTATUS = NTSTATUS(0xC0500004_u32 as _);
pub const STATUS_VOLSNAP_BOOTFILE_NOT_VALID: NTSTATUS = NTSTATUS(0xC0500003_u32 as _);
pub const STATUS_VOLSNAP_HIBERNATE_READY: NTSTATUS = NTSTATUS(0x125_u32 as _);
pub const STATUS_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT: NTSTATUS = NTSTATUS(0xC0500005_u32 as _);
pub const STATUS_VOLSNAP_PREPARE_HIBERNATE: NTSTATUS = NTSTATUS(0xC0000407_u32 as _);
pub const STATUS_VOLUME_DIRTY: NTSTATUS = NTSTATUS(0xC0000806_u32 as _);
pub const STATUS_VOLUME_DISMOUNTED: NTSTATUS = NTSTATUS(0xC000026E_u32 as _);
pub const STATUS_VOLUME_MOUNTED: NTSTATUS = NTSTATUS(0x109_u32 as _);
pub const STATUS_VOLUME_NOT_CLUSTER_ALIGNED: NTSTATUS = NTSTATUS(0xC00004A4_u32 as _);
pub const STATUS_VOLUME_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00004C6_u32 as _);
pub const STATUS_VOLUME_NOT_UPGRADED: NTSTATUS = NTSTATUS(0xC000029C_u32 as _);
pub const STATUS_VOLUME_UPGRADE_DISABLED: NTSTATUS = NTSTATUS(0xC00004DB_u32 as _);
pub const STATUS_VOLUME_UPGRADE_DISABLED_TILL_OS_DOWNGRADE_EXPIRED: NTSTATUS = NTSTATUS(0xC00004DC_u32 as _);
pub const STATUS_VOLUME_UPGRADE_NOT_NEEDED: NTSTATUS = NTSTATUS(0xC00004D9_u32 as _);
pub const STATUS_VOLUME_UPGRADE_PENDING: NTSTATUS = NTSTATUS(0xC00004DA_u32 as _);
pub const STATUS_VOLUME_WRITE_ACCESS_DENIED: NTSTATUS = NTSTATUS(0xC00004D3_u32 as _);
pub const STATUS_VRF_VOLATILE_CFG_AND_IO_ENABLED: NTSTATUS = NTSTATUS(0xC0000C08_u32 as _);
pub const STATUS_VRF_VOLATILE_NMI_REGISTERED: NTSTATUS = NTSTATUS(0xC0000C0E_u32 as _);
pub const STATUS_VRF_VOLATILE_NOT_RUNNABLE_SYSTEM: NTSTATUS = NTSTATUS(0xC0000C0B_u32 as _);
pub const STATUS_VRF_VOLATILE_NOT_STOPPABLE: NTSTATUS = NTSTATUS(0xC0000C09_u32 as _);
pub const STATUS_VRF_VOLATILE_NOT_SUPPORTED_RULECLASS: NTSTATUS = NTSTATUS(0xC0000C0C_u32 as _);
pub const STATUS_VRF_VOLATILE_PROTECTED_DRIVER: NTSTATUS = NTSTATUS(0xC0000C0D_u32 as _);
pub const STATUS_VRF_VOLATILE_SAFE_MODE: NTSTATUS = NTSTATUS(0xC0000C0A_u32 as _);
pub const STATUS_VRF_VOLATILE_SETTINGS_CONFLICT: NTSTATUS = NTSTATUS(0xC0000C0F_u32 as _);
pub const STATUS_VSM_DMA_PROTECTION_NOT_IN_USE: NTSTATUS = NTSTATUS(0xC0450001_u32 as _);
pub const STATUS_VSM_NOT_INITIALIZED: NTSTATUS = NTSTATUS(0xC0450000_u32 as _);
pub const STATUS_WAIT_0: NTSTATUS = NTSTATUS(0x0_u32 as _);
pub const STATUS_WAIT_1: NTSTATUS = NTSTATUS(0x1_u32 as _);
pub const STATUS_WAIT_2: NTSTATUS = NTSTATUS(0x2_u32 as _);
pub const STATUS_WAIT_3: NTSTATUS = NTSTATUS(0x3_u32 as _);
pub const STATUS_WAIT_63: NTSTATUS = NTSTATUS(0x3F_u32 as _);
pub const STATUS_WAIT_FOR_OPLOCK: NTSTATUS = NTSTATUS(0x367_u32 as _);
pub const STATUS_WAKE_SYSTEM: NTSTATUS = NTSTATUS(0x40000294_u32 as _);
pub const STATUS_WAKE_SYSTEM_DEBUGGER: NTSTATUS = NTSTATUS(0x80000007_u32 as _);
pub const STATUS_WAS_LOCKED: NTSTATUS = NTSTATUS(0x40000019_u32 as _);
pub const STATUS_WAS_UNLOCKED: NTSTATUS = NTSTATUS(0x40000017_u32 as _);
pub const STATUS_WEAK_WHFBKEY_BLOCKED: NTSTATUS = NTSTATUS(0xC00001B3_u32 as _);
pub const STATUS_WIM_NOT_BOOTABLE: NTSTATUS = NTSTATUS(0xC0000487_u32 as _);
pub const STATUS_WMI_ALREADY_DISABLED: NTSTATUS = NTSTATUS(0xC0000302_u32 as _);
pub const STATUS_WMI_ALREADY_ENABLED: NTSTATUS = NTSTATUS(0xC0000303_u32 as _);
pub const STATUS_WMI_GUID_DISCONNECTED: NTSTATUS = NTSTATUS(0xC0000301_u32 as _);
pub const STATUS_WMI_GUID_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000295_u32 as _);
pub const STATUS_WMI_INSTANCE_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000296_u32 as _);
pub const STATUS_WMI_ITEMID_NOT_FOUND: NTSTATUS = NTSTATUS(0xC0000297_u32 as _);
pub const STATUS_WMI_NOT_SUPPORTED: NTSTATUS = NTSTATUS(0xC00002DD_u32 as _);
pub const STATUS_WMI_READ_ONLY: NTSTATUS = NTSTATUS(0xC00002C6_u32 as _);
pub const STATUS_WMI_SET_FAILURE: NTSTATUS = NTSTATUS(0xC00002C7_u32 as _);
pub const STATUS_WMI_TRY_AGAIN: NTSTATUS = NTSTATUS(0xC0000298_u32 as _);
pub const STATUS_WOF_FILE_RESOURCE_TABLE_CORRUPT: NTSTATUS = NTSTATUS(0xC000A2A7_u32 as _);
pub const STATUS_WOF_WIM_HEADER_CORRUPT: NTSTATUS = NTSTATUS(0xC000A2A5_u32 as _);
pub const STATUS_WOF_WIM_RESOURCE_TABLE_CORRUPT: NTSTATUS = NTSTATUS(0xC000A2A6_u32 as _);
pub const STATUS_WORKING_SET_LIMIT_RANGE: NTSTATUS = NTSTATUS(0x40000002_u32 as _);
pub const STATUS_WORKING_SET_QUOTA: NTSTATUS = NTSTATUS(0xC00000A1_u32 as _);
pub const STATUS_WOW_ASSERTION: NTSTATUS = NTSTATUS(0xC0009898_u32 as _);
pub const STATUS_WRONG_COMPARTMENT: NTSTATUS = NTSTATUS(0xC000A085_u32 as _);
pub const STATUS_WRONG_CREDENTIAL_HANDLE: NTSTATUS = NTSTATUS(0xC00002F2_u32 as _);
pub const STATUS_WRONG_EFS: NTSTATUS = NTSTATUS(0xC000028F_u32 as _);
pub const STATUS_WRONG_PASSWORD_CORE: NTSTATUS = NTSTATUS(0xC0000149_u32 as _);
pub const STATUS_WRONG_VOLUME: NTSTATUS = NTSTATUS(0xC0000012_u32 as _);
pub const STATUS_WX86_BREAKPOINT: NTSTATUS = NTSTATUS(0x4000001F_u32 as _);
pub const STATUS_WX86_CONTINUE: NTSTATUS = NTSTATUS(0x4000001D_u32 as _);
pub const STATUS_WX86_CREATEWX86TIB: NTSTATUS = NTSTATUS(0x40000028_u32 as _);
pub const STATUS_WX86_EXCEPTION_CHAIN: NTSTATUS = NTSTATUS(0x40000022_u32 as _);
pub const STATUS_WX86_EXCEPTION_CONTINUE: NTSTATUS = NTSTATUS(0x40000020_u32 as _);
pub const STATUS_WX86_EXCEPTION_LASTCHANCE: NTSTATUS = NTSTATUS(0x40000021_u32 as _);
pub const STATUS_WX86_FLOAT_STACK_CHECK: NTSTATUS = NTSTATUS(0xC0000270_u32 as _);
pub const STATUS_WX86_INTERNAL_ERROR: NTSTATUS = NTSTATUS(0xC000026F_u32 as _);
pub const STATUS_WX86_SINGLE_STEP: NTSTATUS = NTSTATUS(0x4000001E_u32 as _);
pub const STATUS_WX86_UNSIMULATE: NTSTATUS = NTSTATUS(0x4000001C_u32 as _);
pub const STATUS_XMLDSIG_ERROR: NTSTATUS = NTSTATUS(0xC000A084_u32 as _);
pub const STATUS_XML_ENCODING_MISMATCH: NTSTATUS = NTSTATUS(0xC0150021_u32 as _);
pub const STATUS_XML_PARSE_ERROR: NTSTATUS = NTSTATUS(0xC000A083_u32 as _);
pub const STG_E_ABNORMALAPIEXIT: windows_core::HRESULT = windows_core::HRESULT(0x800300FA_u32 as _);
pub const STG_E_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80030005_u32 as _);
pub const STG_E_BADBASEADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80030110_u32 as _);
pub const STG_E_CANTSAVE: windows_core::HRESULT = windows_core::HRESULT(0x80030103_u32 as _);
pub const STG_E_CSS_AUTHENTICATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80030306_u32 as _);
pub const STG_E_CSS_KEY_NOT_ESTABLISHED: windows_core::HRESULT = windows_core::HRESULT(0x80030308_u32 as _);
pub const STG_E_CSS_KEY_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80030307_u32 as _);
pub const STG_E_CSS_REGION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8003030A_u32 as _);
pub const STG_E_CSS_SCRAMBLED_SECTOR: windows_core::HRESULT = windows_core::HRESULT(0x80030309_u32 as _);
pub const STG_E_DEVICE_UNRESPONSIVE: windows_core::HRESULT = windows_core::HRESULT(0x8003020A_u32 as _);
pub const STG_E_DISKISWRITEPROTECTED: windows_core::HRESULT = windows_core::HRESULT(0x80030013_u32 as _);
pub const STG_E_DOCFILECORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80030109_u32 as _);
pub const STG_E_DOCFILETOOLARGE: windows_core::HRESULT = windows_core::HRESULT(0x80030111_u32 as _);
pub const STG_E_EXTANTMARSHALLINGS: windows_core::HRESULT = windows_core::HRESULT(0x80030108_u32 as _);
pub const STG_E_FILEALREADYEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80030050_u32 as _);
pub const STG_E_FILENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80030002_u32 as _);
pub const STG_E_FIRMWARE_IMAGE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80030209_u32 as _);
pub const STG_E_FIRMWARE_SLOT_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80030208_u32 as _);
pub const STG_E_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80030201_u32 as _);
pub const STG_E_INSUFFICIENTMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80030008_u32 as _);
pub const STG_E_INUSE: windows_core::HRESULT = windows_core::HRESULT(0x80030100_u32 as _);
pub const STG_E_INVALIDFLAG: windows_core::HRESULT = windows_core::HRESULT(0x800300FF_u32 as _);
pub const STG_E_INVALIDFUNCTION: windows_core::HRESULT = windows_core::HRESULT(0x80030001_u32 as _);
pub const STG_E_INVALIDHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80030006_u32 as _);
pub const STG_E_INVALIDHEADER: windows_core::HRESULT = windows_core::HRESULT(0x800300FB_u32 as _);
pub const STG_E_INVALIDNAME: windows_core::HRESULT = windows_core::HRESULT(0x800300FC_u32 as _);
pub const STG_E_INVALIDPARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80030057_u32 as _);
pub const STG_E_INVALIDPOINTER: windows_core::HRESULT = windows_core::HRESULT(0x80030009_u32 as _);
pub const STG_E_LOCKVIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80030021_u32 as _);
pub const STG_E_MEDIUMFULL: windows_core::HRESULT = windows_core::HRESULT(0x80030070_u32 as _);
pub const STG_E_NOMOREFILES: windows_core::HRESULT = windows_core::HRESULT(0x80030012_u32 as _);
pub const STG_E_NOTCURRENT: windows_core::HRESULT = windows_core::HRESULT(0x80030101_u32 as _);
pub const STG_E_NOTFILEBASEDSTORAGE: windows_core::HRESULT = windows_core::HRESULT(0x80030107_u32 as _);
pub const STG_E_NOTSIMPLEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80030112_u32 as _);
pub const STG_E_OLDDLL: windows_core::HRESULT = windows_core::HRESULT(0x80030105_u32 as _);
pub const STG_E_OLDFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80030104_u32 as _);
pub const STG_E_PATHNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80030003_u32 as _);
pub const STG_E_PROPSETMISMATCHED: windows_core::HRESULT = windows_core::HRESULT(0x800300F0_u32 as _);
pub const STG_E_READFAULT: windows_core::HRESULT = windows_core::HRESULT(0x8003001E_u32 as _);
pub const STG_E_RESETS_EXHAUSTED: windows_core::HRESULT = windows_core::HRESULT(0x8003030B_u32 as _);
pub const STG_E_REVERTED: windows_core::HRESULT = windows_core::HRESULT(0x80030102_u32 as _);
pub const STG_E_SEEKERROR: windows_core::HRESULT = windows_core::HRESULT(0x80030019_u32 as _);
pub const STG_E_SHAREREQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80030106_u32 as _);
pub const STG_E_SHAREVIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x80030020_u32 as _);
pub const STG_E_STATUS_COPY_PROTECTION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80030305_u32 as _);
pub const STG_E_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x80030202_u32 as _);
pub const STG_E_TOOMANYOPENFILES: windows_core::HRESULT = windows_core::HRESULT(0x80030004_u32 as _);
pub const STG_E_UNIMPLEMENTEDFUNCTION: windows_core::HRESULT = windows_core::HRESULT(0x800300FE_u32 as _);
pub const STG_E_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x800300FD_u32 as _);
pub const STG_E_WRITEFAULT: windows_core::HRESULT = windows_core::HRESULT(0x8003001D_u32 as _);
pub const STG_S_BLOCK: windows_core::HRESULT = windows_core::HRESULT(0x30201_u32 as _);
pub const STG_S_CANNOTCONSOLIDATE: windows_core::HRESULT = windows_core::HRESULT(0x30206_u32 as _);
pub const STG_S_CONSOLIDATIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x30205_u32 as _);
pub const STG_S_CONVERTED: windows_core::HRESULT = windows_core::HRESULT(0x30200_u32 as _);
pub const STG_S_MONITORING: windows_core::HRESULT = windows_core::HRESULT(0x30203_u32 as _);
pub const STG_S_MULTIPLEOPENS: windows_core::HRESULT = windows_core::HRESULT(0x30204_u32 as _);
pub const STG_S_POWER_CYCLE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x30207_u32 as _);
pub const STG_S_RETRYNOW: windows_core::HRESULT = windows_core::HRESULT(0x30202_u32 as _);
pub const STILL_ACTIVE: NTSTATUS = NTSTATUS(0x103_u32 as _);
pub const STORE_ERROR_LICENSE_REVOKED: i32 = 15864i32;
pub const STORE_ERROR_PENDING_COM_TRANSACTION: i32 = 15863i32;
pub const STORE_ERROR_UNLICENSED: i32 = 15861i32;
pub const STORE_ERROR_UNLICENSED_USER: i32 = 15862i32;
pub const STRICT: u32 = 1u32;
pub const SUCCESS: u32 = 0u32;
pub const S_APPLICATION_ACTIVATION_ERROR_HANDLED_BY_DIALOG: windows_core::HRESULT = windows_core::HRESULT(0x270259_u32 as _);
pub const S_FALSE: windows_core::HRESULT = windows_core::HRESULT(0x1_u32 as _);
pub const S_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const S_STORE_LAUNCHED_FOR_REMEDIATION: windows_core::HRESULT = windows_core::HRESULT(0x270258_u32 as _);
pub const TBSIMP_E_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80290200_u32 as _);
pub const TBSIMP_E_CLEANUP_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80290201_u32 as _);
pub const TBSIMP_E_COMMAND_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x8029020B_u32 as _);
pub const TBSIMP_E_COMMAND_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80290211_u32 as _);
pub const TBSIMP_E_DUPLICATE_VHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80290206_u32 as _);
pub const TBSIMP_E_HASH_BAD_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80290205_u32 as _);
pub const TBSIMP_E_HASH_TABLE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80290216_u32 as _);
pub const TBSIMP_E_INVALID_CONTEXT_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80290202_u32 as _);
pub const TBSIMP_E_INVALID_CONTEXT_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x80290203_u32 as _);
pub const TBSIMP_E_INVALID_OUTPUT_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80290207_u32 as _);
pub const TBSIMP_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80290208_u32 as _);
pub const TBSIMP_E_INVALID_RESOURCE: windows_core::HRESULT = windows_core::HRESULT(0x80290214_u32 as _);
pub const TBSIMP_E_LIST_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8029020E_u32 as _);
pub const TBSIMP_E_LIST_NO_MORE_ITEMS: windows_core::HRESULT = windows_core::HRESULT(0x8029020D_u32 as _);
pub const TBSIMP_E_NOTHING_TO_UNLOAD: windows_core::HRESULT = windows_core::HRESULT(0x80290215_u32 as _);
pub const TBSIMP_E_NOT_ENOUGH_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x8029020F_u32 as _);
pub const TBSIMP_E_NOT_ENOUGH_TPM_CONTEXTS: windows_core::HRESULT = windows_core::HRESULT(0x80290210_u32 as _);
pub const TBSIMP_E_NO_EVENT_LOG: windows_core::HRESULT = windows_core::HRESULT(0x8029021B_u32 as _);
pub const TBSIMP_E_OUT_OF_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8029020C_u32 as _);
pub const TBSIMP_E_PPI_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80290219_u32 as _);
pub const TBSIMP_E_RESOURCE_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x80290213_u32 as _);
pub const TBSIMP_E_RPC_INIT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80290209_u32 as _);
pub const TBSIMP_E_SCHEDULER_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8029020A_u32 as _);
pub const TBSIMP_E_TOO_MANY_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x80290218_u32 as _);
pub const TBSIMP_E_TOO_MANY_TBS_CONTEXTS: windows_core::HRESULT = windows_core::HRESULT(0x80290217_u32 as _);
pub const TBSIMP_E_TPM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80290204_u32 as _);
pub const TBSIMP_E_TPM_INCOMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0x8029021A_u32 as _);
pub const TBSIMP_E_UNKNOWN_ORDINAL: windows_core::HRESULT = windows_core::HRESULT(0x80290212_u32 as _);
pub const TBS_E_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80284012_u32 as _);
pub const TBS_E_BAD_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80284002_u32 as _);
pub const TBS_E_BUFFER_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x8028400E_u32 as _);
pub const TBS_E_COMMAND_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x8028400D_u32 as _);
pub const TBS_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x80284005_u32 as _);
pub const TBS_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80284001_u32 as _);
pub const TBS_E_INVALID_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80284004_u32 as _);
pub const TBS_E_INVALID_CONTEXT_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x80284007_u32 as _);
pub const TBS_E_INVALID_OUTPUT_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80284003_u32 as _);
pub const TBS_E_IOERROR: windows_core::HRESULT = windows_core::HRESULT(0x80284006_u32 as _);
pub const TBS_E_NO_EVENT_LOG: windows_core::HRESULT = windows_core::HRESULT(0x80284011_u32 as _);
pub const TBS_E_OWNERAUTH_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80284015_u32 as _);
pub const TBS_E_PPI_FUNCTION_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80284014_u32 as _);
pub const TBS_E_PPI_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8028400C_u32 as _);
pub const TBS_E_PROVISIONING_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80284016_u32 as _);
pub const TBS_E_PROVISIONING_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80284013_u32 as _);
pub const TBS_E_SERVICE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80284010_u32 as _);
pub const TBS_E_SERVICE_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80284008_u32 as _);
pub const TBS_E_SERVICE_START_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x8028400B_u32 as _);
pub const TBS_E_TOO_MANY_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x8028400A_u32 as _);
pub const TBS_E_TOO_MANY_TBS_CONTEXTS: windows_core::HRESULT = windows_core::HRESULT(0x80284009_u32 as _);
pub const TBS_E_TPM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8028400F_u32 as _);
pub const TPC_E_INITIALIZE_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80040223_u32 as _);
pub const TPC_E_INVALID_CONFIGURATION: windows_core::HRESULT = windows_core::HRESULT(0x80040239_u32 as _);
pub const TPC_E_INVALID_DATA_FROM_RECOGNIZER: windows_core::HRESULT = windows_core::HRESULT(0x8004023A_u32 as _);
pub const TPC_E_INVALID_INPUT_RECT: windows_core::HRESULT = windows_core::HRESULT(0x80040219_u32 as _);
pub const TPC_E_INVALID_PACKET_DESCRIPTION: windows_core::HRESULT = windows_core::HRESULT(0x80040233_u32 as _);
pub const TPC_E_INVALID_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80040241_u32 as _);
pub const TPC_E_INVALID_RIGHTS: windows_core::HRESULT = windows_core::HRESULT(0x80040236_u32 as _);
pub const TPC_E_INVALID_STROKE: windows_core::HRESULT = windows_core::HRESULT(0x80040222_u32 as _);
pub const TPC_E_NOT_RELEVANT: windows_core::HRESULT = windows_core::HRESULT(0x80040232_u32 as _);
pub const TPC_E_NO_DEFAULT_TABLET: windows_core::HRESULT = windows_core::HRESULT(0x80040212_u32 as _);
pub const TPC_E_OUT_OF_ORDER_CALL: windows_core::HRESULT = windows_core::HRESULT(0x80040237_u32 as _);
pub const TPC_E_QUEUE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80040238_u32 as _);
pub const TPC_E_RECOGNIZER_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80040235_u32 as _);
pub const TPC_E_UNKNOWN_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004021B_u32 as _);
pub const TPC_S_INTERRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x40253_u32 as _);
pub const TPC_S_NO_DATA_TO_PROCESS: windows_core::HRESULT = windows_core::HRESULT(0x40254_u32 as _);
pub const TPC_S_TRUNCATED: windows_core::HRESULT = windows_core::HRESULT(0x40252_u32 as _);
pub const TPMAPI_E_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80290108_u32 as _);
pub const TPMAPI_E_AUTHORIZATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80290109_u32 as _);
pub const TPMAPI_E_AUTHORIZATION_REVOKED: windows_core::HRESULT = windows_core::HRESULT(0x80290126_u32 as _);
pub const TPMAPI_E_AUTHORIZING_KEY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80290128_u32 as _);
pub const TPMAPI_E_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80290106_u32 as _);
pub const TPMAPI_E_EMPTY_TCG_LOG: windows_core::HRESULT = windows_core::HRESULT(0x8029011A_u32 as _);
pub const TPMAPI_E_ENCRYPTION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80290110_u32 as _);
pub const TPMAPI_E_ENDORSEMENT_AUTH_NOT_NULL: windows_core::HRESULT = windows_core::HRESULT(0x80290125_u32 as _);
pub const TPMAPI_E_FIPS_RNG_CHECK_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80290119_u32 as _);
pub const TPMAPI_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80290107_u32 as _);
pub const TPMAPI_E_INVALID_AUTHORIZATION_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80290129_u32 as _);
pub const TPMAPI_E_INVALID_CONTEXT_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x8029010A_u32 as _);
pub const TPMAPI_E_INVALID_CONTEXT_PARAMS: windows_core::HRESULT = windows_core::HRESULT(0x80290115_u32 as _);
pub const TPMAPI_E_INVALID_DELEGATE_BLOB: windows_core::HRESULT = windows_core::HRESULT(0x80290114_u32 as _);
pub const TPMAPI_E_INVALID_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x8029010E_u32 as _);
pub const TPMAPI_E_INVALID_KEY_BLOB: windows_core::HRESULT = windows_core::HRESULT(0x80290116_u32 as _);
pub const TPMAPI_E_INVALID_KEY_PARAMS: windows_core::HRESULT = windows_core::HRESULT(0x80290111_u32 as _);
pub const TPMAPI_E_INVALID_KEY_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x8029010F_u32 as _);
pub const TPMAPI_E_INVALID_MIGRATION_AUTHORIZATION_BLOB: windows_core::HRESULT = windows_core::HRESULT(0x80290112_u32 as _);
pub const TPMAPI_E_INVALID_OUTPUT_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80290103_u32 as _);
pub const TPMAPI_E_INVALID_OWNER_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x80290118_u32 as _);
pub const TPMAPI_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80290104_u32 as _);
pub const TPMAPI_E_INVALID_PCR_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80290117_u32 as _);
pub const TPMAPI_E_INVALID_PCR_INDEX: windows_core::HRESULT = windows_core::HRESULT(0x80290113_u32 as _);
pub const TPMAPI_E_INVALID_POLICYAUTH_BLOB_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8029012E_u32 as _);
pub const TPMAPI_E_INVALID_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80290100_u32 as _);
pub const TPMAPI_E_INVALID_TCG_LOG_ENTRY: windows_core::HRESULT = windows_core::HRESULT(0x8029011B_u32 as _);
pub const TPMAPI_E_INVALID_TPM_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8029012D_u32 as _);
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80290127_u32 as _);
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_OTHER: windows_core::HRESULT = windows_core::HRESULT(0x8029012B_u32 as _);
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x8029012A_u32 as _);
pub const TPMAPI_E_MESSAGE_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x8029010D_u32 as _);
pub const TPMAPI_E_NOT_ENOUGH_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80290101_u32 as _);
pub const TPMAPI_E_NO_AUTHORIZATION_CHAIN_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80290122_u32 as _);
pub const TPMAPI_E_NV_BITS_NOT_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x8029011F_u32 as _);
pub const TPMAPI_E_NV_BITS_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80290120_u32 as _);
pub const TPMAPI_E_OUT_OF_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80290105_u32 as _);
pub const TPMAPI_E_OWNER_AUTH_NOT_NULL: windows_core::HRESULT = windows_core::HRESULT(0x80290124_u32 as _);
pub const TPMAPI_E_POLICY_DENIES_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x8029011E_u32 as _);
pub const TPMAPI_E_SEALING_KEY_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x8029012C_u32 as _);
pub const TPMAPI_E_SEALING_KEY_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80290121_u32 as _);
pub const TPMAPI_E_SVN_COUNTER_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80290123_u32 as _);
pub const TPMAPI_E_TBS_COMMUNICATION_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8029010B_u32 as _);
pub const TPMAPI_E_TCG_INVALID_DIGEST_ENTRY: windows_core::HRESULT = windows_core::HRESULT(0x8029011D_u32 as _);
pub const TPMAPI_E_TCG_SEPARATOR_ABSENT: windows_core::HRESULT = windows_core::HRESULT(0x8029011C_u32 as _);
pub const TPMAPI_E_TOO_MUCH_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80290102_u32 as _);
pub const TPMAPI_E_TPM_COMMAND_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8029010C_u32 as _);
pub const TPM_20_E_ASYMMETRIC: windows_core::HRESULT = windows_core::HRESULT(0x80280081_u32 as _);
pub const TPM_20_E_ATTRIBUTES: windows_core::HRESULT = windows_core::HRESULT(0x80280082_u32 as _);
pub const TPM_20_E_AUTHSIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280144_u32 as _);
pub const TPM_20_E_AUTH_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80280145_u32 as _);
pub const TPM_20_E_AUTH_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8028008E_u32 as _);
pub const TPM_20_E_AUTH_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80280125_u32 as _);
pub const TPM_20_E_AUTH_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80280124_u32 as _);
pub const TPM_20_E_AUTH_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8028012F_u32 as _);
pub const TPM_20_E_BAD_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x802800A2_u32 as _);
pub const TPM_20_E_BAD_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80280150_u32 as _);
pub const TPM_20_E_BINDING: windows_core::HRESULT = windows_core::HRESULT(0x802800A5_u32 as _);
pub const TPM_20_E_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80280909_u32 as _);
pub const TPM_20_E_COMMAND_CODE: windows_core::HRESULT = windows_core::HRESULT(0x80280143_u32 as _);
pub const TPM_20_E_COMMAND_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280142_u32 as _);
pub const TPM_20_E_CONTEXT_GAP: windows_core::HRESULT = windows_core::HRESULT(0x80280901_u32 as _);
pub const TPM_20_E_CPHASH: windows_core::HRESULT = windows_core::HRESULT(0x80280151_u32 as _);
pub const TPM_20_E_CURVE: windows_core::HRESULT = windows_core::HRESULT(0x802800A6_u32 as _);
pub const TPM_20_E_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80280120_u32 as _);
pub const TPM_20_E_ECC_CURVE: windows_core::HRESULT = windows_core::HRESULT(0x80280123_u32 as _);
pub const TPM_20_E_ECC_POINT: windows_core::HRESULT = windows_core::HRESULT(0x802800A7_u32 as _);
pub const TPM_20_E_EXCLUSIVE: windows_core::HRESULT = windows_core::HRESULT(0x80280121_u32 as _);
pub const TPM_20_E_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x802800A3_u32 as _);
pub const TPM_20_E_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80280101_u32 as _);
pub const TPM_20_E_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x8028008B_u32 as _);
pub const TPM_20_E_HASH: windows_core::HRESULT = windows_core::HRESULT(0x80280083_u32 as _);
pub const TPM_20_E_HIERARCHY: windows_core::HRESULT = windows_core::HRESULT(0x80280085_u32 as _);
pub const TPM_20_E_HMAC: windows_core::HRESULT = windows_core::HRESULT(0x80280119_u32 as _);
pub const TPM_20_E_INITIALIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280100_u32 as _);
pub const TPM_20_E_INSUFFICIENT: windows_core::HRESULT = windows_core::HRESULT(0x8028009A_u32 as _);
pub const TPM_20_E_INTEGRITY: windows_core::HRESULT = windows_core::HRESULT(0x8028009F_u32 as _);
pub const TPM_20_E_KDF: windows_core::HRESULT = windows_core::HRESULT(0x8028008C_u32 as _);
pub const TPM_20_E_KEY: windows_core::HRESULT = windows_core::HRESULT(0x8028009C_u32 as _);
pub const TPM_20_E_KEY_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280087_u32 as _);
pub const TPM_20_E_LOCALITY: windows_core::HRESULT = windows_core::HRESULT(0x80280907_u32 as _);
pub const TPM_20_E_LOCKOUT: windows_core::HRESULT = windows_core::HRESULT(0x80280921_u32 as _);
pub const TPM_20_E_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80280904_u32 as _);
pub const TPM_20_E_MGF: windows_core::HRESULT = windows_core::HRESULT(0x80280088_u32 as _);
pub const TPM_20_E_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80280089_u32 as _);
pub const TPM_20_E_NEEDS_TEST: windows_core::HRESULT = windows_core::HRESULT(0x80280153_u32 as _);
pub const TPM_20_E_NONCE: windows_core::HRESULT = windows_core::HRESULT(0x8028008F_u32 as _);
pub const TPM_20_E_NO_RESULT: windows_core::HRESULT = windows_core::HRESULT(0x80280154_u32 as _);
pub const TPM_20_E_NV_AUTHORIZATION: windows_core::HRESULT = windows_core::HRESULT(0x80280149_u32 as _);
pub const TPM_20_E_NV_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x8028014C_u32 as _);
pub const TPM_20_E_NV_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80280148_u32 as _);
pub const TPM_20_E_NV_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80280146_u32 as _);
pub const TPM_20_E_NV_RATE: windows_core::HRESULT = windows_core::HRESULT(0x80280920_u32 as _);
pub const TPM_20_E_NV_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280147_u32 as _);
pub const TPM_20_E_NV_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x8028014B_u32 as _);
pub const TPM_20_E_NV_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80280923_u32 as _);
pub const TPM_20_E_NV_UNINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8028014A_u32 as _);
pub const TPM_20_E_OBJECT_HANDLES: windows_core::HRESULT = windows_core::HRESULT(0x80280906_u32 as _);
pub const TPM_20_E_OBJECT_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80280902_u32 as _);
pub const TPM_20_E_PARENT: windows_core::HRESULT = windows_core::HRESULT(0x80280152_u32 as _);
pub const TPM_20_E_PCR: windows_core::HRESULT = windows_core::HRESULT(0x80280127_u32 as _);
pub const TPM_20_E_PCR_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x80280128_u32 as _);
pub const TPM_20_E_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x80280126_u32 as _);
pub const TPM_20_E_POLICY_CC: windows_core::HRESULT = windows_core::HRESULT(0x802800A4_u32 as _);
pub const TPM_20_E_POLICY_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8028009D_u32 as _);
pub const TPM_20_E_PP: windows_core::HRESULT = windows_core::HRESULT(0x80280090_u32 as _);
pub const TPM_20_E_PRIVATE: windows_core::HRESULT = windows_core::HRESULT(0x8028010B_u32 as _);
pub const TPM_20_E_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x8028008D_u32 as _);
pub const TPM_20_E_REBOOT: windows_core::HRESULT = windows_core::HRESULT(0x80280130_u32 as _);
pub const TPM_20_E_RESERVED_BITS: windows_core::HRESULT = windows_core::HRESULT(0x802800A1_u32 as _);
pub const TPM_20_E_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80280922_u32 as _);
pub const TPM_20_E_SCHEME: windows_core::HRESULT = windows_core::HRESULT(0x80280092_u32 as _);
pub const TPM_20_E_SELECTOR: windows_core::HRESULT = windows_core::HRESULT(0x80280098_u32 as _);
pub const TPM_20_E_SENSITIVE: windows_core::HRESULT = windows_core::HRESULT(0x80280155_u32 as _);
pub const TPM_20_E_SEQUENCE: windows_core::HRESULT = windows_core::HRESULT(0x80280103_u32 as _);
pub const TPM_20_E_SESSION_HANDLES: windows_core::HRESULT = windows_core::HRESULT(0x80280905_u32 as _);
pub const TPM_20_E_SESSION_MEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80280903_u32 as _);
pub const TPM_20_E_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x8028009B_u32 as _);
pub const TPM_20_E_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280095_u32 as _);
pub const TPM_20_E_SYMMETRIC: windows_core::HRESULT = windows_core::HRESULT(0x80280096_u32 as _);
pub const TPM_20_E_TAG: windows_core::HRESULT = windows_core::HRESULT(0x80280097_u32 as _);
pub const TPM_20_E_TESTING: windows_core::HRESULT = windows_core::HRESULT(0x8028090A_u32 as _);
pub const TPM_20_E_TICKET: windows_core::HRESULT = windows_core::HRESULT(0x802800A0_u32 as _);
pub const TPM_20_E_TOO_MANY_CONTEXTS: windows_core::HRESULT = windows_core::HRESULT(0x8028012E_u32 as _);
pub const TPM_20_E_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8028008A_u32 as _);
pub const TPM_20_E_UNBALANCED: windows_core::HRESULT = windows_core::HRESULT(0x80280131_u32 as _);
pub const TPM_20_E_UPGRADE: windows_core::HRESULT = windows_core::HRESULT(0x8028012D_u32 as _);
pub const TPM_20_E_VALUE: windows_core::HRESULT = windows_core::HRESULT(0x80280084_u32 as _);
pub const TPM_20_E_YIELDED: windows_core::HRESULT = windows_core::HRESULT(0x80280908_u32 as _);
pub const TPM_E_AREA_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8028003C_u32 as _);
pub const TPM_E_ATTESTATION_CHALLENGE_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x80290412_u32 as _);
pub const TPM_E_AUDITFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80280004_u32 as _);
pub const TPM_E_AUDITFAIL_SUCCESSFUL: windows_core::HRESULT = windows_core::HRESULT(0x80280031_u32 as _);
pub const TPM_E_AUDITFAIL_UNSUCCESSFUL: windows_core::HRESULT = windows_core::HRESULT(0x80280030_u32 as _);
pub const TPM_E_AUTH2FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8028001D_u32 as _);
pub const TPM_E_AUTHFAIL: windows_core::HRESULT = windows_core::HRESULT(0x80280001_u32 as _);
pub const TPM_E_AUTH_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x8028003B_u32 as _);
pub const TPM_E_BADCONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x8028005A_u32 as _);
pub const TPM_E_BADINDEX: windows_core::HRESULT = windows_core::HRESULT(0x80280002_u32 as _);
pub const TPM_E_BADTAG: windows_core::HRESULT = windows_core::HRESULT(0x8028001E_u32 as _);
pub const TPM_E_BAD_ATTRIBUTES: windows_core::HRESULT = windows_core::HRESULT(0x80280042_u32 as _);
pub const TPM_E_BAD_COUNTER: windows_core::HRESULT = windows_core::HRESULT(0x80280045_u32 as _);
pub const TPM_E_BAD_DATASIZE: windows_core::HRESULT = windows_core::HRESULT(0x8028002B_u32 as _);
pub const TPM_E_BAD_DELEGATE: windows_core::HRESULT = windows_core::HRESULT(0x80280059_u32 as _);
pub const TPM_E_BAD_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80280058_u32 as _);
pub const TPM_E_BAD_KEY_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80280028_u32 as _);
pub const TPM_E_BAD_LOCALITY: windows_core::HRESULT = windows_core::HRESULT(0x8028003D_u32 as _);
pub const TPM_E_BAD_MIGRATION: windows_core::HRESULT = windows_core::HRESULT(0x80280029_u32 as _);
pub const TPM_E_BAD_MODE: windows_core::HRESULT = windows_core::HRESULT(0x8028002C_u32 as _);
pub const TPM_E_BAD_ORDINAL: windows_core::HRESULT = windows_core::HRESULT(0x8028000A_u32 as _);
pub const TPM_E_BAD_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80280003_u32 as _);
pub const TPM_E_BAD_PARAM_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280019_u32 as _);
pub const TPM_E_BAD_PRESENCE: windows_core::HRESULT = windows_core::HRESULT(0x8028002D_u32 as _);
pub const TPM_E_BAD_SCHEME: windows_core::HRESULT = windows_core::HRESULT(0x8028002A_u32 as _);
pub const TPM_E_BAD_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80280062_u32 as _);
pub const TPM_E_BAD_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80280034_u32 as _);
pub const TPM_E_BAD_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8028002E_u32 as _);
pub const TPM_E_BUFFER_LENGTH_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8029041E_u32 as _);
pub const TPM_E_CLAIM_TYPE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8029041C_u32 as _);
pub const TPM_E_CLEAR_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80280005_u32 as _);
pub const TPM_E_COMMAND_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80280400_u32 as _);
pub const TPM_E_CONTEXT_GAP: windows_core::HRESULT = windows_core::HRESULT(0x80280047_u32 as _);
pub const TPM_E_DAA_INPUT_DATA0: windows_core::HRESULT = windows_core::HRESULT(0x80280051_u32 as _);
pub const TPM_E_DAA_INPUT_DATA1: windows_core::HRESULT = windows_core::HRESULT(0x80280052_u32 as _);
pub const TPM_E_DAA_ISSUER_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x80280053_u32 as _);
pub const TPM_E_DAA_ISSUER_VALIDITY: windows_core::HRESULT = windows_core::HRESULT(0x80280056_u32 as _);
pub const TPM_E_DAA_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x80280050_u32 as _);
pub const TPM_E_DAA_STAGE: windows_core::HRESULT = windows_core::HRESULT(0x80280055_u32 as _);
pub const TPM_E_DAA_TPM_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x80280054_u32 as _);
pub const TPM_E_DAA_WRONG_W: windows_core::HRESULT = windows_core::HRESULT(0x80280057_u32 as _);
pub const TPM_E_DEACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0x80280006_u32 as _);
pub const TPM_E_DECRYPT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80280021_u32 as _);
pub const TPM_E_DEFEND_LOCK_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80280803_u32 as _);
pub const TPM_E_DELEGATE_ADMIN: windows_core::HRESULT = windows_core::HRESULT(0x8028004D_u32 as _);
pub const TPM_E_DELEGATE_FAMILY: windows_core::HRESULT = windows_core::HRESULT(0x8028004C_u32 as _);
pub const TPM_E_DELEGATE_LOCK: windows_core::HRESULT = windows_core::HRESULT(0x8028004B_u32 as _);
pub const TPM_E_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80280007_u32 as _);
pub const TPM_E_DISABLED_CMD: windows_core::HRESULT = windows_core::HRESULT(0x80280008_u32 as _);
pub const TPM_E_DOING_SELFTEST: windows_core::HRESULT = windows_core::HRESULT(0x80280802_u32 as _);
pub const TPM_E_DUPLICATE_VHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80280402_u32 as _);
pub const TPM_E_EMBEDDED_COMMAND_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80280403_u32 as _);
pub const TPM_E_EMBEDDED_COMMAND_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80280404_u32 as _);
pub const TPM_E_ENCRYPT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80280020_u32 as _);
pub const TPM_E_ERROR_MASK: windows_core::HRESULT = windows_core::HRESULT(0x80280000_u32 as _);
pub const TPM_E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80280009_u32 as _);
pub const TPM_E_FAILEDSELFTEST: windows_core::HRESULT = windows_core::HRESULT(0x8028001C_u32 as _);
pub const TPM_E_FAMILYCOUNT: windows_core::HRESULT = windows_core::HRESULT(0x80280040_u32 as _);
pub const TPM_E_INAPPROPRIATE_ENC: windows_core::HRESULT = windows_core::HRESULT(0x8028000E_u32 as _);
pub const TPM_E_INAPPROPRIATE_SIG: windows_core::HRESULT = windows_core::HRESULT(0x80280027_u32 as _);
pub const TPM_E_INSTALL_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8028000B_u32 as _);
pub const TPM_E_INVALID_AUTHHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80280022_u32 as _);
pub const TPM_E_INVALID_FAMILY: windows_core::HRESULT = windows_core::HRESULT(0x80280037_u32 as _);
pub const TPM_E_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80280401_u32 as _);
pub const TPM_E_INVALID_KEYHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x8028000C_u32 as _);
pub const TPM_E_INVALID_KEYUSAGE: windows_core::HRESULT = windows_core::HRESULT(0x80280024_u32 as _);
pub const TPM_E_INVALID_OWNER_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x80290601_u32 as _);
pub const TPM_E_INVALID_PCR_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80280010_u32 as _);
pub const TPM_E_INVALID_POSTINIT: windows_core::HRESULT = windows_core::HRESULT(0x80280026_u32 as _);
pub const TPM_E_INVALID_RESOURCE: windows_core::HRESULT = windows_core::HRESULT(0x80280035_u32 as _);
pub const TPM_E_INVALID_STRUCTURE: windows_core::HRESULT = windows_core::HRESULT(0x80280043_u32 as _);
pub const TPM_E_IOERROR: windows_core::HRESULT = windows_core::HRESULT(0x8028001F_u32 as _);
pub const TPM_E_KEYNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8028000D_u32 as _);
pub const TPM_E_KEY_ALREADY_FINALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80290414_u32 as _);
pub const TPM_E_KEY_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8028003A_u32 as _);
pub const TPM_E_KEY_NOT_AUTHENTICATED: windows_core::HRESULT = windows_core::HRESULT(0x80290418_u32 as _);
pub const TPM_E_KEY_NOT_FINALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80290411_u32 as _);
pub const TPM_E_KEY_NOT_LOADED: windows_core::HRESULT = windows_core::HRESULT(0x8029040F_u32 as _);
pub const TPM_E_KEY_NOT_SIGNING_KEY: windows_core::HRESULT = windows_core::HRESULT(0x8029041A_u32 as _);
pub const TPM_E_KEY_OWNER_CONTROL: windows_core::HRESULT = windows_core::HRESULT(0x80280044_u32 as _);
pub const TPM_E_KEY_USAGE_POLICY_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80290416_u32 as _);
pub const TPM_E_KEY_USAGE_POLICY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80290415_u32 as _);
pub const TPM_E_LOCKED_OUT: windows_core::HRESULT = windows_core::HRESULT(0x8029041B_u32 as _);
pub const TPM_E_MAXNVWRITES: windows_core::HRESULT = windows_core::HRESULT(0x80280048_u32 as _);
pub const TPM_E_MA_AUTHORITY: windows_core::HRESULT = windows_core::HRESULT(0x8028005F_u32 as _);
pub const TPM_E_MA_DESTINATION: windows_core::HRESULT = windows_core::HRESULT(0x8028005D_u32 as _);
pub const TPM_E_MA_SOURCE: windows_core::HRESULT = windows_core::HRESULT(0x8028005E_u32 as _);
pub const TPM_E_MA_TICKET_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x8028005C_u32 as _);
pub const TPM_E_MIGRATEFAIL: windows_core::HRESULT = windows_core::HRESULT(0x8028000F_u32 as _);
pub const TPM_E_NEEDS_SELFTEST: windows_core::HRESULT = windows_core::HRESULT(0x80280801_u32 as _);
pub const TPM_E_NOCONTEXTSPACE: windows_core::HRESULT = windows_core::HRESULT(0x80280063_u32 as _);
pub const TPM_E_NOOPERATOR: windows_core::HRESULT = windows_core::HRESULT(0x80280049_u32 as _);
pub const TPM_E_NOSPACE: windows_core::HRESULT = windows_core::HRESULT(0x80280011_u32 as _);
pub const TPM_E_NOSRK: windows_core::HRESULT = windows_core::HRESULT(0x80280012_u32 as _);
pub const TPM_E_NOTFIPS: windows_core::HRESULT = windows_core::HRESULT(0x80280036_u32 as _);
pub const TPM_E_NOTLOCAL: windows_core::HRESULT = windows_core::HRESULT(0x80280033_u32 as _);
pub const TPM_E_NOTRESETABLE: windows_core::HRESULT = windows_core::HRESULT(0x80280032_u32 as _);
pub const TPM_E_NOTSEALED_BLOB: windows_core::HRESULT = windows_core::HRESULT(0x80280013_u32 as _);
pub const TPM_E_NOT_FULLWRITE: windows_core::HRESULT = windows_core::HRESULT(0x80280046_u32 as _);
pub const TPM_E_NOT_PCR_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x80290413_u32 as _);
pub const TPM_E_NO_ENDORSEMENT: windows_core::HRESULT = windows_core::HRESULT(0x80280023_u32 as _);
pub const TPM_E_NO_KEY_CERTIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x80290410_u32 as _);
pub const TPM_E_NO_NV_PERMISSION: windows_core::HRESULT = windows_core::HRESULT(0x80280038_u32 as _);
pub const TPM_E_NO_WRAP_TRANSPORT: windows_core::HRESULT = windows_core::HRESULT(0x8028002F_u32 as _);
pub const TPM_E_OWNER_CONTROL: windows_core::HRESULT = windows_core::HRESULT(0x8028004F_u32 as _);
pub const TPM_E_OWNER_SET: windows_core::HRESULT = windows_core::HRESULT(0x80280014_u32 as _);
pub const TPM_E_PCP_AUTHENTICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80290408_u32 as _);
pub const TPM_E_PCP_AUTHENTICATION_IGNORED: windows_core::HRESULT = windows_core::HRESULT(0x80290409_u32 as _);
pub const TPM_E_PCP_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80290406_u32 as _);
pub const TPM_E_PCP_DEVICE_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80290401_u32 as _);
pub const TPM_E_PCP_ERROR_MASK: windows_core::HRESULT = windows_core::HRESULT(0x80290400_u32 as _);
pub const TPM_E_PCP_FLAG_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80290404_u32 as _);
pub const TPM_E_PCP_IFX_RSA_KEY_CREATION_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8029041F_u32 as _);
pub const TPM_E_PCP_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80290407_u32 as _);
pub const TPM_E_PCP_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80290402_u32 as _);
pub const TPM_E_PCP_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80290403_u32 as _);
pub const TPM_E_PCP_KEY_HANDLE_INVALIDATED: windows_core::HRESULT = windows_core::HRESULT(0x80290422_u32 as _);
pub const TPM_E_PCP_KEY_NOT_AIK: windows_core::HRESULT = windows_core::HRESULT(0x80290419_u32 as _);
pub const TPM_E_PCP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80290405_u32 as _);
pub const TPM_E_PCP_PLATFORM_CLAIM_MAY_BE_OUTDATED: windows_core::HRESULT = windows_core::HRESULT(0x40290424_u32 as _);
pub const TPM_E_PCP_PLATFORM_CLAIM_OUTDATED: windows_core::HRESULT = windows_core::HRESULT(0x40290425_u32 as _);
pub const TPM_E_PCP_PLATFORM_CLAIM_REBOOT: windows_core::HRESULT = windows_core::HRESULT(0x40290426_u32 as _);
pub const TPM_E_PCP_POLICY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8029040A_u32 as _);
pub const TPM_E_PCP_PROFILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8029040B_u32 as _);
pub const TPM_E_PCP_RAW_POLICY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80290421_u32 as _);
pub const TPM_E_PCP_TICKET_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80290420_u32 as _);
pub const TPM_E_PCP_UNSUPPORTED_PSS_SALT: windows_core::HRESULT = windows_core::HRESULT(0x40290423_u32 as _);
pub const TPM_E_PCP_VALIDATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8029040C_u32 as _);
pub const TPM_E_PCP_WRONG_PARENT: windows_core::HRESULT = windows_core::HRESULT(0x8029040E_u32 as _);
pub const TPM_E_PERMANENTEK: windows_core::HRESULT = windows_core::HRESULT(0x80280061_u32 as _);
pub const TPM_E_PER_NOWRITE: windows_core::HRESULT = windows_core::HRESULT(0x8028003F_u32 as _);
pub const TPM_E_PPI_ACPI_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80290300_u32 as _);
pub const TPM_E_PPI_BIOS_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80290302_u32 as _);
pub const TPM_E_PPI_BLOCKED_IN_BIOS: windows_core::HRESULT = windows_core::HRESULT(0x80290304_u32 as _);
pub const TPM_E_PPI_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80290303_u32 as _);
pub const TPM_E_PPI_USER_ABORT: windows_core::HRESULT = windows_core::HRESULT(0x80290301_u32 as _);
pub const TPM_E_PROVISIONING_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80290600_u32 as _);
pub const TPM_E_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x8028003E_u32 as _);
pub const TPM_E_REQUIRES_SIGN: windows_core::HRESULT = windows_core::HRESULT(0x80280039_u32 as _);
pub const TPM_E_RESOURCEMISSING: windows_core::HRESULT = windows_core::HRESULT(0x8028004A_u32 as _);
pub const TPM_E_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x80280015_u32 as _);
pub const TPM_E_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80280800_u32 as _);
pub const TPM_E_SHA_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8028001B_u32 as _);
pub const TPM_E_SHA_THREAD: windows_core::HRESULT = windows_core::HRESULT(0x8028001A_u32 as _);
pub const TPM_E_SHORTRANDOM: windows_core::HRESULT = windows_core::HRESULT(0x80280016_u32 as _);
pub const TPM_E_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80280017_u32 as _);
pub const TPM_E_SOFT_KEY_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80290417_u32 as _);
pub const TPM_E_TOOMANYCONTEXTS: windows_core::HRESULT = windows_core::HRESULT(0x8028005B_u32 as _);
pub const TPM_E_TOO_MUCH_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80290602_u32 as _);
pub const TPM_E_TPM_GENERATED_EPS: windows_core::HRESULT = windows_core::HRESULT(0x80290603_u32 as _);
pub const TPM_E_TRANSPORT_NOTEXCLUSIVE: windows_core::HRESULT = windows_core::HRESULT(0x8028004E_u32 as _);
pub const TPM_E_VERSION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8029041D_u32 as _);
pub const TPM_E_WRITE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80280041_u32 as _);
pub const TPM_E_WRONGPCRVAL: windows_core::HRESULT = windows_core::HRESULT(0x80280018_u32 as _);
pub const TPM_E_WRONG_ENTITYTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80280025_u32 as _);
pub const TPM_E_ZERO_EXHAUST_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80290500_u32 as _);
pub const TRUE: BOOL = BOOL(1i32);
pub const TRUST_E_ACTION_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x800B0002_u32 as _);
pub const TRUST_E_BAD_DIGEST: windows_core::HRESULT = windows_core::HRESULT(0x80096010_u32 as _);
pub const TRUST_E_BASIC_CONSTRAINTS: windows_core::HRESULT = windows_core::HRESULT(0x80096019_u32 as _);
pub const TRUST_E_CERT_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80096004_u32 as _);
pub const TRUST_E_COUNTER_SIGNER: windows_core::HRESULT = windows_core::HRESULT(0x80096003_u32 as _);
pub const TRUST_E_EXPLICIT_DISTRUST: windows_core::HRESULT = windows_core::HRESULT(0x800B0111_u32 as _);
pub const TRUST_E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x800B010B_u32 as _);
pub const TRUST_E_FINANCIAL_CRITERIA: windows_core::HRESULT = windows_core::HRESULT(0x8009601E_u32 as _);
pub const TRUST_E_MALFORMED_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80096011_u32 as _);
pub const TRUST_E_NOSIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x800B0100_u32 as _);
pub const TRUST_E_NO_SIGNER_CERT: windows_core::HRESULT = windows_core::HRESULT(0x80096002_u32 as _);
pub const TRUST_E_PROVIDER_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x800B0001_u32 as _);
pub const TRUST_E_SUBJECT_FORM_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x800B0003_u32 as _);
pub const TRUST_E_SUBJECT_NOT_TRUSTED: windows_core::HRESULT = windows_core::HRESULT(0x800B0004_u32 as _);
pub const TRUST_E_SYSTEM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80096001_u32 as _);
pub const TRUST_E_TIME_STAMP: windows_core::HRESULT = windows_core::HRESULT(0x80096005_u32 as _);
pub const TYPE_E_AMBIGUOUSNAME: windows_core::HRESULT = windows_core::HRESULT(0x8002802C_u32 as _);
pub const TYPE_E_BADMODULEKIND: windows_core::HRESULT = windows_core::HRESULT(0x800288BD_u32 as _);
pub const TYPE_E_BUFFERTOOSMALL: windows_core::HRESULT = windows_core::HRESULT(0x80028016_u32 as _);
pub const TYPE_E_CANTCREATETMPFILE: windows_core::HRESULT = windows_core::HRESULT(0x80028CA3_u32 as _);
pub const TYPE_E_CANTLOADLIBRARY: windows_core::HRESULT = windows_core::HRESULT(0x80029C4A_u32 as _);
pub const TYPE_E_CIRCULARTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80029C84_u32 as _);
pub const TYPE_E_DLLFUNCTIONNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8002802F_u32 as _);
pub const TYPE_E_DUPLICATEID: windows_core::HRESULT = windows_core::HRESULT(0x800288C6_u32 as _);
pub const TYPE_E_ELEMENTNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8002802B_u32 as _);
pub const TYPE_E_FIELDNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80028017_u32 as _);
pub const TYPE_E_INCONSISTENTPROPFUNCS: windows_core::HRESULT = windows_core::HRESULT(0x80029C83_u32 as _);
pub const TYPE_E_INVALIDID: windows_core::HRESULT = windows_core::HRESULT(0x800288CF_u32 as _);
pub const TYPE_E_INVALIDSTATE: windows_core::HRESULT = windows_core::HRESULT(0x80028029_u32 as _);
pub const TYPE_E_INVDATAREAD: windows_core::HRESULT = windows_core::HRESULT(0x80028018_u32 as _);
pub const TYPE_E_IOERROR: windows_core::HRESULT = windows_core::HRESULT(0x80028CA2_u32 as _);
pub const TYPE_E_LIBNOTREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8002801D_u32 as _);
pub const TYPE_E_NAMECONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x8002802D_u32 as _);
pub const TYPE_E_OUTOFBOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x80028CA1_u32 as _);
pub const TYPE_E_QUALIFIEDNAMEDISALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80028028_u32 as _);
pub const TYPE_E_REGISTRYACCESS: windows_core::HRESULT = windows_core::HRESULT(0x8002801C_u32 as _);
pub const TYPE_E_SIZETOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x800288C5_u32 as _);
pub const TYPE_E_TYPEMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80028CA0_u32 as _);
pub const TYPE_E_UNDEFINEDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80028027_u32 as _);
pub const TYPE_E_UNKNOWNLCID: windows_core::HRESULT = windows_core::HRESULT(0x8002802E_u32 as _);
pub const TYPE_E_UNSUPFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80028019_u32 as _);
pub const TYPE_E_WRONGTYPEKIND: windows_core::HRESULT = windows_core::HRESULT(0x8002802A_u32 as _);
pub const UCEERR_BLOCKSFULL: windows_core::HRESULT = windows_core::HRESULT(0x88980409_u32 as _);
pub const UCEERR_CHANNELSYNCABANDONED: windows_core::HRESULT = windows_core::HRESULT(0x88980414_u32 as _);
pub const UCEERR_CHANNELSYNCTIMEDOUT: windows_core::HRESULT = windows_core::HRESULT(0x88980413_u32 as _);
pub const UCEERR_COMMANDTRANSPORTDENIED: windows_core::HRESULT = windows_core::HRESULT(0x88980418_u32 as _);
pub const UCEERR_CONNECTIONIDLOOKUPFAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980408_u32 as _);
pub const UCEERR_CTXSTACKFRSTTARGETNULL: windows_core::HRESULT = windows_core::HRESULT(0x88980407_u32 as _);
pub const UCEERR_FEEDBACK_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88980417_u32 as _);
pub const UCEERR_GRAPHICSSTREAMALREADYOPEN: windows_core::HRESULT = windows_core::HRESULT(0x88980420_u32 as _);
pub const UCEERR_GRAPHICSSTREAMUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88980419_u32 as _);
pub const UCEERR_HANDLELOOKUPFAILED: windows_core::HRESULT = windows_core::HRESULT(0x88980405_u32 as _);
pub const UCEERR_ILLEGALHANDLE: windows_core::HRESULT = windows_core::HRESULT(0x88980404_u32 as _);
pub const UCEERR_ILLEGALPACKET: windows_core::HRESULT = windows_core::HRESULT(0x88980402_u32 as _);
pub const UCEERR_ILLEGALRECORDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x8898040C_u32 as _);
pub const UCEERR_INVALIDPACKETHEADER: windows_core::HRESULT = windows_core::HRESULT(0x88980400_u32 as _);
pub const UCEERR_MALFORMEDPACKET: windows_core::HRESULT = windows_core::HRESULT(0x88980403_u32 as _);
pub const UCEERR_MEMORYFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8898040A_u32 as _);
pub const UCEERR_MISSINGBEGINCOMMAND: windows_core::HRESULT = windows_core::HRESULT(0x88980412_u32 as _);
pub const UCEERR_MISSINGENDCOMMAND: windows_core::HRESULT = windows_core::HRESULT(0x88980411_u32 as _);
pub const UCEERR_NO_MULTIPLE_WORKER_THREADS: windows_core::HRESULT = windows_core::HRESULT(0x8898040F_u32 as _);
pub const UCEERR_OUTOFHANDLES: windows_core::HRESULT = windows_core::HRESULT(0x8898040D_u32 as _);
pub const UCEERR_PACKETRECORDOUTOFRANGE: windows_core::HRESULT = windows_core::HRESULT(0x8898040B_u32 as _);
pub const UCEERR_PARTITION_ZOMBIED: windows_core::HRESULT = windows_core::HRESULT(0x88980423_u32 as _);
pub const UCEERR_REMOTINGNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88980410_u32 as _);
pub const UCEERR_RENDERTHREADFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x88980406_u32 as _);
pub const UCEERR_TRANSPORTDISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x88980421_u32 as _);
pub const UCEERR_TRANSPORTOVERLOADED: windows_core::HRESULT = windows_core::HRESULT(0x88980422_u32 as _);
pub const UCEERR_TRANSPORTUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88980416_u32 as _);
pub const UCEERR_UNCHANGABLE_UPDATE_ATTEMPTED: windows_core::HRESULT = windows_core::HRESULT(0x8898040E_u32 as _);
pub const UCEERR_UNKNOWNPACKET: windows_core::HRESULT = windows_core::HRESULT(0x88980401_u32 as _);
pub const UCEERR_UNSUPPORTEDTRANSPORTVERSION: windows_core::HRESULT = windows_core::HRESULT(0x88980415_u32 as _);
pub const UI_E_AMBIGUOUS_MATCH: windows_core::HRESULT = windows_core::HRESULT(0x802A000A_u32 as _);
pub const UI_E_BOOLEAN_EXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x802A0008_u32 as _);
pub const UI_E_CREATE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x802A0001_u32 as _);
pub const UI_E_DIFFERENT_OWNER: windows_core::HRESULT = windows_core::HRESULT(0x802A0009_u32 as _);
pub const UI_E_END_KEYFRAME_NOT_DETERMINED: windows_core::HRESULT = windows_core::HRESULT(0x802A0104_u32 as _);
pub const UI_E_FP_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x802A000B_u32 as _);
pub const UI_E_ILLEGAL_REENTRANCY: windows_core::HRESULT = windows_core::HRESULT(0x802A0003_u32 as _);
pub const UI_E_INVALID_DIMENSION: windows_core::HRESULT = windows_core::HRESULT(0x802A010B_u32 as _);
pub const UI_E_INVALID_OUTPUT: windows_core::HRESULT = windows_core::HRESULT(0x802A0007_u32 as _);
pub const UI_E_LOOPS_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0x802A0105_u32 as _);
pub const UI_E_OBJECT_SEALED: windows_core::HRESULT = windows_core::HRESULT(0x802A0004_u32 as _);
pub const UI_E_PRIMITIVE_OUT_OF_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x802A010C_u32 as _);
pub const UI_E_SHUTDOWN_CALLED: windows_core::HRESULT = windows_core::HRESULT(0x802A0002_u32 as _);
pub const UI_E_START_KEYFRAME_AFTER_END: windows_core::HRESULT = windows_core::HRESULT(0x802A0103_u32 as _);
pub const UI_E_STORYBOARD_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x802A0101_u32 as _);
pub const UI_E_STORYBOARD_NOT_PLAYING: windows_core::HRESULT = windows_core::HRESULT(0x802A0102_u32 as _);
pub const UI_E_TIMER_CLIENT_ALREADY_CONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x802A010A_u32 as _);
pub const UI_E_TIME_BEFORE_LAST_UPDATE: windows_core::HRESULT = windows_core::HRESULT(0x802A0109_u32 as _);
pub const UI_E_TRANSITION_ALREADY_USED: windows_core::HRESULT = windows_core::HRESULT(0x802A0106_u32 as _);
pub const UI_E_TRANSITION_ECLIPSED: windows_core::HRESULT = windows_core::HRESULT(0x802A0108_u32 as _);
pub const UI_E_TRANSITION_NOT_IN_STORYBOARD: windows_core::HRESULT = windows_core::HRESULT(0x802A0107_u32 as _);
pub const UI_E_VALUE_NOT_DETERMINED: windows_core::HRESULT = windows_core::HRESULT(0x802A0006_u32 as _);
pub const UI_E_VALUE_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x802A0005_u32 as _);
pub const UI_E_WINDOW_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0x802A0201_u32 as _);
pub const UI_E_WRONG_THREAD: windows_core::HRESULT = windows_core::HRESULT(0x802A000C_u32 as _);
pub const UTC_E_ACTION_NOT_SUPPORTED_IN_DESTINATION: windows_core::HRESULT = windows_core::HRESULT(0x87C51044_u32 as _);
pub const UTC_E_AGENT_DIAGNOSTICS_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x87C51055_u32 as _);
pub const UTC_E_ALTERNATIVE_TRACE_CANNOT_PREEMPT: windows_core::HRESULT = windows_core::HRESULT(0x87C51002_u32 as _);
pub const UTC_E_AOT_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x87C51003_u32 as _);
pub const UTC_E_API_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x87C5102B_u32 as _);
pub const UTC_E_API_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x87C5103C_u32 as _);
pub const UTC_E_API_RESULT_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x87C51028_u32 as _);
pub const UTC_E_BINARY_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x87C51034_u32 as _);
pub const UTC_E_CANNOT_LOAD_SCENARIO_EDITOR_XML: windows_core::HRESULT = windows_core::HRESULT(0x87C5101F_u32 as _);
pub const UTC_E_CERT_REV_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x87C5103F_u32 as _);
pub const UTC_E_CHILD_PROCESS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x87C5101D_u32 as _);
pub const UTC_E_COMMAND_LINE_NOT_AUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0x87C5101E_u32 as _);
pub const UTC_E_DELAY_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x87C51025_u32 as _);
pub const UTC_E_DEVICE_TICKET_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x87C51026_u32 as _);
pub const UTC_E_DIAGRULES_SCHEMAVERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x87C5100A_u32 as _);
pub const UTC_E_ESCALATION_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x87C5100F_u32 as _);
pub const UTC_E_ESCALATION_CANCELLED_AT_SHUTDOWN: windows_core::HRESULT = windows_core::HRESULT(0x87C5105A_u32 as _);
pub const UTC_E_ESCALATION_DIRECTORY_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x87C5102F_u32 as _);
pub const UTC_E_ESCALATION_NOT_AUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0x87C5101B_u32 as _);
pub const UTC_E_ESCALATION_TIMED_OUT: windows_core::HRESULT = windows_core::HRESULT(0x87C51020_u32 as _);
pub const UTC_E_EVENTLOG_ENTRY_MALFORMED: windows_core::HRESULT = windows_core::HRESULT(0x87C51009_u32 as _);
pub const UTC_E_EXCLUSIVITY_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x87C5102D_u32 as _);
pub const UTC_E_EXE_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x87C5101A_u32 as _);
pub const UTC_E_FAILED_TO_RECEIVE_AGENT_DIAGNOSTICS: windows_core::HRESULT = windows_core::HRESULT(0x87C51056_u32 as _);
pub const UTC_E_FAILED_TO_RESOLVE_CONTAINER_ID: windows_core::HRESULT = windows_core::HRESULT(0x87C51036_u32 as _);
pub const UTC_E_FAILED_TO_START_NDISCAP: windows_core::HRESULT = windows_core::HRESULT(0x87C51040_u32 as _);
pub const UTC_E_FILTER_FUNCTION_RESTRICTED: windows_core::HRESULT = windows_core::HRESULT(0x87C51048_u32 as _);
pub const UTC_E_FILTER_ILLEGAL_EVAL: windows_core::HRESULT = windows_core::HRESULT(0x87C51053_u32 as _);
pub const UTC_E_FILTER_INVALID_COMMAND: windows_core::HRESULT = windows_core::HRESULT(0x87C51052_u32 as _);
pub const UTC_E_FILTER_INVALID_FUNCTION: windows_core::HRESULT = windows_core::HRESULT(0x87C51050_u32 as _);
pub const UTC_E_FILTER_INVALID_FUNCTION_PARAMS: windows_core::HRESULT = windows_core::HRESULT(0x87C51051_u32 as _);
pub const UTC_E_FILTER_INVALID_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x87C51046_u32 as _);
pub const UTC_E_FILTER_MISSING_ATTRIBUTE: windows_core::HRESULT = windows_core::HRESULT(0x87C51045_u32 as _);
pub const UTC_E_FILTER_VARIABLE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x87C51047_u32 as _);
pub const UTC_E_FILTER_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x87C51049_u32 as _);
pub const UTC_E_FORWARDER_ALREADY_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x87C51008_u32 as _);
pub const UTC_E_FORWARDER_ALREADY_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x87C51007_u32 as _);
pub const UTC_E_FORWARDER_PRODUCER_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x87C51012_u32 as _);
pub const UTC_E_GETFILEINFOACTION_FILE_NOT_APPROVED: windows_core::HRESULT = windows_core::HRESULT(0x87C5105B_u32 as _);
pub const UTC_E_GETFILE_EXTERNAL_PATH_NOT_APPROVED: windows_core::HRESULT = windows_core::HRESULT(0x87C5103D_u32 as _);
pub const UTC_E_GETFILE_FILE_PATH_NOT_APPROVED: windows_core::HRESULT = windows_core::HRESULT(0x87C5102E_u32 as _);
pub const UTC_E_INSUFFICIENT_SPACE_TO_START_TRACE: windows_core::HRESULT = windows_core::HRESULT(0x87C51059_u32 as _);
pub const UTC_E_INTENTIONAL_SCRIPT_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x87C51013_u32 as _);
pub const UTC_E_INVALID_AGGREGATION_STRUCT: windows_core::HRESULT = windows_core::HRESULT(0x87C51043_u32 as _);
pub const UTC_E_INVALID_CUSTOM_FILTER: windows_core::HRESULT = windows_core::HRESULT(0x87C5100C_u32 as _);
pub const UTC_E_INVALID_FILTER: windows_core::HRESULT = windows_core::HRESULT(0x87C51019_u32 as _);
pub const UTC_E_KERNELDUMP_LIMIT_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x87C51041_u32 as _);
pub const UTC_E_MISSING_AGGREGATE_EVENT_TAG: windows_core::HRESULT = windows_core::HRESULT(0x87C51042_u32 as _);
pub const UTC_E_MULTIPLE_TIME_TRIGGER_ON_SINGLE_STATE: windows_core::HRESULT = windows_core::HRESULT(0x87C51033_u32 as _);
pub const UTC_E_NO_WER_LOGGER_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x87C51015_u32 as _);
pub const UTC_E_PERFTRACK_ALREADY_TRACING: windows_core::HRESULT = windows_core::HRESULT(0x87C51010_u32 as _);
pub const UTC_E_REACHED_MAX_ESCALATIONS: windows_core::HRESULT = windows_core::HRESULT(0x87C51011_u32 as _);
pub const UTC_E_REESCALATED_TOO_QUICKLY: windows_core::HRESULT = windows_core::HRESULT(0x87C5100E_u32 as _);
pub const UTC_E_RPC_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x87C51029_u32 as _);
pub const UTC_E_RPC_WAIT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x87C5102A_u32 as _);
pub const UTC_E_SCENARIODEF_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x87C51005_u32 as _);
pub const UTC_E_SCENARIODEF_SCHEMAVERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x87C51018_u32 as _);
pub const UTC_E_SCENARIO_HAS_NO_ACTIONS: windows_core::HRESULT = windows_core::HRESULT(0x87C51057_u32 as _);
pub const UTC_E_SCENARIO_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x87C5103B_u32 as _);
pub const UTC_E_SCRIPT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x87C5103A_u32 as _);
pub const UTC_E_SCRIPT_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x87C5100B_u32 as _);
pub const UTC_E_SCRIPT_TYPE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x87C51004_u32 as _);
pub const UTC_E_SETREGKEYACTION_TYPE_NOT_APPROVED: windows_core::HRESULT = windows_core::HRESULT(0x87C5105C_u32 as _);
pub const UTC_E_SETUP_NOT_AUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0x87C5101C_u32 as _);
pub const UTC_E_SETUP_TIMED_OUT: windows_core::HRESULT = windows_core::HRESULT(0x87C51021_u32 as _);
pub const UTC_E_SIF_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x87C51024_u32 as _);
pub const UTC_E_SQM_INIT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x87C51014_u32 as _);
pub const UTC_E_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x87C51038_u32 as _);
pub const UTC_E_TIME_TRIGGER_INVALID_TIME_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x87C51032_u32 as _);
pub const UTC_E_TIME_TRIGGER_ONLY_VALID_ON_SINGLE_TRANSITION: windows_core::HRESULT = windows_core::HRESULT(0x87C51031_u32 as _);
pub const UTC_E_TIME_TRIGGER_ON_START_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x87C51030_u32 as _);
pub const UTC_E_TOGGLE_TRACE_STARTED: windows_core::HRESULT = windows_core::HRESULT(0x87C51001_u32 as _);
pub const UTC_E_TRACEPROFILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x87C51006_u32 as _);
pub const UTC_E_TRACERS_DONT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x87C51016_u32 as _);
pub const UTC_E_TRACE_BUFFER_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x87C51027_u32 as _);
pub const UTC_E_TRACE_MIN_DURATION_REQUIREMENT_NOT_MET: windows_core::HRESULT = windows_core::HRESULT(0x87C5102C_u32 as _);
pub const UTC_E_TRACE_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x87C5100D_u32 as _);
pub const UTC_E_TRACE_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x87C5105D_u32 as _);
pub const UTC_E_TRIGGER_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x87C51022_u32 as _);
pub const UTC_E_TRIGGER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x87C51023_u32 as _);
pub const UTC_E_TRY_GET_SCENARIO_TIMEOUT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x87C5103E_u32 as _);
pub const UTC_E_TTTRACER_RETURNED_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x87C51054_u32 as _);
pub const UTC_E_TTTRACER_STORAGE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x87C51058_u32 as _);
pub const UTC_E_UNABLE_TO_RESOLVE_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x87C51037_u32 as _);
pub const UTC_E_UNAPPROVED_SCRIPT: windows_core::HRESULT = windows_core::HRESULT(0x87C51039_u32 as _);
pub const UTC_E_WINRT_INIT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x87C51017_u32 as _);
pub const VARIANT_FALSE: VARIANT_BOOL = VARIANT_BOOL(0i16);
pub const VARIANT_TRUE: VARIANT_BOOL = VARIANT_BOOL(-1i16);
pub const VIEW_E_DRAW: windows_core::HRESULT = windows_core::HRESULT(0x80040140_u32 as _);
pub const VIEW_E_FIRST: i32 = -2147221184i32;
pub const VIEW_E_LAST: i32 = -2147221169i32;
pub const VIEW_S_ALREADY_FROZEN: windows_core::HRESULT = windows_core::HRESULT(0x40140_u32 as _);
pub const VIEW_S_FIRST: i32 = 262464i32;
pub const VIEW_S_LAST: i32 = 262479i32;
pub const VM_SAVED_STATE_DUMP_E_GUEST_MEMORY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0370501_u32 as _);
pub const VM_SAVED_STATE_DUMP_E_INVALID_VP_STATE: windows_core::HRESULT = windows_core::HRESULT(0xC0370506_u32 as _);
pub const VM_SAVED_STATE_DUMP_E_NESTED_VIRTUALIZATION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC0370503_u32 as _);
pub const VM_SAVED_STATE_DUMP_E_NO_VP_FOUND_IN_PARTITION_STATE: windows_core::HRESULT = windows_core::HRESULT(0xC0370502_u32 as _);
pub const VM_SAVED_STATE_DUMP_E_PARTITION_STATE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0370500_u32 as _);
pub const VM_SAVED_STATE_DUMP_E_VA_NOT_MAPPED: windows_core::HRESULT = windows_core::HRESULT(0xC0370505_u32 as _);
pub const VM_SAVED_STATE_DUMP_E_VP_VTL_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0xC0370509_u32 as _);
pub const VM_SAVED_STATE_DUMP_E_WINDOWS_KERNEL_IMAGE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC0370504_u32 as _);
pub const VOLMGR_KSR_BYPASS: NTSTATUS = NTSTATUS(0x80380003_u32 as _);
pub const VOLMGR_KSR_ERROR: NTSTATUS = NTSTATUS(0x80380001_u32 as _);
pub const VOLMGR_KSR_READ_ERROR: NTSTATUS = NTSTATUS(0x80380002_u32 as _);
pub const WAIT_ABANDONED: WAIT_EVENT = WAIT_EVENT(128u32);
pub const WAIT_ABANDONED_0: WAIT_EVENT = WAIT_EVENT(128u32);
pub const WAIT_FAILED: WAIT_EVENT = WAIT_EVENT(4294967295u32);
pub const WAIT_IO_COMPLETION: WAIT_EVENT = WAIT_EVENT(192u32);
pub const WAIT_OBJECT_0: WAIT_EVENT = WAIT_EVENT(0u32);
pub const WAIT_TIMEOUT: WAIT_EVENT = WAIT_EVENT(258u32);
pub const WARNING_IPSEC_MM_POLICY_PRUNED: i32 = 13024i32;
pub const WARNING_IPSEC_QM_POLICY_PRUNED: i32 = 13025i32;
pub const WARNING_NO_MD5_MIGRATION: u32 = 946u32;
pub const WBREAK_E_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80041783_u32 as _);
pub const WBREAK_E_END_OF_TEXT: windows_core::HRESULT = windows_core::HRESULT(0x80041780_u32 as _);
pub const WBREAK_E_INIT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80041785_u32 as _);
pub const WBREAK_E_QUERY_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80041782_u32 as _);
pub const WEB_E_INVALID_JSON_NUMBER: windows_core::HRESULT = windows_core::HRESULT(0x83750008_u32 as _);
pub const WEB_E_INVALID_JSON_STRING: windows_core::HRESULT = windows_core::HRESULT(0x83750007_u32 as _);
pub const WEB_E_INVALID_XML: windows_core::HRESULT = windows_core::HRESULT(0x83750002_u32 as _);
pub const WEB_E_JSON_VALUE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x83750009_u32 as _);
pub const WEB_E_MISSING_REQUIRED_ATTRIBUTE: windows_core::HRESULT = windows_core::HRESULT(0x83750004_u32 as _);
pub const WEB_E_MISSING_REQUIRED_ELEMENT: windows_core::HRESULT = windows_core::HRESULT(0x83750003_u32 as _);
pub const WEB_E_RESOURCE_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x83750006_u32 as _);
pub const WEB_E_UNEXPECTED_CONTENT: windows_core::HRESULT = windows_core::HRESULT(0x83750005_u32 as _);
pub const WEB_E_UNSUPPORTED_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x83750001_u32 as _);
pub const WEP_E_BUFFER_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x88010009_u32 as _);
pub const WEP_E_FIXED_DATA_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88010002_u32 as _);
pub const WEP_E_HARDWARE_NOT_COMPLIANT: windows_core::HRESULT = windows_core::HRESULT(0x88010003_u32 as _);
pub const WEP_E_LOCK_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x88010004_u32 as _);
pub const WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES: windows_core::HRESULT = windows_core::HRESULT(0x88010001_u32 as _);
pub const WEP_E_NO_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0x88010006_u32 as _);
pub const WEP_E_OS_NOT_PROTECTED: windows_core::HRESULT = windows_core::HRESULT(0x88010007_u32 as _);
pub const WEP_E_PROTECTION_SUSPENDED: windows_core::HRESULT = windows_core::HRESULT(0x88010005_u32 as _);
pub const WEP_E_UNEXPECTED_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x88010008_u32 as _);
pub const WER_E_ALREADY_REPORTING: windows_core::HRESULT = windows_core::HRESULT(0x801B8004_u32 as _);
pub const WER_E_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x801B8001_u32 as _);
pub const WER_E_CRASH_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x801B8000_u32 as _);
pub const WER_E_DUMP_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x801B8005_u32 as _);
pub const WER_E_INSUFFICIENT_CONSENT: windows_core::HRESULT = windows_core::HRESULT(0x801B8006_u32 as _);
pub const WER_E_NETWORK_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x801B8002_u32 as _);
pub const WER_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x801B8003_u32 as _);
pub const WER_E_TOO_HEAVY: windows_core::HRESULT = windows_core::HRESULT(0x801B8007_u32 as _);
pub const WER_S_ASSERT_CONTINUE: windows_core::HRESULT = windows_core::HRESULT(0x1B000A_u32 as _);
pub const WER_S_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x1B0003_u32 as _);
pub const WER_S_DISABLED_ARCHIVE: windows_core::HRESULT = windows_core::HRESULT(0x1B0006_u32 as _);
pub const WER_S_DISABLED_QUEUE: windows_core::HRESULT = windows_core::HRESULT(0x1B0005_u32 as _);
pub const WER_S_IGNORE_ALL_ASSERTS: windows_core::HRESULT = windows_core::HRESULT(0x1B0009_u32 as _);
pub const WER_S_IGNORE_ASSERT_INSTANCE: windows_core::HRESULT = windows_core::HRESULT(0x1B0008_u32 as _);
pub const WER_S_REPORT_ASYNC: windows_core::HRESULT = windows_core::HRESULT(0x1B0007_u32 as _);
pub const WER_S_REPORT_DEBUG: windows_core::HRESULT = windows_core::HRESULT(0x1B0000_u32 as _);
pub const WER_S_REPORT_QUEUED: windows_core::HRESULT = windows_core::HRESULT(0x1B0002_u32 as _);
pub const WER_S_REPORT_UPLOADED: windows_core::HRESULT = windows_core::HRESULT(0x1B0001_u32 as _);
pub const WER_S_REPORT_UPLOADED_CAB: windows_core::HRESULT = windows_core::HRESULT(0x1B000C_u32 as _);
pub const WER_S_SUSPENDED_UPLOAD: windows_core::HRESULT = windows_core::HRESULT(0x1B0004_u32 as _);
pub const WER_S_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x1B000B_u32 as _);
pub const WHV_E_GPA_RANGE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80370305_u32 as _);
pub const WHV_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x80370301_u32 as _);
pub const WHV_E_INVALID_PARTITION_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80370304_u32 as _);
pub const WHV_E_INVALID_VP_REGISTER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80370309_u32 as _);
pub const WHV_E_INVALID_VP_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80370308_u32 as _);
pub const WHV_E_UNKNOWN_CAPABILITY: windows_core::HRESULT = windows_core::HRESULT(0x80370300_u32 as _);
pub const WHV_E_UNKNOWN_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80370302_u32 as _);
pub const WHV_E_UNSUPPORTED_HYPERVISOR_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80370303_u32 as _);
pub const WHV_E_UNSUPPORTED_PROCESSOR_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80370310_u32 as _);
pub const WHV_E_VP_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80370306_u32 as _);
pub const WHV_E_VP_DOES_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80370307_u32 as _);
pub const WINCODEC_ERR_ALREADYLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x88982F0D_u32 as _);
pub const WINCODEC_ERR_BADHEADER: windows_core::HRESULT = windows_core::HRESULT(0x88982F61_u32 as _);
pub const WINCODEC_ERR_BADIMAGE: windows_core::HRESULT = windows_core::HRESULT(0x88982F60_u32 as _);
pub const WINCODEC_ERR_BADMETADATAHEADER: windows_core::HRESULT = windows_core::HRESULT(0x88982F63_u32 as _);
pub const WINCODEC_ERR_BADSTREAMDATA: windows_core::HRESULT = windows_core::HRESULT(0x88982F70_u32 as _);
pub const WINCODEC_ERR_CODECNOTHUMBNAIL: windows_core::HRESULT = windows_core::HRESULT(0x88982F44_u32 as _);
pub const WINCODEC_ERR_CODECPRESENT: windows_core::HRESULT = windows_core::HRESULT(0x88982F43_u32 as _);
pub const WINCODEC_ERR_CODECTOOMANYSCANLINES: windows_core::HRESULT = windows_core::HRESULT(0x88982F46_u32 as _);
pub const WINCODEC_ERR_COMPONENTINITIALIZEFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x88982F8B_u32 as _);
pub const WINCODEC_ERR_COMPONENTNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x88982F50_u32 as _);
pub const WINCODEC_ERR_DUPLICATEMETADATAPRESENT: windows_core::HRESULT = windows_core::HRESULT(0x88982F8D_u32 as _);
pub const WINCODEC_ERR_FRAMEMISSING: windows_core::HRESULT = windows_core::HRESULT(0x88982F62_u32 as _);
pub const WINCODEC_ERR_IMAGESIZEOUTOFRANGE: windows_core::HRESULT = windows_core::HRESULT(0x88982F51_u32 as _);
pub const WINCODEC_ERR_INSUFFICIENTBUFFER: windows_core::HRESULT = windows_core::HRESULT(0x88982F8C_u32 as _);
pub const WINCODEC_ERR_INTERNALERROR: windows_core::HRESULT = windows_core::HRESULT(0x88982F48_u32 as _);
pub const WINCODEC_ERR_INVALIDJPEGSCANINDEX: windows_core::HRESULT = windows_core::HRESULT(0x88982F96_u32 as _);
pub const WINCODEC_ERR_INVALIDPROGRESSIVELEVEL: windows_core::HRESULT = windows_core::HRESULT(0x88982F95_u32 as _);
pub const WINCODEC_ERR_INVALIDQUERYCHARACTER: windows_core::HRESULT = windows_core::HRESULT(0x88982F93_u32 as _);
pub const WINCODEC_ERR_INVALIDQUERYREQUEST: windows_core::HRESULT = windows_core::HRESULT(0x88982F90_u32 as _);
pub const WINCODEC_ERR_INVALIDREGISTRATION: windows_core::HRESULT = windows_core::HRESULT(0x88982F8A_u32 as _);
pub const WINCODEC_ERR_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x88982F0C_u32 as _);
pub const WINCODEC_ERR_PALETTEUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88982F45_u32 as _);
pub const WINCODEC_ERR_PROPERTYNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x88982F40_u32 as _);
pub const WINCODEC_ERR_PROPERTYNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88982F41_u32 as _);
pub const WINCODEC_ERR_PROPERTYSIZE: windows_core::HRESULT = windows_core::HRESULT(0x88982F42_u32 as _);
pub const WINCODEC_ERR_PROPERTYUNEXPECTEDTYPE: windows_core::HRESULT = windows_core::HRESULT(0x88982F8E_u32 as _);
pub const WINCODEC_ERR_REQUESTONLYVALIDATMETADATAROOT: windows_core::HRESULT = windows_core::HRESULT(0x88982F92_u32 as _);
pub const WINCODEC_ERR_SOURCERECTDOESNOTMATCHDIMENSIONS: windows_core::HRESULT = windows_core::HRESULT(0x88982F49_u32 as _);
pub const WINCODEC_ERR_STREAMNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88982F73_u32 as _);
pub const WINCODEC_ERR_STREAMREAD: windows_core::HRESULT = windows_core::HRESULT(0x88982F72_u32 as _);
pub const WINCODEC_ERR_STREAMWRITE: windows_core::HRESULT = windows_core::HRESULT(0x88982F71_u32 as _);
pub const WINCODEC_ERR_TOOMUCHMETADATA: windows_core::HRESULT = windows_core::HRESULT(0x88982F52_u32 as _);
pub const WINCODEC_ERR_UNEXPECTEDMETADATATYPE: windows_core::HRESULT = windows_core::HRESULT(0x88982F91_u32 as _);
pub const WINCODEC_ERR_UNEXPECTEDSIZE: windows_core::HRESULT = windows_core::HRESULT(0x88982F8F_u32 as _);
pub const WINCODEC_ERR_UNKNOWNIMAGEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x88982F07_u32 as _);
pub const WINCODEC_ERR_UNSUPPORTEDOPERATION: windows_core::HRESULT = windows_core::HRESULT(0x88982F81_u32 as _);
pub const WINCODEC_ERR_UNSUPPORTEDPIXELFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x88982F80_u32 as _);
pub const WINCODEC_ERR_UNSUPPORTEDVERSION: windows_core::HRESULT = windows_core::HRESULT(0x88982F0B_u32 as _);
pub const WINCODEC_ERR_VALUEOUTOFRANGE: windows_core::HRESULT = windows_core::HRESULT(0x88982F05_u32 as _);
pub const WINCODEC_ERR_WIN32ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88982F94_u32 as _);
pub const WINCODEC_ERR_WRONGSTATE: windows_core::HRESULT = windows_core::HRESULT(0x88982F04_u32 as _);
pub const WININET_E_ASYNC_THREAD_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80072F0F_u32 as _);
pub const WININET_E_BAD_AUTO_PROXY_SCRIPT: windows_core::HRESULT = windows_core::HRESULT(0x80072F86_u32 as _);
pub const WININET_E_BAD_OPTION_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x80072EEA_u32 as _);
pub const WININET_E_BAD_REGISTRY_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80072EF6_u32 as _);
pub const WININET_E_CANNOT_CONNECT: windows_core::HRESULT = windows_core::HRESULT(0x80072EFD_u32 as _);
pub const WININET_E_CHG_POST_IS_NON_SECURE: windows_core::HRESULT = windows_core::HRESULT(0x80072F0A_u32 as _);
pub const WININET_E_CLIENT_AUTH_CERT_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80072F0C_u32 as _);
pub const WININET_E_CLIENT_AUTH_NOT_SETUP: windows_core::HRESULT = windows_core::HRESULT(0x80072F0E_u32 as _);
pub const WININET_E_CONNECTION_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x80072EFE_u32 as _);
pub const WININET_E_CONNECTION_RESET: windows_core::HRESULT = windows_core::HRESULT(0x80072EFF_u32 as _);
pub const WININET_E_COOKIE_DECLINED: windows_core::HRESULT = windows_core::HRESULT(0x80072F82_u32 as _);
pub const WININET_E_COOKIE_NEEDS_CONFIRMATION: windows_core::HRESULT = windows_core::HRESULT(0x80072F81_u32 as _);
pub const WININET_E_DECODING_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80072F8F_u32 as _);
pub const WININET_E_DIALOG_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80072F11_u32 as _);
pub const WININET_E_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80072F83_u32 as _);
pub const WININET_E_DOWNLEVEL_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80072F77_u32 as _);
pub const WININET_E_EXTENDED_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80072EE3_u32 as _);
pub const WININET_E_FAILED_DUETOSECURITYCHECK: windows_core::HRESULT = windows_core::HRESULT(0x80072F8B_u32 as _);
pub const WININET_E_FORCE_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80072F00_u32 as _);
pub const WININET_E_HANDLE_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80072F04_u32 as _);
pub const WININET_E_HEADER_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80072F7B_u32 as _);
pub const WININET_E_HEADER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80072F76_u32 as _);
pub const WININET_E_HTTPS_HTTP_SUBMIT_REDIR: windows_core::HRESULT = windows_core::HRESULT(0x80072F14_u32 as _);
pub const WININET_E_HTTPS_TO_HTTP_ON_REDIR: windows_core::HRESULT = windows_core::HRESULT(0x80072F08_u32 as _);
pub const WININET_E_HTTP_TO_HTTPS_ON_REDIR: windows_core::HRESULT = windows_core::HRESULT(0x80072F07_u32 as _);
pub const WININET_E_INCORRECT_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80072EFB_u32 as _);
pub const WININET_E_INCORRECT_HANDLE_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80072EF3_u32 as _);
pub const WININET_E_INCORRECT_HANDLE_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80072EF2_u32 as _);
pub const WININET_E_INCORRECT_PASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x80072EEE_u32 as _);
pub const WININET_E_INCORRECT_USER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80072EED_u32 as _);
pub const WININET_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80072EE4_u32 as _);
pub const WININET_E_INVALID_CA: windows_core::HRESULT = windows_core::HRESULT(0x80072F0D_u32 as _);
pub const WININET_E_INVALID_HEADER: windows_core::HRESULT = windows_core::HRESULT(0x80072F79_u32 as _);
pub const WININET_E_INVALID_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80072EF0_u32 as _);
pub const WININET_E_INVALID_OPTION: windows_core::HRESULT = windows_core::HRESULT(0x80072EE9_u32 as _);
pub const WININET_E_INVALID_PROXY_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80072F01_u32 as _);
pub const WININET_E_INVALID_QUERY_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80072F7A_u32 as _);
pub const WININET_E_INVALID_SERVER_RESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x80072F78_u32 as _);
pub const WININET_E_INVALID_URL: windows_core::HRESULT = windows_core::HRESULT(0x80072EE5_u32 as _);
pub const WININET_E_ITEM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80072EFC_u32 as _);
pub const WININET_E_LOGIN_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80072EEF_u32 as _);
pub const WININET_E_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: windows_core::HRESULT = windows_core::HRESULT(0x80072F8E_u32 as _);
pub const WININET_E_MIXED_SECURITY: windows_core::HRESULT = windows_core::HRESULT(0x80072F09_u32 as _);
pub const WININET_E_NAME_NOT_RESOLVED: windows_core::HRESULT = windows_core::HRESULT(0x80072EE7_u32 as _);
pub const WININET_E_NEED_UI: windows_core::HRESULT = windows_core::HRESULT(0x80072F02_u32 as _);
pub const WININET_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80072F8C_u32 as _);
pub const WININET_E_NOT_PROXY_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80072EF4_u32 as _);
pub const WININET_E_NOT_REDIRECTED: windows_core::HRESULT = windows_core::HRESULT(0x80072F80_u32 as _);
pub const WININET_E_NO_CALLBACK: windows_core::HRESULT = windows_core::HRESULT(0x80072EF9_u32 as _);
pub const WININET_E_NO_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80072EF8_u32 as _);
pub const WININET_E_NO_DIRECT_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x80072EF7_u32 as _);
pub const WININET_E_NO_NEW_CONTAINERS: windows_core::HRESULT = windows_core::HRESULT(0x80072F13_u32 as _);
pub const WININET_E_OPERATION_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80072EF1_u32 as _);
pub const WININET_E_OPTION_NOT_SETTABLE: windows_core::HRESULT = windows_core::HRESULT(0x80072EEB_u32 as _);
pub const WININET_E_OUT_OF_HANDLES: windows_core::HRESULT = windows_core::HRESULT(0x80072EE1_u32 as _);
pub const WININET_E_POST_IS_NON_SECURE: windows_core::HRESULT = windows_core::HRESULT(0x80072F0B_u32 as _);
pub const WININET_E_PROTOCOL_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80072EE8_u32 as _);
pub const WININET_E_PROXY_SERVER_UNREACHABLE: windows_core::HRESULT = windows_core::HRESULT(0x80072F85_u32 as _);
pub const WININET_E_REDIRECT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80072F7C_u32 as _);
pub const WININET_E_REDIRECT_NEEDS_CONFIRMATION: windows_core::HRESULT = windows_core::HRESULT(0x80072F88_u32 as _);
pub const WININET_E_REDIRECT_SCHEME_CHANGE: windows_core::HRESULT = windows_core::HRESULT(0x80072F10_u32 as _);
pub const WININET_E_REGISTRY_VALUE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80072EF5_u32 as _);
pub const WININET_E_REQUEST_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80072EFA_u32 as _);
pub const WININET_E_RETRY_DIALOG: windows_core::HRESULT = windows_core::HRESULT(0x80072F12_u32 as _);
pub const WININET_E_SECURITY_CHANNEL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80072F7D_u32 as _);
pub const WININET_E_SEC_CERT_CN_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80072F06_u32 as _);
pub const WININET_E_SEC_CERT_DATE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80072F05_u32 as _);
pub const WININET_E_SEC_CERT_ERRORS: windows_core::HRESULT = windows_core::HRESULT(0x80072F17_u32 as _);
pub const WININET_E_SEC_CERT_REVOKED: windows_core::HRESULT = windows_core::HRESULT(0x80072F8A_u32 as _);
pub const WININET_E_SEC_CERT_REV_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80072F19_u32 as _);
pub const WININET_E_SEC_INVALID_CERT: windows_core::HRESULT = windows_core::HRESULT(0x80072F89_u32 as _);
pub const WININET_E_SERVER_UNREACHABLE: windows_core::HRESULT = windows_core::HRESULT(0x80072F84_u32 as _);
pub const WININET_E_SHUTDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80072EEC_u32 as _);
pub const WININET_E_TCPIP_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80072F7F_u32 as _);
pub const WININET_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80072EE2_u32 as _);
pub const WININET_E_UNABLE_TO_CACHE_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80072F7E_u32 as _);
pub const WININET_E_UNABLE_TO_DOWNLOAD_SCRIPT: windows_core::HRESULT = windows_core::HRESULT(0x80072F87_u32 as _);
pub const WININET_E_UNRECOGNIZED_SCHEME: windows_core::HRESULT = windows_core::HRESULT(0x80072EE6_u32 as _);
pub const WINML_ERR_INVALID_BINDING: windows_core::HRESULT = windows_core::HRESULT(0x88900002_u32 as _);
pub const WINML_ERR_INVALID_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x88900001_u32 as _);
pub const WINML_ERR_SIZE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x88900004_u32 as _);
pub const WINML_ERR_VALUE_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x88900003_u32 as _);
pub const WINVER: u32 = 1280u32;
pub const WINVER_MAXVER: u32 = 2560u32;
pub const WPN_E_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x803E0117_u32 as _);
pub const WPN_E_ALL_URL_NOT_COMPLETED: windows_core::HRESULT = windows_core::HRESULT(0x803E0203_u32 as _);
pub const WPN_E_CALLBACK_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x803E0206_u32 as _);
pub const WPN_E_CHANNEL_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0x803E0100_u32 as _);
pub const WPN_E_CHANNEL_REQUEST_NOT_COMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x803E0101_u32 as _);
pub const WPN_E_CLOUD_AUTH_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803E011A_u32 as _);
pub const WPN_E_CLOUD_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803E0109_u32 as _);
pub const WPN_E_CLOUD_DISABLED_FOR_APP: windows_core::HRESULT = windows_core::HRESULT(0x803E020B_u32 as _);
pub const WPN_E_CLOUD_INCAPABLE: windows_core::HRESULT = windows_core::HRESULT(0x803E0110_u32 as _);
pub const WPN_E_CLOUD_SERVICE_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803E011B_u32 as _);
pub const WPN_E_DEV_ID_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x803E0120_u32 as _);
pub const WPN_E_DUPLICATE_CHANNEL: windows_core::HRESULT = windows_core::HRESULT(0x803E0104_u32 as _);
pub const WPN_E_DUPLICATE_REGISTRATION: windows_core::HRESULT = windows_core::HRESULT(0x803E0118_u32 as _);
pub const WPN_E_FAILED_LOCK_SCREEN_UPDATE_INTIALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x803E011C_u32 as _);
pub const WPN_E_GROUP_ALPHANUMERIC: windows_core::HRESULT = windows_core::HRESULT(0x803E020A_u32 as _);
pub const WPN_E_GROUP_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x803E0209_u32 as _);
pub const WPN_E_IMAGE_NOT_FOUND_IN_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x803E0202_u32 as _);
pub const WPN_E_INTERNET_INCAPABLE: windows_core::HRESULT = windows_core::HRESULT(0x803E0113_u32 as _);
pub const WPN_E_INVALID_APP: windows_core::HRESULT = windows_core::HRESULT(0x803E0102_u32 as _);
pub const WPN_E_INVALID_CLOUD_IMAGE: windows_core::HRESULT = windows_core::HRESULT(0x803E0204_u32 as _);
pub const WPN_E_INVALID_HTTP_STATUS_CODE: windows_core::HRESULT = windows_core::HRESULT(0x803E012B_u32 as _);
pub const WPN_E_NOTIFICATION_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803E0111_u32 as _);
pub const WPN_E_NOTIFICATION_HIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x803E0107_u32 as _);
pub const WPN_E_NOTIFICATION_ID_MATCHED: windows_core::HRESULT = windows_core::HRESULT(0x803E0205_u32 as _);
pub const WPN_E_NOTIFICATION_INCAPABLE: windows_core::HRESULT = windows_core::HRESULT(0x803E0112_u32 as _);
pub const WPN_E_NOTIFICATION_NOT_POSTED: windows_core::HRESULT = windows_core::HRESULT(0x803E0108_u32 as _);
pub const WPN_E_NOTIFICATION_POSTED: windows_core::HRESULT = windows_core::HRESULT(0x803E0106_u32 as _);
pub const WPN_E_NOTIFICATION_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x803E0115_u32 as _);
pub const WPN_E_NOTIFICATION_TYPE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803E0114_u32 as _);
pub const WPN_E_OUTSTANDING_CHANNEL_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x803E0103_u32 as _);
pub const WPN_E_OUT_OF_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x803E0200_u32 as _);
pub const WPN_E_PLATFORM_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803E0105_u32 as _);
pub const WPN_E_POWER_SAVE: windows_core::HRESULT = windows_core::HRESULT(0x803E0201_u32 as _);
pub const WPN_E_PUSH_NOTIFICATION_INCAPABLE: windows_core::HRESULT = windows_core::HRESULT(0x803E0119_u32 as _);
pub const WPN_E_STORAGE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x803E0208_u32 as _);
pub const WPN_E_TAG_ALPHANUMERIC: windows_core::HRESULT = windows_core::HRESULT(0x803E012A_u32 as _);
pub const WPN_E_TAG_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x803E0116_u32 as _);
pub const WPN_E_TOAST_NOTIFICATION_DROPPED: windows_core::HRESULT = windows_core::HRESULT(0x803E0207_u32 as _);
pub const WS_E_ADDRESS_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x803D000B_u32 as _);
pub const WS_E_ADDRESS_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803D000C_u32 as _);
pub const WS_E_ENDPOINT_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x803D0005_u32 as _);
pub const WS_E_ENDPOINT_ACTION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803D0011_u32 as _);
pub const WS_E_ENDPOINT_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x803D0014_u32 as _);
pub const WS_E_ENDPOINT_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x803D000F_u32 as _);
pub const WS_E_ENDPOINT_FAULT_RECEIVED: windows_core::HRESULT = windows_core::HRESULT(0x803D0013_u32 as _);
pub const WS_E_ENDPOINT_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803D000E_u32 as _);
pub const WS_E_ENDPOINT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x803D000D_u32 as _);
pub const WS_E_ENDPOINT_TOO_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x803D0012_u32 as _);
pub const WS_E_ENDPOINT_UNREACHABLE: windows_core::HRESULT = windows_core::HRESULT(0x803D0010_u32 as _);
pub const WS_E_INVALID_ENDPOINT_URL: windows_core::HRESULT = windows_core::HRESULT(0x803D0020_u32 as _);
pub const WS_E_INVALID_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x803D0000_u32 as _);
pub const WS_E_INVALID_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x803D0003_u32 as _);
pub const WS_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x803D0017_u32 as _);
pub const WS_E_NO_TRANSLATION_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803D0009_u32 as _);
pub const WS_E_NUMERIC_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x803D0002_u32 as _);
pub const WS_E_OBJECT_FAULTED: windows_core::HRESULT = windows_core::HRESULT(0x803D0001_u32 as _);
pub const WS_E_OPERATION_ABANDONED: windows_core::HRESULT = windows_core::HRESULT(0x803D0007_u32 as _);
pub const WS_E_OPERATION_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x803D0004_u32 as _);
pub const WS_E_OPERATION_TIMED_OUT: windows_core::HRESULT = windows_core::HRESULT(0x803D0006_u32 as _);
pub const WS_E_OTHER: windows_core::HRESULT = windows_core::HRESULT(0x803D0021_u32 as _);
pub const WS_E_PROXY_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x803D0016_u32 as _);
pub const WS_E_PROXY_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x803D0015_u32 as _);
pub const WS_E_PROXY_REQUIRES_BASIC_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D0018_u32 as _);
pub const WS_E_PROXY_REQUIRES_DIGEST_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D0019_u32 as _);
pub const WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D001B_u32 as _);
pub const WS_E_PROXY_REQUIRES_NTLM_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D001A_u32 as _);
pub const WS_E_QUOTA_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x803D0008_u32 as _);
pub const WS_E_SECURITY_SYSTEM_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x803D0023_u32 as _);
pub const WS_E_SECURITY_TOKEN_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x803D0022_u32 as _);
pub const WS_E_SECURITY_VERIFICATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x803D000A_u32 as _);
pub const WS_E_SERVER_REQUIRES_BASIC_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D001C_u32 as _);
pub const WS_E_SERVER_REQUIRES_DIGEST_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D001D_u32 as _);
pub const WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D001F_u32 as _);
pub const WS_E_SERVER_REQUIRES_NTLM_AUTH: windows_core::HRESULT = windows_core::HRESULT(0x803D001E_u32 as _);
pub const WS_S_ASYNC: windows_core::HRESULT = windows_core::HRESULT(0x3D0000_u32 as _);
pub const WS_S_END: windows_core::HRESULT = windows_core::HRESULT(0x3D0001_u32 as _);
pub const XACT_E_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004D019_u32 as _);
pub const XACT_E_ABORTING: windows_core::HRESULT = windows_core::HRESULT(0x8004D029_u32 as _);
pub const XACT_E_ALREADYINPROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004D018_u32 as _);
pub const XACT_E_ALREADYOTHERSINGLEPHASE: windows_core::HRESULT = windows_core::HRESULT(0x8004D000_u32 as _);
pub const XACT_E_CANTRETAIN: windows_core::HRESULT = windows_core::HRESULT(0x8004D001_u32 as _);
pub const XACT_E_CLERKEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8004D081_u32 as _);
pub const XACT_E_CLERKNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8004D080_u32 as _);
pub const XACT_E_COMMITFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004D002_u32 as _);
pub const XACT_E_COMMITPREVENTED: windows_core::HRESULT = windows_core::HRESULT(0x8004D003_u32 as _);
pub const XACT_E_CONNECTION_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x8004D01D_u32 as _);
pub const XACT_E_CONNECTION_DOWN: windows_core::HRESULT = windows_core::HRESULT(0x8004D01C_u32 as _);
pub const XACT_E_DEST_TMNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8004D022_u32 as _);
pub const XACT_E_FIRST: u32 = 2147799040u32;
pub const XACT_E_HEURISTICABORT: windows_core::HRESULT = windows_core::HRESULT(0x8004D004_u32 as _);
pub const XACT_E_HEURISTICCOMMIT: windows_core::HRESULT = windows_core::HRESULT(0x8004D005_u32 as _);
pub const XACT_E_HEURISTICDAMAGE: windows_core::HRESULT = windows_core::HRESULT(0x8004D006_u32 as _);
pub const XACT_E_HEURISTICDANGER: windows_core::HRESULT = windows_core::HRESULT(0x8004D007_u32 as _);
pub const XACT_E_INDOUBT: windows_core::HRESULT = windows_core::HRESULT(0x8004D016_u32 as _);
pub const XACT_E_INVALIDCOOKIE: windows_core::HRESULT = windows_core::HRESULT(0x8004D015_u32 as _);
pub const XACT_E_INVALIDLSN: windows_core::HRESULT = windows_core::HRESULT(0x8004D084_u32 as _);
pub const XACT_E_ISOLATIONLEVEL: windows_core::HRESULT = windows_core::HRESULT(0x8004D008_u32 as _);
pub const XACT_E_LAST: u32 = 2147799083u32;
pub const XACT_E_LOGFULL: windows_core::HRESULT = windows_core::HRESULT(0x8004D01A_u32 as _);
pub const XACT_E_LU_TX_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8004D02C_u32 as _);
pub const XACT_E_NETWORK_TX_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8004D024_u32 as _);
pub const XACT_E_NOASYNC: windows_core::HRESULT = windows_core::HRESULT(0x8004D009_u32 as _);
pub const XACT_E_NOENLIST: windows_core::HRESULT = windows_core::HRESULT(0x8004D00A_u32 as _);
pub const XACT_E_NOIMPORTOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x8004D014_u32 as _);
pub const XACT_E_NOISORETAIN: windows_core::HRESULT = windows_core::HRESULT(0x8004D00B_u32 as _);
pub const XACT_E_NORESOURCE: windows_core::HRESULT = windows_core::HRESULT(0x8004D00C_u32 as _);
pub const XACT_E_NOTCURRENT: windows_core::HRESULT = windows_core::HRESULT(0x8004D00D_u32 as _);
pub const XACT_E_NOTIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004D017_u32 as _);
pub const XACT_E_NOTRANSACTION: windows_core::HRESULT = windows_core::HRESULT(0x8004D00E_u32 as _);
pub const XACT_E_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004D00F_u32 as _);
pub const XACT_E_PARTNER_NETWORK_TX_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8004D025_u32 as _);
pub const XACT_E_PULL_COMM_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8004D02B_u32 as _);
pub const XACT_E_PUSH_COMM_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8004D02A_u32 as _);
pub const XACT_E_RECOVERYINPROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004D082_u32 as _);
pub const XACT_E_REENLISTTIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004D01E_u32 as _);
pub const XACT_E_REPLAYREQUEST: windows_core::HRESULT = windows_core::HRESULT(0x8004D085_u32 as _);
pub const XACT_E_TIP_CONNECT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004D01F_u32 as _);
pub const XACT_E_TIP_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8004D023_u32 as _);
pub const XACT_E_TIP_PROTOCOL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004D020_u32 as _);
pub const XACT_E_TIP_PULL_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004D021_u32 as _);
pub const XACT_E_TMNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8004D01B_u32 as _);
pub const XACT_E_TRANSACTIONCLOSED: windows_core::HRESULT = windows_core::HRESULT(0x8004D083_u32 as _);
pub const XACT_E_UNABLE_TO_LOAD_DTC_PROXY: windows_core::HRESULT = windows_core::HRESULT(0x8004D028_u32 as _);
pub const XACT_E_UNABLE_TO_READ_DTC_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x8004D027_u32 as _);
pub const XACT_E_UNKNOWNRMGRID: windows_core::HRESULT = windows_core::HRESULT(0x8004D010_u32 as _);
pub const XACT_E_WRONGSTATE: windows_core::HRESULT = windows_core::HRESULT(0x8004D011_u32 as _);
pub const XACT_E_WRONGUOW: windows_core::HRESULT = windows_core::HRESULT(0x8004D012_u32 as _);
pub const XACT_E_XA_TX_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8004D026_u32 as _);
pub const XACT_E_XTIONEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8004D013_u32 as _);
pub const XACT_S_ABORTING: windows_core::HRESULT = windows_core::HRESULT(0x4D008_u32 as _);
pub const XACT_S_ALLNORETAIN: windows_core::HRESULT = windows_core::HRESULT(0x4D007_u32 as _);
pub const XACT_S_ASYNC: windows_core::HRESULT = windows_core::HRESULT(0x4D000_u32 as _);
pub const XACT_S_DEFECT: windows_core::HRESULT = windows_core::HRESULT(0x4D001_u32 as _);
pub const XACT_S_FIRST: u32 = 315392u32;
pub const XACT_S_LAST: u32 = 315408u32;
pub const XACT_S_LASTRESOURCEMANAGER: windows_core::HRESULT = windows_core::HRESULT(0x4D010_u32 as _);
pub const XACT_S_LOCALLY_OK: windows_core::HRESULT = windows_core::HRESULT(0x4D00A_u32 as _);
pub const XACT_S_MADECHANGESCONTENT: windows_core::HRESULT = windows_core::HRESULT(0x4D005_u32 as _);
pub const XACT_S_MADECHANGESINFORM: windows_core::HRESULT = windows_core::HRESULT(0x4D006_u32 as _);
pub const XACT_S_OKINFORM: windows_core::HRESULT = windows_core::HRESULT(0x4D004_u32 as _);
pub const XACT_S_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x4D002_u32 as _);
pub const XACT_S_SINGLEPHASE: windows_core::HRESULT = windows_core::HRESULT(0x4D009_u32 as _);
pub const XACT_S_SOMENORETAIN: windows_core::HRESULT = windows_core::HRESULT(0x4D003_u32 as _);
pub const XENROLL_E_CANNOT_ADD_ROOT_CERT: windows_core::HRESULT = windows_core::HRESULT(0x80095001_u32 as _);
pub const XENROLL_E_KEYSPEC_SMIME_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80095005_u32 as _);
pub const XENROLL_E_KEY_NOT_EXPORTABLE: windows_core::HRESULT = windows_core::HRESULT(0x80095000_u32 as _);
pub const XENROLL_E_RESPONSE_KA_HASH_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80095004_u32 as _);
pub const XENROLL_E_RESPONSE_KA_HASH_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80095002_u32 as _);
pub const XENROLL_E_RESPONSE_UNEXPECTED_KA_HASH: windows_core::HRESULT = windows_core::HRESULT(0x80095003_u32 as _);
pub const _WIN32_IE_MAXVER: u32 = 2560u32;
pub const _WIN32_MAXVER: u32 = 2560u32;
pub const _WIN32_WINDOWS_MAXVER: u32 = 2560u32;
pub const _WIN32_WINNT_MAXVER: u32 = 2560u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DUPLICATE_HANDLE_OPTIONS(pub u32);
impl windows_core::TypeKind for DUPLICATE_HANDLE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DUPLICATE_HANDLE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DUPLICATE_HANDLE_OPTIONS").field(&self.0).finish()
    }
}
impl DUPLICATE_HANDLE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DUPLICATE_HANDLE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DUPLICATE_HANDLE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DUPLICATE_HANDLE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DUPLICATE_HANDLE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DUPLICATE_HANDLE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GENERIC_ACCESS_RIGHTS(pub u32);
impl windows_core::TypeKind for GENERIC_ACCESS_RIGHTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GENERIC_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GENERIC_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl GENERIC_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GENERIC_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GENERIC_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GENERIC_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GENERIC_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GENERIC_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HANDLE_FLAGS(pub u32);
impl windows_core::TypeKind for HANDLE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HANDLE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDLE_FLAGS").field(&self.0).finish()
    }
}
impl HANDLE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HANDLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HANDLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HANDLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HANDLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HANDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NTSTATUS_FACILITY_CODE(pub u32);
impl windows_core::TypeKind for NTSTATUS_FACILITY_CODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NTSTATUS_FACILITY_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NTSTATUS_FACILITY_CODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NTSTATUS_SEVERITY_CODE(pub u32);
impl windows_core::TypeKind for NTSTATUS_SEVERITY_CODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NTSTATUS_SEVERITY_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NTSTATUS_SEVERITY_CODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WAIT_EVENT(pub u32);
impl windows_core::TypeKind for WAIT_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WAIT_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WAIT_EVENT").field(&self.0).finish()
    }
}
#[must_use]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WIN32_ERROR(pub u32);
impl windows_core::TypeKind for WIN32_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WIN32_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WIN32_ERROR").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct APP_LOCAL_DEVICE_ID {
    pub value: [u8; 32],
}
impl Copy for APP_LOCAL_DEVICE_ID {}
impl Clone for APP_LOCAL_DEVICE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for APP_LOCAL_DEVICE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("APP_LOCAL_DEVICE_ID").field("value", &self.value).finish()
    }
}
impl windows_core::TypeKind for APP_LOCAL_DEVICE_ID {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for APP_LOCAL_DEVICE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for APP_LOCAL_DEVICE_ID {}
impl Default for APP_LOCAL_DEVICE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[must_use]
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct BOOL(pub i32);
impl Default for BOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for BOOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for BOOL {}
impl core::fmt::Debug for BOOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BOOL").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for BOOL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct BOOLEAN(pub u8);
impl Default for BOOLEAN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for BOOLEAN {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for BOOLEAN {}
impl core::fmt::Debug for BOOLEAN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BOOLEAN").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for BOOLEAN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct COLORREF(pub u32);
impl Default for COLORREF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for COLORREF {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for COLORREF {}
impl core::fmt::Debug for COLORREF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COLORREF").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for COLORREF {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Copy for DECIMAL {}
impl Clone for DECIMAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DECIMAL {
    type TypeKind = windows_core::CopyType;
}
impl Default for DECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Copy for DECIMAL_0 {}
impl Clone for DECIMAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DECIMAL_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DECIMAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
impl Copy for DECIMAL_0_0 {}
impl Clone for DECIMAL_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DECIMAL_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DECIMAL_0_0").field("scale", &self.scale).field("sign", &self.sign).finish()
    }
}
impl windows_core::TypeKind for DECIMAL_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DECIMAL_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.scale == other.scale && self.sign == other.sign
    }
}
impl Eq for DECIMAL_0_0 {}
impl Default for DECIMAL_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Copy for DECIMAL_1 {}
impl Clone for DECIMAL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DECIMAL_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DECIMAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
impl Copy for DECIMAL_1_0 {}
impl Clone for DECIMAL_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DECIMAL_1_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DECIMAL_1_0").field("Lo32", &self.Lo32).field("Mid32", &self.Mid32).finish()
    }
}
impl windows_core::TypeKind for DECIMAL_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DECIMAL_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Lo32 == other.Lo32 && self.Mid32 == other.Mid32
    }
}
impl Eq for DECIMAL_1_0 {}
impl Default for DECIMAL_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl Copy for FILETIME {}
impl Clone for FILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FILETIME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FILETIME").field("dwLowDateTime", &self.dwLowDateTime).field("dwHighDateTime", &self.dwHighDateTime).finish()
    }
}
impl windows_core::TypeKind for FILETIME {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowDateTime == other.dwLowDateTime && self.dwHighDateTime == other.dwHighDateTime
    }
}
impl Eq for FILETIME {}
impl Default for FILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FLOAT128 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl Copy for FLOAT128 {}
impl Clone for FLOAT128 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FLOAT128 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FLOAT128").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl windows_core::TypeKind for FLOAT128 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FLOAT128 {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl Eq for FLOAT128 {}
impl Default for FLOAT128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HANDLE(pub isize);
impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HANDLE {}
impl core::fmt::Debug for HANDLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDLE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HANDLE_PTR(pub usize);
impl Default for HANDLE_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HANDLE_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HANDLE_PTR {}
impl core::fmt::Debug for HANDLE_PTR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDLE_PTR").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HANDLE_PTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HGLOBAL(pub *mut core::ffi::c_void);
impl HGLOBAL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HGLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HGLOBAL {}
impl core::fmt::Debug for HGLOBAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HGLOBAL").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HGLOBAL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HINSTANCE(pub isize);
impl HINSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for HINSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HINSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HINSTANCE {}
impl core::fmt::Debug for HINSTANCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HINSTANCE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HINSTANCE {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::CanInto<HMODULE> for HINSTANCE {}
impl From<HINSTANCE> for HMODULE {
    fn from(value: HINSTANCE) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HLOCAL(pub *mut core::ffi::c_void);
impl HLOCAL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HLOCAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HLOCAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HLOCAL {}
impl core::fmt::Debug for HLOCAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HLOCAL").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HLOCAL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HLSURF(pub isize);
impl Default for HLSURF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HLSURF {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HLSURF {}
impl core::fmt::Debug for HLSURF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HLSURF").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HLSURF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HMODULE(pub isize);
impl HMODULE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for HMODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HMODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HMODULE {}
impl core::fmt::Debug for HMODULE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HMODULE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HMODULE {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::CanInto<HINSTANCE> for HMODULE {}
impl From<HMODULE> for HINSTANCE {
    fn from(value: HMODULE) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HRSRC(pub isize);
impl HRSRC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HRSRC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HRSRC {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HRSRC {}
impl core::fmt::Debug for HRSRC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HRSRC").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HRSRC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HSPRITE(pub isize);
impl Default for HSPRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HSPRITE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HSPRITE {}
impl core::fmt::Debug for HSPRITE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HSPRITE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HSPRITE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HSTR(pub isize);
impl Default for HSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HSTR {}
impl core::fmt::Debug for HSTR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HSTR").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HSTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HUMPD(pub isize);
impl Default for HUMPD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HUMPD {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HUMPD {}
impl core::fmt::Debug for HUMPD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HUMPD").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HUMPD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct HWND(pub isize);
impl Default for HWND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for HWND {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HWND {}
impl core::fmt::Debug for HWND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HWND").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HWND {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::CanInto<HANDLE> for HWND {}
impl From<HWND> for HANDLE {
    fn from(value: HWND) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct LPARAM(pub isize);
impl Default for LPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for LPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for LPARAM {}
impl core::fmt::Debug for LPARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LPARAM").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for LPARAM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct LRESULT(pub isize);
impl Default for LRESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for LRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for LRESULT {}
impl core::fmt::Debug for LRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LRESULT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for LRESULT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl Copy for LUID {}
impl Clone for LUID {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for LUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LUID").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl windows_core::TypeKind for LUID {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for LUID {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl Eq for LUID {}
impl Default for LUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[must_use]
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct NTSTATUS(pub i32);
impl Default for NTSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for NTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for NTSTATUS {}
impl core::fmt::Debug for NTSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NTSTATUS").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for NTSTATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
impl Copy for POINT {}
impl Clone for POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for POINT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("POINT").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl windows_core::TypeKind for POINT {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for POINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for POINT {}
impl Default for POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct POINTL {
    pub x: i32,
    pub y: i32,
}
impl Copy for POINTL {}
impl Clone for POINTL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for POINTL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("POINTL").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl windows_core::TypeKind for POINTL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for POINTL {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for POINTL {}
impl Default for POINTL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct POINTS {
    pub x: i16,
    pub y: i16,
}
impl Copy for POINTS {}
impl Clone for POINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for POINTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("POINTS").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl windows_core::TypeKind for POINTS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for POINTS {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for POINTS {}
impl Default for POINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PSID(pub *mut core::ffi::c_void);
impl PSID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PSID {}
impl core::fmt::Debug for PSID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSID").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PSID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Copy for RECT {}
impl Clone for RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for RECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RECT").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl windows_core::TypeKind for RECT {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for RECT {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl Eq for RECT {}
impl Default for RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RECTL {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Copy for RECTL {}
impl Clone for RECTL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for RECTL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RECTL").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl windows_core::TypeKind for RECTL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for RECTL {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl Eq for RECTL {}
impl Default for RECTL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct SHANDLE_PTR(pub isize);
impl Default for SHANDLE_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for SHANDLE_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for SHANDLE_PTR {}
impl core::fmt::Debug for SHANDLE_PTR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SHANDLE_PTR").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for SHANDLE_PTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
impl Copy for SIZE {}
impl Clone for SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for SIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SIZE").field("cx", &self.cx).field("cy", &self.cy).finish()
    }
}
impl windows_core::TypeKind for SIZE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.cx == other.cx && self.cy == other.cy
    }
}
impl Eq for SIZE {}
impl Default for SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl Copy for SYSTEMTIME {}
impl Clone for SYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for SYSTEMTIME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SYSTEMTIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDayOfWeek", &self.wDayOfWeek).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).finish()
    }
}
impl windows_core::TypeKind for SYSTEMTIME {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for SYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDayOfWeek == other.wDayOfWeek && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond && self.wMilliseconds == other.wMilliseconds
    }
}
impl Eq for SYSTEMTIME {}
impl Default for SYSTEMTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PWSTR,
}
impl Copy for UNICODE_STRING {}
impl Clone for UNICODE_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for UNICODE_STRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UNICODE_STRING").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl windows_core::TypeKind for UNICODE_STRING {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for UNICODE_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl Eq for UNICODE_STRING {}
impl Default for UNICODE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct VARIANT_BOOL(pub i16);
impl Default for VARIANT_BOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for VARIANT_BOOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for VARIANT_BOOL {}
impl core::fmt::Debug for VARIANT_BOOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARIANT_BOOL").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for VARIANT_BOOL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct WPARAM(pub usize);
impl Default for WPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for WPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for WPARAM {}
impl core::fmt::Debug for WPARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WPARAM").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for WPARAM {
    type TypeKind = windows_core::CopyType;
}
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type NEARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type PAPCFUNC = Option<unsafe extern "system" fn(parameter: usize)>;
pub type PROC = Option<unsafe extern "system" fn() -> isize>;
impl BOOL {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(windows_core::Error::from_win32())
        }
    }
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.as_bool()
    }
}
impl From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}
impl From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}
impl From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl PartialEq<BOOL> for bool {
    fn eq(&self, other: &BOOL) -> bool {
        *self == other.as_bool()
    }
}
impl core::ops::Not for BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            Self(0)
        } else {
            Self(1)
        }
    }
}
impl windows_core::IntoParam<BOOL> for bool {
    unsafe fn into_param(self) -> windows_core::Param<BOOL> {
        windows_core::Param::Owned(self.into())
    }
}
impl BOOLEAN {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(windows_core::Error::from_win32())
        }
    }
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl From<BOOLEAN> for bool {
    fn from(value: BOOLEAN) -> Self {
        value.as_bool()
    }
}
impl From<&BOOLEAN> for bool {
    fn from(value: &BOOLEAN) -> Self {
        value.as_bool()
    }
}
impl From<bool> for BOOLEAN {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}
impl From<&bool> for BOOLEAN {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl PartialEq<bool> for BOOLEAN {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl PartialEq<BOOLEAN> for bool {
    fn eq(&self, other: &BOOLEAN) -> bool {
        *self == other.as_bool()
    }
}
impl core::ops::Not for BOOLEAN {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            Self(0)
        } else {
            Self(1)
        }
    }
}
impl windows_core::IntoParam<BOOLEAN> for bool {
    unsafe fn into_param(self) -> windows_core::Param<BOOLEAN> {
        windows_core::Param::Owned(self.into())
    }
}
impl NTSTATUS {
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 >= 0
    }
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }
    #[inline]
    pub const fn to_hresult(self) -> windows_core::HRESULT {
        windows_core::HRESULT::from_nt(self.0)
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.to_hresult().into())
        }
    }
}
impl From<NTSTATUS> for windows_core::HRESULT {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult()
    }
}
impl From<NTSTATUS> for windows_core::Error {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult().into()
    }
}
impl VARIANT_BOOL {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(windows_core::Error::from_win32())
        }
    }
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl From<VARIANT_BOOL> for bool {
    fn from(value: VARIANT_BOOL) -> Self {
        value.as_bool()
    }
}
impl From<&VARIANT_BOOL> for bool {
    fn from(value: &VARIANT_BOOL) -> Self {
        value.as_bool()
    }
}
impl From<bool> for VARIANT_BOOL {
    fn from(value: bool) -> Self {
        if value {
            VARIANT_TRUE
        } else {
            VARIANT_FALSE
        }
    }
}
impl From<&bool> for VARIANT_BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl PartialEq<bool> for VARIANT_BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl PartialEq<VARIANT_BOOL> for bool {
    fn eq(&self, other: &VARIANT_BOOL) -> bool {
        *self == other.as_bool()
    }
}
impl core::ops::Not for VARIANT_BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            VARIANT_FALSE
        } else {
            VARIANT_TRUE
        }
    }
}
impl WIN32_ERROR {
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == 0
    }
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }
    #[inline]
    pub const fn to_hresult(self) -> windows_core::HRESULT {
        windows_core::HRESULT::from_win32(self.0)
    }
    #[inline]
    pub fn from_error(error: &windows_core::Error) -> Option<Self> {
        let hresult = error.code().0 as u32;
        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(Self(hresult & 0xFFFF))
        } else {
            None
        }
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.to_hresult().into())
        }
    }
}
impl From<WIN32_ERROR> for windows_core::HRESULT {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult()
    }
}
impl From<WIN32_ERROR> for windows_core::Error {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult().into()
    }
}
