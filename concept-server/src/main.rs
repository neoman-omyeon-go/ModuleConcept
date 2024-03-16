#[macro_use]
extern crate rocket;
extern crate testingapp;
use testingapp::tempmod::print_random_number;   

#[get("/")]
fn index() -> &'static str{
    print_random_number();
    "Hello World!"
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
    .mount("/", routes![index])
}
