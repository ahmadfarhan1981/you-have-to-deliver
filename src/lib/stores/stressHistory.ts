import {writable} from "svelte/store";

export type StressHistorySnapshot = {
    person_id: number;

    raw_input_history: number[];
    felt_stress_history: number[];

    average_raw_input: number;
    average_felt_stress: number;

    max_raw_input: number;
    max_felt_stress: number;
};
export const stressHistorySnapshots = writable<StressHistorySnapshot[]>([]);

export const stressHistoryEventName = 'stress_history_snapshot'; // match your backend emit