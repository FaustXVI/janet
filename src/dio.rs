use crate::radio_protocol::*;
use std::time::Duration;
use crate::radio::Signal;

pub enum Status {
    ON,
    OFF,
    UP,
    DOWN,
}

pub struct DioMessage {
    brand: u8,
    address: u16,
    status: u8,
}

impl DioMessage {
    pub fn new(address: u32, status: Status) -> Self {
        let brand =((0x00FF0000 & address) >> 16) as u8;
        let last_bit = (brand == 0x7c) as u8;
        DioMessage {
            brand ,
            address : (0x0000FFFF & address) as u16,
            status: match status {
                Status::ON | Status::DOWN => 0x90 | last_bit,
                _ => 0x80 | last_bit,
            },
        }
    }
}

impl IntoIterator for DioMessage {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.brand, ((self.address & 0xFF00) >> 8) as u8, (self.address & 0xFF) as u8, self.status].into_iter()
    }
}

lazy_static! {
    pub static ref DIO_PROTOCOL: RadioProtocol<DioMessage> = {
        RadioProtocol::<DioMessage>::new(
            Header(vec![Signal::HIGH(Duration::from_micros(283)), Signal::LOW(Duration::from_micros(2793))]),
            Footer(vec![Signal::HIGH(Duration::from_micros(283)), Signal::LOW(Duration::from_micros(10740))]),
            Zero(vec![Signal::HIGH(Duration::from_micros(283)), Signal::LOW(Duration::from_micros(283)), Signal::HIGH(Duration::from_micros(283)), Signal::LOW(Duration::from_micros(1355))]),
            One(vec![Signal::HIGH(Duration::from_micros(283)), Signal::LOW(Duration::from_micros(1355)), Signal::HIGH(Duration::from_micros(283)), Signal::LOW(Duration::from_micros(283))]),
            10)
    };
}


#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;

    #[test]
    fn transforms_to_bytes_down() {
        let m = DioMessage::new(0x271234, Status::DOWN);
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x27,0x12,0x34,0x90]));
    }

    #[test]
    fn transforms_to_bytes_down_7c_version() {
        let m = DioMessage::new(0x7c1234, Status::DOWN);
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x7c,0x12,0x34,0x91]));
    }

    #[test]
    fn transforms_to_bytes_up() {
        let m = DioMessage::new(0x271234, Status::UP);
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x27,0x12,0x34,0x80]));
    }


    #[test]
    fn transforms_to_bytes_up_7c_version() {
        let m = DioMessage::new(0x7c1234, Status::UP);
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x7c,0x12,0x34,0x81]));
    }
}