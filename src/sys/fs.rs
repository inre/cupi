use std::fmt::Display;
use std::io::prelude::*;
use std::io;
use std::fs::{OpenOptions, File};
use std::os::unix::io::{RawFd, FromRawFd, AsRawFd};
use mio::{Token, Evented, Ready, PollOpt, Poll};
use mio::unix::EventedFd;

#[derive(Debug)]
pub struct Selector {
    sys: File
}

impl Selector {
    fn readable() -> OpenOptions {
        let mut opt = OpenOptions::new();
        opt.read(true);
        opt
    }

    fn writable() -> OpenOptions {
        let mut opt = OpenOptions::new();
        opt.read(true).write(true);
        opt
    }
}

impl FromRawFd for Selector {
    unsafe fn from_raw_fd(fd: RawFd) -> Selector {
        Selector { sys: File::from_raw_fd(fd) }
    }
}

impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        self.sys.as_raw_fd()
    }
}

impl Evented for Selector {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        EventedFd(&self.sys.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        EventedFd(&self.sys.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.sys.as_raw_fd()).deregister(poll)
    }
}

impl Read for Selector {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        self.sys.read(dst)
    }
}

impl Write for Selector {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        self.sys.write(src)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.sys.flush()
    }
}

impl Seek for Selector {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.sys.seek(pos)
    }
}

pub struct GPIOSelector;

impl GPIOSelector {
    pub fn open<D: Display>(name: D) -> io::Result<Selector> {
        Ok(Selector {
            sys: try!(Selector::writable().open(format!("/sys/class/gpio/{}", name)))
        })
    }

    pub fn write<T: Display, D: Display>(name: T, src: D) -> io::Result<()> {
        let mut sel = try!(Self::open(name));
        try!(write!(sel.sys, "{}", src));
        Ok(())
    }
}

pub struct GPIOPinSelector;

impl GPIOPinSelector {

    pub fn open<D: Display>(pin: usize, name: D) -> io::Result<Selector> {
        Ok(Selector {
            sys: try!(Selector::writable().open(format!("/sys/class/gpio/gpio{}/{}", pin, name)))
        })
    }

    pub fn open_rd<D: Display>(pin: usize, name: D) -> io::Result<Selector> {
        Ok(Selector {
            sys: try!(Selector::readable().open(format!("/sys/class/gpio/gpio{}/{}", pin, name)))
        })
    }

    pub fn write<T: Display, D: Display>(pin: usize, name: T, src: D) -> io::Result<()> {
        let mut sel = try!(Self::open(pin, name));
        try!(write!(sel.sys, "{}", src));
        Ok(())
    }
}
