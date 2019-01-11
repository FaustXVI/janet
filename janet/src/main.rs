extern crate sysfs_gpio;

use janet::led::blink;
use sysfs_gpio::Pin;
use sysfs_gpio::Direction;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let led = Pin::new(23);
    led.export().unwrap();
    if led.set_direction(Direction::Low).is_err() {
        sleep(Duration::from_millis(500));
        led.set_direction(Direction::Low).unwrap();
    };
    blink(&led, |d| sleep(d));
}
