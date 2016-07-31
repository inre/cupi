use mcp23x17::{MCP23S17, PinInput, PinOutput, Port, GroupInput, GroupOutput};
use mio::{EventLoop, Handler, Token};
use {sys, Result};

// PiFace 2
#[allow(dead_code)]
pub struct PiFace {
    port_out: Port,
    port_in: Port,
    inputs: GroupInput,
    pin_int: sys::Pin,
    interrupt: sys::PinInput
}

impl PiFace {
    pub unsafe fn new() -> Result<PiFace> {
        let mcp23s17 = try!(MCP23S17::new(0));
        let port_out = mcp23s17.porta();
        let mut port_in = mcp23s17.portb();

        let mut inputs  = try!(port_in.group_input(0xFF));
        // pull up always on
        try!(inputs.pull_up());

        let mut pin_int = sys::Pin::new(25);
        // export interrupt pin
        try!(pin_int.export());
        let interrupt = try!(pin_int.input());

        Ok(PiFace {
            port_out: port_out,
            port_in: port_in,
            inputs: inputs,
            pin_int: pin_int,
            interrupt: interrupt
        })
    }
/*
    pub fn export(&mut self) -> Result<&mut Self> {
        println!("exporting");
        try!(self.pin_int.export());
        println!("exported");
        Ok(self)
    }*/

    pub fn input(&mut self, pin: usize) -> Result<PinInput> {
        assert!(pin < 8);
        Ok(try!(self.port_in.input(pin)))
    }

    pub fn output(&mut self, pin: usize) -> Result<PinOutput> {
        assert!(pin < 8);
        Ok(try!(self.port_out.output(pin)))
    }

    pub fn group_input(&mut self, mask: u8) -> Result<GroupInput> {
        Ok(try!(self.port_in.group_input(mask)))
    }

    pub fn group_output(&mut self, mask: u8) -> Result<GroupOutput> {
        Ok(try!(self.port_out.group_output(mask)))
    }

    pub fn trigger<H: Handler>(&mut self, event_loop: &mut EventLoop<H>, token: Token) -> Result<()> {
        Ok(try!(self.interrupt.trigger(event_loop, token, sys::Edge::FallingEdge)))
    }

    pub fn stop_trigger<H: Handler>(&mut self, event_loop: &mut EventLoop<H>) -> Result<()> {
        Ok(try!(self.interrupt.stop_trigger(event_loop)))
    }
}
