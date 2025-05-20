use super::components::Person;
use super::stats::{Stats, StatsConfig};
use crate::sim::person::components::{Gender, PersonId, ProfilePicture, ProfilePictureCategory};
use crate::sim::person::registry::PersonRegistry;
use crate::sim::resources::global::AssetBasePath;
use dashmap::DashSet;
use legion::systems::CommandBuffer;
use legion::Entity;
use rand::seq::IteratorRandom;
use rand::{rng, Rng};
use rand_distr::{Distribution, Normal};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, error, trace};
use crate::sim::systems::global::UsedProfilePictureRegistry;

pub fn bounded_normal(mean: f64, std_dev: f64, min: u16, max: u16) -> u16 {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rng();

    for _ in 0..10 {
        let sample = normal.sample(&mut rng).round();
        if sample >= min.into() && sample <= max.into() {
            return sample as u16;
        }
    }

    // Escape hatch: clamp after 10 failed tries
    let fallback = normal.sample(&mut rng).round();
    return fallback.clamp(min.into(), max.into()) as u16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TalentGrade {
    Basic,
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
            let val = bounded_normal(mean, std_dev, 0, 100);
            (val, val as u32 * 1000)
        }};
    }

    let (judgement, judgement_raw) = gen!();
    let (creativity, creativity_raw) = gen!();
    let (systems, systems_raw) = gen!();
    let (precision, precision_raw) = gen!();
    let (focus, focus_raw) = gen!();
    let (discipline, discipline_raw) = gen!();
    let (empathy, empathy_raw) = gen!();
    let (communication, communication_raw) = gen!();
    let (resilience, resilience_raw) = gen!();
    let (adaptability, adaptability_raw) = gen!();

    let config =StatsConfig {
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
    config.into()
}

fn generate_profile_picture(
    gender: Gender,
    used_picture_registry: &UsedProfilePictureRegistry,
) -> ProfilePicture {
    let used_picture_set = &used_picture_registry.used_profile_pictures;
    // HARDCODED DEBUG VALUES. This block should not be change. TODO Externalize these values.
    let mut last_portrait = HashMap::new();
    last_portrait.insert((Gender::Female, ProfilePictureCategory::Office), 4);
    last_portrait.insert((Gender::Female, ProfilePictureCategory::Social), 1);
    last_portrait.insert((Gender::Male, ProfilePictureCategory::Office), 3);
    last_portrait.insert((Gender::Male, ProfilePictureCategory::Social), 1);

    const MAX_ATTEMPTS:usize = 10;

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

#[tracing::instrument(level = "debug", skip(cmd))]
pub fn spawn_person(
    cmd: &mut CommandBuffer,
    tier: TalentGrade,
    asset_path: &AssetBasePath,
    used_portraits: &UsedProfilePictureRegistry,
    person_registry: &Arc<PersonRegistry>,
) -> (PersonId, Entity) {
    debug!("Spawning person");
    let id = person_registry.generate_id();
    // let peron_id = person_registry.generate_id();
    let gender = random_gender();
    let person = Person {
        gender: gender,
        name: generate_full_name(&gender, &asset_path).expect("Cannot generate full name"),
        person_id: id.clone(),
    };

    let profile_picture = generate_profile_picture(gender, used_portraits);
    let stats = generate_stats(tier);
    let entity = cmd.push((person, stats, profile_picture));
    person_registry.insert(id, entity);
    (id, entity)
}
