use std::marker::PhantomData;
use std::time::Duration;

pub struct Header(pub Vec<u64>);

pub struct Footer(pub Vec<u64>);

pub struct Zero(pub Vec<u64>);

pub struct One(pub Vec<u64>);

pub struct RadioProtocol<T> {
    header: Vec<u64>,
    footer: Vec<u64>,
    zero: Vec<u64>,
    one: Vec<u64>,
    message: PhantomData<T>,
}

impl<T> RadioProtocol<T> {
    pub fn new(header: Header, footer: Footer, zero: Zero, one: One) -> Self {
        RadioProtocol {
            header: header.0,
            footer: footer.0,
            zero: zero.0,
            one: one.0,
            message: PhantomData,
        }
    }
}

type Timings = Vec<u64>;

struct Byte(u8);

impl Byte {
    fn at(&self, index: u8) -> u8 {
        self.0 & (1 << index)
    }
    fn iter(&self) -> impl Iterator<Item=u8> {
        let l: Vec<u8> = (0..8).rev().map(|i| self.at(i)).collect();
        l.into_iter()
    }
}


impl<T: IntoIterator<Item=u8>> RadioProtocol<T> {
    pub fn timings_for(&self, message: T) -> Timings {
        let mut r = vec![];
        r.push(self.header.clone());
        r.append(&mut message.into_iter()
            .map(|b| self.timings_for_byte(Byte(b)))
            .collect());
        r.push(self.footer.clone());
        r.concat()
    }

    fn timings_for_byte(&self, byte: Byte) -> Timings {
        byte.iter().map(|b|
            if b == 0 {
                return self.zero.clone();
            } else {
                return self.one.clone();
            }
        ).collect::<Vec<Timings>>().concat()
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;

    struct FakeMessage(pub Vec<u8>);

    impl IntoIterator for FakeMessage {
        type Item = u8;
        type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    #[test]
    fn send_byte() {
        let protocol = RadioProtocol::new(Header(vec![13]), Footer(vec![37]), Zero(vec![0]), One(vec![1]));
        let timings = protocol.timings_for(FakeMessage(vec![5]));
        assert_that!(&timings, contains_in_order(vec![13,0,0,0,0,0,1,0,1,37]));
    }

    #[test]
    fn send_bytes() {
        let z = 10;
        let o = 11;
        let protocol = RadioProtocol::new(Header(vec![4, 2]), Footer(vec![13, 37]), Zero(vec![z]), One(vec![o]));
        let timings = protocol.timings_for(FakeMessage(vec![3, 7]));
        assert_that!(&timings, contains_in_order(vec![4,2,z,z,z,z,z,z,o,o,z,z,z,z,z,o,o,o,13,37]));
    }
}