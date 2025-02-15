// src/lib.rs

// Declare all public modules here
pub mod data;
pub mod manage_item;
pub mod server_hyper;

// Re-export public APIs for easier access
pub use data::{create_item, delete_item, read_items, update_item, Db, Item};
pub use server_hyper::handle_request;
