use std::ptr;
use std::thread;
use std::time::Duration;
use nix::sys::time::TimeVal;

const USEC_PER_SEC: u64 = 1_000_000;
const USEC_TO_NANOS: u64 = 1_000;

mod ffi {
    use nix::sys::time::TimeVal;
    use libc::{c_int};

    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct TimeZone {
        pub tz_minuteswest: c_int,
        pub tz_dsttime: c_int,
    }

    extern {
        pub fn gettimeofday(timeval: *mut TimeVal, timezone: *mut TimeZone) -> c_int;
    }
}

#[inline(always)]
pub fn delay_hard(usec: u64) {
    unsafe {
        let mut timenow = TimeVal::zero();
        let null: *const i32 = ptr::null();
        ffi::gettimeofday(&mut timenow, null as *mut ffi::TimeZone);

        let timeend = timenow + TimeVal::microseconds(usec as i64);
        while timenow < timeend {
            ffi::gettimeofday(&mut timenow, null as *mut ffi::TimeZone);
        }
    }
}

#[inline(always)]
pub fn delay_usec(usec: u64) {
    if usec == 0 {
        return;
    } else if usec < 100 {
        delay_hard(usec);
    } else {
        let secs = usec / USEC_PER_SEC;
        let nanos = ((usec % USEC_PER_SEC) * USEC_TO_NANOS) as u32;
        thread::sleep(Duration::new(secs, nanos));
    }
}

#[inline(always)]
pub fn delay_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

// millis
// micros

#[cfg(test)]
mod test {
    use super::{delay_hard, delay_usec, delay_ms};

    #[test]
    fn test_delay_hard() {
        delay_hard(50); // pause 50 microseconds
    }

    #[test]
    fn test_delay_usec() {
        delay_usec(150); // pause 150 microseconds
    }

    #[test]
    fn test_delay_ms() {
        delay_ms(1); // pause 1 millisecond
    }
}
