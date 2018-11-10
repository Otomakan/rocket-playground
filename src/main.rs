
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
use rocket_contrib::serve::StaticFiles;
// use Rock

use rocket::Request;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// fn main() {
//     rocket::ignite().mount("/", StaticFiles::from("static")).launch();
// }
// #[get("/")]
// fn main() {
//     rocket::ignite()
//         .mount("/public", StaticFiles::from("../static_files"))
//         .launch();
// }
// #[get("/myfile")]
fn myfiles()-> StaticFiles{
	StaticFiles::from("static_files")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/lol", 
	StaticFiles::from("static_files"))
}

fn main() {
    rocket().launch();
}