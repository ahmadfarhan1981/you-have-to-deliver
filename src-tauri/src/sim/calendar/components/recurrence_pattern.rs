use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecurrencePattern {
    None,
    Daily,
    Weekly,
    Weekdays,  // Mon-Fri (days 1-5)
    Custom { days_of_week: Vec<u8> }, // 1=Monday, 7=Sunday
    EveryNWeeks { n: u8, day_of_week: u8 }, // Every N weeks on specific day
}
