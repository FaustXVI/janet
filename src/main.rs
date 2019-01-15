extern crate sysfs_gpio;

use sysfs_gpio::Pin;
use sysfs_gpio::Direction;
use std::thread::sleep;
use std::time::Duration;
use janet::message_sender::MessageSender;
use janet::blyss::Blyss;
use janet::message_sender::Message;
use janet::message_sender::Channel;
use janet::message_sender::SubChannel;
use janet::message_sender::Status;


#[macro_use]
extern crate rouille;

fn main() {
    let addr = "0.0.0.0:80";
    println!("Now listening on {}", &addr);

    // The `start_server` starts listening forever on the given address.
    rouille::start_server(addr, move |request| {
        // The closure passed to `start_server` will be called once for each client request. It
        // will be called multiple times concurrently when there are multiple clients.

        // Here starts the real handler for the request.
        //
        // The `router!` macro is very similar to a `match` expression in core Rust. The macro
        // takes the request as parameter and will jump to the first block that matches the
        // request.
        //
        // Each of the possible blocks builds a `Response` object. Just like most things in Rust,
        // the `router!` macro is an expression whose value is the `Response` built by the block
        // that was called. Since `router!` is the last piece of code of this closure, the
        // `Response` is then passed back to the `start_server` function and sent to the client.
        router!(request,
            (GET) (/) => {
                // If the request's URL is `/`, we jump here.
                // This block builds a `Response` object that redirects to the `/hello/world`.
                rouille::Response::html(r###"
                <h1>Janet</h1>
                <div style="margin:2em">
                <a href="/On">On</a>
                </div>

                <div style="margin:2em">
                <a href="/Off">Off</a>
                </div>
                "###)
            },

            (GET) (/{command: String}) => {
                // If the request's URL is for example `/foo`, we jump here.
                //
                // This route is similar to the previous one, but this time we have a `String`.
                // Parsing into a `String` never fails.
                let status = parse(command);
                match status {
                        Some(status) => {
                            let pin = Pin::new(23);
                            pin.export().unwrap();
                            if pin.set_direction(Direction::Low).is_err() {
                                sleep(Duration::from_millis(500));
                                pin.set_direction(Direction::Low).unwrap();
                            };
                            let sender = MessageSender::new(Blyss::new(pin));
                            let message = Message::new(0x7057, Channel::ChannelC, SubChannel::Channel1, status);
                            sender.send(&message);
                            rouille::Response::redirect_308("/")
                        }
                        None => rouille::Response::redirect_308("/")
                    }
                // Builds a `Response` object that contains "hello, " followed with the value
                // of `id`.
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}
/*
fn main() {
    let args: Vec<String> = env::args().collect();
    let status = from_command_line(args);
    match status {
        Some(status) => {
            let pin = Pin::new(23);
            pin.export().unwrap();
            if pin.set_direction(Direction::Low).is_err() {
                sleep(Duration::from_millis(500));
                pin.set_direction(Direction::Low).unwrap();
            };
            let sender = MessageSender::new(Blyss::new(pin));
            let message = Message::new(0x7057, Channel::ChannelC, SubChannel::Channel1, status);
            sender.send(&message);
        }
        None => println!("usage : janet [On|Off]")
    }
}
*/

fn parse(arg: String) -> Option<Status> {
    match arg.as_str() {
        "On" => Some(Status::On),
        "Off" => Some(Status::Off),
        _ => None
    }
}