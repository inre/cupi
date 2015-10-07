use {Result, Error};
use sys::{cpuinfo, memory};
use std::{fmt, result};

use self::Hardware::{
    RaspberryPi
};

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub hardware: Hardware,
    pub cpu: CPU,
    pub memory: Option<u32>,
    pub overvolted: bool
}

#[derive(Clone, Copy, Debug)]
pub enum Hardware {
    RaspberryPi(RaspberryModel, RaspberryRevision, RaspberryMaker),
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
pub enum RaspberryModel { A, B, BP, AP, CM, B2, UN }

impl<'a> From<&'a RaspberryModel> for &'static str {
    fn from(model: &'a RaspberryModel) -> Self {
        match *model {
            RaspberryModel::A => "Model A",
            RaspberryModel::B => "Model B",
            RaspberryModel::BP => "Model B+",
            RaspberryModel::AP => "Model A+",
            RaspberryModel::CM => "Compute Module",
            RaspberryModel::B2 => "Model 2B",
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
pub enum RaspberryRevision { V1, V11, V12, V2, UN }

impl RaspberryRevision {
    const PIN_TO_GPIO: [usize; 32] = [
        // Primary
        17, 18, 27, 22, 23, 24, 25, 4,
        // Additional
        2, 3, 8, 7, 10, 9, 11, 14, 15,
        // Pi B rev.2
        28, 29, 30, 31,
        // B+, Pi2
        5,  6, 13, 19, 26, 12, 16, 20, 21, 0, 1
    ];
}

impl<'a> From<&'a RaspberryRevision> for &'static str {
    fn from(rev: &'a RaspberryRevision) -> Self {
        match *rev {
            RaspberryRevision::V1  => "1",
            RaspberryRevision::V11 => "1.1",
            RaspberryRevision::V12 => "1.2",
            RaspberryRevision::V2  => "2",
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
pub enum RaspberryMaker { Egoman, Sony, Qisda, MBest, Unknown }

impl<'a> From<&'a RaspberryMaker> for &'static str {
    fn from(maker: &'a RaspberryMaker) -> Self {
        match *maker {
            RaspberryMaker::Egoman  => "Egoman",
            RaspberryMaker::Sony    => "Sony",
            RaspberryMaker::Qisda   => "Qisda",
            RaspberryMaker::MBest   => "MBest",
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
            Hardware::RaspberryPi(model, _, _) => {
                let max_pins = match model {
                    RaspberryModel::BP |
                    RaspberryModel::AP |
                    RaspberryModel::B2 |
                    RaspberryModel::CM => 32,
                    _ => 21
                };
                if pin >= max_pins {
                    return Err(Error::UnconnectedPin);
                };
                Ok(RaspberryRevision::PIN_TO_GPIO[pin])
            },
            Hardware::Unknown => Err(Error::UnsupportedHardware)
        }
    }
}

pub fn board() -> Board {
    let memory = match memory() {
        Ok(m)  => Some(m.total),
        Err(_) => None
    };

    match cpuinfo() {
        Ok(cpuinfo) => {
            let rev = cpuinfo.0.get("Revision");
            match cpuinfo.0.get("Hardware") {
                Some(hardware) => match hardware.as_str() {
                    "BCM2708" => {
                        match rev {
                            Some(ref rev) => {
                                let size = rev.len();
                                let overvolted  = size > 4;
                                let revision: &str = &rev[size-4..size];
                                let hardware = match revision.as_ref() {
                                    "0002" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V1, RaspberryMaker::Egoman),
                                    "0003" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V11, RaspberryMaker::Egoman),
                                    "0004" | "000e" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMaker::Sony),
                                    "0005" | "0009" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMaker::Qisda),
                                    "0006" | "0007" | "000d" | "000f" => RaspberryPi(RaspberryModel::B, RaspberryRevision::V2, RaspberryMaker::Egoman),
                                    "0008" => RaspberryPi(RaspberryModel::A, RaspberryRevision::V2, RaspberryMaker::Sony),
                                    "0010" => RaspberryPi(RaspberryModel::BP, RaspberryRevision::V12, RaspberryMaker::Sony),
                                    "0011" | "0014" => RaspberryPi(RaspberryModel::CM, RaspberryRevision::V12, RaspberryMaker::Sony),
                                    "0012" => RaspberryPi(RaspberryModel::AP, RaspberryRevision::V12, RaspberryMaker::Sony),
                                    "0013" => RaspberryPi(RaspberryModel::BP, RaspberryRevision::V12, RaspberryMaker::MBest),
                                    _      => RaspberryPi(RaspberryModel::UN, RaspberryRevision::UN, RaspberryMaker::Unknown)
                                };
                                Board { hardware: hardware, memory: memory, cpu: CPU::BCM2708, overvolted: overvolted }
                            },
                            None => Board { hardware: Hardware::Unknown, memory: memory, cpu: CPU::BCM2708, overvolted: false }
                        }
                    },
                    "BCM2709" => Board { hardware: RaspberryPi(RaspberryModel::B2, RaspberryRevision::V11, RaspberryMaker::Sony), memory: memory, cpu: CPU::BCM2709, overvolted: false },
                    _         => Board { hardware: Hardware::Unknown, memory: memory, cpu: CPU::Unknown, overvolted: false },
                },
                None => Board { hardware: Hardware::Unknown, memory: memory, cpu: CPU::Unknown, overvolted: false }
            }
        },
        Err(_) => {
            Board { hardware: Hardware::Unknown, memory: memory, cpu: CPU::Unknown, overvolted: false }
        }
    }
}
