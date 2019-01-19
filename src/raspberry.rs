use crate::house::House;
use sysfs_gpio::Pin;
use sysfs_gpio::Direction;
use std::time::Duration;
use crate::blyss_sender::MessageSender;
use crate::blyss::Blyss;
use crate::house::MyHouse;
use std::thread::sleep;

pub fn create_house() -> Box<House> {
    let pin = Pin::new(23);
    pin.export().unwrap();
    if pin.set_direction(Direction::Low).is_err() {
        sleep(Duration::from_millis(500));
        pin.set_direction(Direction::Low).unwrap();
    };
    let sender = MessageSender::new(Blyss::new(pin));
    Box::new(MyHouse::new(sender))
}