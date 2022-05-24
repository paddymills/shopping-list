
use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};

use crate::SubRoute;

#[derive(Default, Serialize, Deserialize)]
struct ListItem<'a> {
    checked: bool,
    name: &'a str,
    qty: i32,
    store: &'a str
}

pub struct ShoppingList { }

#[get("/")]
fn main() -> Template {
    Template::render("list", context! { page: "List" })
}

#[get("/items")]
fn items() -> Template {
    Template::render("list/items", context! { items: vec![
        ListItem { name: "apples",  store: "Wegmans", ..Default::default() },
        ListItem { name: "oranges", store: "Wegmans", ..Default::default() },
        ListItem { name: "eggs",    store: "Wegmans", ..Default::default() }
    ] })
}

impl SubRoute for ShoppingList {
    fn routes() -> Vec<rocket::Route> {
        routes![main, items]
    }
}
