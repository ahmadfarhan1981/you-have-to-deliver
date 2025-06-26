use crate::action_queues::game_speed_manager::GameSpeedManagerCommand::SetGameSpeed;
use crate::action_queues::sim_manager::SimManager;
use crate::integrations::events::{emit_app_event, AppEventType};
use crate::integrations::snapshots::snapshots::SnapshotState;
use crate::integrations::ui::AppContext;
use crate::master_data::skills::{GLOBAL_SKILLS, SKILL_DEFS};
use crate::sim::calendar::components::EventType;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::new_game::new_game::StartingEmployeesConfig;
use crate::sim::person::components::PersonId;
use crate::sim::person::skills::ecs_components::{
    DomainContext, DomainCoordination, DomainExecution, DomainInterpersonal, TierApplied,
    TierConceptual, TierFoundational,
};
use crate::sim::person::skills::{Domain, GlobalSkill, SkillId, SkillSet, Tier};
use crate::sim::person::spawner::bounded_normal;
use crate::sim::person::stats::{StatType, Stats};
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::{AssetBasePath, Dirty};
use crate::sim::systems::global::UsedProfilePictureRegistry;
use legion::query::{ComponentFilter, EntityFilter, FilterResult};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{component, system, Entity, EntityStore, IntoQuery, Query, World};
use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{info, trace};
use tracing_subscriber::fmt::writer::WithFilter;

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
pub fn emit_done_setup_event(#[resource] app_context: &Arc<AppContext>)
{
    info!("emit_done_setup_event");
    emit_app_event(&app_context.app_handle, AppEventType::InitDone)
}

// Create a resource to trigger employee generation
#[derive(Default)]
pub struct ShouldGenerateEmployees(pub bool);

#[system]
#[read_component(ShouldGenerateEmployees)]
pub fn generate_employees(
    cmd: &mut CommandBuffer,
    #[resource] asset_base_path: &AssetBasePath,
    #[resource] used_portrait: &UsedProfilePictureRegistry,
    #[resource] person_registry: &Arc<Registry<PersonId, Entity>>,
    #[resource] sim_manager: &Arc<SimManager>,
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
    let global_skills:Vec<_> = GLOBAL_SKILLS.get().unwrap().iter().filter(|(id, skill)|  skill.tier == Tier::Foundational ).map(|(id, skill)| skill).collect();
    // let global_skills = q2.iter(world).cloned().collect::<Vec<_>>();
    let config = (*sim_manager).employees_preset.read().config.clone();
    for (grade, count) in  config {
        for _ in 0..count {
            let (id, entity, person, stats, profile_picture, personality, skillset) =
                spawn_person(cmd, grade, asset_base_path, used_portrait, person_registry,&global_skills, 0);
        }
    }
    info!("Generated employees");
}






pub fn load_global_skills_to_static(
) {
    let mut global_skill_hashmap = HashMap::<SkillId, GlobalSkill>::new();
    
    for skill_def in SKILL_DEFS {
        trace!("Single skill load {}", skill_def.id.to_string());
        let mut global_skill = GlobalSkill::from(skill_def);
        let id = global_skill.id.clone();
        global_skill_hashmap.insert(id, global_skill);
    }
    GLOBAL_SKILLS.set(global_skill_hashmap);
    
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

