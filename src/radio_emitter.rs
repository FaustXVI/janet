pub enum Order {
    LittleEndian,
    LeastSignificant,
}

impl IntoIterator for Order {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Order::LittleEndian => {
                let v: Vec<u8> = (0..8).rev().collect();
                v.into_iter()
            }
            Order::LeastSignificant => {
                let v: Vec<u8> = (0..4).rev().collect();
                v.into_iter()
            }
        }
    }
}

fn most_significant_bits(data: u16) -> u8 {
    ((data & 0xFF00) >> 8) as u8
}

fn least_significant_bits(data: u16) -> u8 {
    (data & 0x00FF) as u8
}

pub trait RadioEmitter {
    fn send_bits(&self, data: u8, range: impl IntoIterator<Item=u8>) -> ();
    fn send_byte(&self, data: u8) -> () {
        self.send_bits(data, Order::LittleEndian);
    }

    fn send_2bytes(&self, data: u16) -> () {
        self.send_byte(most_significant_bits(data));
        self.send_byte(least_significant_bits(data));
    }
    fn header(&self) -> ();
    fn footer(&self) -> ();
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;
    use galvanic_assert::matchers::collection::*;

    #[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
    pub enum Sent {
        HEADER,
        FOOTER,
        DATA(u8, Vec<u8>),
    }

    pub struct InMemoryRadioEmitter {
        pub states: RefCell<Vec<Sent>>
    }

    impl InMemoryRadioEmitter {
        pub fn new() -> Self {
            InMemoryRadioEmitter { states: RefCell::new(vec![]) }
        }
    }

    impl RadioEmitter for InMemoryRadioEmitter {
        fn send_bits(&self, data: u8, range: impl IntoIterator<Item=u8>) -> () {
            self.states.borrow_mut().push(Sent::DATA(data, range.into_iter().collect()))
        }

        fn header(&self) -> () {
            self.states.borrow_mut().push(Sent::HEADER)
        }

        fn footer(&self) -> () {
            self.states.borrow_mut().push(Sent::FOOTER)
        }
    }

    #[test]
    fn send_header() {
        let emitter = InMemoryRadioEmitter::new();
        emitter.header();
        let states = emitter.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        Sent::HEADER
        ]));
    }

    #[test]
    fn send_footer() {
        let emitter = InMemoryRadioEmitter::new();
        emitter.footer();
        let states = emitter.states.into_inner();
        assert_that!(&states, contains_in_order(vec![
        Sent::FOOTER
        ]));
    }

    #[test]
    fn send_data() {
        let emitter = InMemoryRadioEmitter::new();
        emitter.send_bits(10, Order::LittleEndian);
        emitter.send_bits(42, Order::LeastSignificant);
        emitter.send_byte(0x13);
        emitter.send_2bytes(0xabcd);
        let states = emitter.states.into_inner();
        let expected = vec![
            Sent::DATA(10, vec![7, 6, 5, 4, 3, 2, 1, 0]),
            Sent::DATA(42, vec![3, 2, 1, 0]),
            Sent::DATA(0x13, Order::LittleEndian.into_iter().collect()),
            Sent::DATA(0xab, Order::LittleEndian.into_iter().collect()),
            Sent::DATA(0xcd, Order::LittleEndian.into_iter().collect()),
        ];
        assert_that!(&states, contains_in_order(expected));
    }
}