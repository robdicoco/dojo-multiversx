use crate::data::{create_item, read_items, update_item, delete_item, Db, Item};
use hyper::{Body, Request, Response};
use serde_json;

pub async fn handle_create_item(req: Request<Body>, db: Db) -> Result<Response<Body>, hyper::Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let item: Item = match serde_json::from_slice(&body_bytes) {
        Ok(item) => item,
        Err(_) => return Ok(Response::new(Body::from("Invalid JSON"))),
    };
    match create_item(&db, item) {
        Ok(_) => Ok(Response::new(Body::from("Item created"))),
        Err(e) => Ok(Response::new(Body::from(e))),
    }
}

pub async fn handle_read_items(db: Db) -> Result<Response<Body>, hyper::Error> {
    let items = read_items(&db);
    let json = serde_json::to_string(&items).unwrap();
    Ok(Response::new(Body::from(json)))
}

pub async fn handle_update_item(req: Request<Body>, db: Db, id: u32) -> Result<Response<Body>, hyper::Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let name: String = match serde_json::from_slice(&body_bytes) {
        Ok(name) => name,
        Err(_) => return Ok(Response::new(Body::from("Invalid JSON"))),
    };
    match update_item(&db, id, name) {
        Ok(_) => Ok(Response::new(Body::from("Item updated"))),
        Err(e) => Ok(Response::new(Body::from(e))),
    }
}

pub async fn handle_delete_item(db: Db, id: u32) -> Result<Response<Body>, hyper::Error> {
    match delete_item(&db, id) {
        Ok(_) => Ok(Response::new(Body::from("Item deleted"))),
        Err(e) => Ok(Response::new(Body::from(e))),
    }
}