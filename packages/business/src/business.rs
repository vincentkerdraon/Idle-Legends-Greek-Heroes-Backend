extern crate api;
use api::*;
use openai::openai::OpenAI;
use std::collections::HashMap;
extern crate openai;
use tokio::try_join;

use crate::{error::BusinessError, feat, new_hero};

pub fn match_player(
    players: &mut HashMap<PlayerID, PlayerState>,
    input: GenerateRequest,
) -> Result<PlayerState, BusinessError> {
    let mut player_state: PlayerState;

    let input_clone = input.clone();
    let player_id = input.player_id.clone();
    let mut feat_id = input.feat_id.clone();

    if let Some(player_state_ref) = players.get(&input.player_id) {
        player_state = player_state_ref.clone();
    } else {
        player_state = PlayerState {
            player_id: input.player_id,
            ..Default::default()
        };
    }

    if player_state.player_feats.is_empty() && input.feat_id.as_str() == FEAT_ID_INIT {
        //first time with this hero. Send a welcome
        player_state.hero_state = new_hero::prepare_new_hero(&input_clone, &player_state);
        feat_id = FeatID::new(FEAT_ID_INIT);
    } else {
        if player_state.player_feats.contains(&input.feat_id) {
            return Err(BusinessError::FeatAlreadyDoneError());
        }
    }

    player_state.hero_state.hero_feats.push(feat_id.clone());
    player_state.player_feats.push(feat_id);
    players.insert(player_id, player_state.clone());
    Ok(player_state.clone())
}

pub async fn generate(
    generator: OpenAI,
    player_state: PlayerState,
) -> Result<GenerateResponse, BusinessError> {
    let general_prompt_context = "This is used in a video game making ancient greek heroes more powerful until they become gods.".to_string();
    let text_prompt_context = " The answer should contain around 50 signs.";
    let image_prompt_style = " This image is part of a game, where the artistic direction asks for an ancient Greek style, specifically Hellenic style. The artwork should look like a mosaic, sculpture, or vase-painting. It must depict a simple scene showing the main character in a situation. The image should be very clean and easy to understand.";

    let action_prompt: String;
    match feat::feat_description(&player_state.hero_state.hero_feats.last().unwrap()) {
        Ok(text) => action_prompt = text,
        Err(err) => return Err(err),
    }

    let image_prompt = general_prompt_context.clone()
        + &image_prompt_style
        + &player_state.hero_state.hero_image_prompt
        + &action_prompt;

    let text_prompt_system = general_prompt_context.clone() + &text_prompt_context;
    let text_prompt_user = player_state.hero_state.hero_text_prompt + &action_prompt;

    println!(
        "GENERATE...\ntext_prompt_system={}\ntext_prompt_user={}\nimage_prompt={}",
        text_prompt_system, text_prompt_user, image_prompt
    );

    let text_future = generator.generate_text(&text_prompt_system, &text_prompt_user);
    let image_future = generator.generate_image(&image_prompt);

    match try_join!(text_future, image_future) {
        Ok((text, image_url)) => {
            println!("GENERATE=>\ntext={}\nimage_url={}", text, image_url);
            let res = GenerateResponse { image_url, text };
            Ok(res)
        }
        Err(err) => Err(BusinessError::GenerationError(err.to_string())),
    }
}
