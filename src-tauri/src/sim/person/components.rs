use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::error::Category;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub person_id: PersonId,
    pub name: String,
    pub gender: Gender,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct PersonId(pub u32);
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, EnumIter)]
enum ProfilePictureCategory{
    Office = 1,
    Social = 2 
}

pub struct ProfilePicture {
    gender: Gender,
    category: ProfilePictureCategory,
    batch: i8,
    index: i8
}

fn generate_profile_picture(gender: Gender) -> ProfilePicture {
    let mut rng = rand::thread_rng();
    let last_batch_number= 5;
    
    ProfilePicture{
        gender: gender,
        category: ProfilePictureCategory::iter().choose(&mut rng).unwrap(),
        batch: rng.random_range(1..=last_batch_number),
        index: rng.random_range(0..=8),
    }
}