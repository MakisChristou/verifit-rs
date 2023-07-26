# verifit-rs
Verifit-rs is a simple Rest API backend for my android app [Verifit](https://github.com/MakisChristou/verifit) based on Axum. 

# Features
- CRUD Operations
- Email Verification
- Password Hashing using bcrypt
- Simple Logging
- Authentication using json web tokens

# Dependencies

- **dotenvy (0.15.6)** and **dotenvy_macro (0.15.1)**: These libraries are used for loading environment variables from a `.env` file, which is crucial for managing configuration in a secure manner.

- **sea-orm (0.11.0)**: This is an async ORM for Rust. It's used in this project for database operations, specifically with PostgreSQL, as indicated by the `sqlx-postgres` feature.

- **tokio (1.26.0)**: Tokio is a Rust framework for developing applications with asynchronous I/O, networking, and other related features. The `full` feature indicates that all optional components are included.

- **axum (0.6.10)**: Axum is a web application framework that focuses on ergonomics and modularity. It's used in this project to handle HTTP requests and responses.

- **serde (1.0.152)**: Serde is a framework for serializing and deserializing Rust data structures efficiently and generically. It's used in this project for handling JSON data.


## Generate Database Entries
```bash
sea-orm-cli generate entity -o src/database
```

# Running
```bash
cargo run --release
```

