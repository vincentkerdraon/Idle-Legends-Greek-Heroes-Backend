extern crate api;
use api::*;
use std::{collections::HashMap, fmt};
extern crate openai;
use openai::OpenAI;

#[derive(Debug)]
pub enum BusinessError {
    InvalidInputError(String),
    GenerationError(String),
    PlayerNotFoundError(String),
    FeatUnknownError(String),
    FeatAlreadyDoneError(String),
}

impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusinessError::InvalidInputError(msg) => write!(f, "Invalid Input Error: {}", msg),
            BusinessError::GenerationError(msg) => write!(f, "Generation Error: {}", msg),
            BusinessError::PlayerNotFoundError(msg) => write!(f, "Player Not Found Error: {}", msg),
            BusinessError::FeatUnknownError(msg) => write!(f, "Feat Unknown Error: {}", msg),
            BusinessError::FeatAlreadyDoneError(msg) => {
                write!(f, "Feat Already Done Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for BusinessError {}

// #[derive(DerefMut)]
pub struct Business {
    open_ai: OpenAI,
    players: HashMap<PlayerID, PlayerState>,
}

impl Business {
    pub fn new(open_ai: OpenAI) -> Self {
        Self {
            open_ai,
            players: HashMap::new(),
        }
    }

    pub async fn logic(
        &mut self,
        input: GenerateRequest,
    ) -> Result<GenerateResponse, BusinessError> {
        let general_prompt_context = "This is used in a video game making ancient greek heroes more powerful until they become gods.".to_string();
        let text_prompt_context = "The answer should contain around 50 signs.";

        let mut player_state: PlayerState;

        let input_clone = input.clone();
        let player_id = input.player_id.clone();

        if let Some(player_state_ref) = self.players.get(&input.player_id) {
            player_state = player_state_ref.clone();
        } else {
            player_state = PlayerState {
                player_id: input.player_id,
                ..Default::default()
            };
        }

        let mut hero_state: HeroState;
        let action_prompt: String;

        if let Some(hero_state_ref) = player_state.hero_states.last() {
            hero_state = hero_state_ref.clone();
            if hero_state.hero_feats.contains(&input.feat) {
                return Err(BusinessError::FeatAlreadyDoneError(input.feat.to_string()));
            }
            match self.feat_description(&input.feat) {
                Ok(text) => action_prompt = text,
                Err(err) => return Err(err),
            }
        } else {
            //first time with this hero. Send a welcome
            (action_prompt, hero_state) = self.prepare_new_character(&input_clone, &player_state);
        }

        let hero_text_prompt = hero_state.hero_text_prompt.clone();

        hero_state.hero_feats.push(input.feat);
        player_state.hero_states.push(hero_state);
        self.players.insert(player_id, player_state);

        let image_prompt_context = String::new();

        let text_prompt_system = general_prompt_context + text_prompt_context;
        let text_prompt_user = hero_text_prompt + &action_prompt;
        match self
            .open_ai
            .call_openai_text(&text_prompt_system, &text_prompt_user)
            .await
        {
            Ok(text) => Ok(GenerateResponse {
                image_url: "TODO".to_string(),
                text,
            }),
            Err(err) => Err(BusinessError::GenerationError(err.to_string())),
        }
    }

    fn feat_description(&self, feat_id: &FeatID) -> Result<String, BusinessError> {
        match feat_id.as_str() {
            "ooo" => Ok(String::from("aaa")),
            _ => Err(BusinessError::FeatUnknownError(feat_id.to_string())),
        }
    }

    pub fn prepare_new_character(
        &self,
        input: &GenerateRequest,
        player_state: &PlayerState,
    ) -> (String, HeroState) {
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

        return     (
        "The character is between 8 and 14 years old. They are level 0 and doesn't seem to be able to do much.".to_string(),
        heroes[0].clone(),
    );
    }
}
