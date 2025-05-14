use legion::{system, systems::CommandBuffer, world, World};
use crate::sim::resources::global::AssetBasePath;

use super::spawner::{spawn_person, TalentGrade};

#[system]
pub fn generate_employees(cmd: &mut CommandBuffer, #[resource] asset_base_path: &AssetBasePath) {
    cmd.push(spawn_person(TalentGrade::Brilliant, &asset_base_path));
    //generate_employees(asset_base_path);
}