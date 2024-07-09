use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GenerateRequest {
    player_id: String,
    player_feats: Vec<String>,
    hero_id: String,
    hero_feats: Vec<String>,
    new_creation: bool,
}

#[derive(Serialize)]
struct GenerateResponse {
    text: String,
    image_url: String,
}

// curl -X POST http://127.0.0.1:8080/generate \
// -H "Content-Type: application/json" \
// -d '{
//     "player_id": "player123",
//     "player_feats": ["feat1", "feat2"],
//     "hero_id": "hero123",
//     "hero_feats": ["featA", "featB"],
//     "new_creation": true
// }'

#[post("/generate")]
async fn generate(data: web::Json<GenerateRequest>, _client: web::Data<Client>) -> impl Responder {
    // Placeholder logic for generating a prompt based on received data
    let prompt = format!(
        "PlayerID: {}, PlayerFeats: {:?}, HeroID: {}, HeroFeats: {:?}, NewCreation: {}",
        data.player_id, data.player_feats, data.hero_id, data.hero_feats, data.new_creation
    );

    // Call OpenAI API (this is a placeholder and should be replaced with actual API call)
    let response_text = format!("Generated text for prompt: {}", prompt);
    let response_image_url = "https://example.com/generated_image.png".to_string();

    let response = GenerateResponse {
        text: response_text,
        image_url: response_image_url,
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::new();

    println!("running on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(generate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
