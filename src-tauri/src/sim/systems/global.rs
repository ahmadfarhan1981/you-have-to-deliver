use crate::integrations::snapshots::{PersonSnapshot, SnapshotState};
use crate::integrations::snapshots_emitter::snapshots_emitter::{
    SnapshotEmitRegistry, SnapshotFieldEmitter,
};
use crate::integrations::ui::AppContext;
use crate::sim::person::components::ProfilePicture;
use crate::sim::person::skills::{GlobalSkill, SkillId, SkillSet};
use crate::sim::registries::registry::Registry;
use crate::sim::{
    person::components::Person, person::stats::Stats, resources::global::TickCounter,
};
use dashmap::DashSet;
use legion::systems::CommandBuffer;
use legion::{system, Entity, EntityStore, Query};
use std::fmt;
use std::sync::Arc;
use legion::world::SubWorld;
use tauri::Emitter;
use tracing::{error, info};
use crate::sim::person::skills::ecs_components::DomainInterpersonal;
use rayon::prelude::*;
#[system]
pub fn increase_sim_tick(#[resource] tick_counter: &Arc<TickCounter>) {
    tick_counter.tick()
}

#[system]
pub fn print_person(
    // global_skill: Option<&GlobalSkill>,
    // domain_interpersonal: &DomainInterpersonal,
    // stats: &Stats,
    // person: &Person,
    // skill_set: &SkillSet,
    query: &mut Query<(&Stats, &Person, &SkillSet)>,
    #[resource] app_state: &Arc<SnapshotState>,
    query_skill: &mut Query<&GlobalSkill>,
    world: &mut SubWorld,
    #[resource] skill_registry: &Arc<Registry<SkillId, Entity>>


) {
    // info!("Print");
    // for global_skill in query.iter(world) {
    //     // GlobalSkill with TierFoundational but NOT Disabled
    //     info!("@@{:?}", global_skill);
    // }
    // info!("print person");
    // info!("{:?}",global_skill);
    // info!("{:?}",domain_interpersonal);
    // // pub fn print_person(cmd: &mut CommandBuffer, e:&Entity, person: &Person, stats: &Stats, profile_picture: &ProfilePicture, #[resource] app_context: &Arc<AppContext>,) {
    //     info!("Printing person...");
    //info!("Person: {:?}", person);
    // let x = app_state.persons.iter().map(|e| e.value().clone()).collect::<Vec<PersonSnapshot>>();
    // for p in x{
    //     info!("Person: {:?}", p);
    // }
    
    



    
    // query.par_iter(world).for_each(|(stats, person, skillset)| {
    //     for (id, val) in &skillset.skills {
    //                 let entity = skill_registry.get_entity_from_id(id).unwrap();
    //                 let entry = world.entry_ref(entity).unwrap();
    //                 let global_skill = entry.get_component::<GlobalSkill>().unwrap();
    //                 info!({"{:?}{} {:?}", global_skill.name, val, global_skill.related_stats });
    //             }
    // });



    // let all_people =  query.iter(world).collect::<Vec<(&Stats, &Person, &SkillSet)>>();
    //  all_people.par_iter().for_each(|(stats, person, skill_set)| {
    //     info!({"{:?}", person});
    //     info!({"{:?}", stats});
    //
    //     for (id, val) in &skill_set.skills {
    //         let entity = skill_registry.get_entity_from_id(id).unwrap();
    //         let entry = world.entry_ref(entity).unwrap();
    //         let global_skill = entry.get_component::<GlobalSkill>().unwrap();
    //         info!({"{:?}{} {:?}", global_skill.name, val, global_skill.related_stats });
    //     }
    // });




    //     info!({"{:?}", person});
    //     info!({"{:?}", stats});
    //
    //     for (id, val) in &skill_set.skills {
    //         let entity = skill_registry.get_entity_from_id(id).unwrap();
    //         let entry = world.entry_ref(entity).unwrap();
    //         let global_skill = entry.get_component::<GlobalSkill>().unwrap();
    //         info!({"{:?}{} {:?}", global_skill.name, val, global_skill.related_stats });
    //     }
    // println!("Stats: {:?}", stats);
    // println!("Profile picture: {:?}", profile_picture);
}

#[derive(Default)]
pub struct UsedProfilePictureRegistry {
    pub used_profile_pictures: DashSet<ProfilePicture>,
}

impl fmt::Debug for UsedProfilePictureRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UsedProfilePictureRegistry[{}]",
            format!("{} entries", self.used_profile_pictures.len())
        )
        // write!(
        //     f,
        //     "{}[{}. next_id={}]",
        //     "PersonRegistry".bold().bright_cyan(),
        //     format!("{count} entries").green(),
        //     format!("{next}").yellow().bold()
        // )

    }
}

// pub fn test() {
//     let mut snapshot_registry = SnapshotEmitRegistry::new();
//     let snapshot_state = SnapshotState::default();
//     let g = snapshot_state.game_speed;

//     let x = SnapshotFieldEmitter {
//         field: Arc::new(g),
//         config: Default::default(),
//     };
//     snapshot_registry.register(x);
// }

pub trait ReplaceIfChanged {
    fn replace_if_changed(&mut self, new: Self) -> bool;
}

// impl<T> ReplaceIfChanged for T
// where
//     T: PartialEq + Sized,
// {
//     fn replace_if_changed(&mut self, new: Self) -> bool {
//         if *self != new {
//             *self = new;
//             true
//         } else {
//             false
//         }
//     }
// }
//
// pub trait ReplaceIfChangedFromRef {
//     fn replace_if_changed_ref(&mut self, new: &Self) -> bool;
// }
//
// impl<T> ReplaceIfChangedFromRef for T
// where
//     T: PartialEq + Clone,
// {
//     fn replace_if_changed_ref(&mut self, new: &Self) -> bool {
//         if self != new {
//             *self = new.clone();
//             true
//         } else {
//             false
//         }
//     }
// }
