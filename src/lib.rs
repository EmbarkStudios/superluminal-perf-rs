//! [Superluminal Performance](https://superluminal.eu/) profiler Rust API for adding user events to captures.
//!
//! ## How to use
//!
//! In `Cargo.toml` add:
//!
//! ```toml
//! [dependencies]
//! superluminal-perf = "0.1.0"
//! ```
//!
//! Example usage:
//!
//! ```rust
//! superluminal_perf::begin_event(b"my-event\0");
//! calc();
//! superluminal_perf::end_event();
//!
//! superluminal_perf::begin_event(b"my-event2\0");
//! calc2();
//! superluminal_perf::end_event();
//! ```
//!
//! On non-Windows platforms the events will be compiled out.

#![allow(unused_variables)]
#![warn(
    clippy::all,
    clippy::doc_markdown,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::pub_enum_variant_names,
    clippy::mem_forget,
    clippy::use_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    rust_2018_idioms,
    future_incompatible,
    trivial_numeric_casts,
    unstable_features,
    nonstandard_style,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    missing_docs
)]

#[cfg(all(feature = "enable", target_os = "windows"))]
use superluminal_perf_sys as ffi;

/// Check if the API is enabled
pub const fn enabled() -> bool {
    cfg!(all(feature = "enable", target_os = "windows"))
}

/// Begin an instrumentation event with the specified ID
///
/// The ID for a specific scope must be the same over the lifetime of the program
pub fn begin_event(id: &'static [u8]) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let id_cstr = std::ffi::CStr::from_bytes_with_nul(id)
            .expect("Invalid ID string, must be null-terminated and not contain interior null");
        ffi::PerformanceAPI_BeginEvent(id_cstr.as_ptr(), std::ptr::null(), ffi::DEFAULT_COLOR)
    }
}

/// Begin an instrumentation event with the specified ID and color
///
/// The ID for a specific scope must be the same over the lifetime of the program
pub fn begin_event_with_color(id: &'static [u8], color: u32) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let id_cstr = std::ffi::CStr::from_bytes_with_nul(id)
            .expect("Invalid ID string, must be null-terminated and not contain interior null");
        ffi::PerformanceAPI_BeginEvent(id_cstr.as_ptr(), std::ptr::null(), color)
    }
}

/// Begin an instrumentation event with the specified ID and runtime data
///
/// The ID for a specific scope must be the same over the lifetime of the program.
/// The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime.
pub fn begin_event_with_data(id: &'static [u8], data: &[u8]) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let id_cstr = std::ffi::CStr::from_bytes_with_nul(id)
            .expect("Invalid ID string, must be null-terminated and not contain interior null");
        let data_cstr = std::ffi::CStr::from_bytes_with_nul(data)
            .expect("Invalid data string, must be null-terminated and not contain interior null");
        ffi::PerformanceAPI_BeginEvent(id_cstr.as_ptr(), data_cstr.as_ptr(), ffi::DEFAULT_COLOR)
    }
}

/// End an instrumentation event. Must be matched with a call to `BeginEvent` within the same function
pub fn end_event() {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        // PerformanceAPI_EndEvent returns a struct which is only used to prevent calls to it from being tail-call optimized.
        // We ignore the return value here so the caller of end_event doesn't need to deal with it.
        let _ = ffi::PerformanceAPI_EndEvent();
    }
}

/// Set the name of the current thread to the specified thread name
pub fn set_current_thread_name(name: &str) -> Result<(), std::ffi::NulError> {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let name_cstr = std::ffi::CString::new(name)?;
        ffi::PerformanceAPI_SetCurrentThreadName(name_cstr.as_ptr())
    }
    Ok(())
}
