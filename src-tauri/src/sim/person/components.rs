use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Copy, Serialize, Deserialize, Hash, Eq, PartialEq, Default)]
pub enum Gender {
    #[default]Male,
    Female,
}


#[derive(Debug, EnumIter, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Default, Copy)]
pub enum ProfilePictureCategory{
    #[default]Office,
    Social,
}


#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Hash, Default, Copy)]
pub struct ProfilePicture {
    pub gender: Gender,
    pub category: ProfilePictureCategory,
    pub batch: i8,
    pub index: i8
}

