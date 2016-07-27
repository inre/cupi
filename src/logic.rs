use std;
use std::fmt::{Display, Formatter};
use Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Logic {
  High,
  Low
}

impl Logic {
    pub fn inverse(self) -> Logic {
        match self {
            Logic::High => Logic::Low,
            Logic::Low  => Logic::High
        }
    }
}

impl Into<usize> for Logic {
    fn into(self) -> usize {
        match self {
            Logic::High => 1,
            Logic::Low  => 0
        }
    }
}

impl Display for Logic {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        let v: usize = (*self).into();
        write!(f, "{}", v)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Logic3 {
  High,
  Low,
  Z
}

pub trait DigitalLogic {
  fn logic_level(&self) -> Logic;
}

impl DigitalLogic for i32 {
  fn logic_level(&self) -> Logic {
    match *self { 0 => Logic::Low, _ => Logic::High }
  }
}

impl DigitalLogic for u32 {
  fn logic_level(&self) -> Logic {
    match *self { 0 => Logic::Low, _ => Logic::High }
  }
}

impl DigitalLogic for isize {
  fn logic_level(&self) -> Logic {
    match *self { 0 => Logic::Low, _ => Logic::High }
  }
}

impl DigitalLogic for usize {
  fn logic_level(&self) -> Logic {
    match *self { 0 => Logic::Low, _ => Logic::High }
  }
}

impl DigitalLogic for Logic {
  fn logic_level(&self) -> Logic {
    *self
  }
}

pub trait DigitalRead {
  fn digital_read(&mut self) -> Result<Logic>;

  fn get(&mut self) -> Result<Logic> {
    self.digital_read()
  }

  fn is_high(&mut self) -> Result<bool> {
      match try!(self.digital_read()) {
          Logic::High => Ok(true),
          Logic::Low => Ok(false)
      }
  }

  fn is_low(&mut self) -> Result<bool> {
      match try!(self.digital_read()) {
          Logic::High => Ok(false),
          Logic::Low => Ok(true)
      }
  }
}

pub trait DigitalWrite {
  fn digital_write<L: DigitalLogic>(&mut self, level: L) -> Result<()>;

  fn set<L: DigitalLogic>(&mut self, level: L) -> Result<()> {
    self.digital_write(level)
  }

  fn high(&mut self) -> Result<()> {
    self.digital_write(Logic::High)
  }

  fn low(&mut self) -> Result<()> {
    self.digital_write(Logic::Low)
  }
}

pub trait AnalogRead {
  fn analog_read(&self) -> usize;
}

pub trait AnalogWrite {
  fn analog_write(&self, value: usize);
}

#[cfg(test)]
mod test {
    use super::Logic;

    #[test]
    fn logic_inverse() {
        let low = Logic::Low;
        let high = Logic::High;
        assert_eq!(low.inverse(), high);
        assert_eq!(low, high.inverse());
    }
}
