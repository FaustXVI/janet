use crate::pin::DigitalOutput;
use std::time::Duration;

pub struct Replay<T: DigitalOutput> {
    output: T
}

pub trait Replayer {
    fn play(&self, timings: &[u64]);
}

impl<T: DigitalOutput> Replay<T> {
    pub fn new(output: T) -> Self {
        Replay { output }
    }
}

impl<T: DigitalOutput> Replayer for Replay<T> {
    fn play(&self, timings: &[u64]) {
        if timings.len() % 2 == 0 {
            for (index, timing) in timings.iter().enumerate() {
                if index % 2 == 0 {
                    self.output.high_during(Duration::from_micros(*timing))
                } else {
                    self.output.low_during(Duration::from_micros(*timing))
                }
            }
        }
    }
}

#[cfg(test)]
pub mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use crate::pin::mock::InMemoryPin;
    use crate::pin::mock::PinState::*;

    #[test]
    fn replay_timings() {
        let replay = Replay::new(InMemoryPin::new());
        replay.play(&vec![500, 23, 10, 20]);
        let states = replay.output.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (HIGH, Duration::from_micros(500)),
        (LOW, Duration::from_micros(23)),
        (HIGH, Duration::from_micros(10)),
        (LOW, Duration::from_micros(20)),
        ]));
    }

    #[test]
    fn ignore_malformed_data() {
        let replay = Replay::new(InMemoryPin::new());
        replay.play(&vec![500, 23, 10]);
        let states = replay.output.states.into_inner();
        assert_that!(&states, contains_in_order(vec![]));
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;

    pub struct InMemoryReplayer {
        pub timings: RefCell<Vec<u64>>
    }

    impl InMemoryReplayer {
        pub fn new() -> Self {
            InMemoryReplayer { timings: RefCell::new(vec![]) }
        }
    }

    impl Replayer for InMemoryReplayer {
        fn play(&self, timings: &[u64]) {
            self.timings.borrow_mut().extend(timings);
        }
    }
}