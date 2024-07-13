use api::{GenerateRequest, HeroState, PlayerState};

pub fn prepare_new_hero(input: &GenerateRequest, player_state: &PlayerState) -> HeroState {
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
