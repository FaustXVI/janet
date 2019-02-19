use crate::pin::DigitalOutput;
use std::time::Duration;

#[derive(Copy, Clone)]
pub enum FrequencyLevel {
    HIGH,
    LOW,
}

#[derive(Copy, Clone)]
pub struct Signal(pub FrequencyLevel, pub Duration);

pub trait Radio {
    fn play(&self, signals: &[Signal]);
    // fn sendMessage(&self, message,protocol); remove play fn
}

impl<T: DigitalOutput> Radio for T {
    fn play(&self, signals: &[Signal]) {
        for Signal(f, d) in signals {
            match *f {
                FrequencyLevel::HIGH => self.high_during(*d),
                FrequencyLevel::LOW => self.low_during(*d)
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

    #[test]
    fn replay_timings() {
        let replay = InMemoryPin::new();
        replay.play(&vec![
            Signal(FrequencyLevel::HIGH, Duration::from_micros(500)),
            Signal(FrequencyLevel::LOW, Duration::from_micros(23)),
            Signal(FrequencyLevel::HIGH, Duration::from_micros(10)),
            Signal(FrequencyLevel::LOW, Duration::from_micros(20)),
        ]);
        let states = replay.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (PinState::HIGH, Duration::from_micros(500)),
        (PinState::LOW, Duration::from_micros(23)),
        (PinState::HIGH, Duration::from_micros(10)),
        (PinState::LOW, Duration::from_micros(20)),
        ]));
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;

    pub struct InMemoryRadio {
        pub timings: RefCell<Vec<Signal>>
    }

    impl InMemoryRadio {
        pub fn new() -> Self {
            InMemoryRadio { timings: RefCell::new(vec![]) }
        }
    }

    impl Radio for InMemoryRadio {
        fn play(&self, signals: &[Signal]) {
            for s in signals{
                self.timings.borrow_mut().push(*s);
            }
        }
    }
}