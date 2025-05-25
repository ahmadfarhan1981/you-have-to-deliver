use legion::{system, Entity};
use legion::systems::CommandBuffer;
use std::sync::Arc;
use tracing::{info, trace};
use crate::master_data::skills::SKILL_DEFS;
use crate::sim::person::components::PersonId;
use crate::sim::person::skills::{GlobalSkill, SkillId};

use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::AssetBasePath;
use crate::sim::systems::global::UsedProfilePictureRegistry;

#[system]
pub fn generate_employees(
    cmd: &mut CommandBuffer,
    #[resource] asset_base_path: &AssetBasePath,
    #[resource] used_portrait: &UsedProfilePictureRegistry,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,

) {
    use crate::sim::person::spawner::spawn_person;
    use crate::sim::person::spawner::TalentGrade::*;

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
            spawn_person(cmd, grade, asset_base_path, used_portrait, person_registry);
        }
    }
}

#[system]
pub fn load_global_skills(
    cmd: &mut CommandBuffer,
    #[resource] skill_registry: &Arc<Registry<SkillId, Entity>>,
) {
    info!("{}", SKILL_DEFS.len());
    for skill_def in SKILL_DEFS{
        trace!("Single skill load {}", skill_def.id.to_string());
        let mut global_skill = GlobalSkill::from(skill_def);

        let id = SkillId(skill_registry.generate_id());
        global_skill.id = id;
        let entity = cmd.push((global_skill,));
        skill_registry.insert(id, entity);

        // info!("{:?}", skill_registry.get_entity_from_id(&id).unwrap());
    }
    info!("Loading global skills... {}", skill_registry );
}
