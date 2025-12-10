#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, TempFile, NamedFile};
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::tokio::fs::read_dir;
use std::path::PathBuf;

#[derive(FromForm)]
struct UploadForm<'r> {
    file: TempFile<'r>,
}

#[post("/upload/<filename>", data = "<form>")]
async fn upload_chunk(filename: &str, mut form: Form<UploadForm<'_>>) -> Json<String> {
    let path = format!("data/{filename}");

    if let Err(e) = form.file.persist_to(&path).await {
        return Json(format!("Erro ao salvar arquivo: {e}"));
    }

    Json(format!("Chunk {filename} salvo com sucesso!"))
}

#[get("/chunk/<filename>")]
async fn get_chunk(filename: &str) -> Option<NamedFile> {
    let path = PathBuf::from("data").join(filename);
    NamedFile::open(path).await.ok()
}

#[get("/chunks")]
async fn list_chunks() -> Json<Vec<String>> {
    let mut chunks = Vec::new();

    if let Ok(mut entries) = read_dir("data").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(filename) = entry.file_name().to_str() {
                chunks.push(filename.to_string());
            }
        }
    }

    Json(chunks)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![upload_chunk, get_chunk, list_chunks])
        .mount("/files", FileServer::from("data"))
}
