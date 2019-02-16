use crate::radio_protocol::*;

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
        DioMessage {
            brand: 0x27,
            address,
            status: match status {
                Status::ON | Status::DOWN => 0x90,
                _ => 0x80,
            },
        }
    }
}

impl Message for DioMessage {
    fn as_iter(&self) -> Box<Iterator<Item=u8>> {
        Box::new(vec![self.brand, ((self.address & 0xFF00) >> 8) as u8, (self.address & 0xFF) as u8, self.status].into_iter())
    }
}

lazy_static! {
    pub static ref DIO_PROTOCOL: RadioProtocol<DioMessage> = {
        RadioProtocol::<DioMessage>::new(
            Header(vec![283, 2793]),
            Footer(vec![283, 10740]),
            Zero(vec![283, 283, 283, 1355]),
            One(vec![283, 1355, 283, 283]))
    };
}


#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;

    #[test]
    fn transforms_to_bytes_down() {
        let m = DioMessage::new(0x1234, Status::DOWN);
        let bytes = m.as_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x27,0x12,0x34,0x90]));
    }

    #[test]
    fn transforms_to_bytes_up() {
        let m = DioMessage::new(0x1234, Status::UP);
        let bytes = m.as_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x27,0x12,0x34,0x80]));
    }
}