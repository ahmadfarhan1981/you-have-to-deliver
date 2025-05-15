use std::collections::HashSet;
use dashmap::DashSet;
use legion::{system, systems::CommandBuffer, world, World};
use crate::sim::person::components::ProfilePicture;
use crate::sim::resources::global::AssetBasePath;

use super::spawner::{spawn_person, TalentGrade};

#[system]
pub fn generate_employees(
    cmd: &mut CommandBuffer,
    #[resource] asset_base_path: &AssetBasePath,
    #[resource] used_portrait: &DashSet<ProfilePicture>,
) {
    use TalentGrade::*;

    let per_grade = [
        (Basic, 2),
        (Apt, 6),
        (Sharp, 2),
        (Gifted, 1),
        (Brilliant, 1),
        (Exceptional, 1),
    ];

    for (grade, count) in per_grade {
        for _ in 0..count {
            let entity = spawn_person(grade, asset_base_path, used_portrait);
            cmd.push(entity);
        }
    }
}
