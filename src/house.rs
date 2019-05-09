use std::str::FromStr;
use crate::dio::DioMessage;
use crate::dio::DIO_PROTOCOL;
use crate::dio;
use crate::dooya::DOOYA_PROTOCOL;
use crate::dooya;
use crate::celexon;
use crate::radio::Radio;
use std::time::Duration;
use std::sync::Mutex;
use crate::radio_protocol::RadioProtocol;

pub struct MyHouse<R>
    where R: Radio
{
    radio: Mutex<R>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Room {
    Kitchen,
    LivingRoom,
    BedRoom,
}

impl FromStr for Room {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kitchen" | "KITCHEN" | "kitchen" => Ok(Room::Kitchen),
            "LivingRoom" | "LIVING_ROOM" | "living_room" | "Livingroom" => Ok(Room::LivingRoom),
            "BedRoom" | "BED_ROOM" | "bed_room" | "Bedroom" => Ok(Room::BedRoom),
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

impl<R> MyHouse<R>
    where R: Radio {
    pub fn new(radio: R) -> Self {
        MyHouse { radio: Mutex::new(radio) }
    }
    fn send<M>(&self, message: M, protocol: &RadioProtocol<M>) where M: IntoIterator<Item=u8> {
        let r = self.radio.lock().expect("Can't get lock on radio");
        unsafe {
            r.send(message, protocol);
        }
    }
}

impl<R> House for MyHouse<R>
    where R: Radio {
    fn light(&self, room: Room, status: LightStatus) {
        let a = match room {
            Room::LivingRoom => 0x1337,
            Room::Kitchen => 0x0042,
            Room::BedRoom => 0x985c,
        };
        let s = match status {
            LightStatus::ON => dio::Status::ON,
            LightStatus::OFF => dio::Status::OFF
        };
        let message = DioMessage::new(a, s);
        self.send(message, &DIO_PROTOCOL);
    }

    fn blinds(&self, room: Room, status: BlindStatus) {
        if room == Room::BedRoom {
            let s = match status {
                BlindStatus::DOWN => celexon::Status::DOWN,
                BlindStatus::UP => celexon::Status::UP
            };
            self.send(s, &celexon::CELEXON_PROTOCOL)
        } else {
            let a = match room {
                Room::LivingRoom => 0x0932,
                Room::Kitchen => 0x2600,
                Room::BedRoom => 0x0000,
            };
            let s = match status {
                BlindStatus::DOWN => dio::Status::DOWN,
                BlindStatus::UP => dio::Status::UP
            };
            let message = DioMessage::new(a, s);
            self.send(message, &DIO_PROTOCOL)
        }
    }

    fn screen(&self, status: BlindStatus) {
        let message = match status {
            BlindStatus::DOWN => dooya::Status::DOWN,
            BlindStatus::UP => dooya::Status::UP,
        };
        self.send(message, &DOOYA_PROTOCOL)
    }

    // modes should be configuration
    fn cinema(&self) {
        self.blinds(Room::LivingRoom, BlindStatus::DOWN);
        self.blinds(Room::Kitchen, BlindStatus::DOWN);
        self.screen(BlindStatus::DOWN);
        self.light(Room::LivingRoom, LightStatus::OFF);
        self.light(Room::Kitchen, LightStatus::OFF);
    }

    fn goodmorning(&self) {
        self.blinds(Room::LivingRoom, BlindStatus::UP);
        self.blinds(Room::Kitchen, BlindStatus::UP);
        self.blinds(Room::BedRoom, BlindStatus::UP);
        self.screen(BlindStatus::UP);
        self.light(Room::LivingRoom, LightStatus::OFF);
        self.light(Room::BedRoom, LightStatus::OFF);
        self.light(Room::Kitchen, LightStatus::OFF);
    }

    fn goodnight(&self) {
        self.blinds(Room::LivingRoom, BlindStatus::DOWN);
        self.blinds(Room::Kitchen, BlindStatus::DOWN);
        self.blinds(Room::BedRoom, BlindStatus::DOWN);
        self.screen(BlindStatus::UP);
        self.light(Room::LivingRoom, LightStatus::OFF);
        self.light(Room::BedRoom, LightStatus::OFF);
        self.light(Room::Kitchen, LightStatus::OFF);
    }
}

#[cfg(target_arch = "arm")]
pub fn house() -> impl House {
    use sysfs_gpio::Pin;
    use sysfs_gpio::Direction;
    use std::thread::sleep;

    let pin = Pin::new(23);
    pin.export().unwrap();
    if pin.set_direction(Direction::Low).is_err() {
        sleep(Duration::from_millis(500));
        pin.set_direction(Direction::Low).unwrap();
    };
    MyHouse::new(pin)
}

#[cfg(not(target_arch = "arm"))]
pub fn house() -> impl House {
    use crate::pin::DigitalOutput;

    #[derive(Debug, Clone)]
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

    let pin = FakeDigitalOutput::new(23);
    MyHouse::new(pin)
}

#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::*;
    use crate::radio::mock::InMemoryRadio;

    #[test]
    fn lights() {
        for (room, status, message) in vec![
            (Room::LivingRoom, LightStatus::ON, DioMessage::new(0x1337, dio::Status::ON)),
            (Room::LivingRoom, LightStatus::OFF, DioMessage::new(0x1337, dio::Status::OFF)),
            (Room::BedRoom, LightStatus::ON, DioMessage::new(0x985c, dio::Status::ON)),
            (Room::BedRoom, LightStatus::OFF, DioMessage::new(0x985c, dio::Status::OFF)),
            (Room::Kitchen, LightStatus::ON, DioMessage::new(0x0042, dio::Status::ON)),
            (Room::Kitchen, LightStatus::OFF, DioMessage::new(0x0042, dio::Status::OFF)),
        ] {
            let radio = InMemoryRadio::new();
            let house = MyHouse::new(radio);
            house.light(room, status);
            let received = house.radio.lock().unwrap().received(message, &DIO_PROTOCOL);
            assert_that!(&received, eq(true));
        }
    }

    #[test]
    fn blinds() {
        for (room, status, message, protocol) in vec![
            (Room::LivingRoom, BlindStatus::DOWN, DioMessage::new(0x0932, dio::Status::DOWN), &DIO_PROTOCOL),
            (Room::LivingRoom, BlindStatus::UP, DioMessage::new(0x0932, dio::Status::UP), &DIO_PROTOCOL),
            (Room::Kitchen, BlindStatus::DOWN, DioMessage::new(0x2600, dio::Status::DOWN), &DIO_PROTOCOL),
            (Room::Kitchen, BlindStatus::UP, DioMessage::new(0x2600, dio::Status::UP), &DIO_PROTOCOL),
        ] {
            let radio = InMemoryRadio::new();
            let house = MyHouse::new(radio);
            house.blinds(room, status);
            let received = house.radio.lock().unwrap().received(message, protocol);
            assert_that!(&received, eq(true));
        }
        for (room, status, message, protocol) in vec![
            (Room::BedRoom, BlindStatus::DOWN, celexon::Status::DOWN, &celexon::CELEXON_PROTOCOL),
            (Room::BedRoom, BlindStatus::UP, celexon::Status::UP, &celexon::CELEXON_PROTOCOL),
        ] {
            let radio = InMemoryRadio::new();
            let house = MyHouse::new(radio);
            house.blinds(room, status);
            let received = house.radio.lock().unwrap().received(message, protocol);
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
            let house = MyHouse::new(radio);
            house.screen(status);
            let received = house.radio.lock().unwrap().received(message, &DOOYA_PROTOCOL);
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
        for string in &["BedRoom", "BED_ROOM", "bed_room", "Bedroom"] {
            assert_eq!(string.parse::<Room>().unwrap(), Room::BedRoom);
        }
        assert_eq!("plop".parse::<Room>().is_err(), true);
    }
}