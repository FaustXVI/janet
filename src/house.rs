use crate::sender::Sender;
use crate::blyss_sender::Status;
use crate::blyss_sender::BlyssMessage;
use crate::blyss_sender::Channel;
use crate::blyss_sender::SubChannel;
use std::str::FromStr;
use crate::replay::Replayer;

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
}

impl<T: Sender<Message=BlyssMessage>, R: Replayer> MyHouse<T, R> {
    pub fn new(light: T, replayer: R) -> Self {
        MyHouse { light, replayer }
    }
}

impl<T: Sender<Message=BlyssMessage>, R: Replayer> House for MyHouse<T, R> {
    fn light(&self, _room: Room, status: LightStatus) {
        let message = BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, status.into());
        self.light.send(message);
    }

    fn blinds(&self, room: Room, status: BlindStatus) {
        let message = match (room, status) {
            (Room::LivingRoom, BlindStatus::DOWN) => vec![280, 288, 1352, 288, 1344, 288, 288, 288, 280, 284, 1356, 284, 280, 288, 1356, 284, 280, 284, 1360, 284, 280, 284, 1344, 284, 10736, 296, 2788, 296, 272, 296, 1344, 296, 272, 292, 1348, 296, 1336, 296, 280, 292, 272, 296, 1348, 292, 272, 292, 1348, 292, 1340, 292, 284, 292, 1340, 292, 284, 292, 1340, 292, 288, 292, 276, 292, 1348, 292, 276, 288, 1352, 292, 276, 288, 1352, 292, 272, 292, 1352, 288, 1344, 288, 288, 288, 276, 288, 1352, 292, 276, 288, 1352, 288, 1344, 292, 292, 288, 276, 288, 1356, 288, 276, 288, 1356, 288, 1344, 288, 288, 284, 1348, 284, 292, 284, 280, 284, 1356, 288, 280, 284, 1356, 288, 1348, 284, 288, 284, 284, 284, 1364, 284, 1348, 284, 292, 288, 276, 288, 1352, 288, 280, 284, 1356, 284, 1348, 284, 292, 284, 280, 284, 1360, 284, 280, 284, 1360, 284, 280, 284, 1356, 284, 284, 284, 1344, 280, 10740, 292, 2792, 292, 272, 296, 1348, 292, 272, 292, 1352, 292, 1340, 292, 280, 296, 272, 292, 1348, 292, 272, 296, 1348, 292, 1340, 292, 284, 288, 1344, 288, 288, 288, 1344, 288, 292, 292, 276, 288, 1352, 288, 280, 288, 1352, 288, 276, 288, 1356, 288, 276, 288, 1356, 288, 1340, 288, 288, 288, 280, 284, 1356, 288, 276, 288, 1356, 288, 1344, 288, 292, 288, 280, 284, 1356, 288, 280, 284, 1356, 284, 1348, 284, 292, 284, 1348, 284, 288, 288, 280, 284, 1356, 288, 280, 284, 1356, 284, 1348, 284, 292, 284, 280, 288, 1364, 284, 1348, 284, 288, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 292, 284, 280, 284, 1360, 284, 280, 284, 1360, 284, 280, 284, 1356, 288, 280, 284, 1344, 280, 10744, 288, 2792, 296, 272, 292, 1348, 296, 272, 292, 1348, 292, 1340, 292, 284, 292, 272, 296, 1348, 292, 272, 292, 1352, 292, 1340, 292, 284, 288, 1344, 288, 288, 288, 1344, 288, 292, 292, 276, 288, 1352, 288, 280, 288, 1352, 288, 280, 288, 1352, 288, 276, 288, 1352, 292, 1344, 288, 288, 284, 280, 288, 1352, 288, 280, 284, 1356, 288, 1344, 288, 296, 284, 280, 288, 1356, 284, 280, 288, 1356, 284, 1344, 288, 288, 288, 1344, 288, 288, 288, 276, 288, 1356, 284, 280, 288, 1356, 284, 1348, 284, 292, 284, 280, 284, 1364, 284, 1352, 280, 292, 284, 284, 280, 1360, 284, 280, 284, 1360, 284, 1348, 284, 288, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 284, 284, 1356, 284, 280, 284, 1348, 280, 10740, 292, 2792, 296, 272, 292, 1348, 292, 272, 296, 1348, 292, 1344, 288, 284, 292, 272, 292, 1352, 292, 272, 292, 1352, 288, 1344, 288, 288, 288, 1344, 288, 284, 292, 1340, 292, 292, 288, 276, 292, 1352, 288, 276, 288, 1356, 288, 276, 288, 1356, 288, 276, 288, 1356, 284, 1348, 284, 288, 288, 280, 284, 1356, 288, 280, 284, 1356, 288, 1344, 288, 296, 284, 280, 288, 1352, 288, 280, 284, 1356, 288, 1344, 288, 288, 288],
            (Room::LivingRoom, BlindStatus::UP) => vec![1348, 284, 292, 284, 1348, 284, 296, 284, 280, 284, 1360, 284, 284, 280, 1356, 284, 284, 284, 1356, 284, 280, 284, 1360, 284, 1344, 284, 292, 284, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 300, 284, 284, 280, 1360, 280, 284, 284, 1360, 280, 1352, 280, 296, 280, 1348, 280, 296, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 296, 280, 288, 276, 1368, 284, 1348, 280, 296, 280, 284, 280, 1364, 276, 288, 280, 1364, 276, 288, 276, 1364, 280, 288, 276, 1360, 284, 284, 280, 1360, 280, 284, 280, 1360, 284, 284, 280, 1352, 276, 10744, 284, 2796, 288, 276, 292, 1352, 288, 276, 288, 1352, 288, 1344, 288, 288, 288, 280, 284, 1356, 284, 280, 288, 1352, 288, 1344, 288, 288, 284, 1348, 284, 292, 284, 1348, 284, 296, 284, 280, 288, 1352, 288, 280, 284, 1356, 284, 280, 288, 1356, 284, 284, 280, 1360, 284, 1348, 280, 296, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 304, 280, 284, 280, 1360, 284, 280, 284, 1356, 284, 1348, 284, 292, 284, 1348, 280, 296, 280, 284, 284, 1360, 280, 284, 280, 1360, 284, 1348, 284, 292, 280, 288, 280, 1368, 280, 1352, 280, 296, 276, 288, 280, 1360, 280, 288, 276, 1364, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 288, 276, 1364, 280, 284, 280, 1360, 280, 288, 280, 1348, 276, 10744, 288, 2796, 288, 276, 288, 1356, 288, 276, 288, 1352, 288, 1344, 288, 288, 288, 276, 288, 1352, 288, 280, 288, 1356, 284, 1344, 288, 288, 284, 1348, 284, 292, 284, 1344, 288, 296, 284, 280, 288, 1356, 284, 280, 284, 1360, 284, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 292, 284, 284, 280, 1360, 280, 284, 284, 1360, 280, 1348, 284, 300, 284, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 292, 280, 1352, 280, 296, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 296, 280, 284, 280, 1372, 280, 1348, 280, 296, 280, 284, 284, 1356, 284, 284, 280, 1360, 284, 284, 280, 1360, 280, 288, 280, 1360, 280, 284, 280, 1364, 276, 288, 280, 1360, 280, 288, 276, 1352, 276, 10744, 288, 2792, 292, 276, 288, 1352, 288, 276, 292, 1348, 292, 1344, 288, 284, 288, 280, 288, 1352, 288, 276, 288, 1356, 288, 1344, 284, 288, 288, 1348, 284, 288, 288, 1344, 288, 296, 284, 280, 284, 1356, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 288, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 300, 280, 284, 284, 1356, 284, 284, 280, 1360, 284, 1348, 284, 292, 280, 1352, 284, 288, 284, 284, 280, 1360, 284, 284, 280, 1360, 284, 1348, 280, 296, 280, 284, 284, 1364, 284, 1348, 284, 292, 280, 284, 284, 1360, 280, 288, 276, 1364, 280, 284, 280, 1360, 280, 288, 280, 1360, 280, 284, 284, 1360, 280, 284, 280, 1364, 280, 284, 280, 1348, 276],
            (Room::Kitchen, BlindStatus::DOWN) => vec![1348, 272, 300, 272, 1348, 272, 308, 272, 288, 272, 1356, 276, 288, 272, 1352, 280, 1344, 272, 296, 276, 292, 272, 1352, 276, 288, 276, 1356, 272, 1348, 272, 296, 276, 1344, 276, 296, 272, 292, 272, 1364, 272, 292, 272, 1352, 276, 288, 272, 1360, 272, 288, 272, 1360, 272, 288, 272, 1360, 268, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 296, 264, 1368, 268, 1348, 272, 304, 268, 292, 272, 1356, 272, 292, 272, 1356, 272, 1344, 276, 300, 272, 288, 272, 1356, 276, 292, 268, 1360, 272, 292, 268, 1360, 268, 292, 272, 1348, 264, 10664, 276, 2784, 276, 280, 280, 1352, 276, 284, 280, 1348, 280, 1344, 276, 292, 280, 288, 272, 1356, 276, 284, 276, 1352, 276, 1348, 272, 296, 276, 1344, 276, 300, 272, 1344, 276, 308, 272, 288, 272, 1356, 276, 284, 276, 1356, 272, 1348, 272, 300, 272, 292, 272, 1356, 272, 288, 272, 1360, 272, 1344, 276, 304, 264, 1352, 268, 296, 276, 292, 272, 1368, 268, 288, 272, 1360, 272, 288, 272, 1356, 276, 288, 272, 1356, 276, 288, 272, 1360, 268, 288, 276, 1356, 272, 296, 268, 1360, 268, 292, 268, 1364, 268, 292, 268, 1368, 272, 1348, 268, 304, 268, 296, 268, 1360, 268, 292, 272, 1356, 272, 1348, 272, 300, 272, 296, 264, 1360, 272, 288, 272, 1360, 268, 296, 268, 1360, 268, 292, 272, 1348, 264, 10664, 276, 2784, 276, 288, 272, 1352, 276, 288, 276, 1352, 276, 1344, 276, 296, 276, 284, 276, 1356, 276, 284, 276, 1356, 276, 1340, 280, 288, 280, 1344, 276, 300, 272, 1344, 276, 304, 276, 284, 276, 1352, 280, 284, 276, 1352, 276, 1344, 276, 300, 272, 288, 276, 1356, 272, 292, 268, 1360, 272, 1348, 272, 300, 272, 1348, 272, 300, 272, 288, 272, 1364, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 292, 268, 1360, 272, 288, 272, 1360, 272, 292, 268, 1356, 272, 292, 272, 1356, 272, 292, 272, 1368, 268, 1352, 268, 296, 276, 300, 264, 1352, 276, 288, 272, 1360, 272, 1348, 272, 300, 268, 296, 268, 1360, 268, 292, 272, 1360, 268, 296, 268, 1360, 268, 292, 268, 1352, 264, 10660, 276, 2784, 276, 288, 276, 1348, 280, 288, 276, 1348, 280, 1344, 276, 296, 276, 288, 272, 1356, 276, 284, 276, 1352, 276, 1344, 276, 296, 276, 1348, 272, 296, 276, 1344, 276, 304, 272, 292, 272, 1356, 272, 292, 272, 1352, 276, 1348, 272, 300, 272, 292, 272, 1356, 272, 288, 272, 1356, 276, 1348, 272, 292, 276, 1348, 272, 300, 272, 288, 276, 1364, 272, 288, 272, 1360, 272, 292, 268, 1360, 272, 292, 268, 1360, 272, 292, 268, 1356, 272, 292, 272, 1356, 272, 292, 272, 1360, 268, 296, 268, 1356, 272, 292, 272, 1364, 272, 1344, 276, 304, 264, 292, 272, 1356, 272, 296, 268, 1356, 272, 1348, 272, 304, 268, 292, 272, 1356, 272, 292, 272, 1360, 268, 288, 276, 1360, 268, 292, 272, 1344, 268],
            (Room::Kitchen, BlindStatus::UP) => vec![1340, 276, 300, 272, 1348, 272, 308, 272, 288, 276, 1352, 276, 288, 276, 1356, 272, 1348, 272, 300, 272, 292, 272, 1356, 272, 288, 276, 1356, 272, 1348, 272, 296, 276, 1344, 276, 296, 276, 288, 276, 1360, 276, 288, 272, 1360, 272, 288, 272, 1356, 276, 288, 272, 1360, 272, 292, 268, 1360, 272, 288, 272, 1360, 272, 288, 272, 1356, 276, 292, 268, 1356, 276, 292, 268, 1364, 272, 1348, 272, 304, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 296, 268, 1360, 268, 292, 272, 1360, 268, 296, 268, 1344, 272, 10660, 276, 2780, 280, 284, 280, 1348, 280, 284, 276, 1352, 280, 1340, 280, 292, 280, 284, 276, 1352, 280, 284, 276, 1352, 276, 1344, 276, 300, 272, 1348, 272, 300, 272, 1344, 276, 304, 276, 288, 276, 1356, 272, 288, 276, 1352, 276, 1344, 276, 296, 276, 288, 276, 1352, 276, 288, 276, 1352, 276, 1344, 276, 296, 276, 1344, 276, 296, 272, 292, 272, 1364, 272, 292, 272, 1352, 276, 288, 276, 1356, 272, 292, 272, 1352, 276, 288, 272, 1360, 272, 288, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1364, 272, 1344, 272, 304, 268, 296, 268, 1356, 272, 296, 268, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 288, 272, 1364, 264, 292, 272, 1360, 268, 292, 272, 1348, 264, 10656, 276, 2784, 280, 280, 280, 1352, 276, 284, 280, 1348, 280, 1340, 276, 296, 276, 284, 280, 1352, 276, 284, 280, 1352, 276, 1340, 280, 296, 272, 1344, 276, 300, 272, 1344, 276, 304, 276, 288, 272, 1356, 276, 284, 276, 1352, 276, 1344, 276, 300, 272, 288, 276, 1356, 272, 288, 272, 1356, 276, 1344, 272, 300, 272, 1348, 272, 300, 272, 288, 276, 1360, 276, 288, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 288, 272, 1360, 272, 288, 272, 1360, 268, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1364, 272, 1348, 268, 300, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 292, 268, 1360, 268, 296, 268, 1360, 268, 292, 272, 1356, 272, 292, 268, 1352, 264, 10660, 276, 2780, 280, 284, 276, 1352, 276, 288, 276, 1352, 276, 1344, 276, 292, 280, 284, 276, 1352, 276, 288, 276, 1352, 276, 1344, 276, 296, 276, 1344, 276, 296, 276, 1344, 272, 304, 276, 288, 276, 1352, 276, 288, 276, 1352, 276, 1344, 272, 300, 272, 292, 272, 1356, 272, 288, 276, 1356, 272, 1348, 272, 300, 272, 1344, 272, 300, 272, 292, 272, 1364, 272, 288, 276, 1356, 272, 288, 272, 1356, 276, 288, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 292, 268, 1360, 272, 292, 268, 1364, 272, 1352, 268, 304, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 292, 272, 1356, 272, 288, 272, 1360, 272, 292, 268, 1360, 268, 292, 272, 1360, 268, 296, 268, 1348, 264],
        };
        self.replayer.play(&message)
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
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::On),
        ]));
    }

    #[test]
    fn switch_off() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let house = MyHouse::new(sender, InMemoryReplayer::new());
        house.light(Room::LivingRoom, LightStatus::OFF);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::Off),
        ]));
    }

    #[test]
    fn living_room_blind_down() {
        for (room, status, message) in vec![
            (Room::LivingRoom, BlindStatus::DOWN, vec![280, 288, 1352, 288, 1344, 288, 288, 288, 280, 284, 1356, 284, 280, 288, 1356, 284, 280, 284, 1360, 284, 280, 284, 1344, 284, 10736, 296, 2788, 296, 272, 296, 1344, 296, 272, 292, 1348, 296, 1336, 296, 280, 292, 272, 296, 1348, 292, 272, 292, 1348, 292, 1340, 292, 284, 292, 1340, 292, 284, 292, 1340, 292, 288, 292, 276, 292, 1348, 292, 276, 288, 1352, 292, 276, 288, 1352, 292, 272, 292, 1352, 288, 1344, 288, 288, 288, 276, 288, 1352, 292, 276, 288, 1352, 288, 1344, 292, 292, 288, 276, 288, 1356, 288, 276, 288, 1356, 288, 1344, 288, 288, 284, 1348, 284, 292, 284, 280, 284, 1356, 288, 280, 284, 1356, 288, 1348, 284, 288, 284, 284, 284, 1364, 284, 1348, 284, 292, 288, 276, 288, 1352, 288, 280, 284, 1356, 284, 1348, 284, 292, 284, 280, 284, 1360, 284, 280, 284, 1360, 284, 280, 284, 1356, 284, 284, 284, 1344, 280, 10740, 292, 2792, 292, 272, 296, 1348, 292, 272, 292, 1352, 292, 1340, 292, 280, 296, 272, 292, 1348, 292, 272, 296, 1348, 292, 1340, 292, 284, 288, 1344, 288, 288, 288, 1344, 288, 292, 292, 276, 288, 1352, 288, 280, 288, 1352, 288, 276, 288, 1356, 288, 276, 288, 1356, 288, 1340, 288, 288, 288, 280, 284, 1356, 288, 276, 288, 1356, 288, 1344, 288, 292, 288, 280, 284, 1356, 288, 280, 284, 1356, 284, 1348, 284, 292, 284, 1348, 284, 288, 288, 280, 284, 1356, 288, 280, 284, 1356, 284, 1348, 284, 292, 284, 280, 288, 1364, 284, 1348, 284, 288, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 292, 284, 280, 284, 1360, 284, 280, 284, 1360, 284, 280, 284, 1356, 288, 280, 284, 1344, 280, 10744, 288, 2792, 296, 272, 292, 1348, 296, 272, 292, 1348, 292, 1340, 292, 284, 292, 272, 296, 1348, 292, 272, 292, 1352, 292, 1340, 292, 284, 288, 1344, 288, 288, 288, 1344, 288, 292, 292, 276, 288, 1352, 288, 280, 288, 1352, 288, 280, 288, 1352, 288, 276, 288, 1352, 292, 1344, 288, 288, 284, 280, 288, 1352, 288, 280, 284, 1356, 288, 1344, 288, 296, 284, 280, 288, 1356, 284, 280, 288, 1356, 284, 1344, 288, 288, 288, 1344, 288, 288, 288, 276, 288, 1356, 284, 280, 288, 1356, 284, 1348, 284, 292, 284, 280, 284, 1364, 284, 1352, 280, 292, 284, 284, 280, 1360, 284, 280, 284, 1360, 284, 1348, 284, 288, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 284, 284, 1356, 284, 280, 284, 1348, 280, 10740, 292, 2792, 296, 272, 292, 1348, 292, 272, 296, 1348, 292, 1344, 288, 284, 292, 272, 292, 1352, 292, 272, 292, 1352, 288, 1344, 288, 288, 288, 1344, 288, 284, 292, 1340, 292, 292, 288, 276, 292, 1352, 288, 276, 288, 1356, 288, 276, 288, 1356, 288, 276, 288, 1356, 284, 1348, 284, 288, 288, 280, 284, 1356, 288, 280, 284, 1356, 288, 1344, 288, 296, 284, 280, 288, 1352, 288, 280, 284, 1356, 288, 1344, 288, 288, 288]),
            (Room::LivingRoom, BlindStatus::UP, vec![1348, 284, 292, 284, 1348, 284, 296, 284, 280, 284, 1360, 284, 284, 280, 1356, 284, 284, 284, 1356, 284, 280, 284, 1360, 284, 1344, 284, 292, 284, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 300, 284, 284, 280, 1360, 280, 284, 284, 1360, 280, 1352, 280, 296, 280, 1348, 280, 296, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 296, 280, 288, 276, 1368, 284, 1348, 280, 296, 280, 284, 280, 1364, 276, 288, 280, 1364, 276, 288, 276, 1364, 280, 288, 276, 1360, 284, 284, 280, 1360, 280, 284, 280, 1360, 284, 284, 280, 1352, 276, 10744, 284, 2796, 288, 276, 292, 1352, 288, 276, 288, 1352, 288, 1344, 288, 288, 288, 280, 284, 1356, 284, 280, 288, 1352, 288, 1344, 288, 288, 284, 1348, 284, 292, 284, 1348, 284, 296, 284, 280, 288, 1352, 288, 280, 284, 1356, 284, 280, 288, 1356, 284, 284, 280, 1360, 284, 1348, 280, 296, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 304, 280, 284, 280, 1360, 284, 280, 284, 1356, 284, 1348, 284, 292, 284, 1348, 280, 296, 280, 284, 284, 1360, 280, 284, 280, 1360, 284, 1348, 284, 292, 280, 288, 280, 1368, 280, 1352, 280, 296, 276, 288, 280, 1360, 280, 288, 276, 1364, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 288, 276, 1364, 280, 284, 280, 1360, 280, 288, 280, 1348, 276, 10744, 288, 2796, 288, 276, 288, 1356, 288, 276, 288, 1352, 288, 1344, 288, 288, 288, 276, 288, 1352, 288, 280, 288, 1356, 284, 1344, 288, 288, 284, 1348, 284, 292, 284, 1344, 288, 296, 284, 280, 288, 1356, 284, 280, 284, 1360, 284, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 292, 284, 284, 280, 1360, 280, 284, 284, 1360, 280, 1348, 284, 300, 284, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 292, 280, 1352, 280, 296, 280, 284, 280, 1360, 284, 284, 280, 1360, 280, 1352, 280, 296, 280, 284, 280, 1372, 280, 1348, 280, 296, 280, 284, 284, 1356, 284, 284, 280, 1360, 284, 284, 280, 1360, 280, 288, 280, 1360, 280, 284, 280, 1364, 276, 288, 280, 1360, 280, 288, 276, 1352, 276, 10744, 288, 2792, 292, 276, 288, 1352, 288, 276, 292, 1348, 292, 1344, 288, 284, 288, 280, 288, 1352, 288, 276, 288, 1356, 288, 1344, 284, 288, 288, 1348, 284, 288, 288, 1344, 288, 296, 284, 280, 284, 1356, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 288, 288, 280, 284, 1356, 284, 284, 284, 1356, 284, 1348, 284, 300, 280, 284, 284, 1356, 284, 284, 280, 1360, 284, 1348, 284, 292, 280, 1352, 284, 288, 284, 284, 280, 1360, 284, 284, 280, 1360, 284, 1348, 280, 296, 280, 284, 284, 1364, 284, 1348, 284, 292, 280, 284, 284, 1360, 280, 288, 276, 1364, 280, 284, 280, 1360, 280, 288, 280, 1360, 280, 284, 284, 1360, 280, 284, 280, 1364, 280, 284, 280, 1348, 276]),
            (Room::Kitchen, BlindStatus::DOWN, vec![1348, 272, 300, 272, 1348, 272, 308, 272, 288, 272, 1356, 276, 288, 272, 1352, 280, 1344, 272, 296, 276, 292, 272, 1352, 276, 288, 276, 1356, 272, 1348, 272, 296, 276, 1344, 276, 296, 272, 292, 272, 1364, 272, 292, 272, 1352, 276, 288, 272, 1360, 272, 288, 272, 1360, 272, 288, 272, 1360, 268, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 296, 264, 1368, 268, 1348, 272, 304, 268, 292, 272, 1356, 272, 292, 272, 1356, 272, 1344, 276, 300, 272, 288, 272, 1356, 276, 292, 268, 1360, 272, 292, 268, 1360, 268, 292, 272, 1348, 264, 10664, 276, 2784, 276, 280, 280, 1352, 276, 284, 280, 1348, 280, 1344, 276, 292, 280, 288, 272, 1356, 276, 284, 276, 1352, 276, 1348, 272, 296, 276, 1344, 276, 300, 272, 1344, 276, 308, 272, 288, 272, 1356, 276, 284, 276, 1356, 272, 1348, 272, 300, 272, 292, 272, 1356, 272, 288, 272, 1360, 272, 1344, 276, 304, 264, 1352, 268, 296, 276, 292, 272, 1368, 268, 288, 272, 1360, 272, 288, 272, 1356, 276, 288, 272, 1356, 276, 288, 272, 1360, 268, 288, 276, 1356, 272, 296, 268, 1360, 268, 292, 268, 1364, 268, 292, 268, 1368, 272, 1348, 268, 304, 268, 296, 268, 1360, 268, 292, 272, 1356, 272, 1348, 272, 300, 272, 296, 264, 1360, 272, 288, 272, 1360, 268, 296, 268, 1360, 268, 292, 272, 1348, 264, 10664, 276, 2784, 276, 288, 272, 1352, 276, 288, 276, 1352, 276, 1344, 276, 296, 276, 284, 276, 1356, 276, 284, 276, 1356, 276, 1340, 280, 288, 280, 1344, 276, 300, 272, 1344, 276, 304, 276, 284, 276, 1352, 280, 284, 276, 1352, 276, 1344, 276, 300, 272, 288, 276, 1356, 272, 292, 268, 1360, 272, 1348, 272, 300, 272, 1348, 272, 300, 272, 288, 272, 1364, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 292, 268, 1360, 272, 288, 272, 1360, 272, 292, 268, 1356, 272, 292, 272, 1356, 272, 292, 272, 1368, 268, 1352, 268, 296, 276, 300, 264, 1352, 276, 288, 272, 1360, 272, 1348, 272, 300, 268, 296, 268, 1360, 268, 292, 272, 1360, 268, 296, 268, 1360, 268, 292, 268, 1352, 264, 10660, 276, 2784, 276, 288, 276, 1348, 280, 288, 276, 1348, 280, 1344, 276, 296, 276, 288, 272, 1356, 276, 284, 276, 1352, 276, 1344, 276, 296, 276, 1348, 272, 296, 276, 1344, 276, 304, 272, 292, 272, 1356, 272, 292, 272, 1352, 276, 1348, 272, 300, 272, 292, 272, 1356, 272, 288, 272, 1356, 276, 1348, 272, 292, 276, 1348, 272, 300, 272, 288, 276, 1364, 272, 288, 272, 1360, 272, 292, 268, 1360, 272, 292, 268, 1360, 272, 292, 268, 1356, 272, 292, 272, 1356, 272, 292, 272, 1360, 268, 296, 268, 1356, 272, 292, 272, 1364, 272, 1344, 276, 304, 264, 292, 272, 1356, 272, 296, 268, 1356, 272, 1348, 272, 304, 268, 292, 272, 1356, 272, 292, 272, 1360, 268, 288, 276, 1360, 268, 292, 272, 1344, 268]),
            (Room::Kitchen, BlindStatus::UP, vec![1340, 276, 300, 272, 1348, 272, 308, 272, 288, 276, 1352, 276, 288, 276, 1356, 272, 1348, 272, 300, 272, 292, 272, 1356, 272, 288, 276, 1356, 272, 1348, 272, 296, 276, 1344, 276, 296, 276, 288, 276, 1360, 276, 288, 272, 1360, 272, 288, 272, 1356, 276, 288, 272, 1360, 272, 292, 268, 1360, 272, 288, 272, 1360, 272, 288, 272, 1356, 276, 292, 268, 1356, 276, 292, 268, 1364, 272, 1348, 272, 304, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 296, 268, 1360, 268, 292, 272, 1360, 268, 296, 268, 1344, 272, 10660, 276, 2780, 280, 284, 280, 1348, 280, 284, 276, 1352, 280, 1340, 280, 292, 280, 284, 276, 1352, 280, 284, 276, 1352, 276, 1344, 276, 300, 272, 1348, 272, 300, 272, 1344, 276, 304, 276, 288, 276, 1356, 272, 288, 276, 1352, 276, 1344, 276, 296, 276, 288, 276, 1352, 276, 288, 276, 1352, 276, 1344, 276, 296, 276, 1344, 276, 296, 272, 292, 272, 1364, 272, 292, 272, 1352, 276, 288, 276, 1356, 272, 292, 272, 1352, 276, 288, 272, 1360, 272, 288, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1364, 272, 1344, 272, 304, 268, 296, 268, 1356, 272, 296, 268, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 288, 272, 1364, 264, 292, 272, 1360, 268, 292, 272, 1348, 264, 10656, 276, 2784, 280, 280, 280, 1352, 276, 284, 280, 1348, 280, 1340, 276, 296, 276, 284, 280, 1352, 276, 284, 280, 1352, 276, 1340, 280, 296, 272, 1344, 276, 300, 272, 1344, 276, 304, 276, 288, 272, 1356, 276, 284, 276, 1352, 276, 1344, 276, 300, 272, 288, 276, 1356, 272, 288, 272, 1356, 276, 1344, 272, 300, 272, 1348, 272, 300, 272, 288, 276, 1360, 276, 288, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 288, 272, 1360, 272, 288, 272, 1360, 268, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1364, 272, 1348, 268, 300, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 292, 268, 1360, 268, 296, 268, 1360, 268, 292, 272, 1356, 272, 292, 268, 1352, 264, 10660, 276, 2780, 280, 284, 276, 1352, 276, 288, 276, 1352, 276, 1344, 276, 292, 280, 284, 276, 1352, 276, 288, 276, 1352, 276, 1344, 276, 296, 276, 1344, 276, 296, 276, 1344, 272, 304, 276, 288, 276, 1352, 276, 288, 276, 1352, 276, 1344, 272, 300, 272, 292, 272, 1356, 272, 288, 276, 1356, 272, 1348, 272, 300, 272, 1344, 272, 300, 272, 292, 272, 1364, 272, 288, 276, 1356, 272, 288, 272, 1356, 276, 288, 272, 1356, 272, 292, 272, 1356, 272, 292, 272, 1356, 272, 292, 268, 1360, 272, 292, 268, 1360, 272, 292, 268, 1364, 272, 1352, 268, 304, 268, 292, 272, 1360, 268, 292, 272, 1360, 268, 292, 272, 1356, 272, 288, 272, 1360, 272, 292, 268, 1360, 268, 292, 272, 1360, 268, 296, 268, 1348, 264]),
        ] {
            let replayer = InMemoryReplayer::new();
            let house = MyHouse::new(InMemorySender::new(), replayer);
            house.blinds(room, status);
            let messages = house.replayer.timings.into_inner();
            assert_that!(&messages, contains_in_order(message));
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