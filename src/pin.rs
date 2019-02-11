use sysfs_gpio::Pin;
use std::time::Duration;
use std::thread::sleep;

pub trait DigitalOutput {
    fn high_during(&self, duration: Duration) -> ();
    fn low_during(&self, duration: Duration) -> ();
}

const LOW: u8 = 0;
const HIGH: u8 = 1;

impl DigitalOutput for Pin {
    fn high_during(&self, duration: Duration) -> () {
        self.set_value(HIGH).unwrap();
        sleep(duration);
    }

    fn low_during(&self, duration: Duration) -> () {
        self.set_value(LOW).unwrap();
        sleep(duration);
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    pub enum PinState {
        HIGH,
        LOW,
    }

    pub struct InMemoryPin {
        pub states: RefCell<Vec<(PinState, Duration)>>
    }

    impl InMemoryPin {
        pub fn new() -> Self {
            InMemoryPin {
                states: RefCell::new(Vec::new())
            }
        }
    }

    impl DigitalOutput for InMemoryPin {
        fn high_during(&self, duration: Duration) -> () {
            self.states.borrow_mut().push((PinState::HIGH, duration));
        }

        fn low_during(&self, duration: Duration) -> () {
            self.states.borrow_mut().push((PinState::LOW, duration));
        }
    }
}