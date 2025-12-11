#[macro_use] extern crate rocket;

mod db;
mod models;

use rocket::fs::{FileServer, TempFile, NamedFile};
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::{State, fairing::AdHoc};
use sqlx::PgPool;
use std::path::PathBuf;
use models::{Chunk, NewChunk};
use std::env;

#[derive(FromForm)]
struct UploadForm<'r> {
    file: TempFile<'r>,
}

#[post("/upload/<filename>", data = "<form>")]
async fn upload_chunk(
    filename: &str,
    mut form: Form<UploadForm<'_>>,
    db: &State<PgPool>
) -> Json<String> {
    let path = format!("data/{filename}");

    if let Err(e) = form.file.persist_to(&path).await {
        return Json(format!("Error saving file: {e}"));
    }

    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => return Json(format!("Error reading metadata: {e}")),
    };

    let new_chunk = NewChunk {
        filename: filename.to_string(),
        file_path: path.clone(),
        size: metadata.len() as i64,
        content_type: form.file.content_type().map(|ct| ct.to_string()),
    };

    match db::insert_chunk(db, new_chunk).await {
        Ok(chunk) => Json(format!("Chunk {} successfully saved! ID: {}", chunk.filename, chunk.id)),
        Err(e) => Json(format!("Error saving to database: {e}")),
    }
}

#[get("/chunk/<filename>")]
async fn get_chunk(filename: &str) -> Option<NamedFile> {
    let path = PathBuf::from("data").join(filename);
    NamedFile::open(path).await.ok()
}

#[get("/chunks")]
async fn list_chunks(db: &State<PgPool>) -> Json<Vec<Chunk>> {
    match db::list_all_chunks(db).await {
        Ok(chunks) => Json(chunks),
        Err(_) => Json(vec![]),
    }
}

#[delete("/chunk/<filename>")]
async fn delete_chunk(filename: &str, db: &State<PgPool>) -> Json<String> {
    match db::delete_chunk(db, filename).await {
        Ok(deleted) => {
            if deleted {
                let path = PathBuf::from("data").join(filename);
                if let Err(e) = std::fs::remove_file(&path) {
                    return Json(format!("Chunk removed from database, but error deleting file: {e}"));
                }
                Json(format!("Chunk {} successfully deleted!", filename))
            } else {
                Json(format!("Chunk {} not found", filename))
            }
        }
        Err(e) => Json(format!("Error deleting chunk: {e}")),
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    rocket::build()
        .manage(pool)
        .mount("/", routes![upload_chunk, get_chunk, list_chunks, delete_chunk])
        .mount("/files", FileServer::from("data"))
        .attach(AdHoc::on_ignite("Run Migrations", |rocket| async {
            println!("Database connected!");
            rocket
        }))
}
