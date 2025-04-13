#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::Timer1Pwm;
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    ufmt::uwriteln!(&mut serial, "Rust: Mega2560 started!").ok();

    let mut trig = pins.a5.into_output();
    let echo = pins.a4.into_pull_up_input();

    let mut motor1 = pins.d3.into_output();
    let mut motor2 = pins.d4.into_output();
    let mut motor3 = pins.d5.into_output();
    let mut motor4 = pins.d6.into_output();

    let mut is_stopped = false;

    loop {
        let distance = measure_distance(&mut trig, &echo);

        ufmt::uwriteln!(&mut serial, "Distance: {} cm", distance).ok();

        if distance > 0 && distance < 20 && !is_stopped {
            stop_motors(&mut motor1, &mut motor2, &mut motor3, &mut motor4);
            is_stopped = true;
            ufmt::uwriteln!(&mut serial, "WARN: Obstacle!").ok();
        } else if distance > 25 && is_stopped {
            move_forward(&mut motor1, &mut motor2, &mut motor3, &mut motor4);
            is_stopped = false;
            ufmt::uwriteln!(&mut serial, "INFO: Path cleared").ok();
        }

        arduino_hal::delay_ms(500);
    }
}

fn measure_distance(
    trig: &mut impl OutputPin<Error = core::convert::Infallible>,
    echo: &arduino_hal::hal::port::Pin<arduino_hal::hal::port::mode::Input>,
) -> u16 {
    let timeout = 30000;

    trig.set_low().ok();
    arduino_hal::delay_us(2);
    trig.set_high().ok();
    arduino_hal::delay_us(10);
    trig.set_low().ok();

    // Wait for echo to go high
    let mut count = 0;
    while echo.is_low() {
        count += 1;
        if count > timeout {
            return 0;
        }
    }

    // Measure how long it's high
    count = 0;
    while echo.is_high() {
        count += 1;
        if count > timeout {
            break;
        }
    }

    // Convert to cm (speed of sound: ~0.034 cm/us round-trip)
    let duration = count as u32;
    ((duration * 34) / 2000) as u16
}

fn move_forward(
    m1: &mut impl OutputPin<Error = core::convert::Infallible>,
    m2: &mut impl OutputPin<Error = core::convert::Infallible>,
    m3: &mut impl OutputPin<Error = core::convert::Infallible>,
    m4: &mut impl OutputPin<Error = core::convert::Infallible>,
) {
    m1.set_high().ok();
    m2.set_high().ok();
    m3.set_high().ok();
    m4.set_high().ok();
}

fn stop_motors(
    m1: &mut impl OutputPin<Error = core::convert::Infallible>,
    m2: &mut impl OutputPin<Error = core::convert::Infallible>,
    m3: &mut impl OutputPin<Error = core::convert::Infallible>,
    m4: &mut impl OutputPin<Error = core::convert::Infallible>,
) {
    m1.set_low().ok();
    m2.set_low().ok();
    m3.set_low().ok();
    m4.set_low().ok();
}
