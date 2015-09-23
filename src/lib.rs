#![feature(associated_consts)]
#![feature(core_intrinsics)]
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
    fn write(&self, data: T);
    fn read(&self) -> T;
    fn bitand(&self, data: T);
    fn bitor(&self, data: T);
    fn bitxor(&self, data: T);
}

pub struct Register<R: RegisterDesc> {
    pub ptr: *mut u32,
    pub desc: R
}

impl<R: RegisterDesc> RegisterOperations<u32> for Register<R> {
    #[inline(always)]
    fn write(&self, data: u32) {
        unsafe { *self.ptr = data; }
    }

    #[inline(always)]
    fn read(&self) -> u32 {
        unsafe { *self.ptr }
    }

    #[inline(always)]
    fn bitand(&self, data: u32) {
        unsafe { *self.ptr &= data; }
    }

    #[inline(always)]
    fn bitor(&self, data: u32) {
        unsafe { *self.ptr |= data; }
    }

    #[inline(always)]
    fn bitxor(&self, data: u32) {
        unsafe { *self.ptr ^= data; }
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
