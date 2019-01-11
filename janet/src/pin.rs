use sysfs_gpio::Pin;
use crate::led::Switchable;

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

    pub struct InMemoryPin {
        pub states: RefCell<Vec<bool>>
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
            self.states.borrow_mut().push(true);
        }

        fn switch_off(&self) -> () {
            self.states.borrow_mut().push(false);
        }
    }
}