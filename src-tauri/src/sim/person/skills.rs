

struct SkillRequirement{
    direct_requirement: Vec<SkillLink>,
    indirect_requirement: Vec<SkillLink>,
}

struct SkillBoost{
    direct_boost: Vec<SkillLink>,
    indirect_boost: Vec<SkillLink>,
}


struct GlobalSkill {
    feedback_link:Vec<SkillLink>,
    feedforward_link:Vec<SkillLink>,
}

struct SkillLink{
    target: GlobalSkill,
    factor: u8,

}

struct SkillThreshold{
    upper: u8,
    lower: u8,
}
struct AssignedSkill{

}

