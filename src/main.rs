#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::{
    serde::{Serialize, Deserialize},
    fs::FileServer
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ctx<'r> {
    val: &'r str
}

#[get("/")]
fn index() -> Template {
    Template::render("index", Ctx { val: "test" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("static/"))
        .attach(Template::fairing())
}
