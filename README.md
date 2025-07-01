# Axum Authentication Service

This project provides a ready-to-use, feature-rich authentication service built with Rust and the `axum` web framework. It's designed to be modular and easily integrated into any Rust-based web project.

## Features

-   **User Registration**: Securely register new users with hashed passwords.
-   **User Login**: Authenticate users and issue JSON Web Tokens (JWT).
-   **Password Reset**: A secure, token-based password reset flow via email.
-   **Email Verification**: Ensure users sign up with a valid email address.
-   **Database Agnostic**: Uses `sqlx` for database interaction, currently set up for PostgreSQL.
-   **Configuration**: Easily configurable through a `.env` file.
-   **Asynchronous**: Built on top of `axum` and `tokio` for high performance.

## Getting Started

### Prerequisites

-   Rust toolchain
-   PostgreSQL database
-   An SMTP server for sending emails

### Installation

1.  **Clone the repository:**

    ```bash
    git clone <repository-url>
    cd axum-auth-service
    ```

2.  **Set up the database:**

    -   Connect to your PostgreSQL instance and create a new database.
    -   Run the migration script to create the `users` table:
        ```bash
        psql -U youruser -d yourdatabase -a -f database/migrations/20250701111247_create_users_table.sql
        ```

3.  **Configure the environment:**

    -   Create a `.env` file in the root of the project.
    -   Add your database URL, JWT secret, and SMTP server credentials to the `.env` file:
        ```env
        DATABASE_URL=postgres://user:password@localhost/databasename
        JWT_SECRET=your-super-secret-key
        SMTP_USERNAME=your-smtp-username
        SMTP_PASSWORD=your-smtp-password
        SMTP_HOST=your-smtp-host
        FROM_EMAIL=no-reply@yourdomain.com
        ```

4.  **Run the application:**
    ```bash
    cargo run
    ```

The service will be running at `http://127.0.0.1:8080`.

## API Usage

### Register a new user

-   **Endpoint**: `POST /auth/register`
-   **Body**:
    ```json
    {
        "username": "testuser",
        "email": "test@example.com",
        "password": "password123",
        "phone_number": "1234567890"
    }
    ```

### Login

-   **Endpoint**: `POST /auth/login`
-   **Body**:
    ```json
    {
        "email": "test@example.com",
        "password": "password123"
    }
    ```
-   **Response**:
    ```json
    {
        "token": "your-jwt-token"
    }
    ```

## How to Integrate into Your Project

This service is designed as a modular component. To use it in your existing `axum` project:

1.  Copy the `src/auth` directory into your project's `src` directory.
2.  Add the required dependencies from this project's `Cargo.toml` to your own.
3.  In your `main.rs`, nest the authentication router:

    ```rust
    mod auth;

    // ... in your main function
    let app = Router::new()
        // ... other routes
        .nest("/auth", auth::handlers::create_router())
        .with_state(pool); // Ensure the PgPool is available as state
    ```