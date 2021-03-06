#![feature(proc_macro_hygiene, decl_macro)]

extern crate sysfs_gpio;

use std::thread;
use std::sync::Arc;
use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use rocket::request::Form;
use janet::house::*;


#[macro_use]
extern crate rocket;

pub struct SafeHouse {
    house: Arc<House + Send + Sync>
}

impl SafeHouse {
    fn execute<F>(&self, f: F) where F: FnOnce(Arc<House>) + Send + 'static {
        let h = self.house.clone();
        thread::spawn(move || {
            f(h);
        });
    }
}

#[derive(FromForm)]
struct NewStatus {
    status: String
}

#[derive(FromForm)]
struct Order {
    status: String,
    room: String,
}

#[post("/light", data = "<status>")]
fn light(house: State<SafeHouse>, status: Form<Order>) -> Redirect {
    if let (Ok(room), Ok(status)) = (status.room.parse(), status.status.parse()) {
        house.execute(move |h| {
            h.light(room, status);
        })
    }
    Redirect::to("/")
}

#[post("/blinds", data = "<status>")]
fn blinds(house: State<SafeHouse>, status: Form<Order>) -> Redirect {
    if let (Ok(room), Ok(status)) = (status.room.parse(), status.status.parse()) {
        house.execute(move |h| {
            h.blinds(room, status);
        });
    }
    Redirect::to("/")
}

#[post("/screen", data = "<status>")]
fn screen(house: State<SafeHouse>, status: Form<NewStatus>) -> Redirect {
    if let Ok(s) = status.status.parse() {
        house.execute(move |h| {
            h.screen(s);
        });
    }
    Redirect::to("/")
}

#[derive(FromForm)]
struct Mode {
    mode: String,
}

#[post("/mode", data = "<mode>")]
fn mode(house: State<SafeHouse>, mode: Form<Mode>) -> Redirect {
    match mode.mode.as_str() {
        "cinema" => house.execute(|h| h.cinema()),
        "goodmorning" => house.execute(|h| h.goodmorning()),
        "goodnight" => house.execute(|h| h.goodnight()),
        _ => {}
    };
    Redirect::to("/")
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
        .mount("/api", routes![light,blinds,mode,screen]).launch();
}