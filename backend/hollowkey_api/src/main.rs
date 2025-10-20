use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
}; 


use serde::Serialize;  
use tokio::net::TcpListener;

#[tokio::main] 
async fn main(){    
    
    println!("Starting Hollowkey API server on http://0.0.0.0:3001");

    let app = Router::new()
        .route("/health", get(check_health));

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


async fn check_health() -> (StatusCode, Json<HealthStatus>){
    let status = HealthStatus{
        status: "ok".to_string(),
    };
    (
        StatusCode::OK,
        Json(status)
    )
}

#[derive(Serialize)]
struct HealthStatus{
    status: String,
}
