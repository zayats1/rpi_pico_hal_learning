#![no_std]
#![no_main]
#![deny(unsafe_code)]
#![allow(unused)]

// for writing
use core::fmt::Write;

use cortex_m::prelude::*;
// Traits for converting integers to amounts of time
use fugit::ExtU32;
use nb::block;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;
use rp2040_hal::Clock;
// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;
use rp_pico::hal::pac;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use crate::servo::Servo;

mod servo;

const DELAY_TIME: u32 = 4;

#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Configure the Timer peripheral in count-down mode
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut count_down = timer.count_down();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // UART TX (characters sent from pico) on pin 1 (GPIO0) and RX (on pin 2 (GPIO1)
    let uart_pins = (
        pins.gpio0.into_mode::<hal::gpio::FunctionUart>(),
        pins.gpio1.into_mode::<hal::gpio::FunctionUart>(),
    );

    // Create a UART driver
    let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            hal::uart::common_configs::_9600_8_N_1,
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.set_div_int(20u8); // 50 hz   1/50= 0.020 s
    pwm.enable();

    // Output channel B on PWM0 to the GPIO1 pin
    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio3);

    let mut delay = |ms: u32| {
        count_down.start(ms.millis());
        let _ = block!(count_down.wait());
    };

    // todo calculate duty_on_zero(depending on period)
    let mut my_servo = Servo::new(channel, 1620);
    // Enable ADC1
    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);

    // Configure GPIO26 as an ADC input
    let mut adc_pin_0 = pins.gpio26.into_floating_input();

    loop {
        for deg in 0..180 {
            my_servo.set_angle(deg);
            delay(DELAY_TIME);
        }
        for deg in (0..180).rev() {
            my_servo.set_angle(deg);
            delay(DELAY_TIME);
        }
    }
}
