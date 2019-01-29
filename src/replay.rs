use crate::pin::DigitalOutput;
use std::time::Duration;

pub struct Replay<T: DigitalOutput> {
    output: T
}

impl<T: DigitalOutput> Replay<T> {
    pub fn new(output: T) -> Self {
        Replay { output }
    }

    pub fn play(&self, timings: &[u64]) {
        for (index, timing) in timings.iter().enumerate() {
            if index % 2 == 0 {
                self.output.low_during(Duration::from_micros(*timing))
            } else {
                self.output.high_during(Duration::from_micros(*timing))
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
    fn replay_a_saved_code() {
        let replay = Replay::new(InMemoryPin::new());
        replay.play(&vec![500, 23, 10]);
        let states = replay.output.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (LOW, Duration::from_micros(500)),
        (HIGH, Duration::from_micros(23)),
        (LOW, Duration::from_micros(10)),
        ]));
    }
}