use crate::sender::Sender;
use crate::blyss_sender::Status;
use crate::blyss_sender::BlyssMessage;
use crate::blyss_sender::Channel;
use crate::blyss_sender::SubChannel;
use std::str::FromStr;
use crate::replay::Replayer;
use std::thread::sleep;
use std::time::Duration;

pub struct MyHouse<T: Sender<Message=BlyssMessage>, R: Replayer> {
    light: T,
    replayer: R,
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

impl<T: Sender<Message=BlyssMessage>, R: Replayer> MyHouse<T, R> {
    pub fn new(light: T, replayer: R) -> Self {
        MyHouse { light, replayer }
    }
}

const INTER_MESSAGE_PAUSE: Duration = Duration::from_millis(100);

impl<T: Sender<Message=BlyssMessage>, R: Replayer> House for MyHouse<T, R> {
    fn light(&self, _room: Room, status: LightStatus) {
        let message = BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel2, status.into());
        self.light.send(message);
    }

    fn blinds(&self, room: Room, status: BlindStatus) {
        let message = match (room, status) {
            (Room::LivingRoom, BlindStatus::DOWN) => vec![10740, 283, 2793, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283],
            (Room::LivingRoom, BlindStatus::UP) => vec![10744, 284, 2795, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284],
            (Room::Kitchen, BlindStatus::DOWN) => vec![10663, 279, 2784, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279],
            (Room::Kitchen, BlindStatus::UP) => vec![10659, 279, 2781, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279],
        };
        for _ in 0..10 {
            self.replayer.play(&message)
        }
    }

    fn screen(&self, status: BlindStatus) {
        let message = match status {
            BlindStatus::DOWN => vec![8897, 4805, 1519, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 724, 371, 724, 371, 371, 724, 371, 724, 724, 371, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 724, 371, 724, 371, 724, 371, 371, 724, 724, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 371, 371, 724, 724, 371, 371, 724, 371, 724, 371, 724, 724, 371, 371, 724, 371, 724, 724, 371, 724, 371, 371, 724, 371, 724, 724, 371, 724],
            BlindStatus::UP => vec![8906, 4795, 1522, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 727, 368, 727, 368, 368, 727, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 727, 368, 727, 368, 727, 368, 368, 727, 727, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 368, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 727],
        };
        for _ in 0..10 {
            self.replayer.play(&message)
        }
    }

    fn cinema(&self) {
        self.light(Room::LivingRoom, LightStatus::OFF);
        sleep(INTER_MESSAGE_PAUSE);
        self.blinds(Room::LivingRoom, BlindStatus::DOWN);
        sleep(INTER_MESSAGE_PAUSE);
        self.blinds(Room::Kitchen, BlindStatus::DOWN);
        sleep(INTER_MESSAGE_PAUSE);
        self.screen(BlindStatus::DOWN);
    }

    fn goodmorning(&self) {
        self.light(Room::LivingRoom, LightStatus::OFF);
        sleep(INTER_MESSAGE_PAUSE);
        self.blinds(Room::LivingRoom, BlindStatus::UP);
        sleep(INTER_MESSAGE_PAUSE);
        self.blinds(Room::Kitchen, BlindStatus::UP);
        sleep(INTER_MESSAGE_PAUSE);
        self.screen(BlindStatus::UP);
    }

    fn goodnight(&self) {
        self.light(Room::LivingRoom, LightStatus::OFF);
        sleep(INTER_MESSAGE_PAUSE);
        self.blinds(Room::LivingRoom, BlindStatus::DOWN);
        sleep(INTER_MESSAGE_PAUSE);
        self.blinds(Room::Kitchen, BlindStatus::DOWN);
        sleep(INTER_MESSAGE_PAUSE);
        self.screen(BlindStatus::UP);
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use crate::sender::mock::*;
    use crate::replay::mock::*;

    #[test]
    fn switch_on() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let house = MyHouse::new(sender, InMemoryReplayer::new());
        house.light(Room::LivingRoom, LightStatus::ON);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel2, Status::On),
        ]));
    }

    #[test]
    fn switch_off() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let house = MyHouse::new(sender, InMemoryReplayer::new());
        house.light(Room::LivingRoom, LightStatus::OFF);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel2, Status::Off),
        ]));
    }

    #[test]
    fn blinds() {
        for (room, status, message) in vec![
            (Room::LivingRoom, BlindStatus::DOWN, vec![10740, 283, 2793, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283]),
            (Room::LivingRoom, BlindStatus::UP, vec![10744, 284, 2795, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284]),
            (Room::Kitchen, BlindStatus::DOWN, vec![10663, 279, 2784, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279]),
            (Room::Kitchen, BlindStatus::UP, vec![10659, 279, 2781, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279]),
        ] {
            let replayer = InMemoryReplayer::new();
            let house = MyHouse::new(InMemorySender::new(), replayer);
            house.blinds(room, status);
            let messages = house.replayer.timings.into_inner();
            assert_that!(&messages, contains_in_order(repeat(message,10)));
        }
    }

    #[test]
    fn screen() {
        for (status, message) in vec![
            (BlindStatus::DOWN, vec![8897, 4805, 1519, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 724, 371, 724, 371, 371, 724, 371, 724, 724, 371, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 724, 371, 724, 371, 724, 371, 371, 724, 724, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 371, 724, 371, 371, 724, 724, 371, 371, 724, 371, 724, 371, 724, 724, 371, 371, 724, 371, 724, 724, 371, 724, 371, 371, 724, 371, 724, 724, 371, 724]),
            (BlindStatus::UP, vec![8906, 4795, 1522, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 727, 368, 727, 368, 368, 727, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 727, 368, 727, 368, 727, 368, 368, 727, 727, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 368, 727, 368, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 727, 368, 368, 727, 368, 727, 368, 727, 727]),
        ] {
            let replayer = InMemoryReplayer::new();
            let house = MyHouse::new(InMemorySender::new(), replayer);
            house.screen(status);
            let messages = house.replayer.timings.into_inner();
            assert_that!(&messages, contains_in_order(repeat(message,10)));
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

    fn repeat(v: Vec<u64>, n: usize) -> Vec<u64> {
        vec![v].into_iter().cycle().take(n).flatten().collect::<Vec<u64>>()
    }
}