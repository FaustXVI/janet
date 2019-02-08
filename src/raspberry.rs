use crate::house::House;
use sysfs_gpio::Pin;
use sysfs_gpio::Direction;
use std::time::Duration;
use crate::blyss_sender::MessageSender;
use crate::blyss::Blyss;
use crate::house::MyHouse;
use std::thread::sleep;
use crate::pin::DigitalOutput;
use crate::replay::Replay;
use crate::house::CycleGenerator;
use crate::blyss_sender::BlyssMessage;

pub fn create_house() -> impl House {
    let pin = Pin::new(23);
    pin.export().unwrap();
    if pin.set_direction(Direction::Low).is_err() {
        sleep(Duration::from_millis(500));
        pin.set_direction(Direction::Low).unwrap();
    };
    let sender = MessageSender::new(Blyss::new(pin.clone()));
    let replayer = Replay::new(pin);
    let iter = BlyssMessage::rolling_codes();
    MyHouse::new(sender, replayer, CycleGenerator::new(
iter
    ))
}

pub fn create_fake_house() -> impl House {
    let pin = FakeDigitalOutput::new(23);
    let sender = MessageSender::new(Blyss::new(pin.clone()));
    let replayer = Replay::new(pin);
    let iter = BlyssMessage::rolling_codes();
    MyHouse::new(sender, replayer, CycleGenerator::new(iter))
}

#[derive(Debug,Clone)]
pub struct FakeDigitalOutput {
    pin: usize
}

impl FakeDigitalOutput {
    fn new(pin: usize) -> Self {
        FakeDigitalOutput { pin }
    }
}

impl DigitalOutput for FakeDigitalOutput {
    fn high_during(&self, duration: Duration) -> () {
        println!("high for {:?} on {}", duration, self.pin)
    }

    fn low_during(&self, duration: Duration) -> () {
        println!("low for {:?} on {}", duration, self.pin)
    }
}