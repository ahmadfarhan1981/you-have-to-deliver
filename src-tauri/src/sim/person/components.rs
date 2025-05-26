use fmt::Display;
use serde::{Deserialize, Serialize};
use std::fmt;
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
    #[default]
    Male,
    Female,
}
impl Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Gender::Male => "male",
            Gender::Female => "female",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, EnumIter, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Default, Copy)]
pub enum ProfilePictureCategory {
    #[default]
    Office,
    Social,
}
impl ProfilePictureCategory {
    pub fn as_file_category_number(&self) -> i8 {
        match self {
            ProfilePictureCategory::Office => 1,
            ProfilePictureCategory::Social => 2,
        }
    }
}
impl ProfilePictureCategory {
    pub fn from_file_category_number(val: u8) -> Option<Self> {
        match val {
            1 => Some(ProfilePictureCategory::Office),
            2 => Some(ProfilePictureCategory::Social),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Hash, Default, Copy)]
pub struct ProfilePicture {
    pub gender: Gender,
    pub category: ProfilePictureCategory,
    pub batch: i8,
    pub index: i8,
}
