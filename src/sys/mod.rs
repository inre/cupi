use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::process::Command;
use {Result, Error};

pub use self::gpio::{
    Pin,
    PinInput,
    PinOutput,
};

pub use self::fs::{
    Selector,
    GPIOSelector,
    GPIOPinSelector
};

mod gpio;
mod fs;

#[derive(Copy, Clone, Debug)]
pub enum Edge {
    NoInterrupt,
    RisingEdge,
    FallingEdge,
    BothEdges
}

#[derive(Clone, Debug)]
pub struct CPUInfo(pub HashMap<String, String>);

pub fn cpuinfo() -> Result<CPUInfo> {
    let mut f = try!(File::open("/proc/cpuinfo"));
    let mut s = String::new();
    let mut h = HashMap::new();

    try!(f.read_to_string(&mut s));

    let v: Vec<&str> = s.split("\n").collect();
    for i in &v {
        let l: Vec<&str> = i.splitn(2,":").collect();
        if l.len() >= 2 {
            h.insert(l[0].trim().to_string(), l[1].trim().to_string());
        }
    }
    Ok(CPUInfo(h))
}

#[derive(Copy, Clone, Debug)]
pub struct Memory {
    pub total: u32,
    pub used: u32,
    pub free: u32,
    pub shared: u32,
    pub buffers: u32,
    pub cached: u32
}

pub fn memory() -> Result<Memory> {
    let o = try!(Command::new("free").output());
    let f = o.stdout;
    let s = try!(String::from_utf8(f));
    //try!(f.read_to_string(&mut s));
    let v: Vec<&str> = s.split("\n").collect();
    for i in &v {
        let w: Vec<&str> = i.split(" ").filter(|&s| s.len() != 0).collect();
        if w[0] == "Mem:" && w.len() == 7 {
            let total   = try!(w[1].parse::<u32>());
            let used    = try!(w[2].parse::<u32>());
            let free    = try!(w[3].parse::<u32>());
            let shared  = try!(w[4].parse::<u32>());
            let buffers = try!(w[5].parse::<u32>());
            let cached  = try!(w[6].parse::<u32>());

            return Ok(Memory { total: total, used: used, free: free, shared: shared, buffers: buffers, cached: cached })
        }
    };
    Err(Error::UnexpectedError)
}
