export const personalityAxisSpectrum: Record<PersonalityAxis, { low: string; high: string }> = {
    Assertiveness: {
        low: 'Accommodating',
        high: 'Dominant'
    },
    StructurePreference: {
        low: 'Spontaneous',
        high: 'Methodical'
    },
    Openness: {
        low: 'Practical',
        high: 'Idealistic'
    },
    Sociability: {
        low: 'Reserved',
        high: 'Outgoing'
    },
    Influence: {
        low: 'Suggestive',
        high: 'Persuasive'
    }
};
export type PersonalitySnapshot = {
    assertiveness: number;
    structure_preference: number;
    openness: number;
    sociability: number;
    influence: number;

    assertiveness_description: string;
    structure_preference_description: string;
    openness_description: string;
    sociability_description: string;
    influence_description: string;
};
export type PersonalityAxis =
    | 'Assertiveness'
    | 'StructurePreference'
    | 'Openness'
    | 'Sociability'
    | 'Influence';
export const allPersonalityAxes: PersonalityAxis[] = [
    'Assertiveness',
    'StructurePreference',
    'Openness',
    'Sociability',
    'Influence'
];
export type PersonalityAxisValue = {
    value: number;
    desc: string;
};
export const personalityAxisIcons: Record<PersonalityAxis, string> = {
    Assertiveness: '🗣️',
    StructurePreference: '📅',
    Openness: '💡',
    Sociability: '👥',
    Influence: '🎯'
};
export const personalityAxisLabels: Record<PersonalityAxis, string> = {
    Assertiveness: 'Assertiveness',
    StructurePreference: 'Structure Preference',
    Openness: 'Openness',
    Sociability: 'Sociability',
    Influence: 'Influence'
};

export function getAxisValue(snapshot: PersonalitySnapshot, axis: PersonalityAxis): PersonalityAxisValue {
    switch (axis) {
        case 'Assertiveness':
            return {
                value: snapshot.assertiveness,
                desc: snapshot.assertiveness_description
            };
        case 'StructurePreference':
            return {
                value: snapshot.structure_preference,
                desc: snapshot.structure_preference_description
            };
        case 'Openness':
            return {
                value: snapshot.openness,
                desc: snapshot.openness_description
            };
        case 'Sociability':
            return {
                value: snapshot.sociability,
                desc: snapshot.sociability_description
            };
        case 'Influence':
            return {
                value: snapshot.influence,
                desc: snapshot.influence_description
            };
    }
}