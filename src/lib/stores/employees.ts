// src/lib/stores/employees.ts
import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export type StatsSnapshot = {
    // Cognition
    judgement: number;
    creativity: number;

    // Perception
    systems: number;
    precision: number;

    // Drive
    focus: number;
    discipline: number;

    // Social
    empathy: number;
    communication: number;

    // Defense
    resilience: number;
    adaptability: number;
};

export type ProfilePictureSnapshot = {
    gender: string;
    category: number; // i8 in Rust maps to number in TS (valid range: -128 to 127)
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

export const employees = writable<PersonSnapshot[]>([]);

async function pollEmployees() {
    async function fetch() {
        try {
            const result = await invoke<Record<number, PersonSnapshot>>("get_persons");
            employees.set(Object.values(result)); // flatten if returned as a HashMap
        } catch (err) {
            console.error("Failed to poll persons:", err);
        }
    }

    await fetch(); // Initial call
    const interval = setInterval(fetch, 5000);

    return () => clearInterval(interval); // For manual teardown if needed
}

await pollEmployees(); // Start polling immediately when the module is loaded
