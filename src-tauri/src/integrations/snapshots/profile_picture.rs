use serde::{Deserialize, Serialize};
use crate::sim::person::components::ProfilePicture;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfilePictureSnapshot {
    pub gender: String,
    pub category: i8,
    pub batch: i8,
    pub index: i8,
}

impl PartialEq<&ProfilePicture> for ProfilePictureSnapshot {
    fn eq(&self, other: &&ProfilePicture) -> bool {
        self.gender == other.gender.to_string()
            && self.category == other.category.as_file_category_number()
            && self.batch == other.batch
            && self.index == other.index
    }
}

impl From<&ProfilePicture> for ProfilePictureSnapshot {
    fn from(picture: &ProfilePicture) -> Self {
        Self {
            gender: picture.gender.to_string(),
            category: picture.category.as_file_category_number(),
            batch: picture.batch,
            index: picture.index,
        }
    }
}