// Copyright 2022 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// If this file changes, update this file upstream and update windows pre-built libraries
// that upstream uses.
//
// TODO(b/239836957): Add how to generate and update pre-built library.

#![allow(deref_nullptr)]

/* automatically generated by rust-bindgen 0.56.0 */

pub const JOB_OBJECT_UILIMIT_NONE: u32 = 0;
pub const JOB_OBJECT_UILIMIT_HANDLES: u32 = 1;
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: u32 = 2;
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: u32 = 4;
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: u32 = 8;
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: u32 = 16;
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: u32 = 32;
pub const JOB_OBJECT_UILIMIT_DESKTOP: u32 = 64;
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: u32 = 128;
pub const JOB_OBJECT_UILIMIT_ALL: u32 = 255;
pub type size_t = ::std::os::raw::c_ulonglong;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
pub type HANDLE = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PROCESS_INFORMATION {
    pub hProcess: HANDLE,
    pub hThread: HANDLE,
    pub dwProcessId: DWORD,
    pub dwThreadId: DWORD,
}
#[test]
fn bindgen_test_layout__PROCESS_INFORMATION() {
    assert_eq!(
        ::std::mem::size_of::<_PROCESS_INFORMATION>(),
        24usize,
        concat!("Size of: ", stringify!(_PROCESS_INFORMATION))
    );
    assert_eq!(
        ::std::mem::align_of::<_PROCESS_INFORMATION>(),
        8usize,
        concat!("Alignment of ", stringify!(_PROCESS_INFORMATION))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).hProcess as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(hProcess)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).hThread as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(hThread)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).dwProcessId as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(dwProcessId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).dwThreadId as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(dwThreadId)
        )
    );
}
pub type PROCESS_INFORMATION = _PROCESS_INFORMATION;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ResultCode {
    SBOX_ALL_OK = 0,
    SBOX_ERROR_GENERIC = 1,
    SBOX_ERROR_BAD_PARAMS = 2,
    SBOX_ERROR_UNSUPPORTED = 3,
    SBOX_ERROR_NO_SPACE = 4,
    SBOX_ERROR_INVALID_IPC = 5,
    SBOX_ERROR_FAILED_IPC = 6,
    SBOX_ERROR_NO_HANDLE = 7,
    SBOX_ERROR_UNEXPECTED_CALL = 8,
    SBOX_ERROR_WAIT_ALREADY_CALLED = 9,
    SBOX_ERROR_CHANNEL_ERROR = 10,
    SBOX_ERROR_CANNOT_CREATE_DESKTOP = 11,
    SBOX_ERROR_CANNOT_CREATE_WINSTATION = 12,
    SBOX_ERROR_FAILED_TO_SWITCH_BACK_WINSTATION = 13,
    SBOX_ERROR_INVALID_APP_CONTAINER = 14,
    SBOX_ERROR_INVALID_CAPABILITY = 15,
    SBOX_ERROR_CANNOT_INIT_APPCONTAINER = 16,
    SBOX_ERROR_PROC_THREAD_ATTRIBUTES = 17,
    SBOX_ERROR_CREATE_PROCESS = 18,
    SBOX_ERROR_DELEGATE_PRE_SPAWN = 19,
    SBOX_ERROR_ASSIGN_PROCESS_TO_JOB_OBJECT = 20,
    SBOX_ERROR_SET_THREAD_TOKEN = 21,
    SBOX_ERROR_GET_THREAD_CONTEXT = 22,
    SBOX_ERROR_DUPLICATE_TARGET_INFO = 23,
    SBOX_ERROR_SET_LOW_BOX_TOKEN = 24,
    SBOX_ERROR_CREATE_FILE_MAPPING = 25,
    SBOX_ERROR_DUPLICATE_SHARED_SECTION = 26,
    SBOX_ERROR_MAP_VIEW_OF_SHARED_SECTION = 27,
    SBOX_ERROR_APPLY_ASLR_MITIGATIONS = 28,
    SBOX_ERROR_SETUP_BASIC_INTERCEPTIONS = 29,
    SBOX_ERROR_SETUP_INTERCEPTION_SERVICE = 30,
    SBOX_ERROR_INITIALIZE_INTERCEPTIONS = 31,
    SBOX_ERROR_SETUP_NTDLL_IMPORTS = 32,
    SBOX_ERROR_SETUP_HANDLE_CLOSER = 33,
    SBOX_ERROR_CANNOT_GET_WINSTATION = 34,
    SBOX_ERROR_CANNOT_QUERY_WINSTATION_SECURITY = 35,
    SBOX_ERROR_CANNOT_GET_DESKTOP = 36,
    SBOX_ERROR_CANNOT_QUERY_DESKTOP_SECURITY = 37,
    SBOX_ERROR_CANNOT_SETUP_INTERCEPTION_CONFIG_BUFFER = 38,
    SBOX_ERROR_CANNOT_COPY_DATA_TO_CHILD = 39,
    SBOX_ERROR_CANNOT_SETUP_INTERCEPTION_THUNK = 40,
    SBOX_ERROR_CANNOT_RESOLVE_INTERCEPTION_THUNK = 41,
    SBOX_ERROR_CANNOT_WRITE_INTERCEPTION_THUNK = 42,
    SBOX_ERROR_CANNOT_FIND_BASE_ADDRESS = 43,
    SBOX_ERROR_CREATE_APPCONTAINER = 44,
    SBOX_ERROR_CREATE_APPCONTAINER_ACCESS_CHECK = 45,
    SBOX_ERROR_CREATE_APPCONTAINER_CAPABILITY = 46,
    SBOX_ERROR_CANNOT_INIT_JOB = 47,
    SBOX_ERROR_INVALID_LOWBOX_SID = 48,
    SBOX_ERROR_CANNOT_CREATE_RESTRICTED_TOKEN = 49,
    SBOX_ERROR_CANNOT_SET_DESKTOP_INTEGRITY = 50,
    SBOX_ERROR_CANNOT_CREATE_LOWBOX_TOKEN = 51,
    SBOX_ERROR_CANNOT_MODIFY_LOWBOX_TOKEN_DACL = 52,
    SBOX_ERROR_CANNOT_CREATE_RESTRICTED_IMP_TOKEN = 53,
    SBOX_ERROR_CANNOT_DUPLICATE_PROCESS_HANDLE = 54,
    SBOX_ERROR_CANNOT_LOADLIBRARY_EXECUTABLE = 55,
    SBOX_ERROR_CANNOT_FIND_VARIABLE_ADDRESS = 56,
    SBOX_ERROR_CANNOT_WRITE_VARIABLE_VALUE = 57,
    SBOX_ERROR_INVALID_WRITE_VARIABLE_SIZE = 58,
    SBOX_ERROR_CANNOT_INIT_BROKERSERVICES = 59,
    SBOX_ERROR_CANNOT_UPDATE_JOB_PROCESS_LIMIT = 60,
    SBOX_ERROR_CANNOT_CREATE_LOWBOX_IMPERSONATION_TOKEN = 61,
    SBOX_ERROR_UNSANDBOXED_PROCESS = 62,
    SBOX_ERROR_LAST = 63,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IntegrityLevel {
    INTEGRITY_LEVEL_SYSTEM = 0,
    INTEGRITY_LEVEL_HIGH = 1,
    INTEGRITY_LEVEL_MEDIUM = 2,
    INTEGRITY_LEVEL_MEDIUM_LOW = 3,
    INTEGRITY_LEVEL_LOW = 4,
    INTEGRITY_LEVEL_BELOW_LOW = 5,
    INTEGRITY_LEVEL_UNTRUSTED = 6,
    INTEGRITY_LEVEL_LAST = 7,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TokenLevel {
    USER_LOCKDOWN = 0,
    USER_RESTRICTED = 1,
    USER_LIMITED = 2,
    USER_INTERACTIVE = 3,
    USER_RESTRICTED_NON_ADMIN = 4,
    USER_NON_ADMIN = 5,
    USER_RESTRICTED_SAME_ACCESS = 6,
    USER_UNPROTECTED = 7,
    USER_LAST = 8,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JobLevel {
    JOB_LOCKDOWN = 0,
    JOB_RESTRICTED = 1,
    JOB_LIMITED_USER = 2,
    JOB_INTERACTIVE = 3,
    JOB_UNPROTECTED = 4,
    JOB_NONE = 5,
}
pub type MitigationFlags = u64;
pub const MITIGATION_DEP: MitigationFlags = 1;
pub const MITIGATION_DEP_NO_ATL_THUNK: MitigationFlags = 2;
pub const MITIGATION_SEHOP: MitigationFlags = 4;
pub const MITIGATION_RELOCATE_IMAGE: MitigationFlags = 8;
pub const MITIGATION_RELOCATE_IMAGE_REQUIRED: MitigationFlags = 16;
pub const MITIGATION_HEAP_TERMINATE: MitigationFlags = 32;
pub const MITIGATION_BOTTOM_UP_ASLR: MitigationFlags = 64;
pub const MITIGATION_HIGH_ENTROPY_ASLR: MitigationFlags = 128;
pub const MITIGATION_STRICT_HANDLE_CHECKS: MitigationFlags = 256;
pub const MITIGATION_DLL_SEARCH_ORDER: MitigationFlags = 512;
pub const MITIGATION_HARDEN_TOKEN_IL_POLICY: MitigationFlags = 1024;
pub const MITIGATION_WIN32K_DISABLE: MitigationFlags = 2048;
pub const MITIGATION_EXTENSION_POINT_DISABLE: MitigationFlags = 4096;
pub const MITIGATION_DYNAMIC_CODE_DISABLE: MitigationFlags = 8192;
pub const MITIGATION_DYNAMIC_CODE_DISABLE_WITH_OPT_OUT: MitigationFlags = 16384;
pub const MITIGATION_DYNAMIC_CODE_OPT_OUT_THIS_THREAD: MitigationFlags = 32768;
pub const MITIGATION_NONSYSTEM_FONT_DISABLE: MitigationFlags = 65536;
pub const MITIGATION_FORCE_MS_SIGNED_BINS: MitigationFlags = 131072;
pub const MITIGATION_IMAGE_LOAD_NO_REMOTE: MitigationFlags = 262144;
pub const MITIGATION_IMAGE_LOAD_NO_LOW_LABEL: MitigationFlags = 524288;
pub const MITIGATION_IMAGE_LOAD_PREFER_SYS32: MitigationFlags = 1048576;
pub const MITIGATION_RESTRICT_INDIRECT_BRANCH_PREDICTION: MitigationFlags = 2097152;
pub const MITIGATION_CET_DISABLED: MitigationFlags = 4194304;
pub const MITIGATION_KTM_COMPONENT: MitigationFlags = 8388608;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SubSystem {
    SUBSYS_FILES = 0,
    SUBSYS_NAMED_PIPES = 1,
    SUBSYS_PROCESS = 2,
    SUBSYS_REGISTRY = 3,
    SUBSYS_SYNC = 4,
    SUBSYS_WIN32K_LOCKDOWN = 5,
    SUBSYS_SIGNED_BINARY = 6,
    SUBSYS_SOCKET = 7,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Semantics {
    FILES_ALLOW_ANY = 0,
    FILES_ALLOW_READONLY = 1,
    FILES_ALLOW_QUERY = 2,
    FILES_ALLOW_DIR_ANY = 3,
    NAMEDPIPES_ALLOW_ANY = 4,
    PROCESS_MIN_EXEC = 5,
    PROCESS_ALL_EXEC = 6,
    EVENTS_ALLOW_ANY = 7,
    EVENTS_ALLOW_READONLY = 8,
    REG_ALLOW_READONLY = 9,
    REG_ALLOW_ANY = 10,
    FAKE_USER_GDI_INIT = 11,
    SIGNED_ALLOW_LOAD = 12,
    SOCKET_ALLOW_BROKER = 13,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BrokerServices {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TargetServices {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProcessState {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TargetPolicy {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PolicyInfo {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sbox_broker_init(broker: *mut BrokerServices) -> ResultCode;
}
extern "C" {
    pub fn sbox_create_policy(broker: *mut BrokerServices) -> *mut TargetPolicy;
}
extern "C" {
    pub fn sbox_release_policy(policy: *mut TargetPolicy);
}
extern "C" {
    pub fn sbox_spawn_target(
        broker: *mut BrokerServices,
        exe_path: *const wchar_t,
        command_line: *const wchar_t,
        policy: *mut TargetPolicy,
        last_warning: *mut ResultCode,
        last_error: *mut DWORD,
        target: *mut PROCESS_INFORMATION,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_wait_for_all_targets(broker: *mut BrokerServices) -> ResultCode;
}
extern "C" {
    pub fn sbox_target_init(target: *mut TargetServices) -> ResultCode;
}
extern "C" {
    pub fn sbox_lower_token(target: *mut TargetServices);
}
extern "C" {
    pub fn sbox_get_state(target: *mut TargetServices) -> *mut ProcessState;
}
extern "C" {
    pub fn get_broker_services() -> *mut BrokerServices;
}
extern "C" {
    pub fn get_target_services() -> *mut TargetServices;
}
extern "C" {
    pub fn sbox_set_token_level(
        policy: *mut TargetPolicy,
        initial: TokenLevel,
        lockdown: TokenLevel,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_get_initial_token_level(policy: *mut TargetPolicy) -> TokenLevel;
}
extern "C" {
    pub fn sbox_get_lockdown_token_level(policy: *mut TargetPolicy) -> TokenLevel;
}
extern "C" {
    pub fn sbox_set_job_level(
        policy: *mut TargetPolicy,
        job_level: JobLevel,
        ui_exceptions: u32,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_get_job_level(policy: *mut TargetPolicy) -> JobLevel;
}
extern "C" {
    pub fn sbox_set_job_memory_limit(policy: *mut TargetPolicy, memory_limit: size_t)
        -> ResultCode;
}
extern "C" {
    pub fn sbox_set_integrity_level(policy: *mut TargetPolicy, level: IntegrityLevel)
        -> ResultCode;
}
extern "C" {
    pub fn sbox_set_delayed_integrity_level(
        policy: *mut TargetPolicy,
        level: IntegrityLevel,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_get_integrity_level(policy: *mut TargetPolicy) -> IntegrityLevel;
}
extern "C" {
    pub fn sbox_set_alternate_desktop(
        policy: *mut TargetPolicy,
        alternate_winstation: bool,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_create_alternate_desktop(
        policy: *mut TargetPolicy,
        alternate_winstation: bool,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_destroy_alternate_desktop(policy: *mut TargetPolicy);
}
extern "C" {
    pub fn sbox_set_lowbox(policy: *mut TargetPolicy, sid: *const wchar_t) -> ResultCode;
}
extern "C" {
    pub fn sbox_set_process_mitigations(
        policy: *mut TargetPolicy,
        flags: MitigationFlags,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_get_process_mitigations(policy: *mut TargetPolicy) -> MitigationFlags;
}
extern "C" {
    pub fn sbox_set_delayed_process_mitigations(
        policy: *mut TargetPolicy,
        flags: MitigationFlags,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_get_delayed_process_mitigations(policy: *mut TargetPolicy) -> MitigationFlags;
}
extern "C" {
    pub fn sbox_set_disconnect_csrss(policy: *mut TargetPolicy) -> ResultCode;
}
extern "C" {
    pub fn sbox_set_strict_interceptions(policy: *mut TargetPolicy);
}
extern "C" {
    pub fn sbox_set_stdout_handle(policy: *mut TargetPolicy, handle: HANDLE) -> ResultCode;
}
extern "C" {
    pub fn sbox_set_stderr_handle(policy: *mut TargetPolicy, handle: HANDLE) -> ResultCode;
}
extern "C" {
    pub fn sbox_add_rule(
        policy: *mut TargetPolicy,
        subsystem: SubSystem,
        semantics: Semantics,
        pattern: *const wchar_t,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_add_dll_to_unload(
        policy: *mut TargetPolicy,
        dll_name: *const wchar_t,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_add_kernel_object_to_close(
        policy: *mut TargetPolicy,
        handle_type: *const wchar_t,
        handle_name: *const wchar_t,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_add_handle_to_share(policy: *mut TargetPolicy, handle: HANDLE);
}
extern "C" {
    pub fn sbox_set_lockdown_default_dacl(policy: *mut TargetPolicy);
}
extern "C" {
    pub fn sbox_add_restricting_random_sid(policy: *mut TargetPolicy);
}
extern "C" {
    pub fn sbox_add_app_container_profile(
        policy: *mut TargetPolicy,
        package_name: *const wchar_t,
        create_profile: bool,
    ) -> ResultCode;
}
extern "C" {
    pub fn sbox_get_policy_info(policy: *mut TargetPolicy) -> *mut PolicyInfo;
}
extern "C" {
    pub fn sbox_release_policy_info(policy_info: *mut PolicyInfo);
}
extern "C" {
    pub fn sbox_policy_info_json_string(
        policy_info: *mut PolicyInfo,
    ) -> *const ::std::os::raw::c_char;
}