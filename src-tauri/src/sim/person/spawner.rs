use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use crate::sim::components::stats::Stats;
use crate::sim::person;
use crate::sim::person::components::{Gender, PersonId};
use crate::sim::resources::global::AssetBasePath;
use rand::seq::IteratorRandom;

use super::components::Person;


pub fn bounded_normal(mean: f64, std_dev: f64, min: u16, max: u16) -> u16 {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = thread_rng();

    for _ in 0..10 {
        let sample = normal.sample(&mut rng).round();
        if sample >= min.into() && sample <= max.into() {
            return sample as u16;
        }
    }

    // Escape hatch: clamp after 10 failed tries
    let fallback = normal.sample(&mut rng).round();
    return (fallback.clamp(min.into(), max.into()) as u16 );
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
            TalentGrade::Basic       => (45.0, 20.0), // Lots of weak stats, occasional spikes
            TalentGrade::Apt         => (55.0, 15.0),
            TalentGrade::Sharp       => (65.0, 12.0),
            TalentGrade::Gifted      => (75.0, 9.0),
            TalentGrade::Brilliant   => (85.0, 6.0),
            TalentGrade::Exceptional => (93.0, 4.0),  // Near-perfect, but not boringly maxed
        }
    }
}


pub fn get_random_line_from_file(asset_path: AssetBasePath) -> Option<String> {
    // Build full path to names/first.txt
    let base_path = Path::new(&asset_path.0);
    let file_path = base_path.join("names").join("first.txt");
    println!("Resolved file path: {:?}", file_path);

    // Attempt to open the file
    let file = File::open(&file_path).ok()?;
    let reader = BufReader::new(file);

    // Choose a random line using iterator sampling
    let mut rng = thread_rng();
    let line = reader.lines().filter_map(Result::ok).choose(&mut rng);

    if line.is_none() {
        println!("File is empty or unreadable!");
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
pub fn generate_name_part(gender: &Gender, part: &NameParts, asset_path: &AssetBasePath) -> Option<String> {
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
    println!("Resolved file path: {:?}", file_path);

    let file = File::open(&file_path).map_err(|e| {
        eprintln!("Failed to open file {:?}: {}", file_path, e);
    }).ok()?;

    let reader = BufReader::new(file);
    let mut rng = thread_rng();

    let line = reader.lines().filter_map(Result::ok).choose(&mut rng);

    if line.is_none() {
        eprintln!("No lines found in file: {:?}", file_path);
    }

    line
}


pub fn random_gender() -> Gender {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Gender::Male
    } else {
        Gender::Female
    }
}


fn generate_stats(tier: TalentGrade) -> Stats {
    let (mean, std_dev) = tier.stat_distribution();
    let mut rng = thread_rng();

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

    Stats {
        judgement,
        judgement_raw,
        creativity,
        creativity_raw,
        systems,
        systems_raw,
        precision,
        precision_raw,
        focus,
        focus_raw,
        discipline,
        discipline_raw,
        empathy,
        empathy_raw,
        communication,
        communication_raw,
        resilience,
        resilience_raw,
        adaptability,
        adaptability_raw,
    }
}


pub fn spawn_person(tier:TalentGrade, asset_path: &AssetBasePath) -> (Person, Stats) {
    println!("Spawning person");
    let gender = random_gender();
     let person = Person {
         gender: gender,
         name: generate_full_name(&gender, &asset_path).expect("Cannot generate full name"),
        person_id: PersonId(1),
     
    };
    let stats = generate_stats(tier);
    return (person, stats);
    
}