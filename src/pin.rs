use sysfs_gpio::Pin;
use std::time::Duration;
use std::thread::sleep;

pub trait Switchable {
    fn switch_on_during(&self, duration: Duration) -> ();
    fn switch_off_during(&self, duration: Duration) -> ();
}

impl Switchable for Pin {
    fn switch_on_during(&self, duration: Duration) -> () {
        self.set_value(1).unwrap();
        sleep(duration);
    }

    fn switch_off_during(&self, duration: Duration) -> () {
        self.set_value(0).unwrap();
        sleep(duration);
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;
    use crate::pin::mock::PinState::*;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq,Copy, Clone)]
    pub enum PinState {
        ON,
        OFF,
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

    impl Switchable for InMemoryPin {
        fn switch_on_during(&self, duration: Duration) -> () {
            self.states.borrow_mut().push((ON, duration));
        }

        fn switch_off_during(&self, duration: Duration) -> () {
            self.states.borrow_mut().push((OFF, duration));
        }
    }
}