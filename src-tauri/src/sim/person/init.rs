use crate::master_data::skills::SKILL_DEFS;
use crate::sim::person::components::PersonId;
use crate::sim::person::skills::{Domain, GlobalSkill, SkillId, Tier};
use legion::systems::CommandBuffer;
use legion::{component, system, Entity, EntityStore, IntoQuery, Query, World};
use std::sync::Arc;
use legion::world::SubWorld;
use tracing::{info, trace};
use tracing_subscriber::fmt::writer::WithFilter;
use crate::integrations::ui::AppContext;
use crate::sim::person::skills::ecs_components::{DomainContext, DomainCoordination, DomainExecution, DomainInterpersonal, TierApplied, TierConceptual, TierFoundational};
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::AssetBasePath;
use crate::sim::systems::global::UsedProfilePictureRegistry;



#[system]
pub fn generate_employees(
    cmd: &mut CommandBuffer,
    #[resource] asset_base_path: &AssetBasePath,
    #[resource] used_portrait: &UsedProfilePictureRegistry,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,
    query: &mut Query<&GlobalSkill>,
    world: &mut SubWorld,
) {
    use crate::sim::person::spawner::spawn_person;
    use crate::sim::person::spawner::TalentGrade::*;


    // let per_grade = [
    //     (Basic, 4),
    //     (Apt, 8),
    //     (Sharp, 3),
    //     (Gifted, 2),
    //     (Brilliant, 1),
    //     (Exceptional, 1),
    // ];
    let per_grade = [
        (Basic, 0),
        (Apt, 20),
        (Sharp, 0),
        (Gifted, 0),
        (Brilliant, 0),
        (Exceptional, 0),
    ];
    for (grade, count) in per_grade {
        for _ in 0..count {
            let (id, entity) = spawn_person(cmd, grade, asset_base_path, used_portrait, person_registry);

            // let skills = query.


        }
    }
    info!("Generated employees");
    for global_skill in query.iter(world) {
        // GlobalSkill with TierFoundational but NOT Disabled
        info!("@@{:?}", global_skill);
    }
    let mut q2 = <&GlobalSkill>::query().filter(component::<TierFoundational>()).filter(component::<DomainCoordination>());
    let global_skills = q2.iter(world).collect::<Vec<_>>();

    for skill in global_skills{
        info!("{:?}", skill);
    }
}

#[system]
pub fn load_global_skills(
    cmd: &mut CommandBuffer,
    #[resource] skill_registry: &Arc<Registry<SkillId, Entity>>,
) {
    // info!("{}", SKILL_DEFS.len());
    for skill_def in SKILL_DEFS {
        trace!("Single skill load {}", skill_def.id.to_string());
        let mut global_skill = GlobalSkill::from(skill_def);

        let id = SkillId(skill_registry.generate_id());
        global_skill.id = id;
        let tier = global_skill.tier;
        let domain = global_skill.domain.clone();
        // info!("Skill for {:?}", global_skill.clone());
        let entity = cmd.push((global_skill,));


        match tier{
            Tier::Foundational => {
                cmd.add_component (entity, TierFoundational)
            }
            Tier::Conceptual => {
                cmd.add_component (entity,TierConceptual)
            }
            Tier::Applied => {
                cmd.add_component (entity, TierApplied)
            }
        }
        for d in domain{
            match d{
                Domain::Execution => {
                    cmd.add_component (entity, DomainExecution)
                }
                Domain::Coordination => {
                    cmd.add_component (entity, DomainCoordination)
                }
                Domain::Interpersonal => {
                    cmd.add_component (entity, DomainInterpersonal)
                }
                Domain::Contextual => {
                    cmd.add_component(entity, DomainContext)
                }
            }
        }

        skill_registry.insert(id, entity);


        // info!("{:?}", skill_registry.get_entity_from_id(&id).unwrap());
    }
    info!("Loading global skills... {}", skill_registry);
}
