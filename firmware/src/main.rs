#![no_std]
#![no_main]
#![feature(const_option)]
#![feature(generic_const_exprs)]

// Frustratingly required for `bytes` :(
// https://github.com/tokio-rs/bytes/issues/479
extern crate alloc;

// Don't allow any heap allocations.
struct NoAlloc;
unsafe impl GlobalAlloc for NoAlloc {
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
    }
}

#[global_allocator]
static GLOBAL: NoAlloc = NoAlloc;

use arduino_hal::{default_serial, hal::adc};
use core::{alloc::GlobalAlloc, ptr::null_mut};

use panic_halt as _;

mod dial;
use dial::*;
mod serialisable;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 9600);
    let mut adc = arduino_hal::adc::Adc::new(dp.ADC, Default::default());

    let mut led = pins.d13.into_output();

    let adc6 = adc::channel::ADC6;
    let adc5 = pins.a5.into_analog_input(&mut adc);

    let dials = Dials::new([
        Dial::new(adc6.into_channel()),
        Dial::new(adc5.into_channel()),
    ]);

    loop {
        led.toggle();
        //let percentages = dials.snapshot(&mut adc).void_unwrap();
        //serial_write_u16::<Void>(&mut serial, percentages[0].to_be().to_bits()).unwrap();
        arduino_hal::delay_ms(1000);
    }
}
