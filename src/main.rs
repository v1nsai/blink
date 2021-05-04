#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};
// use ruduino::cores::atmega328p::{port};
use avr_delay::delay_ms;

#[no_mangle]
pub extern fn main() {
    port::B5::set_output();
    
    loop {
        port::B5::set_high();
        
        delay_ms(1000);

        port::B5::set_low();

        delay_ms(1000);
    }
}
