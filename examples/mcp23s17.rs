extern crate cupi;

use cupi::{Logic, DigitalWrite, DigitalRead, delay_ms};
use cupi::mcp23x17::MCP23S17;

fn main() {
    let mut mcp23s17 = unsafe { MCP23S17::new(0).unwrap() };
    let mut port_out = mcp23s17.porta();
    let mut port_in = mcp23s17.portb();

    let mut pinin0 = port_in.input(0).unwrap();
    let mut pinin1 = port_in.input(1).unwrap();
    let mut pinin2 = port_in.input(2).unwrap();
    let mut pinin3 = port_in.input(3).unwrap();

    pinin0.pull_up();
    pinin1.pull_up();
    pinin2.pull_up();
    pinin3.pull_up();

    let mut pinout0 = port_out.output(0).unwrap();
    let mut pinout1 = port_out.output(1).unwrap();
    let mut pinout2 = port_out.output(2).unwrap();
    let mut pinout3 = port_out.output(3).unwrap();

    for i in 0..1000 {
        match pinin0.get().unwrap() {
            Logic::Low  => {
                pinout0.set(1).unwrap();
                delay_ms(500);
                pinout0.set(0).unwrap();

            }
            Logic::High => ()
        }
        match pinin1.get().unwrap() {
            Logic::Low  => {
                pinout1.set(1).unwrap();
                delay_ms(500);
                pinout1.set(0).unwrap();

            }
            Logic::High => ()
        }
        match pinin2.get().unwrap() {
            Logic::Low  => {
                pinout2.set(1).unwrap();
                delay_ms(500);
                pinout2.set(0).unwrap();

            }
            Logic::High => ()
        }
        match pinin3.get().unwrap() {
            Logic::Low  => {
                pinout3.set(1).unwrap();
                delay_ms(500);
                pinout3.set(0).unwrap();

            }
            Logic::High => ()
        }
        delay_ms(100);
    }
}
