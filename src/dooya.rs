use crate::radio_protocol::*;

pub enum Status {
    UP,
    DOWN,
}

impl Message for Status {
    fn as_iter(&self) -> Box<Iterator<Item=u8>> {
        match self {
            Status::DOWN => Box::new(vec![0x06, 0x41, 0xdf, 0xd1, 0x33].into_iter()),
            Status::UP => Box::new(vec![0x06, 0x41, 0xdf, 0xd1, 0x11].into_iter()),
        }
    }
}

lazy_static! {
    pub static ref DOOYA_PROTOCOL: RadioProtocol<Status> = {
        RadioProtocol::<Status>::new(
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
        let m = Status::DOWN;
        let bytes = m.as_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x06,0x41,0xdf,0xd1,0x33]));
    }

    #[test]
    fn transforms_to_bytes_up() {
        let m = Status::UP;
        let bytes = m.as_iter().collect::<Vec<u8>>();
        assert_that!(&bytes, contains_in_order(vec![0x06,0x41,0xdf,0xd1,0x11]));
    }
}