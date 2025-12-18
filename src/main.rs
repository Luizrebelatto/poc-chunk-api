#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::response::Responder;
use rocket::{Request, Response, State};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

type Manifest = HashMap<String, String>;

struct CachedFile(NamedFile);

impl<'r> Responder<'r, 'static> for CachedFile {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build_from(self.0.respond_to(req)?)
            .header(Header::new(
                "Cache-Control",
                "public, max-age=86400, immutable",
            ))
            .ok()
    }
}

#[get("/chunks/<script_id>")]
async fn get_chunk(script_id: &str, manifest: &State<Arc<Manifest>>) -> Result<CachedFile, (rocket::http::Status, String)> {
    let file_name = manifest
        .get(script_id)
        .ok_or_else(|| {
            (
                rocket::http::Status::NotFound,
                format!("Internal error while sending the chunk.", script_id),
            )
        })?;

    let file_path = PathBuf::from("chunks").join(file_name);

    NamedFile::open(&file_path)
        .await
        .map(CachedFile)
        .map_err(|_| {
            (
                rocket::http::Status::InternalServerError,
                "Erro interno ao enviar o chunk.".to_string(),
            )
        })
}

async fn load_manifest() -> Manifest {
    match fs::read_to_string("manifest.json").await {
        Ok(content) => {
            match serde_json::from_str(&content) {
                Ok(manifest) => {
                    println!("Manifest loaded: {:?}", manifest);
                    manifest
                }
                Err(e) => {
                    eprintln!("Error parsing manifest.json: {} — fallback to empty.", e);
                    HashMap::new()
                }
            }
        }
        Err(_) => {
            eprintln!("No manifest.json found — fallback to direct name.");
            HashMap::new()
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let manifest = Arc::new(load_manifest().await);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));


    rocket::custom(figment)
        .manage(manifest)
        .mount("/", routes![index, get_chunk])
}
