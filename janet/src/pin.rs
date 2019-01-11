use sysfs_gpio::Pin;

pub trait Switchable {
    fn switch_on(&self) -> ();
    fn switch_off(&self) -> ();
}

impl Switchable for Pin {
    fn switch_on(&self) -> () {
        self.set_value(1).unwrap()
    }

    fn switch_off(&self) -> () {
        self.set_value(0).unwrap()
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;
    use crate::pin::mock::PinState::*;

    #[derive(Debug,Ord, PartialOrd, Eq, PartialEq)]
    pub enum PinState{
        ON,
        OFF
    }

    pub struct InMemoryPin {
        pub states: RefCell<Vec<PinState>>
    }

    impl InMemoryPin {
        pub fn new() -> Self {
            InMemoryPin {
                states: RefCell::new(Vec::new())
            }
        }
    }

    impl Switchable for InMemoryPin {
        fn switch_on(&self) -> () {
            self.states.borrow_mut().push(ON);
        }

        fn switch_off(&self) -> () {
            self.states.borrow_mut().push(OFF);
        }
    }
}