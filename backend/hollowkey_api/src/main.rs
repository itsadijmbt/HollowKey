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

    let app = app();

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/health", get(check_health))
}

async fn check_health() -> (StatusCode, Json<HealthStatus>) {
    let res = HealthStatus{
        status : "ok".to_string(),
    };
    (StatusCode::OK, Json(res))
}

#[derive(Serialize)]
struct HealthStatus {
    status: String,
}

#[cfg(test)]
mod tests {
    use super::*; 
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_check_health(){
        let app = app();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();


        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        assert_eq!(body_str, r#"{"status":"ok"}"#);
    }
}