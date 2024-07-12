use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

extern crate openai;
use openai::OpenAI;

extern crate api;
use api::*;

extern crate business;

struct AppState {
    generator: OpenAI,
    players: HashMap<PlayerID, PlayerState>,
}

#[post("/generate")]
async fn generate(
    input: web::Json<GenerateRequest>,
    state: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    if !input.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let mut state_mutex = state.lock().unwrap();

    let player_state: PlayerState;
    match business::match_player(&mut state_mutex.players, input.into_inner()) {
        Ok(ps) => player_state = ps,
        Err(err) => match err {
            //FIXME
            // FeatAlreadyDoneError => {
            //     return HttpResponse::Found().finish();
            // }
            _ => {
                eprintln!("Error: {}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
    }
    //release the mutex
    let open_ai = state_mutex.generator.clone();
    drop(state_mutex);

    // return HttpResponse::InternalServerError().finish();

    match business::generate(open_ai, player_state).await {
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
    let open_ai_secret = openai::load_secret().expect("Failed to initialize OpenAI");
    // static open_ai_secret_static: &'static str = open_ai_secret;

    //data is shared across all workers
    let players = HashMap::new();
    let state = AppState {
        players: players,
        generator: OpenAI::new(open_ai_secret),
    };
    let state_arc = Arc::new(Mutex::new(state));

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state_arc.clone()))
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
