use crate::radio_emitter::RadioEmitter;
use crate::radio_emitter::Order;
use crate::sender::Sender;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    On = 0x00,
    Off = 0x01,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubChannel {
    Channel1 = 0x08,
    Channel2 = 0x04,
    Channel3 = 0x02,
    Channel4 = 0x01,
    Channel5 = 0x03,
    AllChannels = 0x00,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Channel {
    ChannelA = 0x00,
    ChannelB = 0x01,
    ChannelC = 0x02,
    ChannelD = 0x03,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlyssMessage {
    timestamp: u8,
    rolling_code: u8,
    status: Status,
    sub_channel: SubChannel,
    channel: Channel,
    address: u16,
    brand: u8,
}

impl BlyssMessage {
    /*
    const byte RF_ROLLING_CODE[] = {
        0x98, 0xDA, 0x1E, 0xE6, 0x67
    };
    */
    pub fn new(address: u16, channel: Channel, sub_channel: SubChannel, status: Status) -> Self {
        BlyssMessage {
            timestamp: status as u8,
            brand: 0xFE,
            rolling_code: 0x98,
            address,
            channel,
            sub_channel,
            status,
        }
    }
}

pub struct MessageSender<T: RadioEmitter> {
    radio: T
}

impl<T: RadioEmitter> MessageSender<T> {
    pub fn new(radio: T) -> Self {
        MessageSender { radio}
    }
}

const PADDING_VALUE: u8 = 0;

impl<T: RadioEmitter> Sender for MessageSender<T> {
    type Message = BlyssMessage;

    fn send(&self, message: Self::Message) {
        for _ in 0..13 {
            self.radio.header();
            self.radio.send_byte(message.brand);
            self.radio.send_bits(message.channel as u8, Order::LeastSignificant);
            self.radio.send_2bytes(message.address);
            self.radio.send_bits(message.sub_channel as u8, Order::LeastSignificant);
            self.radio.send_bits(message.status as u8, Order::LeastSignificant);
            self.radio.send_byte(message.rolling_code as u8);
            self.radio.send_byte(message.timestamp as u8);
            self.radio.send_bits(PADDING_VALUE, Order::LeastSignificant);
            self.radio.footer();
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use crate::radio_emitter::mock::InMemoryRadioEmitter;
    use crate::radio_emitter::mock::Sent;

    #[test]
    fn send_data() {
        let emitter = MessageSender::new(InMemoryRadioEmitter::new());
        let message = BlyssMessage::new(0x7057, Channel::ChannelC, SubChannel::Channel1, Status::On);
        emitter.send(message);
        let sent = emitter.radio.states.into_inner();
        let full_byte: Vec<u8> = Order::LittleEndian.into_iter().collect();
        let least_significant_bits: Vec<u8> = Order::LeastSignificant.into_iter().collect();
        let expected = repeat_13_times(vec![
            Sent::HEADER,
            Sent::DATA(0xFE, full_byte.clone()),
            Sent::DATA(0x02, least_significant_bits.clone()),
            Sent::DATA(0x70, full_byte.clone()),
            Sent::DATA(0x57, full_byte.clone()),
            Sent::DATA(0x08, least_significant_bits.clone()),
            Sent::DATA(0x00, least_significant_bits.clone()),
            Sent::DATA(0x98, full_byte.clone()),
            Sent::DATA(0x00, full_byte.clone()),
            Sent::DATA(0x00, least_significant_bits.clone()),
            Sent::FOOTER,
        ]);
        assert_that!(&sent, contains_in_order(expected));
    }

    fn repeat_13_times(sent: Vec<Sent>) -> Vec<Sent> {
        vec![sent].iter().cycle().take(13).flat_map(|t| t.iter())
            .map(|t| t.to_owned()).collect()
    }

}