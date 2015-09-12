use std::io::prelude::*;
use std::io::SeekFrom;
use std::os::unix::io::{AsRawFd, RawFd};
use mio::{EventLoop, Handler, Token, EventSet, PollOpt};
use sys::{Edge, Selector, GPIOSelector, GPIOPinSelector};
use {Result, Error, Logic, DigitalLogic, DigitalWrite, DigitalRead, is_root};

#[derive(Debug)]
pub struct Pin {
    pin: usize,
    exported: bool
}

impl Pin {
    pub unsafe fn new(pin: usize) -> Pin {
        Pin { pin: pin, exported: false }
    }

    pub fn export(&mut self) -> Result<()> {
        if !is_root() {
            return Err(Error::RootRequired)
        }
        // re-export
        let _ = GPIOSelector::write("unexport", self.pin);
        try!(GPIOSelector::write("export", self.pin));
        self.exported = true;
        Ok(())
    }

    pub fn input(&self) -> Result<PinInput> {
        // Set direction to in
        try!(GPIOPinSelector::write(self.pin, "direction", "in"));
        // Open value file
        let sel = try!(GPIOPinSelector::open_rd(self.pin, "value"));

        Ok(PinInput { sel: sel, pin: self.pin })
    }

    pub fn output(&self) -> Result<PinOutput> {
        // Set direction to out
        try!(GPIOPinSelector::write(self.pin, "direction", "out"));
        // Open value file
        let sel = try!(GPIOPinSelector::open(self.pin, "value"));
        Ok(PinOutput { sel: sel, pin: self.pin })
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        if self.exported {
            let _ = GPIOSelector::write("unexport", self.pin);
        }
    }
}

#[derive(Debug)]
pub struct PinInput {
    sel: Selector,
    pin: usize
}

impl DigitalRead for PinInput {
    fn digital_read(&mut self) -> Result<Logic> {
        try!(self.sel.seek(SeekFrom::Start(0)));
        let mut buf = [0u8];
        let len = try!(self.sel.read(&mut buf));

        if len == 0 {
            return Err(Error::UnexpectedError);
        }

        match buf[0] {
            b'1' => Ok(Logic::High),
            b'0' => Ok(Logic::Low),
            _ => Err(Error::UnexpectedError),
        }
    }
}

impl PinInput {
    pub fn trigger<H: Handler>(&mut self, event_loop: &mut EventLoop<H>, token: Token, edge: Edge) -> Result<()> {
        // Set edge for trigger
        try!(self.set_edge(edge));
        // Clear io buffer
        let mut s = String::with_capacity(255);
        try!(self.sel.read_to_string(&mut s));
        // Register sel
        try!(event_loop.register(&self.sel, token, EventSet::readable(), PollOpt::edge() | PollOpt::urgent() ));
        Ok(())
    }

    pub fn stop_trigger<H: Handler>(&mut self, event_loop: &mut EventLoop<H>) -> Result<()> {
        try!(event_loop.deregister(&self.sel));
        Ok(())
    }

    fn set_edge(&mut self, edge: Edge) -> Result<()> {
        try!(GPIOPinSelector::write(self.pin, "edge", match edge {
            Edge::NoInterrupt => "none",
            Edge::RisingEdge  => "rising",
            Edge::FallingEdge => "falling",
            Edge::BothEdges   => "both",
        }));
        Ok(())
    }
}

impl AsRawFd for PinInput {
    fn as_raw_fd(&self) -> RawFd {
        self.sel.as_raw_fd()
    }
}

#[derive(Debug)]
pub struct PinOutput {
    sel: Selector,
    pin: usize
}

impl DigitalWrite for PinOutput {
  fn digital_write<L: DigitalLogic>(&mut self, level: L) -> Result<()> {
      let buf: [u8;1] = match level.logic_level() {
          Logic::High => [b'1'],
          Logic::Low => [b'0'],
      };
      let _ = try!(self.sel.write(&buf));
      Ok(())
  }
}

impl AsRawFd for PinOutput {
    fn as_raw_fd(&self) -> RawFd {
        self.sel.as_raw_fd()
    }
}
