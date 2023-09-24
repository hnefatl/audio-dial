#![no_std]
#![no_main]
#![feature(const_option)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

use arduino_hal::{default_serial, hal::adc};

use panic_halt as _;

mod dial;
use dial::*;
use ufmt::uWrite;
mod serialisable;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::adc::Adc::new(dp.ADC, Default::default());

    let mut led = pins.d13.into_output();

    let adc6 = adc::channel::ADC6;
    let adc5 = pins.a5.into_analog_input(&mut adc);

    //let dials = Dials::new([
    //    Dial::new(adc6.into_channel()),
    //    Dial::new(adc5.into_channel()),
    //]);

    let dial = Dial::new(adc6.into_channel());
    loop {
        let dial_value = dial.read(&mut adc);

        let mut mask = 1000u16;
        while mask > 0 {
            let digit = (dial_value / mask) as u32 % 10;
            serial
                .write_char(char::from_digit(digit, 10).unwrap())
                .unwrap();
            mask /= 10;
        }
        serial.write_char('\n').unwrap();
        serial.flush();

        arduino_hal::delay_ms(1000);
    }
}
