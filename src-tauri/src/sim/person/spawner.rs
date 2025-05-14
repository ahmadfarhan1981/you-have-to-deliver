use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};
use serde::{Deserialize, Serialize};
use crate::sim::person::components::Gender;
use crate::sim::resources::global::AssetBasePath;
use rand::seq::IteratorRandom;


pub fn bounded_normal(mean: f64, std_dev: f64, min: f64, max: f64) -> u32 {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = thread_rng();

    for _ in 0..10 {
        let sample = normal.sample(&mut rng).round();
        if sample >= min && sample <= max {
            return sample as u32;
        }
    }

    // Escape hatch: clamp after 10 failed tries
    let fallback = normal.sample(&mut rng).round();
    fallback.clamp(min, max) as u32
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



#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct PersonId(pub u32);
pub fn random_gender() -> Gender {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Gender::Male
    } else {
        Gender::Female
    }
}


pub fn spawn_person() -> () {
    println!("Spawning person");
    
}