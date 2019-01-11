use crate::pin::Switchable;
use std::time::Duration;

const T_TIME: Duration = Duration::from_micros(400);
const H_TIME: Duration = Duration::from_micros(2400);

fn zero(radio: &impl Switchable, mut pause: impl FnMut(Duration) -> ()) -> () {
    radio.switch_off();
    pause(T_TIME * 2);
    radio.switch_on();
    pause(T_TIME);
}

fn one(radio: &impl Switchable, mut pause: impl FnMut(Duration) -> ()) -> () {
    radio.switch_off();
    pause(T_TIME);
    radio.switch_on();
    pause(T_TIME * 2);
}

fn header(radio: &impl Switchable, mut pause: impl FnMut(Duration) -> ()) -> () {
    radio.switch_on();
    pause(H_TIME);
}

fn footer(radio: &impl Switchable, mut pause: impl FnMut(Duration) -> ()) -> () {
    radio.switch_off();
    pause(H_TIME * 10);
}


#[cfg(test)]
mod should {
    use crate::pin::mock::InMemoryPin;
    use crate::pin::mock::PinState::*;
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use std::time::Duration;
    use crate::pin::mock::PinState;

    #[test]
    fn send_a_zero() {
        let signal_pin = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        zero(&signal_pin, |d| { durations.push(d) });
        let states = signal_pin.states.into_inner();
        let state_durations: Vec<(&PinState, &Duration)> = states.iter()
            .zip(durations.iter()).collect();
        assert_that!(&state_durations, contains_in_order(vec![
        (&OFF, &Duration::from_micros(800)),
        (&ON, &Duration::from_micros(400)),
        ]));
    }

    #[test]
    fn send_a_one() {
        let signal_pin = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        one(&signal_pin, |d| { durations.push(d) });
        assert_that!(&durations, contains_in_order(vec![
        Duration::from_micros(400),
        Duration::from_micros(800),
        ]));
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![OFF,ON]))
    }

    #[test]
    fn send_header() {
        let signal_pin = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        header(&signal_pin, |d| { durations.push(d) });
        assert_that!(&durations, contains_in_order(vec![
        Duration::from_micros(2400),
        ]));
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![ON]))
    }

    #[test]
    fn send_footer() {
        let signal_pin = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        footer(&signal_pin, |d| { durations.push(d) });
        assert_that!(&durations, contains_in_order(vec![
        Duration::from_micros(24000),
        ]));
        let states = signal_pin.states.into_inner();
        assert_that!(&states, contains_in_order(vec![OFF]))
    }
}