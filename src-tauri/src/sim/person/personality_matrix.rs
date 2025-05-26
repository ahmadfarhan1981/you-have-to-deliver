use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PersonalityAxis {
    Assertiveness,
    StructurePreference,
    Openness,
    Sociability,
    Influence,
}

#[derive(Debug, Clone, Default, Copy)]
pub struct PersonalityMatrix {
    pub assertiveness: i8,
    pub structure_preference: i8,
    pub openness: i8,
    pub sociability: i8,
    pub influence: i8,
}

impl PersonalityMatrix {
    // Thresholds for each personality descriptor tier
    const EXTREME_THRESHOLD: i8 = 90;
    const VERY_THRESHOLD: i8 = 75;
    const FAIRLY_THRESHOLD: i8 = 35;

    pub fn describe_axis(&self, axis: PersonalityAxis) -> String {
        let value = match axis {
            PersonalityAxis::Assertiveness => self.assertiveness,
            PersonalityAxis::StructurePreference => self.structure_preference,
            PersonalityAxis::Openness => self.openness,
            PersonalityAxis::Sociability => self.sociability,
            PersonalityAxis::Influence => self.influence,
        };

        Self::axis_description(axis, value)
    }

    pub fn describe_all(&self) -> HashMap<PersonalityAxis, String> {
        use PersonalityAxis::*;
        [
            Assertiveness,
            StructurePreference,
            Openness,
            Sociability,
            Influence,
        ]
        .iter()
        .map(|&axis| (axis, self.describe_axis(axis)))
        .collect()
    }

    pub fn overall_summary(&self) -> String {
        let bold_traits: Vec<_> = self
            .describe_all()
            .into_iter()
            .filter(|(_, desc)| desc.contains("Extremely") || desc.contains("Very"))
            .collect();

        match bold_traits.len() {
            0 => "Fairly balanced personality with no dominant traits.".to_string(),
            1..=2 => format!(
                "Notable traits: {}.",
                bold_traits
                    .iter()
                    .map(|(_, d)| d.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => format!(
                "Strong personality mix: {}.",
                bold_traits
                    .iter()
                    .map(|(_, d)| d.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn axis_description(axis: PersonalityAxis, value: i8) -> String {
        use PersonalityAxis::*;

        let tier = match value {
            v if v >= Self::EXTREME_THRESHOLD => 3,
            v if v >= Self::VERY_THRESHOLD => 2,
            v if v >= Self::FAIRLY_THRESHOLD => 1,
            v if v > -Self::FAIRLY_THRESHOLD => 0,
            v if v > -Self::VERY_THRESHOLD => -1,
            v if v > -Self::EXTREME_THRESHOLD => -2,
            _ => -3,
        };

        match axis {
            Assertiveness => match tier {
                -3 => "Avoids conflict at all cost; often overlooked.".into(),
                -2 => "Rarely speaks up, prefers others to take the lead.".into(),
                -1 => "Cooperative and gentle in group dynamics.".into(),
                0 => "Generally balanced — assertive when needed.".into(),
                1 => "Comfortable leading discussions and decisions.".into(),
                2 => "Takes charge easily; may overshadow quieter voices.".into(),
                3 => "Steamrolls over others — a dominant force in any room.".into(),
                _ => unreachable!(),
            },
            StructurePreference => match tier {
                -3 => "Rejects routine entirely; thrives in complete chaos.".into(),
                -2 => "Spontaneous to a fault; struggles with planning.".into(),
                -1 => "Flexible and adaptable, enjoys variety.".into(),
                0 => "Comfortable with both plans and spontaneity.".into(),
                1 => "Appreciates structure and reliable patterns.".into(),
                2 => "Thinks in outlines and checklists; dislikes surprises.".into(),
                3 => "Obsessively structured — everything must go to plan.".into(),
                _ => unreachable!(),
            },
            Openness => match tier {
                -3 => "Grounded to the point of inflexibility — distrusts abstraction.".into(),
                -2 => "Prefers concrete facts over abstract ideas.".into(),
                -1 => "Pragmatic thinker who values results over ideals.".into(),
                0 => "Balances practicality with open-mindedness.".into(),
                1 => "Imaginative and curious about new possibilities.".into(),
                2 => "Frequently explores novel ideas; ideal-driven.".into(),
                3 => "Lives in the world of ideas — a dreamer untethered from practicality.".into(),
                _ => unreachable!(),
            },
            Sociability => match tier {
                -3 => "Deeply private and withdrawn — avoids social contact.".into(),
                -2 => "Quiet and self-contained; rarely initiates interaction.".into(),
                -1 => "Prefers solitude but will engage when needed.".into(),
                0 => "Comfortable in both solitude and company.".into(),
                1 => "Friendly and quick to connect with others.".into(),
                2 => "Energized by interaction; often the center of attention.".into(),
                3 => "Constantly seeks connection — thrives on being surrounded.".into(),
                _ => unreachable!(),
            },
            Influence => match tier {
                -3 => "Easily swayed; lacks presence in discussions.".into(),
                -2 => "Follows more than leads; hesitant to steer opinion.".into(),
                -1 => "Supportive voice who avoids dominating.".into(),
                0 => "Has ideas but doesn’t push them; open to dialogue.".into(),
                1 => "Often convinces others through clear reasoning.".into(),
                2 => "Naturally persuasive — steers conversations with ease.".into(),
                3 => "Commanding presence — bends rooms to their will.".into(),
                _ => unreachable!(),
            },
        }
    }
}
