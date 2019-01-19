pub trait Sender {
    type Message;
    fn send(&self, message: Self::Message);
}


#[cfg(test)]
pub mod mock {
    use crate::sender::Sender;
    use std::cell::RefCell;

    pub struct InMemorySender<T> {
        pub messages: RefCell<Vec<T>>
    }

    impl<T> InMemorySender<T> {
        pub fn new() -> Self {
            InMemorySender { messages: RefCell::new(Vec::new()) }
        }
    }

    impl<T> Sender for InMemorySender<T> {
        type Message = T;

        fn send(&self, message: <Self as Sender>::Message) {
            self.messages.borrow_mut().push(message);
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use super::mock::*;
    use galvanic_assert::matchers::collection::*;


    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct InMemoryMessage {
        payload: String
    }

    impl InMemoryMessage {
        pub fn new(payload: String) -> Self {
            InMemoryMessage { payload }
        }
    }

    #[test]
    fn send_message() {
        let sender = InMemorySender::new();
        let message = InMemoryMessage::new("payload".to_string());
        sender.send(message.clone());
        let messages = sender.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
        message,
        ]));
    }
}