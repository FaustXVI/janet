extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let my_led = Pin::new(2);
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Low)?;
        for _ in 0..100 {
            my_led.set_value(0)?;
            sleep(Duration::from_millis(500));
            my_led.set_value(1)?;
            sleep(Duration::from_millis(500));
        }
        my_led.set_value(0)?;
        Ok(())
    }).unwrap();
}




#[cfg(test)]
mod should {
    #[test]
    fn it_works() {
        assert_eq!(true,true);
    }
}

