extern crate sysfs_gpio;

use janet::blyss_sender::Status;
use std::env;
use janet::raspberry::create_house;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status = from_command_line(args);
    match status {
        Some(status) => {
            let house = create_house();
            house.light(status);
        }
        None => println!("usage : janet [On|Off]")
    }
}

fn from_command_line(arg: Vec<String>) -> Option<Status> {
    let a = arg.get(0).unwrap();
    parse(a.clone())
}

fn parse(arg: String) -> Option<Status> {
    match arg.as_str() {
        "On" => Some(Status::On),
        "Off" => Some(Status::Off),
        _ => None
    }
}