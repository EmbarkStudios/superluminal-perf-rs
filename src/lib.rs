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

// BEGIN - Embark standard lints v0.3
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::explicit_into_iter_loop,
    clippy::filter_map_next,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::pub_enum_variant_names,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::verbose_file_reads,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.3
// crate-specific exceptions:
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
pub fn begin_event(id: &str) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr() as *const i8,
            id.len() as u16,
            std::ptr::null(),
            0,
            ffi::DEFAULT_COLOR,
        )
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
            color,
        )
    }
}

/// Begin an instrumentation event with the specified ID, runtime data, and color
///
/// The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime.
pub fn begin_event_with_data(id: &str, data: &str, color: u32) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent_N(
            id.as_ptr() as *const i8,
            id.len() as u16,
            data.as_ptr() as *const i8,
            data.len() as u16,
            color,
        )
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
        ffi::PerformanceAPI_SetCurrentThreadName_N(name.as_ptr() as *const i8, name.len() as u16)
    }
}
