use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::fmt;
use std::sync::{Arc, Mutex};

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

#[derive(Clone)]
struct AppState {
    openai_secret_key: Arc<Mutex<String>>,
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
async fn generate(data: web::Json<GenerateRequest>, state: web::Data<AppState>) -> impl Responder {
    // Placeholder logic for generating a prompt based on received data
    let prompt = format!(
        "PlayerID: {}, PlayerFeats: {:?}, HeroID: {}, HeroFeats: {:?}, NewCreation: {}",
        data.player_id, data.player_feats, data.hero_id, data.hero_feats, data.new_creation
    );

    match call_openai_text_completion(&prompt, state.clone()).await {
        Ok(response) => {
            // Example response for testing (replace with actual OpenAI response handling)
            let response_text = format!("Generated text for prompt : {}, {}", prompt, response);
            let response_image_url = "https://example.com/generated_image.png".to_string();

            let response = GenerateResponse {
                text: response_text,
                image_url: response_image_url,
            };

            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read OPENAI_SECRET from environment variable
    let openai_secret_key =
        env::var("OPENAI_SECRET").expect("Please set the OPENAI_SECRET environment variable");

    // Create shared state for Actix web server
    let app_state = AppState {
        openai_secret_key: Arc::new(Mutex::new(openai_secret_key)),
    };

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

#[derive(Debug)]
struct OpenAIError;

impl std::error::Error for OpenAIError {}

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

async fn call_openai_text_completion(
    prompt: &str,
    state: web::Data<AppState>,
) -> Result<String, Error> {
    let openai_secret_key = state.openai_secret_key.lock().unwrap().clone();

    let client = Client::new();
    let url = "https://api.openai.com/v1/completions";

    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", openai_secret_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": "answer \"OK\""
                },
                // {
                //     "role": "system",
                //     "content": "You are a helpful assistant."
                // },
                // {
                //     "role": "user",
                //     "content": "Who won the world series in 2020?"
                // },
                // {
                //     "role": "assistant",
                //     "content": "The Los Angeles Dodgers won the World Series in 2020."
                // },
                // {
                //     "role": "user",
                //     "content": "Where was it played?"
                // }
            ],
            "max_tokens": 100
        }))
        .send()
        .await?;

    let json_response: Value = resp.json().await?;
    if let Some(choices) = json_response["choices"].as_array() {
        if let Some(choice) = choices.get(0) {
            if let Some(message) = choice["message"].as_object() {
                if let Some(content) = message["content"].as_str() {
                    return Ok(content.to_string());
                }
            }
        }
    }

    Ok("FIXME fail".to_string())

    // HttpResponse::InternalServerError()

    // Err(Error::new(
    //     reqwest::StatusCode::INTERNAL_SERVER_ERROR,
    //     "Invalid OpenAI completions return",
    // ))

    // Err(Error::from(std::io::Error::new(
    //     std::io::ErrorKind::Other,
    //     "Invalid OpenAI completions return",
    // )));

    // Err(OpenAIError)
}
