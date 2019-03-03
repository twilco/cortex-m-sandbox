//! Initialization code
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler
extern crate stm32l4xx_hal as hal;

use hal::prelude::*;

pub use cortex_m_rt::entry;
pub use hal::{
   delay::Delay, prelude, rcc::CRRC
};

pub fn init() -> Delay {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    Delay::new(cp.SYST, clocks)
}
