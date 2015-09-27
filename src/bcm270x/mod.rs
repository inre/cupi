use {RegisterDesc};

mod gpio;

pub use self::gpio::{
    GPIO,
    PinOptions,
    PinInput,
    PinOutput
};

pub struct BCM2708;
pub struct BCM2709;

// Access from ARM Running Linux
pub trait PeripheralsBase {
    const PERI_BASE: usize;
    const GPIO_PADS: usize;
    const CLOCK_BASE: usize;
    const GPIO_BASE: usize;
    const GPIO_TIMER: usize;
    const GPIO_PWM: usize;
    const PAGE_SIZE: usize;
    const BLOCK_SIZE: usize;
}

impl PeripheralsBase for BCM2709 {
    const PERI_BASE: usize  = 0x3F000000;
    const GPIO_PADS: usize  = 0x00100000;
    const CLOCK_BASE: usize = 0x00101000;
    const GPIO_BASE: usize  = 0x00200000;
    const GPIO_TIMER: usize = 0x0000B000;
    const GPIO_PWM: usize   = 0x0020C000;
    const PAGE_SIZE: usize  = 4*1024;
    const BLOCK_SIZE: usize = 4*1024;
}

impl PeripheralsBase for BCM2708 {
    const PERI_BASE: usize  = 0x20000000;
    const GPIO_PADS: usize  = 0x00100000;
    const CLOCK_BASE: usize = 0x00101000;
    const GPIO_BASE: usize  = 0x00200000;
    const GPIO_TIMER: usize = 0x0000B000;
    const GPIO_PWM: usize   = 0x0020C000;
    const PAGE_SIZE: usize  = 4*1024;
    const BLOCK_SIZE: usize = 4*1024;
}

pub enum GPIORegister {
    GPIOFunctionSelect(usize),
    GPIOPinOutputSet(usize),
    GPIOPinOutputClear(usize),
    GPIOPinLevel(usize),
    GPIOPinEventDetectStatus(usize),
    GPIOPinRisingEdgeDetectEnable(usize),
    GPIOPinFallingEdgeDetectEnable(usize),
    GPIOPinHighDetectEnable(usize),
    GPIOPinLowDetectEnable(usize),
    GPIOPinAsyncRisingEdgeDetect(usize),
    GPIOPinAsyncFallingEdgeDetect(usize),
    GPIOPinPullUpDownEnable,
    GPIOPinPullUpDownEnableClock(usize),
}

impl RegisterDesc for GPIORegister {
    fn offset(&self) -> usize {
        match *self {
            GPIORegister::GPIOFunctionSelect(n)               => n,
            GPIORegister::GPIOPinOutputSet(n)                 => n + 7,
            GPIORegister::GPIOPinOutputClear(n)               => n + 10,
            GPIORegister::GPIOPinLevel(n)                     => n + 13,
            GPIORegister::GPIOPinEventDetectStatus(n)         => n + 16,
            GPIORegister::GPIOPinRisingEdgeDetectEnable(n)    => n + 19,
            GPIORegister::GPIOPinFallingEdgeDetectEnable(n)   => n + 22,
            GPIORegister::GPIOPinHighDetectEnable(n)          => n + 25,
            GPIORegister::GPIOPinLowDetectEnable(n)           => n + 28,
            GPIORegister::GPIOPinAsyncRisingEdgeDetect(n)     => n + 31,
            GPIORegister::GPIOPinAsyncFallingEdgeDetect(n)    => n + 34,
            GPIORegister::GPIOPinPullUpDownEnable             => 37,
            GPIORegister::GPIOPinPullUpDownEnableClock(n)     => n + 38
        }
    }
}

pub enum GPIOFunctionSelect {
    GPIOFunctionInput,
    GPIOFunctionOutput,
    GPIOAlternative(usize),
}

impl GPIOFunctionSelect {
    fn bits(&self) -> u32 {
        match *self {
            GPIOFunctionSelect::GPIOFunctionInput  => 0b000,
            GPIOFunctionSelect::GPIOFunctionOutput => 0b001,
            GPIOFunctionSelect::GPIOAlternative(n) => match n {
                0 => 0b100,
                1 => 0b101,
                2 => 0b110,
                3 => 0b111,
                4 => 0b011,
                5 => 0b010,
                _ => 0b000
            }
        }
    }
}

pub enum SystemTimerRegister {
    SystemTimerControlStatus,
    SystemTimerCounterLower,
    SystemTimerCounterHigher,
    SystemTimerCompare(usize),
}

impl RegisterDesc for SystemTimerRegister {
    fn offset(&self) -> usize {
        match *self {
            SystemTimerRegister::SystemTimerControlStatus  => 0,
            SystemTimerRegister::SystemTimerCounterLower   => 1,
            SystemTimerRegister::SystemTimerCounterHigher  => 2,
            SystemTimerRegister::SystemTimerCompare(n)     => n + 3,
        }
    }
}

pub enum PWMRegister {
    PWMControl,
    PWMStatus,
    PWMDMAConfiguration,
    PWMChannelRange(usize),
    PWMChannelData(usize),
    PWMFIFOInput,
}

impl RegisterDesc for PWMRegister {
    fn offset(&self) -> usize {
        match *self {
            PWMRegister::PWMControl               => 0,
            PWMRegister::PWMStatus                => 1,
            PWMRegister::PWMDMAConfiguration      => 2,
            PWMRegister::PWMChannelRange(n)       => n*3 + 3,
            PWMRegister::PWMChannelData(n)        => n*3 + 4,
            PWMRegister::PWMFIFOInput             => 5,
        }
    }
}

pub enum GPIOClockRegister {
    ClockControl(usize),
    ClockDivisors(usize),
}

impl RegisterDesc for GPIOClockRegister {
    fn offset(&self) -> usize {
        match *self {
            GPIOClockRegister::ClockControl(n)     => n*2 + 40,
            GPIOClockRegister::ClockDivisors(n)    => n*2 + 41,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PullUpDnControl {
    PullUp,
    PullDown,
    PullOff
}

impl PullUpDnControl {
    fn bcm270x_pud(&self) -> u32 {
        match *self {
            PullUpDnControl::PullOff  => 0b00,
            PullUpDnControl::PullDown => 0b01,
            PullUpDnControl::PullUp   => 0b10,
        }
    }
}
