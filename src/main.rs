#![feature(proc_macro_hygiene, decl_macro)]

extern crate sysfs_gpio;

use janet::blyss_sender::Status;
use janet::raspberry::create_house;
use janet::raspberry::create_fake_house;
use rocket::State;
use janet::house::House;
use std::sync::Mutex;
use std::sync::Arc;
use janet::house::MyHouse;
use janet::blyss_sender::MessageSender;
use janet::blyss::Blyss;
use janet::raspberry::FakeDigitalOutput;
use std::sync::MutexGuard;
use rocket::Response;
use rocket::http::ContentType;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use rocket::request::Form;


#[macro_use]
extern crate rocket;

pub struct Resolver {
    house: Arc<Mutex<House + Send>>
}

impl Resolver {
    fn get_house(&self) -> MutexGuard<House + Send + 'static> {
        self.house.lock().unwrap()
    }
}

#[derive(FromForm)]
struct NewStatus{
    status: String
}

#[post("/light", data = "<status>")]
fn light(resolver: State<Resolver>, status: Form<NewStatus>) -> Redirect {
    if let Some(status) = parse(status.into_inner().status) {
        let house = resolver.get_house();
        house.light(status);
    }
    Redirect::to("/")
}

fn main() {
    let house = create_fake_house();
    let resolver = Resolver {
        house: Arc::new(Mutex::new(house))
    };
    rocket::ignite()
        .attach(Template::fairing())
        .manage(resolver)
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![light]).launch();
}

//fn main() {
//    let args: Vec<String> = env::args().collect();
//    let status = from_command_line(args);
//    match status {
//        Some(status) => {
//            let house = create_house();
//            house.light(status);
//        }
//        None => println!("usage : janet [On|Off]")
//    }
//}

fn parse(arg: String) -> Option<Status> {
    match arg.as_str() {
        "On" => Some(Status::On),
        "Off" => Some(Status::Off),
        _ => None
    }
}