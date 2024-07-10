use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

use std::sync::Arc;

extern crate openai;
use openai::OpenAI;

extern crate api;
use api::GenerateRequest;

extern crate business;
use business::Business;

#[derive(Clone)]
struct AppState {
    business: Arc<Business>,
}
/*
curl -X POST http://127.0.0.1:8080/generate \
-H "Content-Type: application/json" \
-d '{
    "player_id": "player123",
    "player_feats": ["feat1", "feat2"],
    "hero_id": "hero123",
    "hero_feats": ["featA", "featB"],
    "new_creation": true
}'
*/

#[post("/generate")]
async fn generate(input: web::Json<GenerateRequest>, state: web::Data<AppState>) -> impl Responder {
    if !input.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    match state.business.logic(input.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read OPENAI_SECRET from environment variable
    let openai = OpenAI::new().expect("Failed to initialize OpenAI");

    // Create shared state for Actix web server
    let app_state = AppState {
        business: Arc::new(Business::new(openai)),
    };

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(generate)
    })
    //FIXME bind from parameters
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}
