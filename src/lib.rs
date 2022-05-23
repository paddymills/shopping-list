#[macro_use] extern crate rocket;

mod auth;
mod list;

pub use auth::User;
pub use list::ShoppingList;

pub trait SubRoute {
    fn routes() -> Vec<rocket::Route>;
}
