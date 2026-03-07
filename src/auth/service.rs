use super::models::{RegisterUser, User};
use super::utils::{generate_token, hash_password, send_email};
use sqlx::{PgPool, SqlitePool};

pub async fn register_user(pool: &SqlitePool, new_user: RegisterUser) -> Result<User, sqlx::Error> {
    let password_hash = hash_password(&new_user.password).unwrap();
    let verification_token = generate_token();

    let user: User = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash, phone_number, verification_token) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(new_user.username)
    .bind(new_user.email.clone())
    .bind(password_hash)
    .bind(new_user.phone_number)
    .bind(&verification_token)
    .fetch_one(pool)
    .await?;

    #[cfg(feature = "use-sqlite")]
    {
        let verification_link: String = format!(
            "http://localhost:8080/auth/verify-email?token={}",
            verification_token
        );
        let email_body: String = format!(
            "Please verify your email by clicking on the following link: {}",
            verification_link
        );
        send_email(&user.email, "Email Verification", &email_body).unwrap();
    }

    Ok(user)
}

pub async fn get_user_by_email(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
}

pub async fn verify_email_token(pool: &SqlitePool, token: &str) -> Result<bool, sqlx::Error> {
    let result: sqlx::sqlite::SqliteQueryResult = sqlx::query!(
        "UPDATE users SET is_email_verified = true, verification_token = NULL WHERE verification_token = $1 AND is_email_verified = false",
        token
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

// ... other service functions like password reset, etc.
