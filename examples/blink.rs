extern crate cupi;

use cupi::{CuPi, delay_ms, DigitalWrite};

fn main() {
    let cupi = CuPi::new().unwrap();
    let mut pinout = cupi.pin(0).unwrap().high().output();
    //let mut pin = cupi.pin_sys(0).unwrap();
    //pin.export().unwrap();
    //let mut pinout = pin.output().unwrap();

    for _ in 0..20 {
        pinout.high().unwrap();
        delay_ms(600);
        pinout.low().unwrap();
        delay_ms(600);
    }
}
