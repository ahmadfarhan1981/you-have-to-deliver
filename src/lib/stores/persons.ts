// src/lib/stores/persons.ts
import {derived, type Readable, writable} from "svelte/store";


import type {StatsSnapshot} from "$lib/models/stats";
import type {PersonalitySnapshot} from "$lib/models/personality";
import type {ProfilePictureSnapshot, SkillSetSnapshot} from "$lib/models/skill";
import {invoke} from "@tauri-apps/api/core";
import type {SimDate} from "$lib/stores/simDate";

export type AssignedSkillSnapshot = {
    skill_id: string;
    value: number;
    skill_name: string;
};


export type PersonSnapshot = {
    stats: StatsSnapshot;
    profile_picture: ProfilePictureSnapshot;
    personality: PersonalitySnapshot;
    person_id: number;
    name: string;
    gender: string;
    talent_grade: TalentGrade;
    assigned_skill: SkillSetSnapshot;
    updated: number;
    team: number | null;
    joined_tick: number;
    joined_gamedate: SimDate;
};
export type TalentGrade =
    | "Basic"
    | "Apt"
    | "Sharp"
    | "Gifted"
    | "Brilliant"
    | "Exceptional";

export type PersonSnapshotWithTotal = PersonSnapshot & {
    total_points : number,
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


export function assignPersonToTeam(personId:number, teamId:number ){
    console.log(personId, teamId)
    if(teamId === -1 ){
        invoke("unassign_team", {personId});

    }else{
        invoke("assign_person_to_team", {personId, teamId});
    }

}

