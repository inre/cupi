use std::sync::{Arc, Mutex};
use map::{SystemMemory, MemoryMap};
use {Result, Error, Logic, DigitalLogic, DigitalWrite, DigitalRead, CPU, RegisterOperations, delay_hard};
use super::{PeripheralsBase, BCM2708, BCM2709, GPIORegister, GPIOFunctionSelect, PullUpDnControl};

pub struct GPIOBase(Mutex<MemoryMap>);

#[derive(Clone)]
pub struct GPIO {
    gpio_base: Arc<GPIOBase>
}

impl GPIO {
    pub unsafe fn new(cpu: CPU) -> Result<GPIO> {
        // Detect CPU
        let ptr = match cpu {
            CPU::BCM2708 => BCM2708::PERI_BASE + BCM2708::GPIO_BASE,
            CPU::BCM2709 => BCM2709::PERI_BASE + BCM2709::GPIO_BASE,
            CPU::Unknown => return Err(Error::UnsupportedHardware),
        };
        let sys_mem = try!(SystemMemory::new());
        let gpio_base = try!(sys_mem.mmap(ptr));
        Ok(GPIO {
            gpio_base: Arc::new(GPIOBase(Mutex::new(gpio_base)))
        })
    }

    pub unsafe fn pin(&self, pin: usize) -> PinOptions {
        PinOptions {
            gpio_base: self.gpio_base.clone(),
            pin: pin,
            pull_ctrl: Some(PullUpDnControl::PullOff),
            default_value: 0
        }
    }
}

#[derive(Clone)]
pub struct PinOptions {
    gpio_base: Arc<GPIOBase>,
    pin: usize,
    pull_ctrl: Option<PullUpDnControl>,
    default_value: usize
}

impl PinOptions {
    pub fn pin(&mut self, pin: usize) -> &mut PinOptions {
        self.pin = pin; self
    }

    pub fn pull_up(&mut self) -> &mut PinOptions {
        self.pull_ctrl = Some(PullUpDnControl::PullUp); self
    }

    pub fn pull_down(&mut self) -> &mut PinOptions {
        self.pull_ctrl = Some(PullUpDnControl::PullDown); self
    }

    pub fn pull_off(&mut self) -> &mut PinOptions {
        self.pull_ctrl = Some(PullUpDnControl::PullOff); self
    }

    pub fn set(&mut self, value: usize) -> &mut PinOptions {
        self.default_value = value; self
    }

    pub fn input(&self) -> PinInput {
        {// we should unlock mutex before copy
            let gpio_base = match self.gpio_base.0.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            let func_reg = gpio_base.register(GPIORegister::GPIOFunctionSelect(self.pin/10));
            let shift = (self.pin % 10) * 3;
            unsafe { func_reg.bitand(!(0b111 << shift)); }
        }
        let pin = PinInput { gpio_base: self.gpio_base.clone(), pin: self.pin };
        match self.pull_ctrl {
            Some(ctrl) => pin.pull_mode(ctrl),
            None => ()
        }
        pin
    }

    pub fn output(&self) -> PinOutput {
        {
            // set pin as input first
            let gpio_base = match self.gpio_base.0.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            let func_reg = gpio_base.register(GPIORegister::GPIOFunctionSelect(self.pin/10));
            let func_shift = (self.pin % 10) * 3;
            // Reset port functions
            unsafe { func_reg.bitand(!(0b111 << func_shift)); }

            // Set default value
            let output_reg = match self.default_value {
                0 => gpio_base.register(GPIORegister::GPIOPinOutputClear(self.pin/32)),
                _ => gpio_base.register(GPIORegister::GPIOPinOutputSet(self.pin/32))
            };
            let output_shift = self.pin % 32;
            unsafe { output_reg.write(1 << output_shift); }

            // Make output
            let bits = GPIOFunctionSelect::GPIOFunctionOutput.bits();
            unsafe { func_reg.bitor(bits << func_shift); }
        }
        PinOutput { gpio_base: self.gpio_base.clone(), pin: self.pin }
    }
}

#[derive(Clone)]
pub struct PinInput {
    gpio_base: Arc<GPIOBase>,
    pin: usize
}

impl PinInput {
    pub fn read(&self) -> Logic {
        let gpio_base = match self.gpio_base.0.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let level_reg = gpio_base.register(GPIORegister::GPIOPinLevel(self.pin/32));
        let shift = self.pin % 32;
        unsafe {
            match level_reg.read() & (1 << shift) {
                0 => Logic::Low,
                _ => Logic::High
            }
        }
    }

    pub fn pull_up(&self) {
        self.pull_mode(PullUpDnControl::PullUp);
    }

    pub fn pull_down(&self) {
        self.pull_mode(PullUpDnControl::PullDown);
    }

    pub fn pull_off(&self) {
        self.pull_mode(PullUpDnControl::PullOff);
    }

    fn pull_mode(&self, mode: PullUpDnControl) {
        let gpio_base = match self.gpio_base.0.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let enable_reg = gpio_base.register(GPIORegister::GPIOPinPullUpDownEnable);
        let clock_reg = gpio_base.register(GPIORegister::GPIOPinPullUpDownEnableClock(self.pin/32));
        let shift = self.pin % 32;
        unsafe {
            enable_reg.write(mode.bcm270x_pud()); delay_hard(5);
            clock_reg.write(1 << shift); delay_hard(5);
            enable_reg.write(0); delay_hard(5);
            clock_reg.write(0); delay_hard(5);
        }
    }
}

impl DigitalRead for PinInput {
    fn digital_read(&mut self) -> Result<Logic> {
      Ok(self.read())
    }
}

#[derive(Clone)]
pub struct PinOutput {
    gpio_base: Arc<GPIOBase>,
    pin: usize
}

impl PinOutput {
    // FIXME: modify regs without mutex lock
    pub fn write<L: DigitalLogic>(&self, value: L) {
        let gpio_base = match self.gpio_base.0.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let output_reg = match value.logic_level() {
            Logic::Low  => gpio_base.register(GPIORegister::GPIOPinOutputClear(self.pin/32)),
            Logic::High => gpio_base.register(GPIORegister::GPIOPinOutputSet(self.pin/32))
        };
        let shift = self.pin % 32;
        let bits  = GPIOFunctionSelect::GPIOFunctionOutput.bits();
        unsafe { output_reg.write(bits << shift); }
    }
}

impl DigitalWrite for PinOutput {
  fn digital_write<L: DigitalLogic>(&mut self, level: L) -> Result<()> {
      self.write(level);
      Ok(())
  }
}

impl Drop for PinOutput {
    fn drop(&mut self) {
        let gpio_base = match self.gpio_base.0.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let func_reg = gpio_base.register(GPIORegister::GPIOFunctionSelect(self.pin/10));
        let shift = (self.pin % 10) * 3;
        unsafe { func_reg.bitand(!(0b111 << shift)); }
    }
}
