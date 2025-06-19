use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
// pub struct Customer {
//     id: CustomerId,
//     name: String,
//     industry: IndustryType,           // e.g., Retail, Finance, Manufacturing
//     reputation: u8,                   // Optional: affects future trust or budget
//     projects: Vec<ProjectOffer>,     // List of projects currently offered
// }
// 
// #[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
// pub struct IndustryType{
//     
// }