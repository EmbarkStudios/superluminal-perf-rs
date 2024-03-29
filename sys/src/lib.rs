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
#![allow()]

use std::os::raw::c_char;

pub const DEFAULT_COLOR: u32 = 0xFF_FF_FF_FF;

#[repr(C)]
pub struct PerformanceAPI_SuppressTailCallOptimization {
    _private: [u64; 3],
}

extern "C" {
    pub fn PerformanceAPI_SetCurrentThreadName(in_thread_name: *const c_char);
    pub fn PerformanceAPI_SetCurrentThreadName_N(
        in_thread_name: *const c_char,
        in_thread_name_len: u16,
    );

    pub fn PerformanceAPI_BeginEvent(in_id: *const c_char, in_data: *const c_char, in_color: u32);
    pub fn PerformanceAPI_BeginEvent_N(
        in_id: *const c_char,
        in_id_len: u16,
        in_data: *const c_char,
        in_data_len: u16,
        in_color: u32,
    );

    pub fn PerformanceAPI_EndEvent() -> PerformanceAPI_SuppressTailCallOptimization;
}
