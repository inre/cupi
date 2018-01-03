use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};
use {Result, RegisterDesc, Logic, DigitalLogic, DigitalWrite, DigitalRead};
use std::io;
use std::sync::{Arc, Mutex};
use sys::Edge;

pub enum MCP23X17Register {
    IODIR(usize),
    IPOL(usize),
    GPINTEN(usize),
    DEFVAL(usize),
    INTCON(usize),
    IOCON(usize),
    GPPU(usize),
    INTF(usize),
    INTCAP(usize),
    GPIO(usize),
    OLAT(usize),
}

impl RegisterDesc for MCP23X17Register {
    fn offset(&self) -> usize {
        match *self {
            MCP23X17Register::IODIR(n)   => n,
            MCP23X17Register::IPOL(n)    => n + 2,
            MCP23X17Register::GPINTEN(n) => n + 4,
            MCP23X17Register::DEFVAL(n)  => n + 6,
            MCP23X17Register::INTCON(n)  => n + 8,
            MCP23X17Register::IOCON(n)   => n + 10,
            MCP23X17Register::GPPU(n)    => n + 12,
            MCP23X17Register::INTF(n)    => n + 14,
            MCP23X17Register::INTCAP(n)  => n + 16,
            MCP23X17Register::GPIO(n)    => n + 18,
            MCP23X17Register::OLAT(n)    => n + 20,
        }
    }
}

impl MCP23X17Register {
    pub fn write(&self, spi: &Spidev, address: usize, value: u8) -> io::Result<()> {
        let mut tr = SpidevTransfer::write(&[
            (CMD_WRITE | (address << 1)) as u8,
            self.offset() as u8,
            value
        ]);
        try!(spi.transfer(&mut tr));
        Ok(())
    }

    pub fn read(&self, spi: &Spidev, address: usize ) -> io::Result<u8> {
        let mut tr = SpidevTransfer::write(&[
            (CMD_READ | (address << 1)) as u8,
            self.offset() as u8,
            0
        ]);
        try!(spi.transfer(&mut tr));
        match tr.rx_buf {
            Some(ref rx_buf) => Ok(rx_buf[2]),
            _ => panic!()
        }
    }
}

const CMD_WRITE: usize = 0x40;
const CMD_READ: usize  = 0x41;

bitflags! {
    struct IOCONRegister: u8 {
        const IOCON_UNUSED = 0x01;
	    const IOCON_INTPOL = 0x02;
	    const IOCON_ODR	   = 0x04;
        const IOCON_HAEN   = 0x08;
        const IOCON_DISSLW = 0x10;
        const IOCON_SEQOP  = 0x20;
        const IOCON_MIRROR = 0x40;
        const IOCON_BANK_MODE = 0x80;
    }
}

pub struct MCP23S17 {
    spi: Arc<Mutex<Spidev>>,
    address: usize
}

impl MCP23S17 {
    pub unsafe fn new(address: usize) -> Result<Self> {
        let mut spi = try!(Spidev::open("/dev/spidev0.0"));
        assert!(address <= 8);
        let mut options = SpidevOptions::new();
        options.bits_per_word(8)
               .max_speed_hz(4_000_000)
               .mode(SPI_MODE_0);

        try!(spi.configure(&options));

        let iocona = MCP23X17Register::IOCON(0);
        try!(iocona.write(&spi, address, (IOCONRegister::IOCON_SEQOP | IOCONRegister::IOCON_HAEN).bits()));
        let ioconb = MCP23X17Register::IOCON(1);
        try!(ioconb.write(&spi, address, (IOCONRegister::IOCON_SEQOP | IOCONRegister::IOCON_HAEN).bits()));

        Ok(MCP23S17 {
            spi: Arc::new(Mutex::new(spi)),
            address: address
        })
    }

    pub fn porta(&self) -> Port {
        Port { spi: self.spi.clone(), address: self.address, port: 0 }
    }

    pub fn portb(&self) -> Port {
        Port { spi: self.spi.clone(), address: self.address, port: 1 }
    }
}

pub struct Port {
    spi: Arc<Mutex<Spidev>>,
    address: usize,
    port: usize,
}

impl Port {
    pub fn input(&mut self, pin: usize) -> Result<PinInput> {
        assert!(pin < 8);
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mask = 1 << pin;
        let iodir = MCP23X17Register::IODIR(self.port);
        // modify bit
        let mut dir = try!(iodir.read(&spi, self.address));
        dir |= mask;
        try!(iodir.write(&spi, self.address, dir));

        Ok(PinInput {
            spi: self.spi.clone(),
            address: self.address,
            port: self.port,
            pin: pin
        })
    }

    pub fn output(&mut self, pin: usize) -> Result<PinOutput> {
        assert!(pin < 8);
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mask = 1 << pin;
        let iodir = MCP23X17Register::IODIR(self.port);
        // modify bit
        let mut dir = try!(iodir.read(&spi, self.address));
        dir &= !mask;
        try!(iodir.write(&spi, self.address, dir));

        Ok(PinOutput {
            spi: self.spi.clone(),
            address: self.address,
            port: self.port,
            pin: pin
        })
    }

    pub fn group_output(&mut self, mask: u8) -> Result<GroupOutput> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let iodir = MCP23X17Register::IODIR(self.port);
        let mut dir = try!(iodir.read(&spi, self.address));
        dir &= !mask;
        try!(iodir.write(&spi, self.address, dir));

        Ok(GroupOutput {
            spi: self.spi.clone(),
            address: self.address,
            port: self.port,
            mask: mask as u8
        })
    }

    pub fn group_input(&mut self, mask: u8) -> Result<GroupInput> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let iodir = MCP23X17Register::IODIR(self.port);
        // modify bit
        let mut dir = try!(iodir.read(&spi, self.address));
        dir |= mask;
        try!(iodir.write(&spi, self.address, dir));

        Ok(GroupInput {
            spi: self.spi.clone(),
            address: self.address,
            port: self.port,
            mask: mask as u8
        })
    }
}

pub struct PinInput {
    spi: Arc<Mutex<Spidev>>,
    address: usize,
    port: usize,
    pin: usize
}

impl PinInput {
    pub fn pull_up(&mut self) -> Result<()> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mask = 1 << self.pin;
        let iodir = MCP23X17Register::GPPU(self.port);
        // modify bit
        let mut dir = try!(iodir.read(&spi, self.address));
        dir |= mask;
        try!(iodir.write(&spi, self.address, dir));
        Ok(())
    }

    pub fn pull_off(&mut self) -> Result<()> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mask = 1 << self.pin;
        let iodir = MCP23X17Register::GPPU(self.port);
        // modify bit
        let mut dir = try!(iodir.read(&spi, self.address));
        dir &= !mask;
        try!(iodir.write(&spi, self.address, dir));
        Ok(())
    }
}

impl DigitalRead for PinInput {
    fn digital_read(&mut self) -> Result<Logic> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mask = 1 << self.pin;
        let gpio = MCP23X17Register::GPIO(self.port);
        let val = try!(gpio.read(&spi, self.address));

        match val & mask {
            0 => Ok(Logic::Low),
            _ => Ok(Logic::High)
        }
    }
}

pub struct PinOutput {
    spi: Arc<Mutex<Spidev>>,
    address: usize,
    port: usize,
    pin: usize
}

impl DigitalWrite for PinOutput {
    fn digital_write<L: DigitalLogic>(&mut self, level: L) -> Result<()> {
      let spi = match self.spi.lock() {
          Ok(guard) => guard,
          Err(poisoned) => poisoned.into_inner(),
      };
      let bit: u8 = 1 << self.pin;
      let gpio = MCP23X17Register::GPIO(self.port);
      let mut val = try!(gpio.read(&spi, self.address));
      match level.logic_level() {
          Logic::Low  => val &= !bit,
          Logic::High => val |= bit
      }
      try!(gpio.write(&spi, self.address, val));
      Ok(())
    }
}

impl Drop for PinOutput {
    fn drop(&mut self) {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mask = 1 << self.pin;
        let iodir = MCP23X17Register::IODIR(self.port);
        // make pin input
        if let Ok(mut dir) = iodir.read(&spi, self.address) {
            dir &= !mask;
            let _ = iodir.write(&spi, self.address, dir);
        }
    }
}

pub struct GroupInput {
    spi: Arc<Mutex<Spidev>>,
    address: usize,
    port: usize,
    mask: u8
}

impl GroupInput {
    pub fn pull_up(&mut self) -> Result<()> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let iodir = MCP23X17Register::GPPU(self.port);
        let mut dir = try!(iodir.read(&spi, self.address));
        dir |= self.mask;
        try!(iodir.write(&spi, self.address, dir));
        Ok(())
    }

    pub fn pull_off(&mut self) -> Result<()> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let iodir = MCP23X17Register::GPPU(self.port);
        let mut dir = try!(iodir.read(&spi, self.address));
        dir &= !self.mask;
        try!(iodir.write(&spi, self.address, dir));
        Ok(())
    }

    pub fn digital_read(&mut self) -> Result<u8> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let gpio = MCP23X17Register::GPIO(self.port);
        let val = try!(gpio.read(&spi, self.address));

        Ok(val & self.mask)
    }

    pub fn interrupt(&mut self, edge: Edge) -> Result<()> {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let defval = MCP23X17Register::DEFVAL(self.port);
        let mut val = try!(defval.read(&spi, self.address));
        match edge {
            Edge::RisingEdge  => val |= self.mask,
            Edge::FallingEdge => val &= !self.mask,
            Edge::BothEdges   => (),
            Edge::NoInterrupt => ()
        }
        println!("{:b}", val);
        try!(defval.write(&spi, self.address, val));

        let intcon = MCP23X17Register::INTCON(self.port);
        let mut val = try!(intcon.read(&spi, self.address));
        match edge {
            Edge::RisingEdge  => val |= self.mask,
            Edge::FallingEdge => val |= self.mask,
            Edge::BothEdges   => val &= !self.mask,
            Edge::NoInterrupt => ()
        }
        println!("{:b}", val);
        try!(intcon.write(&spi, self.address, val));

        // enable interrupts on masked pins
        let gpinten = MCP23X17Register::GPINTEN(self.port);
        let mut val = try!(gpinten.read(&spi, self.address));

        match edge {
            Edge::RisingEdge  => val |= self.mask,
            Edge::FallingEdge => val |= self.mask,
            Edge::BothEdges   => val |= self.mask,
            Edge::NoInterrupt => val &= !self.mask
        }
        println!("{:b}", val);
        try!(gpinten.write(&spi, self.address, val));
        Ok(())
    }

    pub fn stop_interrupt(&mut self) -> Result<()> {
        Ok(try!(self.interrupt(Edge::NoInterrupt)))
    }
}


pub struct GroupOutput {
    spi: Arc<Mutex<Spidev>>,
    address: usize,
    port: usize,
    mask: u8
}

impl DigitalWrite for GroupOutput {
    fn digital_write<L: DigitalLogic>(&mut self, level: L) -> Result<()> {
      let spi = match self.spi.lock() {
          Ok(guard) => guard,
          Err(poisoned) => poisoned.into_inner(),
      };
      let gpio = MCP23X17Register::GPIO(self.port);
      let mut val = try!(gpio.read(&spi, self.address));
      match level.logic_level() {
          Logic::Low  => val &= !self.mask,
          Logic::High => val |= self.mask
      }
      try!(gpio.write(&spi, self.address, val));
      Ok(())
    }
}

impl Drop for GroupOutput {
    fn drop(&mut self) {
        let spi = match self.spi.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let iodir = MCP23X17Register::IODIR(self.port);
        // make pin input
        if let Ok(mut dir) = iodir.read(&spi, self.address) {
            dir &= !self.mask;
            let _ = iodir.write(&spi, self.address, dir);
        }
    }
}
