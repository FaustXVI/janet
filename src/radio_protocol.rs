use std::marker::PhantomData;
use crate::radio::Signal;

type Timings = Vec<Signal>;

pub struct Header(pub Timings);

pub struct Footer(pub Timings);

pub struct Zero(pub Timings);

pub struct One(pub Timings);

pub struct RadioProtocol<T> {
    header: Timings,
    footer: Timings,
    zero: Timings,
    one: Timings,
    repetition: u8,
    message: PhantomData<T>,
}

impl<T> RadioProtocol<T> {
    pub fn new(header: Header, footer: Footer, zero: Zero, one: One, repetition: u8) -> Self {
        RadioProtocol {
            header: header.0,
            footer: footer.0,
            zero: zero.0,
            one: one.0,
            repetition,
            message: PhantomData,
        }
    }
}

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
        let bytes = message.into_iter().collect::<Vec<_>>();
        for _ in 0..self.repetition {
            r.push(self.header.clone());
            r.append(&mut bytes.iter()
                .map(|&b| self.timings_for_byte(Byte(b)))
                .collect());
            r.push(self.footer.clone());
        }
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
    use std::time::Duration;

    #[test]
    fn send_byte() {
        let protocol = RadioProtocol::new(Header(vec![Signal::HIGH(Duration::from_micros(13))]),
                                          Footer(vec![Signal::LOW(Duration::from_micros(37))]),
                                          Zero(vec![Signal::HIGH(Duration::from_micros(0))]),
                                          One(vec![Signal::LOW(Duration::from_micros(1))]),
                                          1);
        let timings = protocol.timings_for(vec![5]);
        assert_that!(&timings, contains_in_order(vec![
        Signal::HIGH(Duration::from_micros(13)),
        Signal::HIGH(Duration::from_micros(0)),
        Signal::HIGH(Duration::from_micros(0)),
        Signal::HIGH(Duration::from_micros(0)),
        Signal::HIGH(Duration::from_micros(0)),
        Signal::HIGH(Duration::from_micros(0)),
        Signal::LOW(Duration::from_micros(1)),
        Signal::HIGH(Duration::from_micros(0)),
        Signal::LOW(Duration::from_micros(1)),
        Signal::LOW(Duration::from_micros(37))]));
    }

    #[test]
    fn send_bytes() {
        let z = 10;
        let o = 11;
        let protocol = RadioProtocol::new(Header(vec![Signal::HIGH(Duration::from_micros(4)), Signal::LOW(Duration::from_micros(2))]),
                                          Footer(vec![Signal::HIGH(Duration::from_micros(13)), Signal::LOW(Duration::from_micros(37))]),
                                          Zero(vec![Signal::HIGH(Duration::from_micros(z))]),
                                          One(vec![Signal::LOW(Duration::from_micros(o))]),
                                          2);
        let timings = protocol.timings_for(vec![3, 7]);
        assert_that!(&timings, contains_in_order(vec![
        Signal::HIGH(Duration::from_micros(4)),
        Signal::LOW(Duration::from_micros(2)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::HIGH(Duration::from_micros(13)),
        Signal::LOW(Duration::from_micros(37)),
        Signal::HIGH(Duration::from_micros(4)),
        Signal::LOW(Duration::from_micros(2)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::HIGH(Duration::from_micros(z)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::LOW(Duration::from_micros(o)),
        Signal::HIGH(Duration::from_micros(13)),
        Signal::LOW(Duration::from_micros(37))
        ]));
    }
}