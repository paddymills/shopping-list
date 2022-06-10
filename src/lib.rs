#[macro_use] extern crate rocket;

mod auth;
mod list;
mod todo;

pub use auth::User;
pub use list::ShoppingList;
pub use todo::Todo;

pub trait SubRoute {
    fn routes() -> Vec<rocket::Route>;
}
