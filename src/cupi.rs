use {Error, Result, Board, board, is_root};
use bcm270x::{GPIO, PinOptions};
use sys;

pub struct CuPi {
    pub board: Board,
    pub is_root: bool,
    gpio: Option<GPIO>
}

impl CuPi {
    pub fn new() -> Result<CuPi> {
        let board = try!(board());
        let is_root = is_root();
        let gpio = match is_root {
            true => Some(try!(GPIO::new(board.cpu))),
            false => None,
        };

        let cupi = CuPi {
            board: board,
            is_root: is_root,
            gpio: gpio
        };

        Ok(cupi)
    }

    pub fn pin(&self, pin: usize) -> Result<PinOptions> {
        let gpio_pin = try!(self.board.pin_to_gpio(pin));
        match self.gpio {
            Some(ref gpio) => Ok(unsafe { gpio.pin(gpio_pin) }),
            None           => Err(Error::RootRequired),
        }
    }

    pub fn pin_sys(&self, pin: usize) -> Result<sys::Pin> {
        let gpio_pin = try!(self.board.pin_to_gpio(pin));
        Ok(unsafe { sys::Pin::new(gpio_pin) })
    }
}
