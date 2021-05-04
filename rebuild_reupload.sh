#!/bin/bash

set -e

export AVR_CPU_FREQUENCY_HZ=16000000

echo "cargo +nightly-2021-01-07 build -Z build-std=core --target avr-atmega328p.json --release --verbose"
cargo +nightly-2021-01-07 build -Z build-std=core --target avr-atmega328p.json --release --verbose
echo "avrdude -patmega328p -carduino -P /dev/ttyUSB0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/blink.elf:e"
avrdude -patmega328p -carduino -P /dev/ttyUSB0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/blink.elf:e
