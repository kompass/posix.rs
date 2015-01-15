#[repr(C)]
#[derive(Copy)]
pub struct statvfs {
    pub f_bsize:   ::ulong_t,
    pub f_frsize:  ::ulong_t,
    pub f_blocks:  ::sys::types::fsblkcnt_t,
    pub f_bfree:   ::sys::types::fsblkcnt_t,
    pub f_bavail:  ::sys::types::fsblkcnt_t,
    pub f_files:   ::sys::types::fsfilcnt_t,
    pub f_ffree:   ::sys::types::fsfilcnt_t,
    pub f_favail:  ::sys::types::fsfilcnt_t,
    pub f_fsid:    ::ulong_t,
    pub f_flag:    ::ulong_t,
    pub f_namemax: ::ulong_t,
    __f_spare: [::int_t; 6],
}

new!(statvfs);

pub const ST_RDONLY: ::int_t = 1;
pub const ST_NOSUID: ::int_t = 2;
