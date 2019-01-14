use std::time::Duration;
use crate::pin::Switchable;

pub fn blink(led: &impl Switchable) -> ()
{
    for _ in 0..10 {
        led.switch_on_during(Duration::from_millis(500));
        led.switch_off_during(Duration::from_millis(500));
    };
}


#[cfg(test)]
mod should {
    use super::*;
    use crate::pin::mock::InMemoryPin;
    use crate::pin::mock::PinState::*;
    use galvanic_assert::matchers::collection::*;
    use crate::pin::mock::PinState;

    #[test]
    fn alternate_on_and_off() {
        let led = InMemoryPin::new();
        blink(&led);
        let vec = led.states.into_inner();
        let one_blink = vec![
            (ON, Duration::from_millis(500)),
            (OFF, Duration::from_millis(500)),
        ];
        let ten_blinks: Vec<(PinState, Duration)> = one_blink.iter().cycle()
            .take(20)
            .map(|t|t.to_owned()).collect();
        assert_that!(&vec, contains_in_order(ten_blinks));
    }
}