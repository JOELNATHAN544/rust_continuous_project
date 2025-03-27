use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::env;

#[derive(Debug, FromRow)]
pub struct Process {
    id: i32,
    name: String,
    status: String,
}

pub async fn create_process_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS processes (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            status VARCHAR(50) NOT NULL DEFAULT 'pending'
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn database(pool: &PgPool, name: &str, status: &str) -> Result<Process, sqlx::Error> {
    // First check if table exists
    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT FROM pg_tables 
            WHERE schemaname = 'public' 
            AND tablename = 'processes'
        )",
    )
    .fetch_one(pool)
    .await?;

    // Create table only if it doesn't exist
    if !table_exists {
        create_process_table(pool).await?;
    }

    // Insert the new process
    let process = sqlx::query_as::<_, Process>(
        r#"
        INSERT INTO processes (name, status)
        VALUES ($1, $2)
        RETURNING id, name, status
        "#,
    )
    .bind(name)
    .bind(status)
    .fetch_one(pool)
    .await?;

    println!("✅ Process recorded in database: {:?}", process);
    Ok(process)
}
