use crate::pin::DigitalOutput;
use crate::radio_protocol::RadioProtocol;
use std::time::Duration;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Signal {
    HIGH(Duration),
    LOW(Duration),
}

pub trait Radio {
    fn send<M>(&self, message: M, protocol: &RadioProtocol<M>)
        where M: IntoIterator<Item=u8>;
}

impl<T: DigitalOutput> Radio for T {
    fn send<M>(&self, message: M, protocol: &RadioProtocol<M>) where M: IntoIterator<Item=u8> {
        for s in protocol.timings_for(message) {
            match s {
                Signal::HIGH(d) => self.high_during(d),
                Signal::LOW(d) => self.low_during(d)
            }
        }
    }
}

#[cfg(test)]
pub mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use crate::pin::mock::InMemoryPin;
    use crate::pin::mock::PinState;
    use crate::radio_protocol::*;

    #[test]
    fn replay_timings() {
        let message = vec![5];
        let protocol = RadioProtocol::new(
            Header(vec![Signal::HIGH(Duration::from_micros(13))]),
            Footer(vec![Signal::LOW(Duration::from_micros(37))]),
            Zero(vec![Signal::LOW(Duration::from_micros(0))]),
            One(vec![Signal::HIGH(Duration::from_micros(1))]),
            1
        );
        let radio = InMemoryPin::new();
        radio.send(message, &protocol);
        let states = radio.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (PinState::HIGH, Duration::from_micros(13)),
        (PinState::LOW, Duration::from_micros(0)),
        (PinState::LOW, Duration::from_micros(0)),
        (PinState::LOW, Duration::from_micros(0)),
        (PinState::LOW, Duration::from_micros(0)),
        (PinState::LOW, Duration::from_micros(0)),
        (PinState::HIGH, Duration::from_micros(1)),
        (PinState::LOW, Duration::from_micros(0)),
        (PinState::HIGH, Duration::from_micros(1)),
        (PinState::LOW, Duration::from_micros(37)),
        ]));
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;

    pub struct InMemoryRadio {
        pub signals: RefCell<Option<Vec<Signal>>>,
    }

    impl InMemoryRadio {
        pub fn new() -> Self {
            InMemoryRadio { signals: RefCell::new(None) }
        }
        pub fn received<M>(&self, message: M, protocol: &RadioProtocol<M>) -> bool
            where M: IntoIterator<Item=u8> {
            match self.signals.borrow().clone() {
                Some(s) => s == protocol.timings_for(message),
                _ => false
            }
        }
    }

    impl Radio for InMemoryRadio {
        fn send<M>(&self, message: M, protocol: &RadioProtocol<M>) where M: IntoIterator<Item=u8> {
            self.signals.replace(Some(protocol.timings_for(message)));
        }
    }
}