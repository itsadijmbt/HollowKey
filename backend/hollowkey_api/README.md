
# Hollowkey API Service

This directory contains the backend API for the Hollowkey project. This service is responsible for handling all business logic, including user authentication, secure vault creation, and data management.

## Tech Stack

-   **Language:** Rust (v1.90.x)
-   **Web Framework:** Axum (v0.8.6)
-   **Asynchronous Runtime:** Tokio (v1.37)
-   **Containerization:** Docker

## Current Features

As part of the initial "Walking Skeleton," the service currently provides:

-   A `/health` endpoint to confirm that the service is running and accessible.

## Getting Started

This service can be run locally for development in two primary ways: directly via the Rust toolchain or as a self-contained Docker container.

### 1. Running with Cargo (Requires Rust)

This method is ideal for active development and fast compilation cycles.

1.  **Navigate to the service directory:**
    ```bash
    cd backend/hollowkey_api
    ```
2.  **Run the application:**
    ```bash
    cargo run
    ```
3.  The server will start and be accessible at `http://localhost:3001`.

### 2. Running with Docker (Recommended for Consistency)

This method runs the application in a container, exactly as it would run in production. It is the best way to ensure consistency.

1.  **Ensure Docker Desktop is running.**
2.  **Navigate to the service directory:**
    ```bash
    cd backend/hollowkey_api
    ```
3.  **Build the Docker image:**
    ```bash
    docker build -t hollowkey-api .
    ```
4.  **Run the container from the image:**
    ```bash
    docker run --rm -p 3001:3001 hollowkey-api
    ```
5.  The server will start inside the container and be accessible at `http://localhost:3001`.

## Testing

To verify that the service is running correctly, make a `GET` request to the health endpoint. You can do this from a new terminal window.

```bash
curl http://localhost:3001/health
````

A successful response will be a JSON object indicating the service is operational:

```json
{
    "status":"ok"
}
```