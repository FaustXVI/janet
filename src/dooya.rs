use crate::radio_protocol::*;
use std::time::Duration;
use crate::radio::Signal;

pub enum Status {
    UP,
    DOWN,
}

impl IntoIterator for Status {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Status::DOWN => vec![0x06, 0x41, 0xdf, 0xd1, 0x33].into_iter(),
            Status::UP => vec![0x06, 0x41, 0xdf, 0xd1, 0x11].into_iter(),
        }
    }
}

lazy_static! {
    pub static ref DOOYA_PROTOCOL: RadioProtocol<Status> = {
        RadioProtocol::<Status>::new(
            Header(vec![Signal::HIGH(Duration::from_micros(4700)), Signal::LOW(Duration::from_micros(1400))]),
            Footer(vec![Signal::LOW(Duration::from_micros(8500))]),
            Zero(vec![Signal::HIGH(Duration::from_micros(250)), Signal::LOW(Duration::from_micros(650))]),
            One(vec![Signal::HIGH(Duration::from_micros(600)), Signal::LOW(Duration::from_micros(300))]),
            10)
    };
}


#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;

    #[test]
    fn transforms_to_bytes_down() {
        let m = Status::DOWN;
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x06,0x41,0xdf,0xd1,0x33]));
    }

    #[test]
    fn transforms_to_bytes_up() {
        let m = Status::UP;
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x06,0x41,0xdf,0xd1,0x11]));
    }
}