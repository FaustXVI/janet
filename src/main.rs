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
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status = from_command_line(args);
    match status {
        Some(status) => {
            let pin = Pin::new(23);
            pin.export().unwrap();
            if pin.set_direction(Direction::Low).is_err() {
                sleep(Duration::from_millis(500));
                pin.set_direction(Direction::Low).unwrap();
            };
            let sender = MessageSender::new(Blyss::new(pin));
            let message = Message::new(0x7057, Channel::ChannelC, SubChannel::Channel1, status);
            sender.send(&message);
        }
        None => println!("usage : janet [On|Off]")
    }
}

fn from_command_line(arg: Vec<String>) -> Option<Status> {
    let a = arg.get(0).unwrap();
    parse(a.clone())
}

fn parse(arg: String) -> Option<Status> {
    match arg.as_str() {
        "On" => Some(Status::On),
        "Off" => Some(Status::Off),
        _ => None
    }
}