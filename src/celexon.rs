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
            Status::DOWN => vec![0x53, 0xe1, 0x3e, 0xd4, 0xfe, 0xff, 0xbc, 0xb1].into_iter(),
            Status::UP => vec![0x53, 0xe1, 0x3e, 0xd4, 0xfe, 0xff, 0xf4, 0xe9].into_iter(),
        }
    }
}

lazy_static! {
    pub static ref CELEXON_PROTOCOL: RadioProtocol<Status> = {
        RadioProtocol::<Status>::new(
            Header(vec![ Signal::HIGH(Duration::from_micros(5164)),Signal::LOW(Duration::from_micros(581))]),
            Footer(vec![Signal::HIGH(Duration::from_micros(581)),Signal::LOW(Duration::from_micros(248))]),
            Zero(vec![Signal::HIGH(Duration::from_micros(581)), Signal::LOW(Duration::from_micros(199))]),
            One(vec![Signal::HIGH(Duration::from_micros(199)), Signal::LOW(Duration::from_micros(581))]),
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

        assert_that!(&bytes, contains_in_order(vec![0x53,0xe1,0x3e,0xd4,0xfe,0xff,0xbc,0xb1]));
    }

    #[test]
    fn transforms_to_bytes_up() {
        let m = Status::UP;
        let bytes = m.into_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x53,0xe1,0x3e,0xd4,0xfe,0xff,0xf4,0xe9]));
    }
}