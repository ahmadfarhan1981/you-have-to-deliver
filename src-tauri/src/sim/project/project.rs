use crate::sim::project::functionality::FunctionalityId;
use crate::sim::project::requirement::RequirementId;

#[derive(Debug)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub requirements: Vec<RequirementId>,
    pub functionalities: Vec<FunctionalityId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(pub u32);
