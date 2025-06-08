use crate::sim::person::components::PersonId;

#[derive(Debug)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub assigned_to: Option<PersonId>,
    pub remaining_work: f32,
    pub contribution_weight: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u32);
