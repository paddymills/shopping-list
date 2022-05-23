
use rocket_dyn_templates::{Template, context};

#[allow(dead_code)]
struct ListItem {
    checked: bool,
    name: String,
    qty: i32,
    store: String
}

#[get("/")]
fn main() -> Template {
    Template::render("list", context! { page: "List" })
}

#[get("/items")]
fn items() -> Template {
    Template::render("list/items", context! { items: vec!["apples", "oranges", "eggs"] })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![main, items]
}
