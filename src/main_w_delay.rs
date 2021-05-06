#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};
// use ruduino::cores::atmega328p::{port};
// use avr_delay::delay_ms;

/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - an i32, the number of times to cycle the loop.
pub fn delay(count: u32) {
    // Our asm busy-wait takes a 16 bit word as an argument,
    // so the max number of loops is 2^16
    let outer_count = count / 65536;
    let last_count = ((count % 65536)+1) as u16;
    for _ in 0..outer_count {
        // Each loop through should be 4 cycles.
        unsafe {llvm_asm!("1: sbiw $0,1
                      brne 1b"
                     :
                     : "w" (0)
                     :
                     :)}
    }
    unsafe {llvm_asm!("1: sbiw $0,1
                      brne 1b"
                 :
                 : "w" (last_count)
                 :
                 :)}
}

///delay for N miliseconds
/// # Arguments
/// * 'ms' - an u32, number of milliseconds to busy-wait
pub fn delay_ms(ms: u32) {
    // microseconds
    let us = ms * 1000;
    delay_us(us);
}

///delay for N microseconds
/// # Arguments
/// * 'ms' - an u32, number of microseconds to busy-wait
pub fn delay_us(us: u32) {
    // picoseconds
    let ps = us * 1000;
    let ps_lp = 1000000000 / (16000000 / 4); // hard coded avr_config::CPU_FREQUENCY_HZ here to simplify
    let loops = (ps / ps_lp) as u32;
    delay(loops);
}

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
