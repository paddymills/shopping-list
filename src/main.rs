#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::{
    serde::{Serialize, Deserialize},
    fs::FileServer
};

use shopping_list::list;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ctx<'r> {
    val: &'r str
}

#[get("/")]
fn index() -> Template {
    Template::render("index", Ctx { val: "test" })
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", context! { val: "not found" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/list", list::routes())
        .mount("/public", FileServer::from("static/"))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
