#!/bin/sh
cargo build -- --debug
elf2uf2-rs -d target/thumbv6m-none-eabi/debug/rpi_pico_hal_learning
