use crate::sim::person::components::{Person, ProfilePicture};
use crate::sim::person::personality_matrix::{PersonalityAxis, PersonalityMatrix};
use crate::sim::person::skills::{GlobalSkill, SkillId};
use crate::sim::person::stats::{StatType, Stats};
use crate::sim::resources::global::{SimDate, TickCounter};
use arc_swap::ArcSwap;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::hash::Hash;
use std::sync::atomic::{AtomicU16, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;

/// this is tha main integration state
#[derive(Debug, Default)]
pub struct SnapshotState {
    pub tick: TickSnapshot,
    pub game_speed: SnapshotField<GameSpeedSnapshot>,
    pub persons: Arc<DashMap<u32, PersonSnapshot>>,
}
#[derive(Debug, Default)]
pub struct SnapshotField<T> {
    pub value: ArcSwap<Arc<T>>,
}
impl<T> From<T> for SnapshotField<T> {
    fn from(value: T) -> Self {
        Self {
            value: ArcSwap::from_pointee(value.into()),
        }
    }
}

impl<T> Clone for SnapshotField<T> {
    fn clone(&self) -> Self {
        Self {
            value: ArcSwap::from(self.value.load_full()),
        }
    }
}

pub struct SnapshotCollection<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    pub map: DashMap<K, V>,
}
impl<K, V> From<DashMap<K, V>> for SnapshotCollection<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    fn from(map: DashMap<K, V>) -> Self {
        Self { map }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GameSpeedSnapshot {
    pub tick: TickSnapshot,
    pub game_speed: AtomicU8,
}
impl Clone for GameSpeedSnapshot {
    fn clone(&self) -> Self {
        Self {
            tick: self.tick.clone(),
            game_speed: AtomicU8::new(self.game_speed.load(Ordering::Relaxed)),
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TickSnapshot {
    tick: AtomicU64,
    year: AtomicU16,
    week: AtomicU8,
    day: AtomicU8,
    quarter_tick: AtomicU8,
}
impl Clone for TickSnapshot {
    fn clone(&self) -> Self {
        Self {
            tick: AtomicU64::new(self.tick.load(Ordering::Relaxed)),
            year: AtomicU16::new(self.year.load(Ordering::Relaxed)),
            week: AtomicU8::new(self.week.load(Ordering::Relaxed)),
            day: AtomicU8::new(self.day.load(Ordering::Relaxed)),
            quarter_tick: AtomicU8::new(self.quarter_tick.load(Ordering::Relaxed)),
        }
    }
}
impl From<TickCounter> for TickSnapshot {
    fn from(value: TickCounter) -> Self {
        let date = value.current_date();
        Self {
            tick: value.value().into(),
            year: date.year.into(),
            week: date.week.into(),
            day: date.day.into(),
            quarter_tick: date.quarter_tick.into(),
        }
    }
}

impl TickSnapshot {
    /// Updates the snapshot to match the given value
    pub fn set(&self, val: &TickCounter) {
        self.tick.store(val.value(), Ordering::Relaxed);

        let SimDate {
            year,
            week,
            day,
            quarter_tick,
        } = val.current_date();
        if self.year.load(Ordering::Relaxed) != year {
            self.year.store(year, Ordering::Relaxed);
        }
        if self.week.load(Ordering::Relaxed) != week {
            self.week.store(week, Ordering::Relaxed);
        }
        if self.day.load(Ordering::Relaxed) != day {
            self.day.store(day, Ordering::Relaxed);
        }
        if self.quarter_tick.load(Ordering::Relaxed) != quarter_tick {
            self.quarter_tick.store(quarter_tick, Ordering::Relaxed);
        }
    }

    /// Reads the current snapshot value
    pub fn get(&self) -> u64 {
        self.tick.load(Ordering::Relaxed)
    }
    pub fn get_date(&self) -> (u16, u8, u8, u8, u64) {
        (
            self.year.load(Ordering::Relaxed),
            self.week.load(Ordering::Relaxed),
            self.day.load(Ordering::Relaxed),
            self.quarter_tick.load(Ordering::Relaxed),
            self.tick.load(Ordering::Relaxed),
        )
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonSnapshot {
    pub(crate) stats: StatsSnapshot,
    pub(crate) profile_picture: ProfilePictureSnapshot,
    pub(crate) person_id: u32,
    pub(crate) name: String,
    pub(crate) gender: String,
    pub(crate) personality: PersonalitySnapshot,
    /// The tick number this snapshot was last updated
    pub updated: u64,
}
impl From<(Person, ProfilePicture, Stats, PersonalityMatrix, u64)> for PersonSnapshot {
    fn from(
        (person, picture, stats, personality, current_tick): (
            Person,
            ProfilePicture,
            Stats,
            PersonalityMatrix,
            u64,
        ),
    ) -> Self {
        Self {
            stats: StatsSnapshot::from(stats),
            profile_picture: ProfilePictureSnapshot::from(picture),
            person_id: person.person_id.0,
            name: person.name,
            gender: person.gender.to_string(),
            personality: PersonalitySnapshot::from(personality),
            updated: current_tick,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfilePictureSnapshot {
    pub gender: String,
    pub category: i8,
    pub batch: i8,
    pub index: i8,
}
impl PartialEq<ProfilePicture> for ProfilePictureSnapshot {
    fn eq(&self, other: &ProfilePicture) -> bool {
        self.gender == other.gender.to_string()
            && self.category == other.category.as_file_category_number()
            && self.batch == other.batch
            && self.index == other.index
    }
}
impl From<ProfilePicture> for ProfilePictureSnapshot {
    fn from(picture: ProfilePicture) -> Self {
        Self {
            gender: picture.gender.to_string(),
            category: picture.category.as_file_category_number(),
            batch: picture.batch,
            index: picture.index,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatsSnapshot {
    // Cognition
    pub judgement: u16,
    pub creativity: u16,

    // Perception
    pub systems: u16,
    pub precision: u16,

    // Drive
    pub focus: u16,
    pub discipline: u16,

    // Social
    pub empathy: u16,
    pub communication: u16,

    // Defense
    pub resilience: u16,
    pub adaptability: u16,
}

impl From<Stats> for StatsSnapshot {
    fn from(s: Stats) -> Self {
        Self {
            judgement: s.get_stat(StatType::Judgement),
            creativity: s.get_stat(StatType::Creativity),
            systems: s.get_stat(StatType::Systems),
            precision: s.get_stat(StatType::Precision),
            focus: s.get_stat(StatType::Focus),
            discipline: s.get_stat(StatType::Discipline),
            empathy: s.get_stat(StatType::Empathy),
            communication: s.get_stat(StatType::Communication),
            resilience: s.get_stat(StatType::Resilience),
            adaptability: s.get_stat(StatType::Adaptability),
        }
    }
}

impl PartialEq<Stats> for StatsSnapshot {
    fn eq(&self, other: &Stats) -> bool {
        self.judgement == other.get_stat(StatType::Judgement)
            && self.creativity == other.get_stat(StatType::Creativity)
            && self.systems == other.get_stat(StatType::Systems)
            && self.precision == other.get_stat(StatType::Precision)
            && self.focus == other.get_stat(StatType::Focus)
            && self.discipline == other.get_stat(StatType::Discipline)
            && self.empathy == other.get_stat(StatType::Empathy)
            && self.communication == other.get_stat(StatType::Communication)
            && self.resilience == other.get_stat(StatType::Resilience)
            && self.adaptability == other.get_stat(StatType::Adaptability)
    }
}

pub struct GlobalSkillSnapshot {
    pub id: u32,
    pub name: String,
    pub description: String,
}
impl From<&GlobalSkill> for GlobalSkillSnapshot {
    fn from(value: &GlobalSkill) -> Self {
        Self {
            id: value.id.0,
            name: value.name.clone(),
            description: value.description.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PersonalitySnapshot {
    pub assertiveness: i8,
    pub structure_preference: i8,
    pub openness: i8,
    pub sociability: i8,
    pub influence: i8,
    pub assertiveness_description: String,
    pub structure_preference_description: String,
    pub openness_description: String,
    pub sociability_description: String,
    pub influence_description: String,
}

impl From<PersonalityMatrix> for PersonalitySnapshot {
    fn from(matrix: PersonalityMatrix) -> Self {
        PersonalitySnapshot {
            assertiveness: matrix.assertiveness,
            structure_preference: matrix.structure_preference,
            openness: matrix.openness,
            sociability: matrix.sociability,
            influence: matrix.influence,
            assertiveness_description: matrix.describe_axis(PersonalityAxis::Assertiveness),
            structure_preference_description: matrix
                .describe_axis(PersonalityAxis::StructurePreference),
            openness_description: matrix.describe_axis(PersonalityAxis::Openness),
            sociability_description: matrix.describe_axis(PersonalityAxis::Sociability),
            influence_description: matrix.describe_axis(PersonalityAxis::Influence),
        }
    }
}

impl PartialEq<PersonalityMatrix> for PersonalitySnapshot {
    fn eq(&self, other: &PersonalityMatrix) -> bool {
        self.assertiveness == other.assertiveness
            && self.structure_preference == other.structure_preference
            && self.openness == other.openness
            && self.sociability == other.sociability
            && self.influence == other.influence
    }
}
