#![feature(proc_macro_hygiene, decl_macro)]

extern crate sysfs_gpio;

use janet::raspberry::create_fake_house;
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

impl Deref for SafeHouse{
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
        house.light(Room::LivingRoom ,status);
    }
    Redirect::to("/")
}

fn main() {
    let house = create_fake_house();
    let resolver = SafeHouse {
        house: Arc::new(house)
    };
    rocket::ignite()
        .attach(Template::fairing())
        .manage(resolver)
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![light]).launch();
}