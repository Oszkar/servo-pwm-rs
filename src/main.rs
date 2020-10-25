use linux_embedded_hal::{I2cdev, Delay};
use pwm_pca9685::{Channel, Pca9685};
use embedded_hal::blocking::delay::DelayMs;

fn main() {
    // set up i2c device and PWM contoller interface
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = (false, false, false, false, false, false);
    let mut pwm = Pca9685::new(dev, address).unwrap();

    let servo_min = 122;
    let servo_max = 614;
    let num_steps = 50;
    let servo_steps = (servo_max as f32 - servo_min as f32) / num_steps as f32;
    
    pwm.enable().unwrap();
    pwm.set_prescale(100).unwrap();
    let mut delay = Delay;

    // move between the two extremes
    println!("Going back and forth between the 2 extremes (going to take 6s)");
    pwm.set_channel_on_off(Channel::C12, 0, servo_max).unwrap();
    delay.delay_ms(3000u16);
    pwm.set_channel_on_off(Channel::C12, 0, servo_min).unwrap();
    delay.delay_ms(3000u16);

    // step through the range
    println!("Stepping through the range (1/4s per step)");
    for i in 1..num_steps {
        let value = (servo_min as f32 + i as f32 * servo_steps) as u16;
        println!("Going to try: {}", value);
        pwm.set_channel_on_off(Channel::C12, 0, value).unwrap();
        delay.delay_ms(250u16);
    }
    
    // release i2c device
    let _dev = pwm.destroy();
}