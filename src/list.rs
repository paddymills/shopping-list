
use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};

use crate::SubRoute;

const DEFUALT_STORE: &str = "Wegmans";

#[derive(Serialize, Deserialize)]
struct ListItem<'a> {
    checked: bool,
    name: &'a str,
    qty: i32,
    store: &'a str
}

impl Default for ListItem<'_> {
    fn default() -> Self {
        Self {
            checked: false,
            name: "-- nothing --",
            qty: 1,
            store: DEFUALT_STORE
        }
    }
}

pub struct ShoppingList { }

impl SubRoute for ShoppingList {
    fn routes() -> Vec<rocket::Route> {
        routes![main, items]
    }
}

#[get("/")]
fn main() -> Template {
    Template::render("list", context! { page: "list" })
}

#[get("/items")]
fn items() -> Template {
    Template::render("list/items", context! { items: vec![
        ListItem { name: "apples",  store: "Wegmans", ..Default::default() },
        ListItem { name: "milk",    store: "Wegmans", ..Default::default() },
        ListItem { name: "eggs",    store: "Costco",  ..Default::default() }
    ] })
}
