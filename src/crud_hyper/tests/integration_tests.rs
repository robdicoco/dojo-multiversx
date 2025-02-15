use hyper::{Body, Client, Method, Request, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Import the crate's public API
use crud_hyper::{data::Item, server_hyper::handle_request};

#[tokio::test]
async fn test_crud_operations() {
    // Initialize the in-memory database
    let db = Arc::new(Mutex::new(HashMap::new()));

    // Start the server in a separate task
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(move |_conn| {
        let db = db.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(req, db.clone())
            }))
        }
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);
    tokio::spawn(server);

    // Create a Hyper client
    let client = Client::new();

    // Test CREATE operation
    let create_url = "http://127.0.0.1:3000/items";
    let create_body = json!({"id": 1, "name": "Test Item"}).to_string();
    let create_req = Request::builder()
        .method(Method::POST)
        .uri(create_url)
        .header("Content-Type", "application/json")
        .body(Body::from(create_body))
        .unwrap();
    let create_resp = client.request(create_req).await.unwrap();
    assert_eq!(create_resp.status(), StatusCode::OK);

    // Test READ operation
    let read_url = "http://127.0.0.1:3000/items";
    let read_req = Request::builder()
        .method(Method::GET)
        .uri(read_url)
        .body(Body::empty())
        .unwrap();
    let read_resp = client.request(read_req).await.unwrap();
    assert_eq!(read_resp.status(), StatusCode::OK);

    let body_bytes = hyper::body::to_bytes(read_resp.into_body()).await.unwrap();
    let items: Vec<Item> = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 1);
    assert_eq!(items[0].name, "Test Item");

    // Test UPDATE operation
    let update_url = "http://127.0.0.1:3000/items/1";
    let update_body = json!("Updated Item").to_string();
    let update_req = Request::builder()
        .method(Method::PUT)
        .uri(update_url)
        .header("Content-Type", "application/json")
        .body(Body::from(update_body))
        .unwrap();
    let update_resp = client.request(update_req).await.unwrap();
    assert_eq!(update_resp.status(), StatusCode::OK);

    // Verify the update
    let read_req_after_update = Request::builder()
        .method(Method::GET)
        .uri(read_url)
        .body(Body::empty())
        .unwrap();
    let read_resp_after_update = client.request(read_req_after_update).await.unwrap();
    let body_bytes_after_update = hyper::body::to_bytes(read_resp_after_update.into_body())
        .await
        .unwrap();
    let items_after_update: Vec<Item> = serde_json::from_slice(&body_bytes_after_update).unwrap();
    assert_eq!(items_after_update[0].name, "Updated Item");

    // Test DELETE operation
    let delete_url = "http://127.0.0.1:3000/items/1";
    let delete_req = Request::builder()
        .method(Method::DELETE)
        .uri(delete_url)
        .body(Body::empty())
        .unwrap();
    let delete_resp = client.request(delete_req).await.unwrap();
    assert_eq!(delete_resp.status(), StatusCode::OK);

    // Verify the deletion
    let read_req_after_delete = Request::builder()
        .method(Method::GET)
        .uri(read_url)
        .body(Body::empty())
        .unwrap();
    let read_resp_after_delete = client.request(read_req_after_delete).await.unwrap();
    let body_bytes_after_delete = hyper::body::to_bytes(read_resp_after_delete.into_body())
        .await
        .unwrap();
    let items_after_delete: Vec<Item> = serde_json::from_slice(&body_bytes_after_delete).unwrap();
    assert!(items_after_delete.is_empty());
}