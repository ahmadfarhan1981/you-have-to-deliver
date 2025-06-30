import { writable } from "svelte/store";

export const workingHoursEventName = "monthly_availability_snapshot";

export type WorkingHoursSnapshot = {
  person_id: number;
  days: boolean[][]; // 28 days with 96 slots each
};

export const workingHours = writable<WorkingHoursSnapshot[]>([]);
