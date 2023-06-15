//! [Superluminal Performance](https://superluminal.eu/) profiler Rust API for adding user events to captures.
//!
//! ## How to use
//!
//! In `Cargo.toml` add:
//!
//! ```toml
//! [dependencies]
//! superluminal-perf = "0.3.0"
//! ```
//!
//! Example usage:
//!
//! ```rust
//! superluminal_perf::begin_event("my-event");
//! calc();
//! superluminal_perf::end_event();
//!
//! superluminal_perf::begin_event("my-event2");
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
#![allow(unsafe_code)]

#[cfg(test)]
mod test;

#[cfg(all(feature = "enable", target_os = "windows"))]
use superluminal_perf_sys as ffi;

/// Check if the API is enabled
pub const fn enabled() -> bool {
    cfg!(all(feature = "enable", target_os = "windows"))
}

/// Begin an instrumentation event with the specified ID
pub fn begin_event(id: &'static str) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr().cast::<i8>(),
            id.len() as u16,
            std::ptr::null(),
            0,
            ffi::DEFAULT_COLOR,
        );
    }
}

/// Begin an instrumentation event with the specified ID and color
pub fn begin_event_with_color(id: &'static str, color: u32) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr().cast::<i8>(),
            id.len() as u16,
            std::ptr::null(),
            0,
            color,
        );
    }
}

/// Begin an instrumentation event with the specified ID, runtime data, and color
///
/// The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime.
pub fn begin_event_with_data(id: &'static str, data: &str, color: u32) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr().cast::<i8>(),
            id.len() as u16,
            data.as_ptr().cast::<i8>(),
            data.len() as u16,
            color,
        );
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
pub fn set_current_thread_name(name: &str) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_SetCurrentThreadName_N(name.as_ptr().cast::<i8>(), name.len() as u16);
    }
}

/// Register a Windows Fiber
pub fn register_fiber(in_fiber_id: u64) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_RegisterFiber(in_fiber_id);
    }
}

/// Unregister a Windows Fiber
pub fn unregister_fiber(in_fiber_id: u64) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_UnregisterFiber(in_fiber_id);
    }
}

/// Begin a Windows Fiber Switch
///
/// Must be called before `SwitchToFiber`.
pub fn begin_fiber_switch(in_current_fiber_id: u64, in_new_fiber_id: u64) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginFiberSwitch(in_current_fiber_id, in_new_fiber_id);
    }
}

/// End a Windows Fiber Switch
///
/// Must be called after `SwitchToFiber`.
pub fn end_fiber_switch(in_fiber_id: u64) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_EndFiberSwitch(in_fiber_id);
    }
}
