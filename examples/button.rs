extern crate cupi;
extern crate mio;

use std::time::Duration;
use mio::{EventLoop, Handler, Token, EventSet};
use cupi::{CuPi};
use cupi::sys::Edge;

const TERM_TOKEN: Token      = Token(0);
const PRESS_TOKEN: Token     = Token(1);
const DEBOUNCE_TOKEN: Token  = Token(2);

struct PressHandler {
    pressed: bool
}

impl Handler for PressHandler {
    type Timeout = Token;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, _: EventSet) {
        match token {
            PRESS_TOKEN => {
                if !self.pressed {
                    self.pressed = true;
                    event_loop.timeout(DEBOUNCE_TOKEN, Duration::from_millis(200)).unwrap();
                    println!("Pressed!");
                    //print!("{}", self.pinin.get().unwrap());
                }
            },
            token => println!("Something with {:?} is ready", token)
        }
    }

    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, token: Token) {
        match token {
            DEBOUNCE_TOKEN => {
                self.pressed = false
            }
            TERM_TOKEN => {
                println!("Stopped.");
                event_loop.shutdown();
            }
            token => println!("Something with {:?} timed out", token)
        }
    }
}

fn main() {
    let cupi = CuPi::new().unwrap();
    let _pull_up = cupi.pin(0).unwrap().pull_up().input();

    let mut pin = cupi.pin_sys(0).unwrap();
    pin.export().unwrap();
    let mut event_loop = EventLoop::new().unwrap();
    let mut pinin = pin.input().unwrap();

    // bind trigger
    pinin.trigger(&mut event_loop, PRESS_TOKEN, Edge::FallingEdge).unwrap();
    let mut handler = PressHandler { pressed: false };

    event_loop.timeout(TERM_TOKEN, Duration::from_millis(15000)).unwrap();
    event_loop.run(&mut handler).unwrap();
}
