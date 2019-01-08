use sysfs_gpio::Pin;
use crate::led::Switchable;

impl Switchable for Pin {
    fn switch_on(&self) -> () {
        self.set_value(1).unwrap()
    }

    fn switch_off(&self) -> () {
        self.set_value(0).unwrap()
    }
}