# Simple Rust Web Backend

This is a simple Rust Web Backend for practice, built with Actix Web. It incorporates token validation middleware.

## Directory Structure

- `src`: Source code directory.
  - `handlers`: Module for request handlers.
    - `handlers.rs`: Implementation of request handlers.
  - `middleware`: Middleware module.
    - `middleware.rs`: Contains the token validation middleware implementation.
  - `models`: Data models module.
    - `models.rs`: Definitions of data models.
  - `services`: Services module.
    - `person.rs`: Service implementations related to persons.
  - `utils`: Utility module.
  - `config.rs`: Code related to configurations.
  - `main.rs`: Entry point of the project.
  - `routes.rs`: Defines all routes and endpoints.
- `target`: Rust's compilation output directory.
- `.gitignore`: Configuration for ignored files in Git.
- `Cargo.lock` & `Cargo.toml`: Dependency and configuration for the Rust project.
- `readme.md`: The README file for the project.

## Features

- **Token Validation Middleware**: Before processing the request, the middleware checks for a token in the request header. Only requests with a valid token are allowed to proceed.(default token is 3394)
  
- **Retrieve Person Information**: Send a GET request to `127.0.0.1:8080/get` to retrieve the first and last name of a person.

## How to Run

```
cargo run
```
1. Ensure Rust and Cargo are installed on your machine.
2. Navigate to the root directory of the project and run:
3. Use your preferred HTTP client or browser to send a request to `127.0.0.1:8080/get` for testing.


