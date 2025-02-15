use crud_hyper::handle_request;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Initialize the in-memory database
    let db = Arc::new(Mutex::new(HashMap::new()));

    // Create the service function
    let make_svc = make_service_fn(|_conn| {
        let db = db.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(req, db.clone())
            }))
        }
    });

    // Bind the server to an address
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}