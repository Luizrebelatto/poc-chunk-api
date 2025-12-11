use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::models::{Chunk, NewChunk};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn insert_chunk(pool: &PgPool, new_chunk: NewChunk) -> Result<Chunk, sqlx::Error> {
    let chunk = sqlx::query_as::<_, Chunk>(
        r#"
        INSERT INTO chunks (filename, file_path, size, content_type)
        VALUES ($1, $2, $3, $4)
        RETURNING id, filename, file_path, size, content_type, created_at, updated_at
        "#
    )
    .bind(&new_chunk.filename)
    .bind(&new_chunk.file_path)
    .bind(new_chunk.size)
    .bind(&new_chunk.content_type)
    .fetch_one(pool)
    .await?;

    Ok(chunk)
}

pub async fn get_chunk_by_filename(pool: &PgPool, filename: &str) -> Result<Option<Chunk>, sqlx::Error> {
    let chunk = sqlx::query_as::<_, Chunk>(
        "SELECT id, filename, file_path, size, content_type, created_at, updated_at FROM chunks WHERE filename = $1"
    )
    .bind(filename)
    .fetch_optional(pool)
    .await?;

    Ok(chunk)
}

pub async fn list_all_chunks(pool: &PgPool) -> Result<Vec<Chunk>, sqlx::Error> {
    let chunks = sqlx::query_as::<_, Chunk>(
        "SELECT id, filename, file_path, size, content_type, created_at, updated_at FROM chunks ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(chunks)
}

pub async fn delete_chunk(pool: &PgPool, filename: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM chunks WHERE filename = $1")
        .bind(filename)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
