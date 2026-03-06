-- migrations/..._create_users_table.sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    verification_token TEXT,
    is_email_verified BOOLEAN DEFAULT FALSE
);