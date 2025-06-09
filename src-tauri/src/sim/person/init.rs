use std::backtrace::Backtrace;
use crate::integrations::ui::AppContext;
use crate::master_data::skills::SKILL_DEFS;
use crate::sim::person::components::PersonId;
use crate::sim::person::skills::ecs_components::{
    DomainContext, DomainCoordination, DomainExecution, DomainInterpersonal, TierApplied,
    TierConceptual, TierFoundational,
};
use crate::sim::person::skills::{Domain, GlobalSkill, SkillId, SkillSet, Tier};
use crate::sim::person::spawner::bounded_normal;
use crate::sim::person::stats::{StatType, Stats};
use crate::sim::registries::registry::{GlobalSkillNameMap, Registry};
use crate::sim::resources::global::{AssetBasePath, Dirty, SimManager};
use crate::sim::systems::global::UsedProfilePictureRegistry;
use legion::query::{ComponentFilter, EntityFilter, FilterResult};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{component, system, Entity, EntityStore, IntoQuery, Query, World};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, trace};
use tracing_subscriber::fmt::writer::WithFilter;
use crate::action_queues::game_speed_manager::GameSpeedManagerCommand::SetGameSpeed;
use crate::integrations::snapshots::snapshots::SnapshotState;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::new_game::new_game::StartingEmployeesConfig;

pub struct FirstRun {
    first : AtomicBool
}
impl FirstRun {
    pub fn is_first_run(&self) -> bool {
        self.first.load(Ordering::Relaxed)
    }
}
impl Default for FirstRun {
    fn default() -> Self {
        Self{
            first : AtomicBool::new(true),
        }
    }
}

impl FirstRun{
    pub fn mark_as_first_run(&self){
        self.first.store(true, Ordering::Relaxed);
    }
}

#[system]
pub fn unset_first_run_flag(#[resource] first: &Arc<FirstRun>)
{
    first.first.store(false, Ordering::Relaxed)
}

#[system]
pub fn generate_employees(
    cmd: &mut CommandBuffer,
    #[resource] asset_base_path: &AssetBasePath,
    #[resource] used_portrait: &UsedProfilePictureRegistry,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,
    #[resource] sim_manager: &Arc<SimManager>,
    query: &mut Query<(&GlobalSkill, &TierFoundational)>,
    world: &mut SubWorld,
) {
    use crate::sim::person::spawner::spawn_person;
    use crate::sim::person::spawner::TalentGrade::*;

    // let per_grade = [
    //     (Basic, 6),
    //     (Apt, 8),
    //     (Sharp, 3),
    //     (Gifted, 2),
    //     (Brilliant, 1),
    //     (Exceptional, 0),
    // ];
    // let per_grade = [
    //     (Basic, 0),
    //     (Apt, 0),
    //     (Sharp, 1),
    //     (Gifted, 0),
    //     (Brilliant, 0),
    //     (Exceptional, 0),
    // ];
    let mut q2 = <&GlobalSkill>::query().filter(component::<TierFoundational>()); //.filter(component::<DomainCoordination>());
    let global_skills = q2.iter(world).cloned().collect::<Vec<_>>();
    let config = (*sim_manager).employees_preset.read().config.clone();
    for (grade, count) in  config {
        for _ in 0..count {
            let (id, entity, person, stats, profile_picture, personality, skillset) =
                spawn_person(cmd, grade, asset_base_path, used_portrait, person_registry,&global_skills, 0);
        }
    }
    info!("Generated employees");
}

#[system]
pub fn load_global_skills(
    cmd: &mut CommandBuffer,
    #[resource] skill_registry: &Arc<Registry<SkillId, Entity>>,
    #[resource] global_skills_name_map: &Arc<GlobalSkillNameMap>,
) {
    // let bt = Backtrace::capture();
    // info!("Captured backtrace:\n{:?}", bt);
    // info!("{}", SKILL_DEFS.len());
    for skill_def in SKILL_DEFS {
        trace!("Single skill load {}", skill_def.id.to_string());
        let mut global_skill = GlobalSkill::from(skill_def);

        let id = SkillId(skill_registry.generate_id());
        global_skill.id = id;
        let tier = global_skill.tier;
        let domain = global_skill.domain.clone();
        global_skills_name_map
            .0
            .insert(global_skill.id, global_skill.name.clone());
        // info!("Skill for {:?}", global_skill.clone());
        let entity = cmd.push((global_skill,));

        match tier {
            Tier::Foundational => cmd.add_component(entity, TierFoundational),
            Tier::Conceptual => cmd.add_component(entity, TierConceptual),
            Tier::Applied => cmd.add_component(entity, TierApplied),
        }
        for d in domain {
            match d {
                Domain::Execution => cmd.add_component(entity, DomainExecution),
                Domain::Coordination => cmd.add_component(entity, DomainCoordination),
                Domain::Interpersonal => cmd.add_component(entity, DomainInterpersonal),
                Domain::Contextual => cmd.add_component(entity, DomainContext),
            }
        }

        skill_registry.insert(id, entity);

        // info!("{:?}", skill_registry.get_entity_from_id(&id).unwrap());
    }
    info!("Loading global skills... {}", skill_registry);
}

#[system]
pub fn init_company(
    #[resource] sim_manager: &Arc<SimManager>,
    cmd: &mut CommandBuffer,
) {
    info!("Initializing company...");
    let preset = sim_manager.company_preset.read();
    let new_company = Company{
        name: preset.name.clone(),
        slogan: preset.slogan.clone(),
    };

    cmd.push((new_company,PlayerControlled, Dirty));
}

