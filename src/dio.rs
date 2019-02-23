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
    pub fn new(address: u16, status: Status) -> Self {
        let brand = 0x27;
        DioMessage {
            brand,
            address,
            status: match status {
                Status::ON | Status::DOWN => 0x90,
                _ => 0x80,
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
    // FIXME there is a 100 micro seconds more sent by janet
        RadioProtocol::<DioMessage>::new(
            Header(vec![Signal::HIGH(Duration::from_micros(183)), Signal::LOW(Duration::from_micros(2693))]),
            Footer(vec![Signal::HIGH(Duration::from_micros(183)), Signal::LOW(Duration::from_micros(10440))]),
            Zero(vec![Signal::HIGH(Duration::from_micros(183)), Signal::LOW(Duration::from_micros(183)), Signal::HIGH(Duration::from_micros(183)), Signal::LOW(Duration::from_micros(1255))]),
            One(vec![Signal::HIGH(Duration::from_micros(183)), Signal::LOW(Duration::from_micros(1255)), Signal::HIGH(Duration::from_micros(183)), Signal::LOW(Duration::from_micros(183))]),
            10)
    };
}


#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;

    #[test]
    fn transforms_to_bytes_down() {
        let m = DioMessage::new(0x1234, Status::DOWN);
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x27,0x12,0x34,0x90]));
    }

    #[test]
    fn transforms_to_bytes_up() {
        let m = DioMessage::new(0x1234, Status::UP);
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x27,0x12,0x34,0x80]));
    }


}