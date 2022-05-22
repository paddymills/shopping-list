
use rocket_dyn_templates::{Template, context};

// struct ListItem {
//     checked: bool,
//     name: String,
//     qty: i32,
//     store: String
// }

#[get("/")]
fn main() -> Template {
    Template::render("todo", context! { page: "List" })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![main]
}
