extern crate api;
use api::*;
use std::{collections::HashMap, fmt};
extern crate openai;
use openai::OpenAI;
use tokio::try_join;

#[derive(Debug)]
pub enum BusinessError {
    GenerationError(String),
    FeatUnknownError(FeatID),
    FeatAlreadyDoneError(),
}

impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusinessError::GenerationError(msg) => write!(f, "Generation Error: {}", msg),
            BusinessError::FeatUnknownError(feat_id) => {
                write!(f, "Feat Unknown Error: {:?}", feat_id)
            }
            BusinessError::FeatAlreadyDoneError() => {
                write!(f, "Feat Already Done")
            }
        }
    }
}

impl std::error::Error for BusinessError {}

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
        player_state.hero_state = prepare_new_hero(&input_clone, &player_state);
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

fn prepare_new_hero(input: &GenerateRequest, player_state: &PlayerState) -> HeroState {
    let heroes = vec![
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Theseus"),
    hero_text_prompt: String::from("Theseus is a heroic ancient Greek man known for his intelligence and bravery. He is the son of Aegeus and Aethra, and is famous for slaying the Minotaur and unifying Attica."),
    hero_image_prompt: String::from("Theseus is a heroic ancient Greek man with short, curly black hair, a strong jawline, and deep-set almond-shaped eyes. He has a muscular build, tanned olive skin, and is dressed in traditional Greek armor with a bronze chest plate and a red cape."),
    ..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Heracles"),
    hero_text_prompt: String::from("Heracles is a legendary ancient Greek man known for his incredible strength and heroic feats. He is the son of Zeus and Alcmene, and is famous for completing the Twelve Labors."),
    hero_image_prompt: String::from("Heracles is a muscular ancient Greek man with short, curly brown hair, a strong jawline, and deep-set eyes. He has tanned skin and is dressed in a lion's skin, holding a large club."),
..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Odysseus"),
    hero_text_prompt: String::from("Odysseus is a cunning ancient Greek man known for his intelligence and resourcefulness. He is the son of Laertes and Anticlea, and is famous for his long journey home from the Trojan War."),
    hero_image_prompt: String::from("Odysseus is a wise ancient Greek man with medium-length, wavy brown hair, a sharp nose, and thoughtful eyes. He has a lean build, fair skin, and is dressed in a simple tunic with a cloak."),
..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Perseus"),
    hero_text_prompt: String::from("Perseus is a brave ancient Greek man known for his heroic deeds. He is the son of Zeus and Danae, and is famous for slaying Medusa and rescuing Andromeda."),
    hero_image_prompt: String::from("Perseus is a brave ancient Greek man with short, wavy black hair, a straight nose, and determined eyes. He has an athletic build, olive skin, and is dressed in Greek armor, holding a reflective shield and a sword."),
..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Achilles"),
    hero_text_prompt: String::from("Achilles is a fierce ancient Greek man known for his prowess in battle. He is the son of Peleus and Thetis, and is famous for his role in the Trojan War and his near invulnerability."),
    hero_image_prompt: String::from("Achilles is a fierce ancient Greek man with long, flowing blonde hair, a strong jawline, and piercing eyes. He has a muscular build, fair skin, and is dressed in bronze armor with a plumed helmet."),
..Default::default()
},
HeroState {  hero_id:input.hero_id.clone() ,
    hero_name: String::from("Jason"),
    hero_text_prompt: String::from("Jason is a determined ancient Greek man known for his quest for the Golden Fleece. He is the son of Aeson, and is famous for leading the Argonauts on their adventures."),
    hero_image_prompt: String::from("Jason is a determined ancient Greek man with short, wavy brown hair, a straight nose, and focused eyes. He has a lean build, tanned skin, and is dressed in a tunic and sandals, holding a golden fleece."),
    ..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Orpheus"),
    hero_text_prompt: String::from("Orpheus is a talented ancient Greek man known for his musical abilities. He is the son of Apollo and Calliope, and is famous for his journey to the underworld to retrieve his wife, Eurydice."),
    hero_image_prompt: String::from("Orpheus is a talented ancient Greek man with long, wavy black hair, a gentle face, and deep, soulful eyes. He has a slender build, fair skin, and is dressed in a simple robe, holding a lyre."),
..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Atalanta"),
    hero_text_prompt: String::from("Atalanta is a beautiful ancient Greek woman known for her speed and hunting skills. She is the daughter of Iasus and Clymene, and is famous for participating in the Calydonian Boar Hunt."),
    hero_image_prompt: String::from("Atalanta is a beautiful ancient Greek woman with long, wavy brown hair, high cheekbones, and full lips. She has an oval face, fair skin, and a lean, athletic build, dressed in a flowing white chiton with golden accents and a laurel wreath on her head."),
..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Hector"),
    hero_text_prompt: String::from("Hector is a noble ancient Greek man known for his bravery and sense of duty. He is the son of Priam and Hecuba, and is famous for his role in the Trojan War."),
    hero_image_prompt: String::from("Hector is a noble ancient Greek man with short, curly black hair, a broad nose, and kind eyes. He has a strong build, olive skin, and is dressed in bronze armor with a red cape."),
..Default::default()
},
HeroState {
    hero_id:input.hero_id.clone(),
    hero_name: String::from("Pandora"),
    hero_text_prompt: String::from("Pandora is a beautiful ancient Greek woman known for her curiosity and the box she opened that released all the evils into the world. She is the first woman created by the gods, gifted with beauty and talents."),
    hero_image_prompt: String::from("Pandora is a beautiful ancient Greek woman with long, flowing blonde hair, delicate features, and bright eyes. She has fair skin and is dressed in a white chiton, holding a beautifully decorated box."),
..Default::default()
}
];

    return heroes[0].clone();
}

pub async fn generate(
    generator: OpenAI,
    player_state: PlayerState,
) -> Result<GenerateResponse, BusinessError> {
    let general_prompt_context = "This is used in a video game making ancient greek heroes more powerful until they become gods.".to_string();
    let text_prompt_context = " The answer should contain around 50 signs.";
    let image_prompt_style = " This image is part of a game, where the artistic direction asks for an ancient Greek style, specifically Hellenic style. The artwork should look like a mosaic, sculpture, or vase-painting. It must depict a simple scene showing the main character in a situation. The image should be very clean and easy to understand.";

    let action_prompt: String;
    match feat_description(&player_state.hero_state.hero_feats.last().unwrap()) {
        Ok(text) => action_prompt = text,
        Err(err) => return Err(err),
    }

    let image_prompt = general_prompt_context.clone()
        + &image_prompt_style
        + &player_state.hero_state.hero_image_prompt
        + &action_prompt;

    let text_prompt_system = general_prompt_context.clone() + &text_prompt_context;
    let text_prompt_user = player_state.hero_state.hero_text_prompt + &action_prompt;

    let text_future = generator.generate_text(&text_prompt_system, &text_prompt_user);
    let image_future = generator.generate_image(&image_prompt);

    match try_join!(text_future, image_future) {
        Ok((text, image_url)) => {
            let res = GenerateResponse { image_url, text };
            Ok(res)
        }
        Err(err) => Err(BusinessError::GenerationError(err.to_string())),
    }
}

fn feat_description(feat_id: &FeatID) -> Result<String, BusinessError> {
    match feat_id.as_str() {
        //FIXME use FEAT_ID_INIT
        "init" => Ok(String::from("The character is between 8 and 14 years old. They are level 0 and doesn't seem to be able to do much.")),
        "featA" => Ok(String::from("The character just killed a lion and seems injured from the fight.")),
        _ => Err(BusinessError::FeatUnknownError(feat_id.clone())),
    }
}

//FIXME
// "prompt": " For this image specifically, the character is the son of Medusa and the Minotaur. He is a 25-year-old male with a straight nose and wavy brown hair. He is standing in front of the Parthenon, a temple dedicated to the goddess Athena. The Parthenon decorative sculptures are considered some of the high points of classical Greek art.",
// "prompt": "Create a historically accurate image of a talented ancient Greek sculptor named Pygmalion working on an exquisitely beautiful ivory statue of a woman. The statue, named Galatea, is so lifelike that she appears real, with delicate features and flowing hair. Pygmalion gazes at her with deep affection and longing, surrounded by his workshop filled with sculpting tools and other artworks. In the background, the goddess Aphrodite looks on, ready to bring the statue to life.",
// "prompt": "Create a realistic image of the ancient Greek Dedalus creating the Labyrinth for King Minos of Crete which imprisoned the Minotaur. It must be historically accurate.",
