use super::components::Person;
use super::stats::{StatType, Stats, StatsConfig};
use crate::sim::person::components::{Gender, PersonId, ProfilePicture, ProfilePictureCategory};
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::person::personality_matrix::{PersonalityAxis, PersonalityMatrix};
use crate::sim::person::skills::{GlobalSkill, SkillSet};
use crate::sim::person::stat_sculpter::{
    sculpt_axis_bias, sculpt_blindspot, sculpt_contrasting_pair, sculpt_monofocus,
};
use crate::sim::registries::registry::Registry;
use crate::sim::resources::global::{AssetBasePath, Dirty};
use crate::sim::systems::global::UsedProfilePictureRegistry;
use crate::sim::utils::debugging::DebugDisplayComponent;
use bincode::{Decode, Encode};
use dashmap::DashSet;
use legion::systems::CommandBuffer;
use legion::Entity;
use rand::prelude::*;
use rand::seq::IteratorRandom;
use rand::{rng, Rng};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace};
use crate::sim::ai::goap::CurrentGoal;
use crate::sim::person::morale::StressLevel;

pub fn bounded_normal(mean: f64, std_dev: f64, min: i16, max: i16) -> i16 {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rng();

    for _ in 0..10 {
        let sample = normal.sample(&mut rng).round();
        if sample >= min.into() && sample <= max.into() {
            return sample as i16;
        }
    }

    // Escape hatch: clamp after 10 failed tries
    let fallback = normal.sample(&mut rng).round();
    return fallback.clamp(min.into(), max.into()) as i16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default, Encode, Decode)]
#[serde(rename_all = "PascalCase")]
pub enum TalentGrade {
    Basic,
    #[default]
    Apt,
    Sharp,
    Gifted,
    Brilliant,
    Exceptional,
}

impl TalentGrade {
    pub fn stat_distribution(self) -> (f64, f64) {
        match self {
            TalentGrade::Basic => (45.0, 20.0), // Lots of weak stats, occasional spikes
            TalentGrade::Apt => (55.0, 15.0),
            TalentGrade::Sharp => (65.0, 12.0),
            TalentGrade::Gifted => (75.0, 9.0),
            TalentGrade::Brilliant => (85.0, 6.0),
            TalentGrade::Exceptional => (93.0, 4.0), // Near-perfect, but not boringly maxed
        }
    }
}
impl Display for TalentGrade {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label = match self {
            TalentGrade::Basic => "Basic",
            TalentGrade::Apt => "Apt",
            TalentGrade::Sharp => "Sharp",
            TalentGrade::Gifted => "Gifted",
            TalentGrade::Brilliant => "Brilliant",
            TalentGrade::Exceptional => "Exceptional",
        };
        write!(f, "{}", label)
    }
}
pub fn get_random_line_from_file(asset_path: AssetBasePath) -> Option<String> {
    // Build full path to names/first.txt
    let base_path = Path::new(&asset_path.0);
    let file_path = base_path.join("names").join("first.txt");
    trace!("Resolved file path: {:?}", file_path);

    // Attempt to open the file
    let file = File::open(&file_path).ok()?;
    let reader = BufReader::new(file);

    // Choose a random line using iterator sampling
    let mut rng = rng();
    let line = reader.lines().filter_map(Result::ok).choose(&mut rng);

    if line.is_none() {
        error!("File is empty or unreadable!");
    }

    line
}

#[derive(Debug, Eq, PartialEq)]
enum NameParts {
    First,
    Last,
}

pub enum NameDictionary {
    Male,
    Female,
    Last,
}

impl NameDictionary {
    pub fn filename(&self) -> &'static str {
        match self {
            NameDictionary::Male => "male.txt",
            NameDictionary::Female => "female.txt",
            NameDictionary::Last => "last.txt",
        }
    }
}

pub fn generate_full_name(gender: &Gender, asset_path: &AssetBasePath) -> Option<String> {
    let first = generate_name_part(gender, &NameParts::First, &asset_path)?;
    let last = generate_name_part(gender, &NameParts::Last, &asset_path)?;
    Some(format!("{} {}", first, last))
}
fn generate_name_part(
    gender: &Gender,
    part: &NameParts,
    asset_path: &AssetBasePath,
) -> Option<String> {
    let base_path = Path::new(&asset_path.0);

    let filename = if *part == NameParts::First {
        match gender {
            Gender::Male => NameDictionary::Male.filename(),
            Gender::Female => NameDictionary::Female.filename(),
        }
    } else {
        NameDictionary::Last.filename()
    };

    let file_path = base_path.join("dictionaries").join("names").join(filename);

    let file = File::open(&file_path)
        .map_err(|e| {
            eprintln!("Failed to open file {:?}: {}", file_path, e);
        })
        .ok()?;

    let reader = BufReader::new(file);
    let mut rng = rng();

    let line = reader.lines().filter_map(Result::ok).choose(&mut rng);

    if line.is_none() {
        eprintln!("No lines found in file: {:?}", file_path);
    }

    line
}

pub fn random_gender() -> Gender {
    let mut rng = rand::rng();
    if rng.random_bool(0.5) {
        Gender::Male
    } else {
        Gender::Female
    }
}

fn generate_stats(tier: TalentGrade) -> Stats {
    let (mean, std_dev) = tier.stat_distribution();

    macro_rules! gen {
        () => {{
            let val = bounded_normal(mean, std_dev, 0, 100) as u16;
            (val, val as u32 * 1000)
        }};
    }

    let (judgement, _judgement_raw) = gen!();
    let (creativity, _creativity_raw) = gen!();
    let (systems, _systems_raw) = gen!();
    let (precision, _precision_raw) = gen!();
    let (focus, _focus_raw) = gen!();
    let (discipline, _discipline_raw) = gen!();
    let (empathy, _empathy_raw) = gen!();
    let (communication, _communication_raw) = gen!();
    let (resilience, _resilience_raw) = gen!();
    let (adaptability, _adaptability_raw) = gen!();

    let config = StatsConfig {
        judgement,
        creativity,
        systems,
        precision,
        focus,
        discipline,
        empathy,
        communication,
        resilience,
        adaptability,
    };
    let mut stats = config.into();
    let mut choices = (1..=4).collect::<Vec<_>>();

    match choices.choose(&mut rng()) {
        None => {}
        Some(&1) => {
            sculpt_monofocus(&mut stats);
        }
        Some(&2) => {
            sculpt_axis_bias(&mut stats);
        }
        Some(&3) => {
            sculpt_blindspot(&mut stats);
        }
        Some(&4) => {
            sculpt_contrasting_pair(&mut stats);
        }
        _ => {}
    }

    sculpt_blindspot(&mut stats);
    stats
}

fn generate_profile_picture(
    gender: Gender,
    used_picture_registry: &UsedProfilePictureRegistry,
) -> ProfilePicture {
    let used_picture_set = &used_picture_registry.used_profile_pictures;
    // HARDCODED DEBUG VALUES. This block should not be change. TODO Externalize these values.
    let mut last_portrait = HashMap::new();
    last_portrait.insert((Gender::Female, ProfilePictureCategory::Office), 10);
    last_portrait.insert((Gender::Female, ProfilePictureCategory::Social), 10);
    last_portrait.insert((Gender::Male, ProfilePictureCategory::Office), 8);
    last_portrait.insert((Gender::Male, ProfilePictureCategory::Social), 9);

    const MAX_ATTEMPTS: usize = 10;

    let mut rng = rand::rng();
    let mut profile_picture = ProfilePicture::default();
    for _ in 0..MAX_ATTEMPTS {
        let random_category = if rng.random_range(0..100) < 70 {
            ProfilePictureCategory::Office
        } else {
            ProfilePictureCategory::Social
        };

        let last_batch_key = (gender, random_category);
        let last_batch_number = *last_portrait.get(&last_batch_key).unwrap();
        let batch_id = rng.random_range(1..=last_batch_number);
        let random_index = rng.random_range(0..=8);

        profile_picture = ProfilePicture {
            gender: gender,
            category: random_category,
            batch: batch_id,
            index: random_index,
        };
        if used_picture_set.insert(profile_picture) {
            return profile_picture;
        }
    }
    profile_picture
}

fn generate_personality_matrix() -> PersonalityMatrix {
    let matrix = PersonalityMatrix {
        assertiveness: bounded_normal(0f64, 40f64, -100, 100) as i8,
        structure_preference: bounded_normal(0f64, 40f64, -100, 100) as i8,
        openness: bounded_normal(0f64, 40f64, -100, 100) as i8,
        sociability: bounded_normal(0f64, 40f64, -100, 100) as i8,
        influence: bounded_normal(0f64, 40f64, -100, 100) as i8,
    };
    // info!("Generated someone that is {},{}", matrix.describe_axis(PersonalityAxis::Assertiveness), matrix.assertiveness);
    // info!("Generated someone that is {},{}", matrix.describe_axis(PersonalityAxis::StructurePreference), matrix.structure_preference);
    // info!("Generated someone that is {},{}", matrix.describe_axis(PersonalityAxis::Openness) , matrix.openness);
    // info!("Generated someone that is {},{}", matrix.describe_axis(PersonalityAxis::Sociability), matrix.sociability );
    // info!("Generated someone that is {},{}", matrix.describe_axis(PersonalityAxis::Influence), matrix.influence );

    return matrix;
}

#[tracing::instrument(level = "trace", skip(cmd, asset_path))]
pub fn spawn_person(
    cmd: &mut CommandBuffer,
    tier: TalentGrade,
    asset_path: &AssetBasePath,
    used_portraits: &UsedProfilePictureRegistry,
    person_registry: &Arc<Registry<PersonId, Entity>>,
    global_skills: &Vec<&GlobalSkill>,
    current_tick: u64,
) -> (
    PersonId,
    Entity,
    Person,
    Stats,
    ProfilePicture,
    PersonalityMatrix,
    SkillSet,
) {
    debug!("Spawning person");
    let id = PersonId(person_registry.generate_id());

    // let peron_id = person_registry.generate_id();
    let gender = random_gender();
    let person = Person {
        gender: gender,
        name: generate_full_name(&gender, &asset_path).expect("Cannot generate full name"),
        person_id: id.clone(),
        team: None,
        talent_grade: tier,
        joined: current_tick,
    };
    let person_clone = person.clone();
    trace!("Created person {}", person.name);
    trace!("{:?}", person);
    let profile_picture = generate_profile_picture(gender, used_portraits);
    let stats = generate_stats(tier);
    let personality_matrix = generate_personality_matrix();
    let skillset = assign_skills(&stats, &global_skills);
    let entity = cmd.push((
        person,
        stats,
        profile_picture,
        personality_matrix,
        skillset.clone(),
        Energy::default(),
        Hunger::default(),
        Dirty,
        DebugDisplayComponent::default(),
        CurrentGoal::default(),
        StressLevel::default(),
    ));
    person_registry.insert(id, entity);

    (
        id,
        entity,
        person_clone,
        stats,
        profile_picture,
        personality_matrix,
        skillset.clone(),
    )
}

fn assign_skills(stats: &Stats, all_skills: &Vec<&GlobalSkill>) -> SkillSet {
    let mut skills = HashMap::new();

    for skill in all_skills {
        let tier = determine_skill_tier(stats, &skill.related_stats);
        if let Some((mean, stddev, min)) = skill_value_by_tier(tier) {
            let value = bounded_normal(mean as f64, stddev as f64, min as i16, 100) as u32;
            skills.insert(skill.id.clone(), value);
        }
    }

    SkillSet { skills }
}

fn determine_skill_tier(stats: &Stats, related_stats: &[StatType]) -> u8 {
    if related_stats.iter().all(|s| stats.get_stat(*s) >= 80) {
        5
    } else if related_stats.iter().all(|s| stats.get_stat(*s) >= 70) {
        4
    } else if related_stats.iter().all(|s| stats.get_stat(*s) >= 60) {
        3
    } else if related_stats.iter().any(|s| stats.get_stat(*s) >= 50) {
        2
    } else {
        1
    }
}

fn skill_value_by_tier(tier: u8) -> Option<(u32, u32, u32)> {
    match tier {
        1 => Some((10, 5, 0)),
        2 => Some((30, 5, 0)),
        3 => Some((50, 10, 40)),
        4 => Some((60, 10, 45)),
        5 => Some((75, 15, 65)),
        _ => None,
    }
}
