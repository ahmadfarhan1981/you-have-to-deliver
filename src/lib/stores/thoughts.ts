import { writable } from 'svelte/store';
import type { SimDate } from './simDate';

export const thoughtsSnapshotEventName = 'thoughts_snapshot';

export type Thought = {
    sim_date: SimDate;
    context: any;
};

export type ThoughtsSnapshot = {
    person_id: number;
    thoughts: Thought[];
};

export const thoughtsSnapshots = writable<ThoughtsSnapshot[]>([]);
