use linux_embedded_hal::{I2cdev, Delay};
use pwm_pca9685::{Channel, Pca9685};
use embedded_hal::blocking::delay::DelayMs;

// define the 2 channels we are going to use
const LEFT: Channel = Channel::C0;
const RIGHT: Channel = Channel::C12;

fn main() {
    // set up i2c device and PWM contoller interface
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = (false, false, false, false, false, false);
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // see readme on these magic numbers
    let servo_min = 122;
    let servo_max = 614;
    let num_steps = 60;
    let servo_steps = (servo_max as f32 - servo_min as f32) / num_steps as f32;

    pwm.enable().unwrap();
    pwm.set_prescale(100).unwrap(); // scale 100 means 60Hz
    let mut delay = Delay;

    // move between the two extremes
    println!("Going back and forth between the 2 extremes (going to take 7s)");
    pwm.set_channel_on_off(LEFT, 0, servo_min).unwrap();
    pwm.set_channel_on_off(RIGHT, 0, servo_max).unwrap();
    delay.delay_ms(3500u16);
    pwm.set_channel_on_off(LEFT, 0, servo_max).unwrap();
    pwm.set_channel_on_off(RIGHT, 0, servo_min).unwrap();
    delay.delay_ms(3500u16);

    // step through the range
    println!("Stepping through the range (1/5s per step)");
    for i in 1..num_steps {
        let curr_step = i as f32 * servo_steps;
        let value_left = (servo_max as f32 - curr_step) as u16;
        let value_right = (servo_min as f32 + curr_step) as u16;
        println!("Going to try: [{} on #0] and [{} on #12]", value_left, value_right);
        pwm.set_channel_on_off(LEFT, 0, value_left).unwrap();
        pwm.set_channel_on_off(RIGHT, 0, value_right).unwrap();
        delay.delay_ms(200u16);
    }

    // release i2c device
    let _dev = pwm.destroy();
}