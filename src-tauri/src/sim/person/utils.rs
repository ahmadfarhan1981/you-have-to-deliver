use std::collections::HashMap;
use legion::Entity;
use crate::sim::person::skills::SkillId;
use crate::sim::registries::registry::Registry;


impl  Registry<SkillId,Entity>{
    // pub fn generate_globalskill_map(&self)->HashMap<SkillId, String>{
    //     let map = HashMap::new();
    //     for (id,entity) in self.iter() {
    //         let entry = world.entry_ref(entity).unwrap();
    //         let global_skill = entry.get_component::<GlobalSkill>().unwrap();
    //         map.insert(id, )
    //     }
    //     // let entity = skill_registry.get_entity_from_id(id).unwrap();
    //     // let entry = world.entry_ref(entity).unwrap();
    //     // let global_skill = entry.get_component::<GlobalSkill>().unwrap();
    //     // info!({"{:?}{} {:?}", global_skill.name, val, global_skill.related_stats });
    // }
}

