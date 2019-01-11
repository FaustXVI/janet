use crate::pin::Switchable;
use std::time::Duration;

const T_TIME: Duration = Duration::from_micros(400);
const H_TIME: Duration = Duration::from_micros(2400);

fn zero(radio: &impl Switchable) -> () {
    radio.switch_off_during(T_TIME * 2);
    radio.switch_on_during(T_TIME);
}

fn one(radio: &impl Switchable) -> () {
    radio.switch_off_during(T_TIME);
    radio.switch_on_during(T_TIME * 2);
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

    #[test]
    fn send_a_zero() {
        let signal_pin = InMemoryPin::new();
        zero(&signal_pin);
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (OFF, Duration::from_micros(800)),
        (ON, Duration::from_micros(400)),
        ]));
    }

    #[test]
    fn send_a_one() {
        let signal_pin = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        one(&signal_pin);
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (OFF, Duration::from_micros(400)),
        (ON, Duration::from_micros(800)),
        ]));
    }

    #[test]
    fn send_header() {
        let signal_pin = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        header(&signal_pin);
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (ON, Duration::from_micros(2400)),
        ]));
    }

    #[test]
    fn send_footer() {
        let signal_pin = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        footer(&signal_pin);
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        (OFF, Duration::from_micros(24000)),
        ]));
    }
}