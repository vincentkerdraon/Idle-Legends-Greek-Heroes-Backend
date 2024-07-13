use serde::{Deserialize, Serialize};
use std::ops::Deref;

/*
curl -v -X POST http://127.0.0.1:8080/generate \
-H "Content-Type: application/json" \
-d '{
    "player_id": "player123",
    "hero_id": "hero123",
    "feat_id": "init"
}'
*/

#[derive(Deserialize, Clone)]
pub struct GenerateRequest {
    pub player_id: PlayerID,
    pub hero_id: HeroID,
    pub feat_id: FeatID,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub text: String,
    pub image_url: String,
}

impl GenerateRequest {
    pub fn is_valid(&self) -> bool {
        !self.player_id.trim().is_empty()
            && !self.hero_id.trim().is_empty()
            && !self.feat_id.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Default, Deserialize)]
pub struct PlayerID(String);
impl Deref for PlayerID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Eq for PlayerID {}

#[derive(Debug, Clone, PartialEq, Hash, Default, Deserialize)]
pub struct HeroID(String);
impl Deref for HeroID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Eq for HeroID {}

#[derive(Debug, Clone, PartialEq, Hash, Default, Deserialize)]
pub struct FeatID(String);
impl FeatID {
    pub fn new(id: &str) -> Self {
        FeatID(id.to_string())
    }
}
impl Deref for FeatID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Eq for FeatID {}

pub static FEAT_ID_INIT: &str = "init";

#[derive(Debug, Clone, Default)]
pub struct HeroState {
    pub hero_id: HeroID,
    pub hero_name: String,
    pub hero_text_prompt: String,
    pub hero_image_prompt: String,
    pub hero_feats: Vec<FeatID>,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub player_id: PlayerID,
    pub hero_state: HeroState,
    pub player_feats: Vec<FeatID>,
}
