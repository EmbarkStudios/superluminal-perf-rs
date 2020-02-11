use libc::c_char;

pub const DEFAULT_COLOR: u32 = 0xFF_FF_FF_FF;

#[repr(C)]
pub struct PerformanceAPI_SuppressTailCallOptimization {
    _private: [u64; 3] 
}

extern "C" {
    pub fn PerformanceAPI_SetCurrentThreadName(in_thread_name: *const c_char);

    pub fn PerformanceAPI_BeginEvent(in_id: *const c_char, in_data: *const c_char, in_color: u32);

    pub fn PerformanceAPI_EndEvent() -> PerformanceAPI_SuppressTailCallOptimization;
}
