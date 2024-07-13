use api::FeatID;

use crate::error::BusinessError;

pub fn feat_description(feat_id: &FeatID) -> Result<String, BusinessError> {
    match feat_id.as_str() {
        //FIXME use FEAT_ID_INIT
        "init" => Ok(String::from("The character is between 8 and 14 years old. They are level 0 and is not able to do much. Show the very beginning")),

        "featA" => Ok(String::from("The character just killed a lion and seems injured from the fight.")),
        _ => Err(BusinessError::FeatUnknownError(feat_id.clone())),
    }
}

//FIXME
// "prompt": " For this image specifically, the character is the son of Medusa and the Minotaur. He is a 25-year-old male with a straight nose and wavy brown hair. He is standing in front of the Parthenon, a temple dedicated to the goddess Athena. The Parthenon decorative sculptures are considered some of the high points of classical Greek art.",
// "prompt": "Create a historically accurate image of a talented ancient Greek sculptor named Pygmalion working on an exquisitely beautiful ivory statue of a woman. The statue, named Galatea, is so lifelike that she appears real, with delicate features and flowing hair. Pygmalion gazes at her with deep affection and longing, surrounded by his workshop filled with sculpting tools and other artworks. In the background, the goddess Aphrodite looks on, ready to bring the statue to life.",
// "prompt": "Create a realistic image of the ancient Greek Dedalus creating the Labyrinth for King Minos of Crete which imprisoned the Minotaur. It must be historically accurate.",
