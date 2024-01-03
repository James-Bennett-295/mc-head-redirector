use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize)]
struct HeadOptions {
    texture: Option<String>,
    uuid: Option<String>,
    name: Option<String>,
}

const HEAD_PROVIDERS_TEXTURE: [&str; 1] = [
    "https://mc-heads.net/avatar/{TEXTURE}/256.png"
];
const HEAD_PROVIDERS_UUID: [&str; 7] = [
    "https://mc-heads.net/avatar/{UUID}/256.png",
    "https://minotar.net/helm/{UUID}/256.png",
    "https://cravatar.eu/helmavatar/{UUID}/256.png",
    "https://crafthead.net/helm/{UUID}/256.png",
    "https://crafatar.com/avatars/{UUID}?size=256&overlay",
    "https://mineskin.eu/helm/{UUID}/256.png",
    "https://api.mineatar.io/face/{UUID}?scale=32&overlay&format=png"
];
const HEAD_PROVIDERS_NAME: [&str; 4] = [
    "https://mc-heads.net/avatar/{NAME}/256.png",
    "https://minotar.net/helm/{NAME}/256.png",
    "https://cravatar.eu/helmavatar/{NAME}/256.png",
    "https://mineskin.eu/helm/{NAME}/256.png",
    //"https://crafthead.net/helm/{NAME}/256.png",
];

fn hash_string(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

#[get("/head.png")]
async fn head(optns: web::Query<HeadOptions>) -> impl Responder {
    if let Some(uuid) = &optns.uuid {
        if !uuid.is_empty() {
            let url_index = (hash_string(&uuid) as usize) % HEAD_PROVIDERS_UUID.len();
            let url = HEAD_PROVIDERS_UUID[url_index].replace("{UUID}", uuid);
            return HttpResponse::TemporaryRedirect()
                .insert_header(("Location", url))
                .finish();
        }
    }
    if let Some(name) = &optns.name {
        if !name.is_empty() {
            let url_index = (hash_string(&name) as usize) % HEAD_PROVIDERS_NAME.len();
            let url = HEAD_PROVIDERS_NAME[url_index].replace("{NAME}", name);
            return HttpResponse::TemporaryRedirect()
                .insert_header(("Location", url))
                .finish();
        }
    }
    if let Some(texture) = &optns.texture {
        if !texture.is_empty() {
            let url_index = (hash_string(&texture) as usize) % HEAD_PROVIDERS_TEXTURE.len();
            let url = HEAD_PROVIDERS_TEXTURE[url_index].replace("{TEXTURE}", texture);
            return HttpResponse::TemporaryRedirect()
                .insert_header(("Location", url))
                .finish();
        }
    }
    HttpResponse::BadRequest()
        .content_type(ContentType::plaintext())
        .body("No texture, uuid, or name provided")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(head))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
