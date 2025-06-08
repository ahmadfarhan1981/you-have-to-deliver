use crate::sim::project::requirement::RequirementId;
use crate::sim::project::task::TaskId;

#[derive(Debug)]
pub struct Functionality {
    pub id: FunctionalityId,
    pub label: String,
    pub related_requirements: Vec<RequirementId>,
    pub task_ids: Vec<TaskId>,
    pub progress: f32, // [0.0 - 1.0]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionalityId(pub u32);