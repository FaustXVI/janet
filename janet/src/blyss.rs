use crate::pin::Switchable;
use std::time::Duration;

const T_TIME: Duration = Duration::from_micros(400);
const H_TIME: Duration = Duration::from_micros(2400);

pub enum Order {
    LittleEndian,
    LeastSignificant,
}

impl IntoIterator for Order {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Order::LittleEndian => {
                let v: Vec<u8> = (0..8).rev().collect();
                v.into_iter()
            }
            Order::LeastSignificant => {
                let v: Vec<u8> = (0..4).rev().collect();
                v.into_iter()
            }
        }
    }
}


fn most_significant_bits(data: u16) -> u8 {
    ((data & 0xFF00) >> 8) as u8
}

fn least_significant_bits(data: u16) -> u8 {
    (data & 0x00FF) as u8
}

struct Blyss<T: Switchable> {
    emitter: Box<T>
}

impl<T: Switchable> Blyss<T> {
    fn new(emitter: Box<T>) -> Self {
        Blyss { emitter }
    }

    fn zero(&self) -> () {
        self.emitter.switch_off_during(T_TIME * 2);
        self.emitter.switch_on_during(T_TIME);
    }

    fn one(&self) -> () {
        self.emitter.switch_off_during(T_TIME);
        self.emitter.switch_on_during(T_TIME * 2);
    }

    fn send_bits(&self, data: u8, range: impl IntoIterator<Item=u8>) -> () {
        for n in range {
            let mask = 0x01 << n;
            if (data & mask) == 0 {
                self.zero();
            } else {
                self.one();
            }
        }
    }

    fn send_byte(&self, data: u8) -> () {
        self.send_bits(data, Order::LittleEndian);
    }

    fn send_2bytes(&self, data: u16) -> () {
        self.send_byte(most_significant_bits(data));
        self.send_byte(least_significant_bits(data));
    }

    fn header(&self) -> () {
        self.emitter.switch_on_during(H_TIME);
    }

    fn footer(&self) -> () {
        self.emitter.switch_off_during(H_TIME * 10);
    }
}

#[cfg(test)]
mod should {
    use crate::pin::mock::InMemoryPin;
    use crate::pin::mock::PinState::*;
    use crate::pin::mock::PinState;
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use std::time::Duration;

    macro_rules! zero {
    ()=>{
        vec![
        (OFF, Duration::from_micros(800)),
        (ON, Duration::from_micros(400))
        ]
    }
    }

    macro_rules! one {
    ()=>{
        vec![
        (OFF, Duration::from_micros(400)),
        (ON, Duration::from_micros(800))
        ]
    }
    }

    #[test]
    fn send_header() {
        let signal_pin = Blyss::new(Box::new(InMemoryPin::new()));
        signal_pin.header();
        let states = signal_pin.emitter.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (ON, Duration::from_micros(2400)),
        ]));
    }

    #[test]
    fn send_footer() {
        let signal_pin = Blyss::new(Box::new(InMemoryPin::new()));
        signal_pin.footer();
        let states = signal_pin.emitter.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (OFF, Duration::from_micros(24000)),
        ]));
    }

    #[test]
    fn send_a_byte() {
        for (data, expected) in vec![
            (0b00000000, flatten(vec![
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
            ])),
            (0b00000001, flatten(vec![
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                one!(),
            ])),
            (0b11000001, flatten(vec![
                one!(),
                one!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                one!(),
            ]))
        ] {
            let signal_pin = Blyss::new(Box::new(InMemoryPin::new()));
            signal_pin.send_byte(data);
            let states = signal_pin.emitter.states.into_inner();
            assert_that!(&states, contains_in_order(expected));
        }
    }

    #[test]
    fn send_some_bits() {
        for (data, expected) in vec![
            (0b00000000, flatten(vec![
                zero!(),
                zero!(),
                zero!(),
                zero!(),
            ])),
            (0b11110000, flatten(vec![
                zero!(),
                zero!(),
                zero!(),
                zero!(),
            ])),
            (0b10000100, flatten(vec![
                zero!(),
                one!(),
                zero!(),
                zero!(),
            ])),
            (0b10001010, flatten(vec![
                one!(),
                zero!(),
                one!(),
                zero!(),
            ])),
        ] {
            let signal_pin = Blyss::new(Box::new(InMemoryPin::new()));
            signal_pin.send_bits(data, Order::LeastSignificant);
            let states = signal_pin.emitter.states.into_inner();
            assert_that!(&states, contains_in_order(expected));
        }
    }

    #[test]
    fn send_two_bytes() {
        for (data, expected) in vec![
            (0b0000000000000000, flatten(vec![
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
                zero!(),
            ])),
            (0b1101010101010101, flatten(vec![
                one!(),
                one!(),
                zero!(),
                one!(),
                zero!(),
                one!(),
                zero!(),
                one!(),
                zero!(),
                one!(),
                zero!(),
                one!(),
                zero!(),
                one!(),
                zero!(),
                one!(),
            ]))
        ] {
            let signal_pin = Blyss::new(Box::new(InMemoryPin::new()));
            signal_pin.send_2bytes(data);
            let states = signal_pin.emitter.states.into_inner();
            assert_that!(&states, contains_in_order(expected));
        }
    }

    fn flatten(binaries: Vec<Vec<(PinState, Duration)>>) -> Vec<(PinState, Duration)> {
        binaries.iter().flat_map(|t| t.iter())
            .map(|t| t.to_owned()).collect()
    }
}