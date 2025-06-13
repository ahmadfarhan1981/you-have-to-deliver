import { writable, type Writable } from 'svelte/store';

export const personDebugDisplays: Writable<AllPersonDebugDisplays> = writable({});

export const personDebugSnapshotEventName = "debug_display_snapshot";



export type DebugDisplayEntry = {
    label: string;
    value: string;
};

// This will represent the HashMap<PersonId, Vec<DebugDisplayEntry>> from Rust
// In TypeScript, a Record is a common way to type this.
export type AllPersonDebugDisplays = Record<number, DebugDisplayEntry[]>;