#![feature(associated_consts)]
#![feature(core_intrinsics)]
#![feature(duration)]
#![feature(thread_sleep)]
#![feature(convert)]
#![feature(core)]

extern crate libc;
extern crate mmap;
extern crate mio;
extern crate spidev;
extern crate core;
extern crate nix;
#[macro_use] extern crate bitflags;

mod result;
mod map;
mod logic;
mod board;
mod cupi;

pub use cupi::{
    CuPi
};

pub use logic::{
    Logic,
    Logic3,
    DigitalLogic,
    DigitalWrite,
    DigitalRead,
    AnalogWrite,
    AnalogRead,
};

pub use result::{
    Result,
    Error
};

pub use board::{
    Board,
    Hardware,
    CPU,
    RaspberryModel,
    RaspberryRevision,
    RaspberryMaker,
    board
};

pub mod bcm270x;
pub mod sys;

pub use bcm270x::{
    PinOptions,
    PinInput,
    PinOutput
};

use std::thread;
use std::time::Duration;
use nix::sys::ioctl::libc::geteuid;

pub trait RegisterDesc {
    fn offset(&self) -> usize;
}

pub trait RegisterOperations<T> {
    unsafe fn write(&self, data: T);
    unsafe fn read(&self) -> T;
    unsafe fn bitand(&self, data: T);
    unsafe fn bitor(&self, data: T);
    unsafe fn bitxor(&self, data: T);
}

pub struct Register<R: RegisterDesc> {
    pub ptr: *mut u32,
    pub desc: R
}

impl<R: RegisterDesc> RegisterOperations<u32> for Register<R> {
    #[inline(always)]
    unsafe fn write(&self, data: u32) {
        *self.ptr = data;
    }

    #[inline(always)]
    unsafe fn read(&self) -> u32 {
        *self.ptr
    }

    #[inline(always)]
    unsafe fn bitand(&self, data: u32) {
        *self.ptr &= data;
    }

    #[inline(always)]
    unsafe fn bitor(&self, data: u32) {
        *self.ptr |= data;
    }

    #[inline(always)]
    unsafe fn bitxor(&self, data: u32) {
        *self.ptr ^= data;
    }
}

#[inline(always)]
pub fn delay(dur: Duration) {
    // FIXME: todo sleep hard if < 100
    thread::sleep(dur);
}

#[inline(always)]
pub fn delay_ms(ms: u64) {
    delay(Duration::from_millis(ms));
}

pub fn is_root() -> bool {
    unsafe { geteuid() == 0 }
}
