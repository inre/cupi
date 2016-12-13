extern crate cupi;
extern crate mio;

use std::time::Duration;
use mio::{Poll, Events, Token, Ready, PollOpt, timer};
use cupi::{CuPi};
use cupi::sys::Edge;

const TERM_TOKEN: Token      = Token(0);
const PRESS_TOKEN: Token     = Token(1);
const DEBOUNCE_TOKEN: Token  = Token(2);

fn main() {
    let cupi = CuPi::new().unwrap();
    let _pull_up = cupi.pin(0).unwrap().pull_up().input();

    let mut pin = cupi.pin_sys(0).unwrap();
    pin.export().unwrap();
    let mut poll = Poll::new().unwrap();
    let mut pinin = pin.input().unwrap();

    // bind pin trigger
    pinin.trigger(&mut poll, PRESS_TOKEN, Edge::FallingEdge).unwrap();
    let mut pressed = false;

    // add debounce timer event
    let mut debounce_timer = timer::Timer::default();
    poll.register(&debounce_timer, DEBOUNCE_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();

    // add global termination timeout event
    let mut timeout = timer::Timer::default();
    poll.register(&timeout, TERM_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();
    timeout.set_timeout(Duration::from_millis(15000), true).unwrap();

    let mut events = Events::with_capacity(10);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                PRESS_TOKEN => if !pressed {
                    pressed = true;
                    debounce_timer.set_timeout(Duration::from_millis(200), false).unwrap();
                    println!("Pressed!");
                    //print!("{}", self.pinin.get().unwrap());
                },
                DEBOUNCE_TOKEN => if pressed {
                    println!("Debounced.");
                    pressed = false;
                },
                TERM_TOKEN => {
                    println!("Stopped.");
                    return;
                },
                _ => unreachable!(),
            }
        }

    }
}
