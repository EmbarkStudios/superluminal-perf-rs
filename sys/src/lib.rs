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

    pub fn PerformanceAPI_RegisterFiber(in_fiber_id: u64);

    pub fn PerformanceAPI_UnregisterFiber(in_fiber_id: u64);

    pub fn PerformanceAPI_BeginFiberSwitch(in_current_fiber_id: u64, in_new_fiber_id: u64);

    pub fn PerformanceAPI_EndFiberSwitch(in_fiber_id: u64);
}
