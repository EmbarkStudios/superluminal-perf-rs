#![allow(unused_variables)]

use std::ffi::CStr;
#[cfg(all(feature = "enable", target_os = "windows"))]
use superluminal_perf_sys as ffi;

/// Begin an instrumentation event with the specified ID
///
/// The ID for a specific scope must be the same over the lifetime of the program
pub fn begin_event(id: &'static [u8]) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let cstr = CStr::from_bytes_with_nul(id)
            .expect("Invalid ID string, must be null-terminated and not contain interior null");
        ffi::PerformanceAPI_BeginEvent(cstr.as_ptr(), std::ptr::null(), ffi::DEFAULT_COLOR)
    }
}

pub fn begin_event2(id: &'static [u8]) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        println!("id.len(): {}, id.ptr: {:?}", id.len(), id.as_ptr());
        ffi::PerformanceAPI_BeginEvent(
            id.as_ptr() as *const i8,
            std::ptr::null(),
            ffi::DEFAULT_COLOR,
        )
    }
}

pub fn begin_event_cstr(id: &'static CStr) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent(id.as_ptr(), std::ptr::null(), ffi::DEFAULT_COLOR)
    }
}

/// Begin an instrumentation event with the specified ID and runtime data
///
/// The ID for a specific scope must be the same over the lifetime of the program.
/// The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime.
pub fn begin_event_with_data(_id: &str, _data: &str) {}

/// End an instrumentation event. Must be matched with a call to BeginEvent within the same function
pub fn end_event() {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_EndEvent();
    }
}

/// Set the name of the current thread to the specified thread name
pub fn set_current_thread_name(_name: &str) {}
