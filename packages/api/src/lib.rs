use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub player_id: String,
    pub player_feats: Vec<String>,
    pub hero_id: String,
    pub hero_feats: Vec<String>,
    pub new_creation: bool,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub text: String,
    pub image_url: String,
}

impl GenerateRequest {
    pub fn is_valid(&self) -> bool {
        !self.player_id.trim().is_empty() && !self.hero_id.trim().is_empty()
    }
}
