use crate::sender::Sender;
use crate::blyss_sender::Status;
use crate::blyss_sender::BlyssMessage;
use crate::blyss_sender::Channel;
use crate::blyss_sender::SubChannel;
use std::str::FromStr;
use std::sync::Mutex;
use std::cell::RefCell;
use crate::dio::DioMessage;
use crate::dio::DIO_PROTOCOL;
use crate::dio;
use crate::dooya::DOOYA_PROTOCOL;
use crate::dooya;
use crate::radio::Radio;

pub struct MyHouse<T, R, G>
    where T: Sender<Message=BlyssMessage>,
          R: Radio,
          G: Generator
{
    light: T,
    radio: R,
    generator: G,
}

pub trait Generator {
    fn gen(&self) -> (u8, u8);
}

pub struct CycleGenerator {
    iterator: Box<Mutex<RefCell<Iterator<Item=(u8, u8)> + Send>>>
}


impl CycleGenerator {
    pub fn new<Iter>(iterator: Iter) -> Self
        where Iter: Iterator<Item=(u8, u8)> + 'static + Send + Clone {
        CycleGenerator { iterator: Box::new(Mutex::new(RefCell::new(iterator.cycle()))) }
    }
}

impl Generator for CycleGenerator {
    fn gen(&self) -> (u8, u8) {
        self.iterator.lock().unwrap().borrow_mut().next().unwrap()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Room {
    Kitchen,
    LivingRoom,
}

impl FromStr for Room {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kitchen" | "KITCHEN" | "kitchen" => Ok(Room::Kitchen),
            "LivingRoom" | "LIVING_ROOM" | "living_room" | "Livingroom" => Ok(Room::LivingRoom),
            _ => Err("Unknown room")
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BlindStatus {
    UP,
    DOWN,
}

impl FromStr for BlindStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Down" | "DOWN" | "down" => Ok(BlindStatus::DOWN),
            "Up" | "UP" | "up" => Ok(BlindStatus::UP),
            _ => Err("Unknown status")
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    fn light(&self, room: Room, status: LightStatus);
    fn blinds(&self, room: Room, status: BlindStatus);
    fn screen(&self, status: BlindStatus);
    fn cinema(&self);
    fn goodmorning(&self);
    fn goodnight(&self);
}

impl<T, R, G> MyHouse<T, R, G>
    where T: Sender<Message=BlyssMessage>,
          R: Radio,
          G: Generator {
    pub fn new(light: T, radio: R, generator: G) -> Self {
        MyHouse { light, radio, generator }
    }
}

impl<T, R, G> House for MyHouse<T, R, G>
    where T: Sender<Message=BlyssMessage>,
          R: Radio,
          G: Generator {
    fn light(&self, _room: Room, status: LightStatus) {
        let (timestamp, rolling_code) = self.generator.gen();
        let message = BlyssMessage::new(timestamp, rolling_code, 0x7057, Channel::ChannelC, SubChannel::Channel1, status.into());
        self.light.send(message);
    }

    fn blinds(&self, room: Room, status: BlindStatus) {
        let a = match room {
            Room::LivingRoom => 0x0932,
            Room::Kitchen => 0x2600
        };
        let s = match status {
            BlindStatus::DOWN => dio::Status::DOWN,
            BlindStatus::UP => dio::Status::UP
        };
        let message = DioMessage::new(a, s);
        self.radio.send(message, &DIO_PROTOCOL);
    }

    fn screen(&self, status: BlindStatus) {
        let message = match status {
            BlindStatus::DOWN => dooya::Status::DOWN,
            BlindStatus::UP => dooya::Status::UP,
        };
        self.radio.send(message, &DOOYA_PROTOCOL);
    }

    fn cinema(&self) {
        self.light(Room::LivingRoom, LightStatus::OFF);
        self.blinds(Room::LivingRoom, BlindStatus::DOWN);
        self.blinds(Room::Kitchen, BlindStatus::DOWN);
        self.screen(BlindStatus::DOWN);
    }

    fn goodmorning(&self) {
        self.light(Room::LivingRoom, LightStatus::OFF);
        self.blinds(Room::LivingRoom, BlindStatus::UP);
        self.blinds(Room::Kitchen, BlindStatus::UP);
        self.screen(BlindStatus::UP);
    }

    fn goodnight(&self) {
        self.light(Room::LivingRoom, LightStatus::OFF);
        self.blinds(Room::LivingRoom, BlindStatus::DOWN);
        self.blinds(Room::Kitchen, BlindStatus::DOWN);
        self.screen(BlindStatus::UP);
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use galvanic_assert::matchers::*;
    use crate::sender::mock::*;
    use crate::pin::mock::InMemoryPin;
    use crate::radio::mock::InMemoryRadio;

    #[test]
    fn switch_on_living_room() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let iter = (0..=1_u8).zip(2..3_u8);
        let house = MyHouse::new(sender, InMemoryPin::new(), CycleGenerator::new(iter));
        house.light(Room::LivingRoom, LightStatus::ON);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0,2,0x7057, Channel::ChannelC, SubChannel::Channel1, Status::On),
        ]));
    }

    #[test]
    fn switch_off_living_room() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let iter = (0..=1_u8).zip(2..3_u8);
        let house = MyHouse::new(sender, InMemoryPin::new(), CycleGenerator::new(iter));
        house.light(Room::LivingRoom, LightStatus::OFF);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0,2,0x7057, Channel::ChannelC, SubChannel::Channel1, Status::Off),
        ]));
    }

    #[test]
    fn blinds() {
        for (room, status, message) in vec![
            (Room::LivingRoom, BlindStatus::DOWN, DioMessage::new(0x0932, dio::Status::DOWN)),
            (Room::LivingRoom, BlindStatus::UP, DioMessage::new(0x0932, dio::Status::UP)),
            (Room::Kitchen, BlindStatus::DOWN, DioMessage::new(0x2600, dio::Status::DOWN)),
            (Room::Kitchen, BlindStatus::UP, DioMessage::new(0x2600, dio::Status::UP)),
        ] {
            let radio = InMemoryRadio::new();
            let iter = (0..=1_u8).zip(2..3_u8);
            let house = MyHouse::new(InMemorySender::new(), radio, CycleGenerator::new(iter));
            house.blinds(room, status);
            let received = house.radio.received(message, &DIO_PROTOCOL);
            assert_that!(&received, eq(true));
        }
    }

    #[test]
    fn screen() {
        for (status, message) in vec![
            (BlindStatus::DOWN, dooya::Status::DOWN),
            (BlindStatus::UP, dooya::Status::UP),
        ] {
            let radio = InMemoryRadio::new();
            let iter = (0..=1_u8).zip(2..3_u8);
            let house = MyHouse::new(InMemorySender::new(), radio, CycleGenerator::new(iter));
            house.screen(status);
            let received = house.radio.received(message, &DOOYA_PROTOCOL);
            assert_that!(&received, eq(true));
        }
    }

    #[test]
    fn parse_light_status() {
        assert_eq!("On".parse::<LightStatus>().unwrap(), LightStatus::ON);
        assert_eq!("ON".parse::<LightStatus>().unwrap(), LightStatus::ON);
        assert_eq!("on".parse::<LightStatus>().unwrap(), LightStatus::ON);
        assert_eq!("Off".parse::<LightStatus>().unwrap(), LightStatus::OFF);
        assert_eq!("OFF".parse::<LightStatus>().unwrap(), LightStatus::OFF);
        assert_eq!("off".parse::<LightStatus>().unwrap(), LightStatus::OFF);
        assert_eq!("plop".parse::<LightStatus>().is_err(), true);
    }

    #[test]
    fn parse_blind_status() {
        for string in &["Down", "DOWN", "down"] {
            assert_eq!(string.parse::<BlindStatus>().unwrap(), BlindStatus::DOWN);
        }
        for string in &["Up", "UP", "up"] {
            assert_eq!(string.parse::<BlindStatus>().unwrap(), BlindStatus::UP);
        }
        assert_eq!("plop".parse::<BlindStatus>().is_err(), true);
    }

    #[test]
    fn parse_room() {
        for string in &["Kitchen", "KITCHEN", "kitchen"] {
            assert_eq!(string.parse::<Room>().unwrap(), Room::Kitchen);
        }
        for string in &["LivingRoom", "LIVING_ROOM", "living_room", "Livingroom"] {
            assert_eq!(string.parse::<Room>().unwrap(), Room::LivingRoom);
        }
        assert_eq!("plop".parse::<Room>().is_err(), true);
    }
}