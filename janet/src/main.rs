extern crate sysfs_gpio;

use janet::led::blink;
use sysfs_gpio::Pin;
use sysfs_gpio::Direction;
use std::thread::sleep;

fn main() {
    let led = Pin::new(23);
    led.export().unwrap();
    led.set_direction(Direction::Low).unwrap();
    blink(&led, |d| sleep(d));
}
