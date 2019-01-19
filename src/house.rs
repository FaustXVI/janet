use crate::sender::Sender;
use crate::blyss_sender::Status;
use crate::blyss_sender::BlyssMessage;
use crate::blyss_sender::Channel;
use crate::blyss_sender::SubChannel;

pub struct MyHouse<T: Sender<Message=BlyssMessage>> {
    light: Box<T>
}

pub trait House {
    fn light(&self, status: Status);
}

impl<T: Sender<Message=BlyssMessage>> MyHouse<T> {
    pub fn new(light: T) -> Self {
        MyHouse { light: Box::new(light) }
    }
}

impl<T: Sender<Message=BlyssMessage>> House for MyHouse<T> {
    fn light(&self, status: Status) {
        let message = BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, status);
        self.light.send(message);
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use crate::sender::mock::*;

    #[test]
    fn switch_on() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let house = MyHouse::new(sender);
        house.light(Status::On);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::On),
        ]));
    }

    #[test]
    fn switch_off() {
        let sender: InMemorySender<BlyssMessage> = InMemorySender::new();
        let house = MyHouse::new(sender);
        house.light(Status::Off);
        let messages = house.light.messages.into_inner();
        assert_that!(&messages, contains_in_order(vec![
            BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::Off),
        ]));
    }
}