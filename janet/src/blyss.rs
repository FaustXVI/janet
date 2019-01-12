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

fn zero(radio: &impl Switchable) -> () {
    radio.switch_off_during(T_TIME * 2);
    radio.switch_on_during(T_TIME);
}

fn one(radio: &impl Switchable) -> () {
    radio.switch_off_during(T_TIME);
    radio.switch_on_during(T_TIME * 2);
}

fn send_bits(radio: &impl Switchable, data: u8, range: impl IntoIterator<Item=u8>) -> () {
    for n in range {
        let mask = 1 << n;
        if (data & mask) == 0 {
            zero(radio);
        } else {
            one(radio);
        }
    }
}

fn send_8bits(radio: &impl Switchable, data: u8) -> () {
    send_bits(radio, data, Order::LittleEndian);
}

fn header(radio: &impl Switchable) -> () {
    radio.switch_on_during(H_TIME);
}

fn footer(radio: &impl Switchable) -> () {
    radio.switch_off_during(H_TIME * 10);
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
        let signal_pin = InMemoryPin::new();
        header(&signal_pin);
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (ON, Duration::from_micros(2400)),
        ]));
    }

    #[test]
    fn send_footer() {
        let signal_pin = InMemoryPin::new();
        footer(&signal_pin);
        let states = signal_pin.states.into_inner();
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
            let signal_pin = InMemoryPin::new();
            send_8bits(&signal_pin, data);
            let states = signal_pin.states.into_inner();
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
            let signal_pin = InMemoryPin::new();
            send_bits(&signal_pin, data, Order::LeastSignificant);
            let states = signal_pin.states.into_inner();
            assert_that!(&states, contains_in_order(expected));
        }
    }

    fn flatten(binaries: Vec<Vec<(PinState, Duration)>>) -> Vec<(PinState, Duration)> {
        binaries.iter().flat_map(|t| t.iter())
            .map(|t| t.to_owned()).collect()
    }
}