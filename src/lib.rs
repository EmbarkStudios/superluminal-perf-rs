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
//!
//! ## Feature flags
//!
//! - `enable` - this flag is used by default and enables calling the Superluminal Performance API. This can be useful to only enable the events only for specific application features

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

#[cfg(test)]
mod test;

#[cfg(all(feature = "enable", target_os = "windows"))]
use superluminal_perf_sys as ffi;

/// Check if the API is enabled
pub const fn enabled() -> bool {
    cfg!(all(feature = "enable", target_os = "windows"))
}

/// Begin an instrumentation event with the specified ID
pub fn begin_event(id: &str) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr() as *const i8, 
            id.len() as u16, 
            std::ptr::null(),
            0,
            ffi::DEFAULT_COLOR)
    }
}

/// Begin an instrumentation event with the specified ID and color
pub fn begin_event_with_color(id: &str, color: u32) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr() as *const i8, 
            id.len() as u16, 
            std::ptr::null(),
            0,
            color)
    }
}

/// Begin an instrumentation event with the specified ID and runtime data
///
/// The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime.
pub fn begin_event_with_data(id: &str, data: &str) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr() as *const i8, 
            id.len() as u16, 
            data.as_ptr() as *const i8, 
            data.len() as u16, 
            ffi::DEFAULT_COLOR)
    }
}

/// End an instrumentation event.
///
/// Must be matched with a call to `begin_event` within the same function
pub fn end_event() {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        // PerformanceAPI_EndEvent returns a struct which is only used to prevent calls to it from being tail-call optimized.
        // We ignore the return value here so the caller of end_event doesn't need to deal with it.
        let _ = ffi::PerformanceAPI_EndEvent();
    }
}

/// Set the name of the current thread
pub fn set_current_thread_name(name: &str) -> Result<(), std::ffi::NulError> {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let name_cstr = std::ffi::CString::new(name)?;
        ffi::PerformanceAPI_SetCurrentThreadName(name_cstr.as_ptr())
    }
    Ok(())
}
