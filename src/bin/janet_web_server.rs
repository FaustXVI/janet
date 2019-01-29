#![feature(proc_macro_hygiene, decl_macro)]

extern crate sysfs_gpio;

use rocket::State;
use janet::house::House;
use std::sync::Arc;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use rocket::request::Form;
use std::ops::Deref;
use janet::house::Room;


#[macro_use]
extern crate rocket;

pub struct SafeHouse {
    house: Arc<House + Send + Sync>
}

impl Deref for SafeHouse {
    type Target = House;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.house.as_ref()
    }
}

#[derive(FromForm)]
struct NewStatus {
    status: String
}

#[post("/light", data = "<status>")]
fn light(house: State<SafeHouse>, status: Form<NewStatus>) -> Redirect {
    if let Ok(status) = status.status.parse() {
        house.light(Room::LivingRoom, status);
    }
    Redirect::to("/")
}

#[derive(FromForm)]
struct Order {
    status: String,
    room: String,
}

#[post("/blinds", data = "<status>")]
fn blinds(house: State<SafeHouse>, status: Form<Order>) -> Redirect {
    if let (Ok(room), Ok(status)) = (status.room.parse(), status.status.parse()) {
        house.blinds(room, status);
    }
    Redirect::to("/")
}

#[cfg(target_arch = "arm")]
fn house() -> impl House {
    janet::raspberry::create_house()
}

#[cfg(not(target_arch = "arm"))]
fn house() -> impl House {
    janet::raspberry::create_fake_house()
}

fn main() {
    let house = house();
    let resolver = SafeHouse {
        house: Arc::new(house)
    };
    rocket::ignite()
        .attach(Template::fairing())
        .manage(resolver)
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![light,blinds]).launch();
}