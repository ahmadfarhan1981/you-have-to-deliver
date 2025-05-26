// src/lib/stores/persons.ts
import {writable, derived, type Readable} from "svelte/store";
import {invoke} from "@tauri-apps/api/core";

export type StatsSnapshot = {
    judgement: number;
    creativity: number;
    systems: number;
    precision: number;
    focus: number;
    discipline: number;
    empathy: number;
    communication: number;
    resilience: number;
    adaptability: number;
};

export type ProfilePictureSnapshot = {
    gender: string;
    category: number; // i8 in Rust maps to number in TS
    batch: number;
    index: number;
};

export type PersonSnapshot = {
    stats: StatsSnapshot;
    profile_picture: ProfilePictureSnapshot;
    personality: PersonSnapshot;
    person_id: number;
    name: string;
    gender: string;
};

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

export type PersonSnapshotWithTotal = PersonSnapshot & {
    total_points : number
};

// Exposed array for easy iteration
export const basePersonArray = writable<PersonSnapshot []>([]);
//export const personArray = writable<PersonSnapshotWithTotal []>([]);

export const personArray:Readable<PersonSnapshotWithTotal []> = derived(basePersonArray, ($people) =>
    $people.map((person) => ({
        ...person,
        total_points: Object.values(person.stats).reduce((sum, val) => sum + val, 0)
    }))
);

// Exposed map for fast lookup by ID
export const persons = derived(personArray, ($array) => {
    const map = new Map<number, PersonSnapshot & {total_points:number}>();
    for (const p of $array) {
        map.set(p.person_id, {...p,
            total_points: Object.values(p.stats).reduce((sum, val) => sum + val, 0)});
    }
    return map;
});
export const personsSnapshotEventName = "persons_snapshot";

export function getProfileImageData(pic: ProfilePictureSnapshot) {
    const gender = pic.gender.slice(0, 1) === "f" ? "f" : "m";
    const pad = (num: number) => String(Math.min(99, Math.max(0, Math.floor(num)))).padStart(2, "0");
    const category = pad(pic.category);
    const batch = pad(pic.batch);
    const index = Math.max(0, Math.min(8, pic.index));
    const fileName = `${gender}${category}${batch}.png`;
    const col = index % 3;
    const row = Math.floor(index / 3);
    const position = `${col * 50}% ${row * 50}%`;
    return {fileName, position};
}

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
