use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};
use serde::{Deserialize, Serialize};
use crate::sim::person::components::Gender;
use crate::sim::resources::global::AssetBasePath;

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


fn get_random_line_from_file(asset_path: AssetBasePath) -> Option<String> {
    println!("Asset path: {:?}", asset_path);

    let name_dictionary_path = asset_path.0.join("dictionaries");//.join(filename);
        //.ok_or("Failed to resolve file path")?;

    let file = File::open(&name_dictionary_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    
    let total_lines = reader.lines().count();
    println!("Total lines: {}", total_lines);

     
    
    if total_lines == 0 {
        println!("File is empty!");
        return None;
    }

    let target = rand::thread_rng().gen_range(0..total_lines);
    println!("Chosen line index: {}", target);
    let result = reader.lines().nth(target).and_then(Result::ok);
    // This second open may also fail silently
    let file2 = File::open(name_dictionary_path).expect("Failed to reopen file");
    let reader2 = BufReader::new(file2);
    let result = reader2.lines().nth(target).and_then(Result::ok);

    if result.is_none() {
        println!("Failed to read the chosen line");
    }

    result
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