use std::time::Duration;


pub trait Switchable {
    fn switch_on(&self) -> ();
    fn switch_off(&self) -> ();
}

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
    use std::cell::RefCell;

    pub struct FakeLed {
        states: RefCell<Vec<bool>>
    }

    impl FakeLed {
        pub fn new() -> Self {
            FakeLed {
                states: RefCell::new(Vec::new())
            }
        }
    }

    impl Switchable for FakeLed {
        fn switch_on(&self) -> () {
            self.states.borrow_mut().push(true);
        }

        fn switch_off(&self) -> () {
            self.states.borrow_mut().push(false);
        }
    }

    #[test]
    fn alternate_on_and_off() {
        let led = FakeLed::new();
        let mut durations: Vec<Duration> = vec![];
        blink(&led, |d| { durations.push(d) });
        let vec = led.states.into_inner();
        let truthfull = vec.iter().filter(|&&b| b).count();
        let falsefull = vec.iter().filter(|&&b| b).count();
        assert_eq!(truthfull, 5);
        assert_eq!(truthfull, falsefull);
        assert_eq!(durations.len(), truthfull + falsefull)
    }
}