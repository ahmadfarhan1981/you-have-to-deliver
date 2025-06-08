#[derive(Debug)]
pub struct Requirement {
    pub id: RequirementId,
    pub description: String,
    // Future: tags, hidden_expectations
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequirementId(pub u32);