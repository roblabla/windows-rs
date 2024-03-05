#[inline]
pub unsafe fn MatchEnumTag<P0, P1>(hmodule: P0, pwcarg: P1, dwnumarg: u32, penumtable: *const TOKEN_VALUE, pdwvalue: *mut u32) -> u32
where
    P0: windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: windows_core::IntoParam<windows_core::PCWSTR>,
{
    windows_targets::link!("netsh.dll" "system" fn MatchEnumTag(hmodule : super::super::Foundation:: HANDLE, pwcarg : windows_core::PCWSTR, dwnumarg : u32, penumtable : *const TOKEN_VALUE, pdwvalue : *mut u32) -> u32);
    MatchEnumTag(hmodule.into_param().abi(), pwcarg.into_param().abi(), dwnumarg, penumtable, pdwvalue)
}
#[inline]
pub unsafe fn MatchToken<P0, P1>(pwszusertoken: P0, pwszcmdtoken: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::IntoParam<windows_core::PCWSTR>,
    P1: windows_core::IntoParam<windows_core::PCWSTR>,
{
    windows_targets::link!("netsh.dll" "system" fn MatchToken(pwszusertoken : windows_core::PCWSTR, pwszcmdtoken : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    MatchToken(pwszusertoken.into_param().abi(), pwszcmdtoken.into_param().abi())
}
#[inline]
pub unsafe fn PreprocessCommand<P0>(hmodule: P0, ppwcarguments: &mut [windows_core::PWSTR], dwcurrentindex: u32, ptttags: Option<&mut [TAG_TYPE]>, dwminargs: u32, dwmaxargs: u32, pdwtagtype: Option<*mut u32>) -> u32
where
    P0: windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("netsh.dll" "system" fn PreprocessCommand(hmodule : super::super::Foundation:: HANDLE, ppwcarguments : *mut windows_core::PWSTR, dwcurrentindex : u32, dwargcount : u32, ptttags : *mut TAG_TYPE, dwtagcount : u32, dwminargs : u32, dwmaxargs : u32, pdwtagtype : *mut u32) -> u32);
    PreprocessCommand(hmodule.into_param().abi(), core::mem::transmute(ppwcarguments.as_ptr()), dwcurrentindex, ppwcarguments.len().try_into().unwrap(), core::mem::transmute(ptttags.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ptttags.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwminargs, dwmaxargs, core::mem::transmute(pdwtagtype.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PrintError<P0>(hmodule: P0, dwerrid: u32) -> u32
where
    P0: windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("netsh.dll" "cdecl" fn PrintError(hmodule : super::super::Foundation:: HANDLE, dwerrid : u32) -> u32);
    PrintError(hmodule.into_param().abi(), dwerrid)
}
#[inline]
pub unsafe fn PrintMessage<P0>(pwszformat: P0) -> u32
where
    P0: windows_core::IntoParam<windows_core::PCWSTR>,
{
    windows_targets::link!("netsh.dll" "cdecl" fn PrintMessage(pwszformat : windows_core::PCWSTR) -> u32);
    PrintMessage(pwszformat.into_param().abi())
}
#[inline]
pub unsafe fn PrintMessageFromModule<P0>(hmodule: P0, dwmsgid: u32) -> u32
where
    P0: windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("netsh.dll" "cdecl" fn PrintMessageFromModule(hmodule : super::super::Foundation:: HANDLE, dwmsgid : u32) -> u32);
    PrintMessageFromModule(hmodule.into_param().abi(), dwmsgid)
}
#[inline]
pub unsafe fn RegisterContext(pchildcontext: *const NS_CONTEXT_ATTRIBUTES) -> u32 {
    windows_targets::link!("netsh.dll" "system" fn RegisterContext(pchildcontext : *const NS_CONTEXT_ATTRIBUTES) -> u32);
    RegisterContext(pchildcontext)
}
#[inline]
pub unsafe fn RegisterHelper(pguidparentcontext: *const windows_core::GUID, pfnregistersubcontext: *const NS_HELPER_ATTRIBUTES) -> u32 {
    windows_targets::link!("netsh.dll" "system" fn RegisterHelper(pguidparentcontext : *const windows_core::GUID, pfnregistersubcontext : *const NS_HELPER_ATTRIBUTES) -> u32);
    RegisterHelper(pguidparentcontext, pfnregistersubcontext)
}
pub const CMD_FLAG_HIDDEN: NS_CMD_FLAGS = NS_CMD_FLAGS(32i32);
pub const CMD_FLAG_INTERACTIVE: NS_CMD_FLAGS = NS_CMD_FLAGS(2i32);
pub const CMD_FLAG_LIMIT_MASK: NS_CMD_FLAGS = NS_CMD_FLAGS(65535i32);
pub const CMD_FLAG_LOCAL: NS_CMD_FLAGS = NS_CMD_FLAGS(8i32);
pub const CMD_FLAG_ONLINE: NS_CMD_FLAGS = NS_CMD_FLAGS(16i32);
pub const CMD_FLAG_PRIORITY: NS_CMD_FLAGS = NS_CMD_FLAGS(-2147483648i32);
pub const CMD_FLAG_PRIVATE: NS_CMD_FLAGS = NS_CMD_FLAGS(1i32);
pub const DEFAULT_CONTEXT_PRIORITY: u32 = 100u32;
pub const ERROR_CMD_NOT_FOUND: u32 = 15004u32;
pub const ERROR_CONTEXT_ALREADY_REGISTERED: u32 = 15019u32;
pub const ERROR_CONTINUE_IN_PARENT_CONTEXT: u32 = 15016u32;
pub const ERROR_DLL_LOAD_FAILED: u32 = 15006u32;
pub const ERROR_ENTRY_PT_NOT_FOUND: u32 = 15005u32;
pub const ERROR_HELPER_ALREADY_REGISTERED: u32 = 15018u32;
pub const ERROR_INIT_DISPLAY: u32 = 15007u32;
pub const ERROR_INVALID_OPTION_TAG: u32 = 15009u32;
pub const ERROR_INVALID_OPTION_VALUE: u32 = 15014u32;
pub const ERROR_INVALID_SYNTAX: u32 = 15001u32;
pub const ERROR_MISSING_OPTION: u32 = 15011u32;
pub const ERROR_NO_CHANGE: u32 = 15003u32;
pub const ERROR_NO_ENTRIES: u32 = 15000u32;
pub const ERROR_NO_TAG: u32 = 15010u32;
pub const ERROR_OKAY: u32 = 15015u32;
pub const ERROR_PARSING_FAILURE: u32 = 15020u32;
pub const ERROR_PROTOCOL_NOT_IN_TRANSPORT: u32 = 15002u32;
pub const ERROR_SHOW_USAGE: u32 = 15013u32;
pub const ERROR_SUPPRESS_OUTPUT: u32 = 15017u32;
pub const ERROR_TAG_ALREADY_PRESENT: u32 = 15008u32;
pub const ERROR_TRANSPORT_NOT_PRESENT: u32 = 15012u32;
pub const GET_RESOURCE_STRING_FN_NAME: windows_core::PCSTR = windows_core::s!("GetResourceString");
pub const MAX_NAME_LEN: u32 = 48u32;
pub const NETSH_ARG_DELIMITER: windows_core::PCWSTR = windows_core::w!("=");
pub const NETSH_CMD_DELIMITER: windows_core::PCWSTR = windows_core::w!(" ");
pub const NETSH_COMMIT: NS_MODE_CHANGE = NS_MODE_CHANGE(0i32);
pub const NETSH_COMMIT_STATE: NS_MODE_CHANGE = NS_MODE_CHANGE(3i32);
pub const NETSH_ERROR_BASE: u32 = 15000u32;
pub const NETSH_ERROR_END: u32 = 15019u32;
pub const NETSH_FLUSH: NS_MODE_CHANGE = NS_MODE_CHANGE(2i32);
pub const NETSH_MAX_CMD_TOKEN_LENGTH: u32 = 128u32;
pub const NETSH_MAX_TOKEN_LENGTH: u32 = 64u32;
pub const NETSH_SAVE: NS_MODE_CHANGE = NS_MODE_CHANGE(4i32);
pub const NETSH_UNCOMMIT: NS_MODE_CHANGE = NS_MODE_CHANGE(1i32);
pub const NETSH_VERSION_50: u32 = 20480u32;
pub const NS_EVENT_FROM_N: NS_EVENTS = NS_EVENTS(4i32);
pub const NS_EVENT_FROM_START: NS_EVENTS = NS_EVENTS(8i32);
pub const NS_EVENT_LAST_N: NS_EVENTS = NS_EVENTS(1i32);
pub const NS_EVENT_LAST_SECS: NS_EVENTS = NS_EVENTS(2i32);
pub const NS_EVENT_LOOP: NS_EVENTS = NS_EVENTS(65536i32);
pub const NS_GET_EVENT_IDS_FN_NAME: windows_core::PCSTR = windows_core::s!("GetEventIds");
pub const NS_REQ_ALLOW_MULTIPLE: NS_REQS = NS_REQS(2i32);
pub const NS_REQ_ONE_OR_MORE: NS_REQS = NS_REQS(3i32);
pub const NS_REQ_PRESENT: NS_REQS = NS_REQS(1i32);
pub const NS_REQ_ZERO: NS_REQS = NS_REQS(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NS_CMD_FLAGS(pub i32);
impl windows_core::TypeKind for NS_CMD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NS_CMD_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NS_CMD_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NS_EVENTS(pub i32);
impl windows_core::TypeKind for NS_EVENTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NS_EVENTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NS_EVENTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NS_MODE_CHANGE(pub i32);
impl windows_core::TypeKind for NS_MODE_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NS_MODE_CHANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NS_MODE_CHANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NS_REQS(pub i32);
impl windows_core::TypeKind for NS_REQS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NS_REQS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NS_REQS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CMD_ENTRY {
    pub pwszCmdToken: windows_core::PCWSTR,
    pub pfnCmdHandler: PFN_HANDLE_CMD,
    pub dwShortCmdHelpToken: u32,
    pub dwCmdHlpToken: u32,
    pub dwFlags: u32,
    pub pOsVersionCheck: PNS_OSVERSIONCHECK,
    pub pfnCustomHelpFn: PFN_CUSTOM_HELP,
}
impl Copy for CMD_ENTRY {}
impl Clone for CMD_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CMD_ENTRY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CMD_ENTRY").field("pwszCmdToken", &self.pwszCmdToken).field("dwShortCmdHelpToken", &self.dwShortCmdHelpToken).field("dwCmdHlpToken", &self.dwCmdHlpToken).field("dwFlags", &self.dwFlags).finish()
    }
}
impl windows_core::TypeKind for CMD_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CMD_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CMD_GROUP_ENTRY {
    pub pwszCmdGroupToken: windows_core::PCWSTR,
    pub dwShortCmdHelpToken: u32,
    pub ulCmdGroupSize: u32,
    pub dwFlags: u32,
    pub pCmdGroup: *mut CMD_ENTRY,
    pub pOsVersionCheck: PNS_OSVERSIONCHECK,
}
impl Copy for CMD_GROUP_ENTRY {}
impl Clone for CMD_GROUP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CMD_GROUP_ENTRY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CMD_GROUP_ENTRY").field("pwszCmdGroupToken", &self.pwszCmdGroupToken).field("dwShortCmdHelpToken", &self.dwShortCmdHelpToken).field("ulCmdGroupSize", &self.ulCmdGroupSize).field("dwFlags", &self.dwFlags).field("pCmdGroup", &self.pCmdGroup).finish()
    }
}
impl windows_core::TypeKind for CMD_GROUP_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CMD_GROUP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NS_CONTEXT_ATTRIBUTES {
    pub Anonymous: NS_CONTEXT_ATTRIBUTES_0,
    pub pwszContext: windows_core::PWSTR,
    pub guidHelper: windows_core::GUID,
    pub dwFlags: u32,
    pub ulPriority: u32,
    pub ulNumTopCmds: u32,
    pub pTopCmds: *mut CMD_ENTRY,
    pub ulNumGroups: u32,
    pub pCmdGroups: *mut CMD_GROUP_ENTRY,
    pub pfnCommitFn: PNS_CONTEXT_COMMIT_FN,
    pub pfnDumpFn: PNS_CONTEXT_DUMP_FN,
    pub pfnConnectFn: PNS_CONTEXT_CONNECT_FN,
    pub pReserved: *mut core::ffi::c_void,
    pub pfnOsVersionCheck: PNS_OSVERSIONCHECK,
}
impl Copy for NS_CONTEXT_ATTRIBUTES {}
impl Clone for NS_CONTEXT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for NS_CONTEXT_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for NS_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NS_CONTEXT_ATTRIBUTES_0 {
    pub Anonymous: NS_CONTEXT_ATTRIBUTES_0_0,
    pub _ullAlign: u64,
}
impl Copy for NS_CONTEXT_ATTRIBUTES_0 {}
impl Clone for NS_CONTEXT_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for NS_CONTEXT_ATTRIBUTES_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NS_CONTEXT_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NS_CONTEXT_ATTRIBUTES_0_0 {
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl Copy for NS_CONTEXT_ATTRIBUTES_0_0 {}
impl Clone for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NS_CONTEXT_ATTRIBUTES_0_0").field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl windows_core::TypeKind for NS_CONTEXT_ATTRIBUTES_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
impl Eq for NS_CONTEXT_ATTRIBUTES_0_0 {}
impl Default for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NS_HELPER_ATTRIBUTES {
    pub Anonymous: NS_HELPER_ATTRIBUTES_0,
    pub guidHelper: windows_core::GUID,
    pub pfnStart: PNS_HELPER_START_FN,
    pub pfnStop: PNS_HELPER_STOP_FN,
}
impl Copy for NS_HELPER_ATTRIBUTES {}
impl Clone for NS_HELPER_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for NS_HELPER_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for NS_HELPER_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NS_HELPER_ATTRIBUTES_0 {
    pub Anonymous: NS_HELPER_ATTRIBUTES_0_0,
    pub _ullAlign: u64,
}
impl Copy for NS_HELPER_ATTRIBUTES_0 {}
impl Clone for NS_HELPER_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for NS_HELPER_ATTRIBUTES_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NS_HELPER_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NS_HELPER_ATTRIBUTES_0_0 {
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl Copy for NS_HELPER_ATTRIBUTES_0_0 {}
impl Clone for NS_HELPER_ATTRIBUTES_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for NS_HELPER_ATTRIBUTES_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NS_HELPER_ATTRIBUTES_0_0").field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl windows_core::TypeKind for NS_HELPER_ATTRIBUTES_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for NS_HELPER_ATTRIBUTES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
impl Eq for NS_HELPER_ATTRIBUTES_0_0 {}
impl Default for NS_HELPER_ATTRIBUTES_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TAG_TYPE {
    pub pwszTag: windows_core::PCWSTR,
    pub dwRequired: u32,
    pub bPresent: super::super::Foundation::BOOL,
}
impl Copy for TAG_TYPE {}
impl Clone for TAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for TAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TAG_TYPE").field("pwszTag", &self.pwszTag).field("dwRequired", &self.dwRequired).field("bPresent", &self.bPresent).finish()
    }
}
impl windows_core::TypeKind for TAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for TAG_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszTag == other.pwszTag && self.dwRequired == other.dwRequired && self.bPresent == other.bPresent
    }
}
impl Eq for TAG_TYPE {}
impl Default for TAG_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TOKEN_VALUE {
    pub pwszToken: windows_core::PCWSTR,
    pub dwValue: u32,
}
impl Copy for TOKEN_VALUE {}
impl Clone for TOKEN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for TOKEN_VALUE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TOKEN_VALUE").field("pwszToken", &self.pwszToken).field("dwValue", &self.dwValue).finish()
    }
}
impl windows_core::TypeKind for TOKEN_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for TOKEN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszToken == other.pwszToken && self.dwValue == other.dwValue
    }
}
impl Eq for TOKEN_VALUE {}
impl Default for TOKEN_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_CUSTOM_HELP = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HANDLE, pwszcmdtoken: windows_core::PCWSTR)>;
pub type PFN_HANDLE_CMD = Option<unsafe extern "system" fn(pwszmachine: windows_core::PCWSTR, ppwcarguments: *mut windows_core::PWSTR, dwcurrentindex: u32, dwargcount: u32, dwflags: u32, pvdata: *const core::ffi::c_void, pbdone: *mut super::super::Foundation::BOOL) -> u32>;
pub type PGET_RESOURCE_STRING_FN = Option<unsafe extern "system" fn(dwmsgid: u32, lpbuffer: windows_core::PCWSTR, nbuffermax: u32) -> u32>;
pub type PNS_CONTEXT_COMMIT_FN = Option<unsafe extern "system" fn(dwaction: u32) -> u32>;
pub type PNS_CONTEXT_CONNECT_FN = Option<unsafe extern "system" fn(pwszmachine: windows_core::PCWSTR) -> u32>;
pub type PNS_CONTEXT_DUMP_FN = Option<unsafe extern "system" fn(pwszrouter: windows_core::PCWSTR, ppwcarguments: *const windows_core::PCWSTR, dwargcount: u32, pvdata: *const core::ffi::c_void) -> u32>;
pub type PNS_DLL_INIT_FN = Option<unsafe extern "system" fn(dwnetshversion: u32, preserved: *mut core::ffi::c_void) -> u32>;
pub type PNS_DLL_STOP_FN = Option<unsafe extern "system" fn(dwreserved: u32) -> u32>;
pub type PNS_HELPER_START_FN = Option<unsafe extern "system" fn(pguidparent: *const windows_core::GUID, dwversion: u32) -> u32>;
pub type PNS_HELPER_STOP_FN = Option<unsafe extern "system" fn(dwreserved: u32) -> u32>;
pub type PNS_OSVERSIONCHECK = Option<unsafe extern "system" fn(cimostype: u32, cimosproductsuite: u32, cimosversion: windows_core::PCWSTR, cimosbuildnumber: windows_core::PCWSTR, cimservicepackmajorversion: windows_core::PCWSTR, cimservicepackminorversion: windows_core::PCWSTR, uireserved: u32, dwreserved: u32) -> super::super::Foundation::BOOL>;
