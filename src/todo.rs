
use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};

use crate::SubRoute;

#[derive(Serialize, Deserialize)]
pub struct Todo<'a> {
    checked: bool,
    id: u32,
    text: &'a str
}

impl Default for Todo<'_> {
    fn default() -> Self {
        Self {
            checked: false,
            id: 0,
            text: "Do stuff..."
        }
    }
}

impl SubRoute for Todo<'_> {
    fn routes() -> Vec<rocket::Route> {
        routes![main, items]
    }
}

#[get("/")]
fn main() -> Template {
    Template::render("todo", context! { page: "todo" })
}

#[get("/items")]
fn items() -> Template {
    Template::render("todo/items", context! { items: vec![
        Todo { checked: false, id: 1, text: "Dev's bed",   ..Default::default() },
        Todo { checked: false, id: 2, text: "Front Porch", ..Default::default() },
        Todo { checked: false, id: 3,                      ..Default::default() }
    ] })
}
