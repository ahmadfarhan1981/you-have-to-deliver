use crate::sim::person::stats::{StatGroup, StatType, Stats};
use crate::sim::utils::random::random_variation;
use rand::prelude::*;
use rand::{rng, Rng};
use std::cmp::min;
use strum::IntoEnumIterator;
use tracing::{debug, info};

/// **Category:** 1 – Spike
/// **Purpose:** One stat dominates, others subtly sacrificed
/// **Logic:**
///
/// - Pick 1 stat randomly → +X (e.g. +10)
/// - Randomly reduce other stats until budget is balanced
pub fn sculpt_monofocus(stats: &mut Stats) {
    /// Settings
    const AMOUNT:i32 = 20;


    let total_points = stats.total(); // track before mutation

    let high_stat = random_stat();
    stats.adjust(high_stat, AMOUNT as f32);

    let mut reduction = AMOUNT;
    let mut others = StatType::iter().filter(|s| *s != high_stat).collect::<Vec<_>>();
    others.shuffle(&mut rng());

    for stat in others {
        if reduction <= 0 { break; }
        let delta = random_variation(min(reduction, 3), 1 );
        if stats.get_stat(stat) >= (delta + 10) as u16 { // avoid floor
            stats.adjust(stat, -delta as f32);
            reduction -= delta;
        }
    }

    stats.normalize_to(total_points); // rebalance if needed
}

fn random_stat() -> StatType {
    StatType::iter().choose(&mut rng()).unwrap()
    // return rng().random_range(StatType)
}


/// **Category:** 2 – Axis Tilt
/// **Purpose:** Slight push within one stat _group_ (e.g. Cognition), no forced pairs
/// **Logic:**
///
/// - Pick 1 group randomly (Cognition, Perception, etc.)
/// - Boost each stat in the group by +3–5
/// - Reduce others proportionally, skipping floors
pub fn sculpt_axis_bias(stats: &mut Stats) {
    let total_points = stats.total();

    let base_modifier = 5;
    let variable = 3;

    let boost_amount = random_variation(base_modifier, variable);
    let group = random_stat_group(); // returns Vec<StatType>

    for stat in group.members() {
        stats.adjust(stat, boost_amount as f32);
    }

    let mut reduction = boost_amount * group.members().len() as i32;
    let mut others = StatType::iter().filter(|s| !group.members().contains(s)).collect::<Vec<_>>();
    others.shuffle(&mut rng());

    for stat in others {
        if reduction <= 0 { break; }
        let delta = min(reduction, 10);
        if stats.get_stat(stat) >= (delta + 10) as u16 {
            debug!("Lowered {:?} by {}", stat, delta);
            stats.adjust(stat, -delta as f32);
            reduction -= delta;
        }
    }

    stats.normalize_to(total_points);
}

pub fn random_stat_group() -> StatGroup {
    StatGroup::iter().choose(&mut rng()).unwrap()
}

/// **Category:** 6 – Negative Space
/// **Purpose:** Set one stat explicitly low
/// **Logic:**
///
/// - Choose 1 stat randomly → set to low (e.g. 15)
/// - Redistribute excess across others
pub fn sculpt_blindspot(stats: &mut Stats) {
    let total_points = stats.total();

    let random_high = stats.stat_filter(55, |v, t| v >= t);
    let mut low_stat = random_stat();
    if !random_high.is_empty() {
        low_stat = random_high.iter().choose(&mut rng()).unwrap().clone();
    }

    let current = stats.get_stat(low_stat);
    if current > 15 {
        let diff = current - 15;
        stats.set_stat(low_stat, 15);

        let mut others = StatType::iter().filter(|s| *s != low_stat).collect::<Vec<_>>();
        others.shuffle(&mut rng());
        for stat in others {
            if diff <= 0 { break; }
            let boost = min(3, diff);

            stats.adjust(stat, boost.into());
        }
    }

    stats.normalize_to(total_points);
}
///
/// **Category:** 3 – Asymmetry
/// **Purpose:** One stat high, one stat low, rest untouched
/// **Logic:**
///
/// - Pick two unrelated stats randomly (not same group)
/// - Push one up, one down
pub fn sculpt_contrasting_pair(stats: &mut Stats) {
    let total_points = stats.total();

    let mut a = random_stat();
    let mut b = random_stat();
    while same_group(a, b) || a == b {
        b = random_stat();
    }

    stats.adjust(a, 8f32);
    stats.adjust(b, -8f32);
    stats.normalize_to(total_points);
}

pub fn same_group(stat1: StatType, stat2: StatType) -> bool {
    stat1.get_group() == stat2.get_group()
}
