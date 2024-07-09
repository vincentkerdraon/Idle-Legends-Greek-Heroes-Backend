use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

use std::sync::Arc;

extern crate openai;
use openai::OpenAI;

extern crate api;
use api::GenerateRequest;
use api::GenerateResponse;

#[derive(Clone)]
struct AppState {
    openai: Arc<OpenAI>,
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
async fn generate(data: web::Json<GenerateRequest>, state: web::Data<AppState>) -> impl Responder {
    let prompt = format!(
        "PlayerID: {}, PlayerFeats: {:?}, HeroID: {}, HeroFeats: {:?}, NewCreation: {}",
        data.player_id, data.player_feats, data.hero_id, data.hero_feats, data.new_creation
    );

    match state
        .openai
        .call_openai_text("system_prompt", &prompt)
        .await
    {
        Ok(response) => {
            // Example response for testing (replace with actual OpenAI response handling)
            let response_text = format!("Generated text: prompt={}, response={}", prompt, response);
            let response_image_url = "https://example.com/generated_image.png".to_string();

            let response = GenerateResponse {
                text: response_text,
                image_url: response_image_url,
            };

            HttpResponse::Ok().json(response)
        }
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
    let openai_arc = Arc::new(openai);

    // Create shared state for Actix web server
    let app_state = AppState {
        openai: openai_arc.clone(),
    };
    //FIXME bind from parameters

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(generate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}
