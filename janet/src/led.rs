use std::time::Duration;
use crate::pin::Switchable;

pub fn blink(led: &impl Switchable, mut pause: impl FnMut(Duration) -> ()) -> ()
{
    for _ in 0..10 {
        led.switch_on();
        pause(Duration::from_millis(500));
        led.switch_off();
        pause(Duration::from_millis(500));
    };
}


#[cfg(test)]
mod should {
    use super::*;
    use crate::pin::mock::InMemoryPin;
    use crate::pin::mock::PinState::*;

    #[test]
    fn alternate_on_and_off() {
        let led = InMemoryPin::new();
        let mut durations: Vec<Duration> = vec![];
        blink(&led, |d| { durations.push(d) });
        let vec = led.states.into_inner();
        let truthfull = vec.iter().filter(|&b| b.eq(&ON)).count();
        let falsefull = vec.iter().filter(|&b| b.eq(&OFF)).count();
        assert_eq!(truthfull, 10);
        assert_eq!(truthfull, falsefull);
        assert_eq!(durations.len(), truthfull + falsefull)
    }
}