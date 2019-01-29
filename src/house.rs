use crate::sender::Sender;
use crate::blyss_sender::Status;
use crate::blyss_sender::BlyssMessage;
use crate::blyss_sender::Channel;
use crate::blyss_sender::SubChannel;
use std::str::FromStr;

pub struct MyHouse<T: Sender<Message=BlyssMessage>> {
    light: T
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LightStatus {
    ON,
    OFF,
}

impl Into<Status> for LightStatus {
    fn into(self) -> Status {
        match self {
            LightStatus::ON => Status::On,
            LightStatus::OFF => Status::Off
        }
    }
}

impl FromStr for LightStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "On" | "ON" | "on" => Ok(LightStatus::ON),
            "Off" | "OFF" | "off" => Ok(LightStatus::OFF),
            _ => Err("Unknown status")
        }
    }
}

pub trait House {
    fn light(&self, status: LightStatus);
}

impl<T: Sender<Message=BlyssMessage>> MyHouse<T> {
    pub fn new(light: T) -> Self {
        MyHouse { light }
    }
}

impl<T: Sender<Message=BlyssMessage>> House for MyHouse<T> {
    fn light(&self, status: LightStatus) {
        let message = BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, status.into());
        self.light.send(message);
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use crate::sender::mock::*;

    #[test]
    fn switch_on() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let house = MyHouse::new(sender);
        house.light(LightStatus::ON);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::On),
        ]));
    }

    #[test]
    fn switch_off() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let house = MyHouse::new(sender);
        house.light(LightStatus::OFF);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::Off),
        ]));
    }

    #[test]
    fn parse_status() {
        assert_eq!("On".parse::<LightStatus>().unwrap(), LightStatus::ON);
        assert_eq!("ON".parse::<LightStatus>().unwrap(), LightStatus::ON);
        assert_eq!("on".parse::<LightStatus>().unwrap(), LightStatus::ON);
        assert_eq!("Off".parse::<LightStatus>().unwrap(), LightStatus::OFF);
        assert_eq!("OFF".parse::<LightStatus>().unwrap(), LightStatus::OFF);
        assert_eq!("off".parse::<LightStatus>().unwrap(), LightStatus::OFF);
        assert_eq!("plop".parse::<LightStatus>().is_err(), true);
    }
}