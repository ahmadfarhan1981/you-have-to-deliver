use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};
use serde::{Deserialize, Serialize};

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


fn get_random_line_from_file(path: &PathBuf) -> Option<String> {
    println!("Opening file: {:?}", path);

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(&file);
    let total_lines = reader.lines().count();
    println!("Total lines: {}", total_lines);

    if total_lines == 0 {
        println!("File is empty!");
        return None;
    }

    let target = rand::thread_rng().gen_range(0..total_lines);
    println!("Chosen line index: {}", target);

    // This second open may also fail silently
    let file2 = File::open(path).expect("Failed to reopen file");
    let reader2 = BufReader::new(file2);
    let result = reader2.lines().nth(target).and_then(Result::ok);

    if result.is_none() {
        println!("Failed to read the chosen line");
    }

    result
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
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