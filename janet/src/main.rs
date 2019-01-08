extern crate sysfs_gpio;

use janet::led::blink;

fn main() {
    blink().unwrap();
}
