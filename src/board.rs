use {Result, Error};
use sys::cpuinfo;
use std::{fmt, result};

use self::Hardware::{
    RaspberryPi
};

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub hardware: Hardware,
    pub cpu: CPU,
    pub overvolted: bool
}

#[derive(Clone, Copy, Debug)]
pub enum Hardware {
    RaspberryPi(RaspberryModel, RaspberryRevision, RaspberryMemory, RaspberryMaker),
    Unknown
}

#[derive(Clone, Copy, Debug)]
pub enum CPU {
    BCM2708,
    BCM2709,
    Unknown
}

// Raspberry Pi

#[derive(Clone, Copy, Debug)]
pub enum RaspberryModel { A, B, BP, AP, CM, P0, P2, P3, UN }

impl<'a> From<&'a RaspberryModel> for &'static str {
    fn from(model: &'a RaspberryModel) -> Self {
        match *model {
            RaspberryModel::A => "Model A",
            RaspberryModel::B => "Model B",
            RaspberryModel::BP => "Model B+",
            RaspberryModel::AP => "Model A+",
            RaspberryModel::CM => "Compute Module",
            RaspberryModel::P0 => "Zero",
            RaspberryModel::P2 => "Model 2",
            RaspberryModel::P3 => "Model 3",
            RaspberryModel::UN => "Unknown"
        }
    }
}

impl fmt::Display for RaspberryModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s: &'static str = self.into();
        try!(write!(f, "{}", s));
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RaspberryRevision { R(u8), V1, V11, V12, V2, UN }

const RESPBERRY_PIN_TO_GPIO: [usize; 32] = [
    // Primary
    17, 18, 27, 22, 23, 24, 25, 4,
    // Additional
    2, 3, 8, 7, 10, 9, 11, 14, 15,
    // Pi B rev.2
    28, 29, 30, 31,
    // B+, Pi2
    5,  6, 13, 19, 26, 12, 16, 20, 21, 0, 1
];

impl<'a> From<&'a RaspberryRevision> for &'static str {
    fn from(rev: &'a RaspberryRevision) -> Self {
        match *rev {
            RaspberryRevision::R(r) => match r {
                0 => "R00",
                1 => "R01",
                2 => "R02",
                3 => "R03",
                4 => "R04",
                5 => "R05",
                _ => "Unknown"
            },
            RaspberryRevision::V1  => "V1",
            RaspberryRevision::V11 => "V1.1",
            RaspberryRevision::V12 => "V1.2",
            RaspberryRevision::V2  => "V2",
            RaspberryRevision::UN  => "Unknown"
        }
    }
}

impl fmt::Display for RaspberryRevision {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s: &'static str = self.into();
        try!(write!(f, "{}", s));
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RaspberryMemory(u32);

impl<'a, 'b> From<&'a RaspberryMemory> for &'static str {
    fn from(memory: &'a RaspberryMemory) -> Self {
        match memory.0 {
            0 => "Unknown",
            256 => "256M",
            512 => "512M",
            1024 => "1G",
            _ => "?"
        }
    }
}

impl fmt::Display for RaspberryMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s: &'static str = self.into();
        try!(write!(f, "{}", s));
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RaspberryMaker { Egoman, Sony, Qisda, Embest, Unknown }

impl<'a> From<&'a RaspberryMaker> for &'static str {
    fn from(maker: &'a RaspberryMaker) -> Self {
        match *maker {
            RaspberryMaker::Egoman  => "Egoman",
            RaspberryMaker::Sony    => "Sony",
            RaspberryMaker::Qisda   => "Qisda",
            RaspberryMaker::Embest   => "Embest",
            RaspberryMaker::Unknown => "Unknown"
        }
    }
}

impl fmt::Display for RaspberryMaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s: &'static str = self.into();
        try!(write!(f, "{}", s));
        Ok(())
    }
}

impl Board {
    pub fn pin_to_gpio(&self, pin: usize) -> Result<usize> {
        match self.hardware {
            Hardware::RaspberryPi(model, _, _,_) => {
                let max_pins = match model {
                    RaspberryModel::B | RaspberryModel::A => 21,
                    _ => 32
                };
                if pin >= max_pins {
                    return Err(Error::UnconnectedPin);
                };
                Ok(RESPBERRY_PIN_TO_GPIO[pin])
            },
            Hardware::Unknown => Err(Error::UnsupportedHardware)
        }
    }
}

pub fn board() -> Board {
    match cpuinfo() {
        Ok(cpuinfo) => {
            let cpu = match cpuinfo.0.get("Hardware") {
                Some(hardware) => match hardware.as_str() {
                    "BCM2708" => CPU::BCM2708,
                    "BCM2709" => CPU::BCM2709,
                    _ => CPU::Unknown
                },
                _ => CPU::Unknown
            };
            match cpuinfo.0.get("Revision") {
                Some(ref rev) => {
                    match u64::from_str_radix(rev, 16) {
                        Ok(revision) => {
                            if (revision &  (1 << 23)) != 0 {
                                let rev = (revision & (0x0F <<  0)) >> 0;
                                let model = match (revision & (0xFF <<  4)) >> 4 {
                                    0 => RaspberryModel::A,
                                    1 => RaspberryModel::B,
                                    2 => RaspberryModel::AP,
                                    3 => RaspberryModel::BP,
                                    4 => RaspberryModel::P2,
                                    6 => RaspberryModel::CM,
                                    8 => RaspberryModel::P3,
                                    9 => RaspberryModel::P0, // Zero
                                    _ => RaspberryModel::UN
                                };
                                let maker = match (revision & (0x0F << 16)) >> 16 {
                                    0 => RaspberryMaker::Sony,
                                    1 => RaspberryMaker::Egoman,
                                    2 | 4 => RaspberryMaker::Embest,
                                    _ => RaspberryMaker::Unknown
                                };
                                let memory = match (revision & (0x07 << 20)) >> 20 {
                                    0 => RaspberryMemory(256),
                                    1 => RaspberryMemory(512),
                                    2 => RaspberryMemory(1024),
                                    _ => RaspberryMemory(0)
                                };
                                Board { hardware: RaspberryPi(model, RaspberryRevision::R(rev as u8), memory, maker), cpu: cpu, overvolted: false }
                            } else {
                                // old way
                                let size = rev.len();
                                let overvolted  = size > 4;
                                let revision: &str = &rev[size-4..size];
                                let hardware = match revision.as_ref() {
                                    "0002" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V1, RaspberryMemory(256), RaspberryMaker::Egoman),
                                    "0003" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V11, RaspberryMemory(256), RaspberryMaker::Egoman),
                                    "0004" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMemory(256), RaspberryMaker::Sony),
                                    "0005" | "0009" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMemory(256), RaspberryMaker::Qisda),
                                    "0006" | "0007" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMemory(256), RaspberryMaker::Egoman),
                                    "0008" => RaspberryPi(RaspberryModel::A, RaspberryRevision::V2, RaspberryMemory(256), RaspberryMaker::Sony),
                                    "000d" | "000f" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMemory(512), RaspberryMaker::Egoman),
                                    "000e" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMemory(512), RaspberryMaker::Sony),
                                    "0010" => RaspberryPi(RaspberryModel::BP, RaspberryRevision::V12, RaspberryMemory(512), RaspberryMaker::Sony),
                                    "0011" | "0014" => RaspberryPi(RaspberryModel::CM, RaspberryRevision::V12, RaspberryMemory(512), RaspberryMaker::Sony),
                                    "0012" => RaspberryPi(RaspberryModel::AP, RaspberryRevision::V12, RaspberryMemory(256), RaspberryMaker::Sony),
                                    "0013" => RaspberryPi(RaspberryModel::BP, RaspberryRevision::V12, RaspberryMemory(512), RaspberryMaker::Egoman),
                                    "0015" => RaspberryPi(RaspberryModel::AP, RaspberryRevision::V11, RaspberryMemory(256), RaspberryMaker::Sony),
                                    _      => RaspberryPi(RaspberryModel::UN, RaspberryRevision::UN, RaspberryMemory(0), RaspberryMaker::Unknown)
                                };
                                Board { hardware: hardware, cpu: cpu, overvolted: overvolted }
                            }
                        },
                        Err(_) => Board { hardware: Hardware::Unknown, cpu: cpu, overvolted: false }
                    }
                },
                None => Board { hardware: Hardware::Unknown, cpu: cpu, overvolted: false }
            }
        },
        Err(_) => {
            Board { hardware: Hardware::Unknown, cpu: CPU::Unknown, overvolted: false }
        }
    }
}
