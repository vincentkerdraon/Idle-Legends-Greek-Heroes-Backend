use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

use std::sync::{Arc, Mutex};

extern crate openai;
use openai::OpenAI;

extern crate api;
use api::GenerateRequest;

extern crate business;
use business::Business;

#[derive(Clone)]
struct AppState {
    business: Arc<Mutex<Business>>,
}

#[post("/generate")]
async fn generate(
    input: web::Json<GenerateRequest>,
    state: web::Data<Arc<Mutex<Business>>>,
) -> impl Responder {
    if !input.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let mut business = state.lock().unwrap();
    match business.logic(input.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // init reads OpenAI secret from environment variable
    let openai: OpenAI = OpenAI::new().expect("Failed to initialize OpenAI");
    let business = Business::new(openai);

    // Create shared state for Actix web server
    let app_state = AppState {
        business: Arc::new(Mutex::new(business)),
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
