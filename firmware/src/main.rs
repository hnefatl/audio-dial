#![no_std]
#![no_main]
#![feature(const_option)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

use panic_halt as _;

use arduino_hal::{default_serial, hal::adc};
use serialisable::SerialWriteDevice;

mod dial;
use dial::*;
mod serialisable;
use lib::data::WireFormat;

const REFRESH_RATE_MS: u16 = 100;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::adc::Adc::new(dp.ADC, Default::default());

    let dials = Dials::new([
        Dial::new(pins.a1.into_analog_input(&mut adc).into_channel()),
        Dial::new(pins.a2.into_analog_input(&mut adc).into_channel()),
        Dial::new(pins.a3.into_analog_input(&mut adc).into_channel()),
        Dial::new(pins.a4.into_analog_input(&mut adc).into_channel()),
        Dial::new(pins.a5.into_analog_input(&mut adc).into_channel()),
        Dial::new(adc::channel::ADC6.into_channel()),
        Dial::new(adc::channel::ADC7.into_channel()),
    ]);

    loop {
        let snapshot = dials.snapshot(&mut adc);
        let encoded: WireFormat<_> = snapshot.into();

        if serial.serialize(&encoded).is_err() {
            // Do nothing, hard to surface errors atm since serial is only used for data.
        }

        arduino_hal::delay_ms(REFRESH_RATE_MS);
    }
}
