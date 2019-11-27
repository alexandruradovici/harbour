use libc;

pub const _SC_CLK_TCK: i32 = libc::_SC_CLK_TCK;

pub fn sysconf(value:i32) -> Option<i64>
{
    let value = unsafe {
        libc::sysconf (value)
    };
    if value < 0 {
        return None;
    }
    Some (value)
}