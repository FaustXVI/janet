extern crate sysfs_gpio;

use sysfs_gpio::Pin;
use sysfs_gpio::Direction;
use std::thread::sleep;
use std::time::Duration;
use janet::message_sender::MessageSender;
use janet::blyss::Blyss;
use janet::message_sender::Message;
use janet::message_sender::Channel;
use janet::message_sender::SubChannel;
use janet::message_sender::Status;

fn main() {
    let pin = Pin::new(23);
    pin.export().unwrap();
    if pin.set_direction(Direction::Low).is_err() {
        sleep(Duration::from_millis(500));
        pin.set_direction(Direction::Low).unwrap();
    };
    let sender = MessageSender::new(Blyss::new(Box::new(pin)));
    let switch_on = Message::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::On);
    let switch_off = Message::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::Off);
    for _ in 0..10 {
        for _ in 0..13 {
            sender.send(&switch_on);
        }
        sleep(Duration::from_secs(5));
        for _ in 0..13 {
            sender.send(&switch_off);
        }
        sleep(Duration::from_secs(5));
    };
}
