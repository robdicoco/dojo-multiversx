use hyper::{Body, Request, Response, Method, StatusCode};
use crate::data::Db;
use crate::manage_item::{handle_create_item, handle_read_items, handle_update_item, handle_delete_item};

pub async fn handle_request(req: Request<Body>, db: Db) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/items") => handle_create_item(req, db).await,
        (&Method::GET, "/items") => handle_read_items(db).await,
        (&Method::PUT, path) if path.starts_with("/items/") => {
            let id: u32 = path[7..].parse().unwrap_or(0);
            handle_update_item(req, db, id).await
        },
        (&Method::DELETE, path) if path.starts_with("/items/") => {
            let id: u32 = path[7..].parse().unwrap_or(0);
            handle_delete_item(db, id).await
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap();
            Ok(response)
        },
    }
}