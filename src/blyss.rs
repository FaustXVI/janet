use crate::pin::DigitalOutput;
use std::time::Duration;
use crate::radio_emitter::RadioEmitter;

const T_TIME: Duration = Duration::from_micros(400);
const H_TIME: Duration = Duration::from_micros(2400);

pub struct Blyss<T: DigitalOutput> {
    emitter: T
}

impl<T: DigitalOutput> Blyss<T> {
    pub fn new(emitter: T) -> Self {
        Blyss { emitter }
    }

    fn zero(&self) -> () {
        self.emitter.low_during(T_TIME * 2);
        self.emitter.high_during(T_TIME);
    }

    fn one(&self) -> () {
        self.emitter.low_during(T_TIME);
        self.emitter.high_during(T_TIME * 2);
    }
}

impl<T: DigitalOutput> RadioEmitter for Blyss<T> {
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

    fn header(&self) -> () {
        self.emitter.high_during(H_TIME);
    }

    fn footer(&self) -> () {
        self.emitter.low_during(H_TIME * 10);
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
    use crate::radio_emitter::Order;

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
        let signal_pin = Blyss::new(InMemoryPin::new());
        signal_pin.header();
        let states = signal_pin.emitter.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (ON, Duration::from_micros(2400)),
        ]));
    }

    #[test]
    fn send_footer() {
        let signal_pin = Blyss::new(InMemoryPin::new());
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
            let signal_pin = Blyss::new(InMemoryPin::new());
            signal_pin.send_bits(data, Order::LittleEndian);
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
            let signal_pin = Blyss::new(InMemoryPin::new());
            signal_pin.send_bits(data, Order::LeastSignificant);
            let states = signal_pin.emitter.states.into_inner();
            assert_that!(&states, contains_in_order(expected));
        }
    }

    fn flatten(binaries: Vec<Vec<(PinState, Duration)>>) -> Vec<(PinState, Duration)> {
        binaries.iter().flat_map(|t| t.iter())
            .map(|t| t.to_owned()).collect()
    }
}