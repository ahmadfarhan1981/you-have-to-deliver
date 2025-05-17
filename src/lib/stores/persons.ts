// src/lib/stores/persons.ts
import { writable, derived } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

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
    person_id: number;
    name: string;
    gender: string;
};

// Exposed array for easy iteration
export  const personArray = writable<PersonSnapshot[]>([]);

// Exposed map for fast lookup by ID
export const persons = derived(personArray, ($array) => {
    const map = new Map<number, PersonSnapshot>();
    for (const p of $array) {
        map.set(p.person_id, p);
    }
    return map;
});

async function pullPersons() {
    async function fetch() {
        try {
            const result = await invoke<Record<number, PersonSnapshot>>("get_persons");
            personArray.set(Object.values(result));
        } catch (err) {
            console.error("Failed to poll persons:", err);
        }
    }

    await fetch(); // Initial call
    const interval = setInterval(fetch, 5000);
    return () => clearInterval(interval);
}

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
    return { fileName, position };
}
await pullPersons();
