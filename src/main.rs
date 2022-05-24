#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;
use serde::{Serialize, Deserialize};

use shopping_list::{ShoppingList, SubRoute};

#[derive(Deserialize, Serialize)]
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
        .mount("/list", ShoppingList::routes())
        .mount("/public", FileServer::from("static/"))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
