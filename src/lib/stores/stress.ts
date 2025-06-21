import {writable} from "svelte/store";

export const stressSnapshotEventName ="stress_snapshot";
export type StressSnapshot = {
    person_id: number;
    current: number;
    daily_accumulator: number;

    average_raw_input: number;
    average_felt_stress: number;

    max_recent_raw_input: number;
    max_recent_felt_stress: number;

    baseline_tolerance: number;
    surge_tolerance: number;
    recovery_rate: number;
};

export const stressSnapshots = writable<StressSnapshot[]>([
]);
