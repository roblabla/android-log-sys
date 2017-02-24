#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum android_LogPriority {
    ANDROID_LOG_UNKNOWN = 0,
    ANDROID_LOG_DEFAULT = 1,
    ANDROID_LOG_VERBOSE = 2,
    ANDROID_LOG_DEBUG = 3,
    ANDROID_LOG_INFO = 4,
    ANDROID_LOG_WARN = 5,
    ANDROID_LOG_ERROR = 6,
    ANDROID_LOG_FATAL = 7,
    ANDROID_LOG_SILENT = 8,
}

extern "C" {
    pub fn __android_log_write(prio: ::std::os::raw::c_int,
                               tag: *const ::std::os::raw::c_char,
                               text: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;

    pub fn __android_log_print(prio: ::std::os::raw::c_int,
                               tag: *const ::std::os::raw::c_char,
                               fmt: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;

    pub fn __android_log_assert(cond: *const ::std::os::raw::c_char,
                                tag: *const ::std::os::raw::c_char,
                                fmt: *const ::std::os::raw::c_char, ...);
}
