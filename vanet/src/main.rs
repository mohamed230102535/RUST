#![no_std]
#![no_main]
use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Motor 1 (PWM = D11, Dir1 = D12, Dir2 = D13)
    let mut motor1_pwm = pins.d11.into_output().into_pwm();
    let motor1_dir1 = pins.d12.into_output();
    let motor1_dir2 = pins.d13.into_output();

    // Motor 2 (PWM = D10, Dir1 = D9, Dir2 = D8)
    let mut motor2_pwm = pins.d10.into_output().into_pwm();
    let motor2_dir1 = pins.d9.into_output();
    let motor2_dir2 = pins.d8.into_output();

    // Initialize PWM (8-bit, ~500Hz)
    motor1_pwm.set_fast_pwm(500);
    motor2_pwm.set_fast_pwm(500);
    motor1_pwm.enable();
    motor2_pwm.enable();

    loop {
        // Motor 1: Forward at 50% speed
        motor1_dir1.set_high();
        motor1_dir2.set_low();
        motor1_pwm.set_duty(128);  // 50% duty (0-255)

        // Motor 2: Reverse at 75% speed
        motor2_dir1.set_low();
        motor2_dir2.set_high();
        motor2_pwm.set_duty(192);  // 75% duty

        arduino_hal::delay_ms(2000);

        // Stop both motors
        motor1_dir1.set_low();
        motor1_dir2.set_low();
        motor2_dir1.set_low();
        motor2_dir2.set_low();
        motor1_pwm.set_duty(0);
        motor2_pwm.set_duty(0);

        arduino_hal::delay_ms(1000);
    }
}