use crate::sender::Sender;
use crate::blyss_sender::Status;
use crate::blyss_sender::BlyssMessage;
use crate::blyss_sender::Channel;
use crate::blyss_sender::SubChannel;
use std::str::FromStr;
use crate::replay::Replayer;
use std::sync::Mutex;
use std::cell::RefCell;
use std::iter;

pub struct MyHouse<T, R, G>
    where T: Sender<Message=BlyssMessage>,
          R: Replayer,
          G: Generator
{
    light: T,
    replayer: R,
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
          R: Replayer,
          G: Generator {
    pub fn new(light: T, replayer: R, generator: G) -> Self {
        MyHouse { light, replayer, generator }
    }
}

impl<T, R, G> House for MyHouse<T, R, G>
    where T: Sender<Message=BlyssMessage>,
          R: Replayer,
          G: Generator {
    fn light(&self, _room: Room, status: LightStatus) {
        let (timestamp, rolling_code) = self.generator.gen();
        let message = BlyssMessage::new(timestamp, rolling_code, 0x7057, Channel::ChannelC, SubChannel::Channel2, status.into());
        self.light.send(message);
    }

    fn blinds(&self, room: Room, status: BlindStatus) {
        let message = match (room, status) {
            (Room::LivingRoom, BlindStatus::DOWN) => vec![10740, 283, 2793, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 1355, 283, 283, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283, 283, 283, 1355, 283],
            (Room::LivingRoom, BlindStatus::UP) => vec![10744, 284, 2795, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 1355, 284, 284, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284, 284, 284, 1355, 284],
            (Room::Kitchen, BlindStatus::DOWN) => vec![10663, 279, 2784, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279],
            (Room::Kitchen, BlindStatus::UP) => vec![10659, 279, 2781, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 1354, 279, 279, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279, 279, 279, 1354, 279],
        };
        let repeated: Vec<_> = iter::repeat(message).take(10)
            .flat_map(|t| t.into_iter())
            .collect();
        self.replayer.play(&repeated)
    }

    fn screen(&self, status: BlindStatus) {
        let message = match status {
            BlindStatus::DOWN => vec![9096, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 744, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 643,
                                      8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 643,
                                      8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 643],
            BlindStatus::UP => vec![9096, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 744, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643,
                                    8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643,
                                    8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643],
        };
        let repeated: Vec<_> = iter::repeat(message).take(30)
            .flat_map(|t| t.into_iter())
            .collect();
        self.replayer.play(&repeated)
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
    use crate::sender::mock::*;
    use crate::replay::mock::*;

    #[test]
    fn switch_on() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let iter = (0..=1_u8).zip(2..3_u8);
        let house = MyHouse::new(sender, InMemoryReplayer::new(), CycleGenerator::new(iter));
        house.light(Room::LivingRoom, LightStatus::ON);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0,2,0x7057, Channel::ChannelC, SubChannel::Channel2, Status::On),
        ]));
    }

    #[test]
    fn switch_off() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let iter = (0..=1_u8).zip(2..3_u8);
        let house = MyHouse::new(sender, InMemoryReplayer::new(), CycleGenerator::new(iter));
        house.light(Room::LivingRoom, LightStatus::OFF);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0,2,0x7057, Channel::ChannelC, SubChannel::Channel2, Status::Off),
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
            let iter = (0..=1_u8).zip(2..3_u8);
            let house = MyHouse::new(InMemorySender::new(), replayer, CycleGenerator::new(iter));
            house.blinds(room, status);
            let messages = house.replayer.timings.into_inner();
            assert_that!(&messages, contains_subset(message));
        }
    }

    #[test]
    fn screen() {
        for (status, message) in vec![
            (BlindStatus::DOWN, vec![9096, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 744, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 643,
                                     8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 643,
                                     8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 643]),
            (BlindStatus::UP, vec![9096, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 744, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643,
                                   8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643,
                                   8883, 4764, 1537, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 313, 744, 313, 744, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 643, 442, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643, 442, 313, 744, 313, 744, 313, 744, 643]),
        ] {
            let replayer = InMemoryReplayer::new();
            let iter = (0..=1_u8).zip(2..3_u8);
            let house = MyHouse::new(InMemorySender::new(), replayer, CycleGenerator::new(iter));
            house.screen(status);
            let messages = house.replayer.timings.into_inner();
            assert_that!(&messages, contains_subset(message));
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